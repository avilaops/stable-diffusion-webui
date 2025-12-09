//! Integração com a Stable Diffusion WebUI (AUTOMATIC1111)
//! Permite gerar imagens de alta qualidade utilizando o backend Python.

use crate::native_gen::NativeImage;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_SAMPLER: &str = "Euler a";
const HEALTH_ENDPOINT: &str = "/sdapi/v1/sd-models";
const TXT2IMG_ENDPOINT: &str = "/sdapi/v1/txt2img";

/// Cliente HTTP para comunicação com a WebUI
pub struct WebUIClient {
    client: Client,
    base_url: String,
}

impl WebUIClient {
    /// Cria cliente a partir de uma URL base (ex: http://127.0.0.1:7860)
    pub fn new<S: Into<String>>(base_url: S) -> Result<Self, String> {
        let base_url = base_url.into().trim_end_matches('/').to_string();

        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| format!("Erro ao criar cliente HTTP: {}", e))?;

        let instance = Self { client, base_url };
        instance.check_connection()?;
        Ok(instance)
    }

    /// Usa variáveis de ambiente para configurar o cliente
    pub fn from_env() -> Result<Self, String> {
        let url = std::env::var("AVILA_DIFFUSION_WEBUI_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:7860".to_string());
        Self::new(url)
    }

    /// Retorna a URL base em uso
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Verifica se o backend está acessível
    fn check_connection(&self) -> Result<(), String> {
        let url = format!("{}{}", self.base_url, HEALTH_ENDPOINT);
        let response = self
            .client
            .get(&url)
            .send()
            .map_err(|e| format!("Falha ao conectar em {}: {}", url, e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(format!(
                "Backend WebUI respondeu com status {}",
                response.status()
            ))
        }
    }

    /// Gera imagem via endpoint txt2img
    pub fn generate(
        &self,
        prompt: &str,
        negative_prompt: Option<&str>,
        width: usize,
        height: usize,
        steps: usize,
        guidance_scale: f32,
        seed: Option<u64>,
    ) -> Result<NativeImage, String> {
        let payload = Txt2ImgRequest {
            prompt,
            negative_prompt,
            width,
            height,
            steps: steps.max(1),
            cfg_scale: guidance_scale,
            sampler_index: DEFAULT_SAMPLER,
            seed: seed.map(|s| s as i64).unwrap_or(-1),
        };

        let url = format!("{}{}", self.base_url, TXT2IMG_ENDPOINT);
        let response = self
            .client
            .post(&url)
            .json(&payload)
            .send()
            .map_err(|e| format!("Erro ao chamar {}: {}", url, e))?
            .error_for_status()
            .map_err(|e| format!("Backend retornou erro HTTP: {}", e))?;

        let mut parsed: Txt2ImgResponse = response
            .json()
            .map_err(|e| format!("Erro ao decodificar resposta JSON: {}", e))?;

        let first = parsed
            .images
            .pop()
            .ok_or_else(|| "Resposta sem imagens".to_string())?;

        let png_bytes = decode_base64_image(&first)?;
        let native = decode_png_to_native(&png_bytes)?;

        Ok(native)
    }
}

#[derive(Serialize)]
struct Txt2ImgRequest<'a> {
    prompt: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    negative_prompt: Option<&'a str>,
    width: usize,
    height: usize,
    steps: usize,
    cfg_scale: f32,
    sampler_index: &'a str,
    seed: i64,
}

#[derive(Deserialize)]
struct Txt2ImgResponse {
    #[serde(default)]
    images: Vec<String>,
    #[serde(default)]
    parameters: serde_json::Value,
    #[serde(default)]
    info: serde_json::Value,
}

fn decode_base64_image(data: &str) -> Result<Vec<u8>, String> {
    let cleaned = if let Some(idx) = data.find("base64,") {
        &data[idx + "base64,".len()..]
    } else {
        data
    };

    BASE64
        .decode(cleaned.trim())
        .map_err(|e| format!("Erro ao decodificar imagem base64: {}", e))
}

fn decode_png_to_native(bytes: &[u8]) -> Result<NativeImage, String> {
    let dyn_img =
        image::load_from_memory(bytes).map_err(|e| format!("Erro ao decodificar PNG: {}", e))?;
    let rgb = dyn_img.to_rgb8();
    let (width, height) = rgb.dimensions();
    let data = rgb.into_raw();

    NativeImage::from_rgb_bytes(width as usize, height as usize, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageEncoder;

    #[test]
    fn test_decode_base64_image() {
        let png = image::RgbImage::from_pixel(2, 2, image::Rgb([255, 0, 0]));
        let mut buffer = Vec::new();
        {
            let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
            encoder
                .write_image(png.as_raw(), 2, 2, image::ColorType::Rgb8)
                .unwrap();
        }

        let encoded = format!("data:image/png;base64,{}", BASE64.encode(buffer));
        let decoded = decode_base64_image(&encoded).unwrap();
        assert!(decoded.len() > 0);
    }

    #[test]
    fn test_native_conversion() {
        let png = image::RgbImage::from_pixel(4, 4, image::Rgb([10, 20, 30]));
        let mut buffer = Vec::new();
        {
            let encoder = image::codecs::png::PngEncoder::new(&mut buffer);
            encoder
                .write_image(png.as_raw(), 4, 4, image::ColorType::Rgb8)
                .unwrap();
        }

        let native = decode_png_to_native(&buffer).unwrap();
        assert_eq!(native.width, 4);
        assert_eq!(native.height, 4);
        assert_eq!(native.data.len(), 4 * 4 * 3);
        assert_eq!(native.data[0], 10);
        assert_eq!(native.data[1], 20);
        assert_eq!(native.data[2], 30);
    }
}
