//! VAE - Variational Autoencoder para Stable Diffusion
//!
//! Converte entre pixel space (512x512) e latent space (64x64)

use crate::safetensors_loader::{ModelInfo, Tensor};
use std::collections::HashMap;

/// VAE - Encoder/Decoder para latent space
pub struct VAE {
    encoder_weights: HashMap<String, Tensor>,
    decoder_weights: HashMap<String, Tensor>,
    scale_factor: f32, // Geralmente 0.18215 para SD 1.x
}

impl VAE {
    /// Cria VAE a partir do modelo carregado
    pub fn from_model(model: &ModelInfo) -> Result<Self, String> {
        println!("🎨 Inicializando VAE:");

        let mut encoder_weights = HashMap::new();
        let mut decoder_weights = HashMap::new();

        // Separar weights do encoder e decoder
        for (key, tensor) in &model.tensors {
            if key.contains("first_stage_model.encoder") {
                encoder_weights.insert(key.clone(), tensor.clone());
            } else if key.contains("first_stage_model.decoder") {
                decoder_weights.insert(key.clone(), tensor.clone());
            }
        }

        println!("   Encoder weights: {}", encoder_weights.len());
        println!("   Decoder weights: {}", decoder_weights.len());

        // Scale factor padrão do SD
        let scale_factor = 0.18215;

        Ok(Self {
            encoder_weights,
            decoder_weights,
            scale_factor,
        })
    }

    /// Encode: pixels (3xHxW) -> latents (4x(H/8)x(W/8))
    pub fn encode(&self, pixels: &[f32], width: usize, height: usize) -> Vec<f32> {
        let latent_h = height / 8;
        let latent_w = width / 8;
        let latent_size = 4 * latent_h * latent_w;

        println!(
            "   🔽 VAE Encode: {}x{} -> {}x{} latent",
            width, height, latent_w, latent_h
        );

        // VERSÃO SIMPLIFICADA: Downsampling + normalização
        // Uma implementação real usaria as convoluções do encoder

        let mut latents = vec![0.0f32; latent_size];

        // Downsampling simples (média de patches 8x8)
        for lat_y in 0..latent_h {
            for lat_x in 0..latent_w {
                for c in 0..4 {
                    let mut sum = 0.0;
                    let mut count = 0;

                    // Patch 8x8 na imagem original
                    for py in 0..8 {
                        for px in 0..8 {
                            let img_y = lat_y * 8 + py;
                            let img_x = lat_x * 8 + px;

                            if img_y < height && img_x < width {
                                // Mapear para canal RGB (c % 3)
                                let rgb_c = c % 3;
                                let idx = (img_y * width + img_x) * 3 + rgb_c;

                                if idx < pixels.len() {
                                    sum += pixels[idx];
                                    count += 1;
                                }
                            }
                        }
                    }

                    // Média e normalização
                    let avg = if count > 0 { sum / count as f32 } else { 0.0 };
                    let lat_idx = (c * latent_h * latent_w) + (lat_y * latent_w) + lat_x;

                    if lat_idx < latent_size {
                        // Normalizar para [-1, 1] e aplicar scale factor
                        latents[lat_idx] = (avg * 2.0 - 1.0) * self.scale_factor;
                    }
                }
            }
        }

        latents
    }

    /// Decode: latents (4x(H/8)x(W/8)) -> pixels (3xHxW)
    pub fn decode(&self, latents: &[f32], width: usize, height: usize) -> Vec<f32> {
        let latent_h = height / 8;
        let latent_w = width / 8;

        println!(
            "   🔼 VAE Decode: {}x{} latent -> {}x{}",
            latent_w, latent_h, width, height
        );

        // VERSÃO SIMPLIFICADA: Upsampling nearest neighbor + desnormalização
        // Uma implementação real usaria as transposed convolutions do decoder

        let mut pixels = vec![0.0f32; height * width * 3];

        for y in 0..height {
            for x in 0..width {
                let lat_y = y / 8;
                let lat_x = x / 8;

                for c in 0..3 {
                    // Pegar do latent correspondente
                    let lat_c = c % 4; // 4 canais no latent
                    let lat_idx = (lat_c * latent_h * latent_w) + (lat_y * latent_w) + lat_x;

                    let latent_val = if lat_idx < latents.len() {
                        latents[lat_idx]
                    } else {
                        0.0
                    };

                    // Desnormalizar: latent -> [0, 1]
                    let pixel_val = (latent_val / self.scale_factor + 1.0) * 0.5;

                    // Clampar [0, 1]
                    let clamped = pixel_val.max(0.0).min(1.0);

                    let pix_idx = (y * width + x) * 3 + c;
                    pixels[pix_idx] = clamped;
                }
            }
        }

        pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vae_roundtrip() {
        let vae = VAE {
            encoder_weights: HashMap::new(),
            decoder_weights: HashMap::new(),
            scale_factor: 0.18215,
        };

        // Imagem 512x512 RGB
        let pixels = vec![0.5f32; 512 * 512 * 3];

        let latents = vae.encode(&pixels, 512, 512);
        assert_eq!(latents.len(), 4 * 64 * 64);

        let reconstructed = vae.decode(&latents, 512, 512);
        assert_eq!(reconstructed.len(), 512 * 512 * 3);
    }
}
