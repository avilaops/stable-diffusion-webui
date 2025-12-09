//! Pipeline Stable Diffusion - Integração completa
//!
//! CLIP -> UNet -> Scheduler -> VAE

use crate::clip::CLIPTextEncoder;
use crate::native_gen::NativeImage;
use crate::safetensors_loader::{ModelInfo, SafeTensorsLoader};
use crate::scheduler::{Scheduler, SchedulerType};
use crate::unet::UNet;
use crate::vae::VAE;
use crate::webui_client::WebUIClient;

/// Pipeline completo de Stable Diffusion
pub struct StableDiffusionPipeline {
    backend: PipelineBackend,
    default_steps: usize,
}

enum PipelineBackend {
    Native(NativeComponents),
    WebUI(WebUIClient),
}

struct NativeComponents {
    model_info: ModelInfo,
    unet: UNet,
    vae: VAE,
    clip: CLIPTextEncoder,
    scheduler: Scheduler,
    scheduler_type: SchedulerType,
}

impl StableDiffusionPipeline {
    /// Carrega modelo e inicializa pipeline
    pub fn load(model_path: &str, num_steps: usize) -> Result<Self, String> {
        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  🚀 AVILA STABLE DIFFUSION - Carregamento Completo         ║");
        println!("╚══════════════════════════════════════════════════════════════╝\n");

        let steps = num_steps.max(1);

        // 1. Carregar modelo SafeTensors
        let loader = SafeTensorsLoader::new(model_path)?;
        let model_info = loader.load()?;

        // 2. Inicializar componentes
        let unet = UNet::from_model(&model_info)?;
        let vae = VAE::from_model(&model_info)?;
        let clip = CLIPTextEncoder::from_model(&model_info)?;
        let scheduler_type = SchedulerType::Euler;
        let scheduler = Scheduler::new(scheduler_type, steps);

        println!("\n✅ Pipeline inicializado com sucesso!");
        println!("   Modelo: {}", model_info.architecture);
        println!(
            "   Parâmetros UNet: {:.2}M",
            unet.parameter_count() as f64 / 1e6
        );
        println!("   Steps padrão: {}\n", steps);

        Ok(Self {
            backend: PipelineBackend::Native(NativeComponents {
                model_info,
                unet,
                vae,
                clip,
                scheduler,
                scheduler_type,
            }),
            default_steps: steps,
        })
    }

    /// Conecta ao backend WebUI (AUTOMATIC1111)
    pub fn from_webui(num_steps: usize) -> Result<Self, String> {
        let client = WebUIClient::from_env()?;
        let steps = num_steps.max(1);

        println!("\n╔══════════════════════════════════════════════════════════════╗");
        println!("║  🌐 AVILA STABLE DIFFUSION - WebUI Backend                ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!("   URL: {}", client.base_url());
        println!("   Steps padrão: {}\n", steps);

        Ok(Self {
            backend: PipelineBackend::WebUI(client),
            default_steps: steps,
        })
    }

    /// Gera imagem a partir de prompt
    pub fn generate(
        &mut self,
        prompt: &str,
        negative_prompt: Option<&str>,
        width: usize,
        height: usize,
        seed: u64,
        guidance_scale: f32,
        steps: usize,
    ) -> Result<NativeImage, String> {
        let steps_to_use = steps.max(1);
        self.default_steps = steps_to_use;

        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║  🎨 GERANDO IMAGEM                                          ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!("📝 Prompt: \"{}\"", prompt);
        if let Some(neg) = negative_prompt {
            if !neg.trim().is_empty() {
                println!("🚫 Negative prompt: \"{}\"", neg);
            }
        }
        println!("🎯 Tamanho: {}x{}", width, height);
        println!("🎲 Seed: {}", seed);
        println!("⚖️  Guidance: {:.1}", guidance_scale);
        println!("🔁 Steps: {}\n", steps_to_use);

        let image = match &mut self.backend {
            PipelineBackend::WebUI(client) => {
                println!("1️⃣ Solicitando geração ao backend WebUI...");
                let img = client.generate(
                    prompt,
                    negative_prompt,
                    width,
                    height,
                    steps_to_use,
                    guidance_scale,
                    Some(seed),
                )?;
                println!(
                    "✅ WebUI retornou imagem {}x{} ({} bytes)",
                    img.width,
                    img.height,
                    img.data.len()
                );
                img
            }
            PipelineBackend::Native(native) => {
                if native.scheduler.num_steps != steps_to_use {
                    println!("🔧 Ajustando scheduler para {} steps", steps_to_use);
                    native.scheduler = Scheduler::new(native.scheduler_type, steps_to_use);
                }

                println!("1️⃣ CLIP Text Encoding...");
                let text_embedding = native.clip.encode(prompt);
                let uncond_embedding = if let Some(neg) = negative_prompt {
                    native.clip.encode(neg)
                } else {
                    native.clip.encode_negative()
                };

                println!("\n2️⃣ Inicializando latent space...");
                let latent_h = height / 8;
                let latent_w = width / 8;
                let latent_size = 4 * latent_h * latent_w;
                let mut latents = Scheduler::generate_noise(latent_size, seed);

                println!(
                    "\n3️⃣ Processo de difusão ({} steps):",
                    native.scheduler.num_steps
                );

                for step in 0..native.scheduler.num_steps {
                    let timestep = step as f32 / native.scheduler.num_steps as f32;

                    if step % 5 == 0 || step == native.scheduler.num_steps - 1 {
                        let progress = (step as f32 / native.scheduler.num_steps as f32) * 100.0;
                        println!(
                            "   Step {}/{} ({:.1}%) - t={:.3}",
                            step + 1,
                            native.scheduler.num_steps,
                            progress,
                            timestep
                        );
                    }

                    let noise_pred_text =
                        native
                            .unet
                            .forward(&latents, timestep, &text_embedding, width, height);

                    let noise_pred_uncond = if guidance_scale > 1.0 {
                        native
                            .unet
                            .forward(&latents, timestep, &uncond_embedding, width, height)
                    } else {
                        noise_pred_text.clone()
                    };

                    let noise_pred: Vec<f32> = if guidance_scale > 1.0 {
                        noise_pred_uncond
                            .iter()
                            .zip(noise_pred_text.iter())
                            .map(|(uncond, text)| uncond + guidance_scale * (text - uncond))
                            .collect()
                    } else {
                        noise_pred_text
                    };

                    latents = native.scheduler.step(&noise_pred, &latents, step);
                }

                println!("\n4️⃣ VAE Decoding para pixel space...");
                let pixels = native.vae.decode(&latents, width, height);

                println!("\n5️⃣ Finalizando imagem...");
                let mut image_data = Vec::with_capacity(width * height * 3);

                for pixel in pixels {
                    let byte = (pixel * 255.0).round().clamp(0.0, 255.0) as u8;
                    image_data.push(byte);
                }

                NativeImage {
                    width,
                    height,
                    data: image_data,
                }
            }
        };

        println!("\n✅ GERAÇÃO CONCLUÍDA!");
        println!("   Dimensões: {}x{}", image.width, image.height);
        println!("   Tamanho: {:.2} MB\n", image.data.len() as f64 / 1e6);

        Ok(image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Requer modelo real
    fn test_pipeline_load() {
        let model_path =
            "d:\\stable-diffusion-webui\\models\\Stable-diffusion\\dreamshaper_8.safetensors";

        if std::path::Path::new(model_path).exists() {
            let result = StableDiffusionPipeline::load(model_path, 20);
            assert!(result.is_ok());
        }
    }
}
