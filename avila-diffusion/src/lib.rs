//! Avila Diffusion v4.0 - Stable Diffusion COMPLETO
//!
//! Sistema completo de geração com modelos SafeTensors reais

pub mod clip; // Text encoder
pub mod history; // Persistent history storage
pub mod native_gen; // Gerador leve (fallback)
pub mod pipeline; // Pipeline completo
pub mod safetensors_loader; // Carregador de modelos
pub mod scheduler; // Processos de difusão
pub mod server_native; // HTTP server
pub mod unet; // Rede de difusão
pub mod vae; // Encoder/Decoder
pub mod webui_client; // Integração com WebUI
                      // Exports públicos
pub use native_gen::{NativeImage, UltraLightGenerator};
pub use pipeline::StableDiffusionPipeline;
pub use safetensors_loader::SafeTensorsLoader;
pub use server_native::run_server;
pub use webui_client::WebUIClient;

/// Motor principal - suporta dois modos:
/// 1. Modo COMPLETO: Carrega modelo .safetensors real
/// 2. Modo LEVE: Gerador procedural nativo
pub struct AvilaDiffusion {
    mode: GenerationMode,
}

enum GenerationMode {
    Full(StableDiffusionPipeline),
    Light(UltraLightGenerator),
}

impl AvilaDiffusion {
    /// Cria motor com modelo completo
    pub fn with_model(model_path: &str, steps: usize) -> Result<Self, String> {
        println!("\n🚀 Inicializando Avila Diffusion COMPLETO...");
        let pipeline = StableDiffusionPipeline::load(model_path, steps)?;

        Ok(Self {
            mode: GenerationMode::Full(pipeline),
        })
    }

    /// Cria motor leve (sem modelo, apenas procedural)
    pub fn new() -> Result<Self, String> {
        let default_steps = std::env::var("AVILA_DIFFUSION_DEFAULT_STEPS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(20);

        match StableDiffusionPipeline::from_webui(default_steps) {
            Ok(pipeline) => {
                println!("╔═══════════════════════════════════════════════════════════╗");
                println!("║     ✅ AVILA DIFFUSION v4.0 - BACKEND WEBUI             ║");
                println!("║     🎯 Alta qualidade via Stable Diffusion real         ║");
                println!("╚═══════════════════════════════════════════════════════════╝");
                return Ok(Self {
                    mode: GenerationMode::Full(pipeline),
                });
            }
            Err(err) => {
                println!("⚠️  WebUI não disponível ({}) - usando modo leve", err);
            }
        }

        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        let generator = UltraLightGenerator::new(seed);

        println!("╔═══════════════════════════════════════════════════════════╗");
        println!("║     ✅ AVILA DIFFUSION v4.0 - MODO LEVE                ║");
        println!("║     ⚡ Geração procedural ultra-rápida                  ║");
        println!("╚═══════════════════════════════════════════════════════════╝");

        Ok(Self {
            mode: GenerationMode::Light(generator),
        })
    }

    /// Gera imagem a partir de prompt
    pub fn text_to_image(
        &mut self,
        prompt: &str,
        negative_prompt: Option<&str>,
        width: usize,
        height: usize,
        steps: usize,
        guidance_scale: f32,
        seed: Option<u64>,
    ) -> Result<NativeImage, String> {
        match &mut self.mode {
            GenerationMode::Full(pipeline) => {
                let seed_value = seed.unwrap_or_else(|| {
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs()
                });

                pipeline.generate(
                    prompt,
                    negative_prompt,
                    width,
                    height,
                    seed_value,
                    guidance_scale,
                    steps,
                )
            }
            GenerationMode::Light(generator) => {
                println!(
                    "🎨 Gerando (modo leve): '{}' ({}x{})",
                    prompt, width, height
                );
                let img = generator.generate(prompt, width, height);
                println!("✅ Concluído!");
                Ok(img)
            }
        }
    }

    /// Informa o modo atual de geração ("full" ou "light").
    pub fn mode_name(&self) -> &'static str {
        match &self.mode {
            GenerationMode::Full(_) => "full",
            GenerationMode::Light(_) => "light",
        }
    }
}
