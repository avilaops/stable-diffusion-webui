use crate::safetensors_loader::{ModelInfo, ModelType};

/// UNet simplificado - núcleo do modelo de difusão (mock)
pub struct UNet {
    model_type: ModelType,
}

impl UNet {
    /// Cria UNet a partir do modelo carregado
    pub fn from_model(model: &ModelInfo) -> Result<Self, String> {
        let (in_channels, out_channels, model_channels) = match model.model_type {
            ModelType::SD15 | ModelType::SD20 => (4, 4, 320),
            ModelType::SDXL => (4, 4, 320),
            ModelType::Unknown => (4, 4, 320),
        };

        println!("🧠 Inicializando UNet (mock)");
        println!("   In channels: {}", in_channels);
        println!("   Out channels: {}", out_channels);
        println!("   Model channels: {}", model_channels);
        println!("   ⚠️  Usando implementação simplificada para testes");

        Ok(Self {
            model_type: model.model_type,
        })
    }

    /// Forward pass mock – aplica apenas uma transformação leve nos dados
    pub fn forward(
        &self,
        latents: &[f32],
        timestep: f32,
        text_embedding: &[f32],
        width: usize,
        height: usize,
    ) -> Vec<f32> {
        let latent_h = (height.max(1) / 8).max(1);
        let latent_w = (width.max(1) / 8).max(1);
        let latent_size = latent_h * latent_w * 4;

        println!(
            "   🔄 UNet mock forward: {}x{} latent (t={:.3})",
            latent_w, latent_h, timestep
        );

        let mut output = vec![0.0f32; latent_size];
        let copy_len = latents.len().min(latent_size);
        output[..copy_len].copy_from_slice(&latents[..copy_len]);

        let signal_scale = 1.0 - timestep;
        if text_embedding.is_empty() {
            for value in &mut output {
                *value *= signal_scale;
            }
        } else {
            let embedding_len = text_embedding.len();
            for (i, value) in output.iter_mut().enumerate() {
                let influence = text_embedding[i % embedding_len] * 0.1;
                *value = *value * signal_scale + influence;
            }
        }

        output
    }

    /// Número de parâmetros do modelo (mock)
    pub fn parameter_count(&self) -> usize {
        // Mantido para compatibilidade com os logs
        match self.model_type {
            ModelType::SDXL => 2_000_000_000,
            ModelType::SD20 => 1_300_000_000,
            ModelType::SD15 => 860_000_000,
            ModelType::Unknown => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_unet_forward() {
        let latents = vec![0.5f32; 64 * 64 * 4];
        let text_emb = vec![0.1f32; 768];

        let model_info = ModelInfo {
            model_type: ModelType::SD15,
            architecture: String::new(),
            tensors: HashMap::new(),
            metadata: HashMap::new(),
        };

        let unet = UNet::from_model(&model_info).unwrap();
        let output = unet.forward(&latents, 0.5, &text_emb, 512, 512);
        assert_eq!(output.len(), 64 * 64 * 4);
    }
}
