//! Carregador de modelos SafeTensors - Compatível com AUTOMATIC1111 WebUI
//!
//! Carrega modelos .safetensors do formato Stable Diffusion v1/v2/SDXL

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Tensor carregado da memória
#[derive(Debug, Clone)]
pub struct Tensor {
    pub shape: Vec<usize>,
    pub dtype: TensorType,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TensorType {
    F32,  // float32
    F16,  // float16
    BF16, // bfloat16
    I64,  // int64
    I32,  // int32
    U8,   // uint8
}

impl TensorType {
    pub fn size(&self) -> usize {
        match self {
            TensorType::F32 | TensorType::I32 => 4,
            TensorType::F16 | TensorType::BF16 => 2,
            TensorType::I64 => 8,
            TensorType::U8 => 1,
        }
    }
}

/// Informações do modelo carregado
#[derive(Debug)]
pub struct ModelInfo {
    pub model_type: ModelType,
    pub architecture: String,
    pub tensors: HashMap<String, Tensor>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModelType {
    SD15, // Stable Diffusion 1.5
    SD20, // Stable Diffusion 2.0/2.1
    SDXL, // Stable Diffusion XL
    Unknown,
}

impl ModelInfo {
    /// Detecta o tipo de modelo pelos tensors presentes
    pub fn detect_model_type(tensors: &HashMap<String, Tensor>) -> ModelType {
        // SDXL tem dimensões específicas
        if let Some(t) = tensors.get("model.diffusion_model.input_blocks.0.0.weight") {
            if t.shape.len() >= 2 && t.shape[1] == 4 {
                return ModelType::SDXL;
            }
        }

        // SD 2.x tem OpenCLIP em vez de CLIP
        if tensors.contains_key("cond_stage_model.model.text_projection") {
            return ModelType::SD20;
        }

        // SD 1.x é o padrão
        if tensors.contains_key(
            "cond_stage_model.transformer.text_model.embeddings.position_embedding.weight",
        ) {
            return ModelType::SD15;
        }

        ModelType::Unknown
    }
}

/// Carregador principal de modelos SafeTensors
pub struct SafeTensorsLoader {
    model_path: String,
}

impl SafeTensorsLoader {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let path_str = path.as_ref().to_string_lossy().to_string();

        if !path.as_ref().exists() {
            return Err(format!("Modelo não encontrado: {}", path_str));
        }

        Ok(Self {
            model_path: path_str,
        })
    }

    /// Carrega o modelo completo da memória
    pub fn load(&self) -> Result<ModelInfo, String> {
        println!("📦 Carregando modelo: {}", self.model_path);

        // Obter tamanho do arquivo
        let file_size = std::fs::metadata(&self.model_path)
            .map_err(|e| format!("Erro ao ler metadados: {}", e))?
            .len();

        println!("   Tamanho: {:.2} GB", file_size as f64 / 1e9);

        // Carregar arquivo completo na memória
        let mut file = File::open(&self.model_path).map_err(|e| format!("Erro ao abrir: {}", e))?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| format!("Erro ao ler: {}", e))?;

        // Usar API real do SafeTensors
        let tensors_file = safetensors::SafeTensors::deserialize(&buffer)
            .map_err(|e| format!("Erro ao carregar SafeTensors: {:?}", e))?;

        // Metadata (simpl ificado)
        let metadata = HashMap::new();

        // Carregar todos os tensors
        let mut tensors = HashMap::new();
        for (name, tensor_view) in tensors_file.tensors() {
            let shape = tensor_view.shape().to_vec();
            let dtype = match tensor_view.dtype() {
                safetensors::Dtype::F32 => TensorType::F32,
                safetensors::Dtype::F16 => TensorType::F16,
                safetensors::Dtype::BF16 => TensorType::BF16,
                safetensors::Dtype::I64 => TensorType::I64,
                safetensors::Dtype::I32 => TensorType::I32,
                safetensors::Dtype::U8 => TensorType::U8,
                _ => TensorType::F32,
            };

            let data = tensor_view.data().to_vec();

            tensors.insert(name.to_string(), Tensor { shape, dtype, data });
        }

        let model_type = ModelInfo::detect_model_type(&tensors);
        let architecture = match model_type {
            ModelType::SD15 => "Stable Diffusion 1.5",
            ModelType::SD20 => "Stable Diffusion 2.x",
            ModelType::SDXL => "Stable Diffusion XL",
            ModelType::Unknown => "Unknown",
        }
        .to_string();

        println!("   Arquitetura: {}", architecture);
        println!("   Tensors carregados: {}", tensors.len());

        Ok(ModelInfo {
            model_type,
            architecture,
            tensors,
            metadata,
        })
    }
    /// Parse do formato binário SafeTensors
    fn parse_safetensors(
        &self,
        _data: &[u8],
    ) -> Result<(HashMap<String, String>, HashMap<String, Tensor>), String> {
        Err("Parse manual de SafeTensors não implementado".to_string())
    }

    /// Parse simplificado do header JSON
    fn parse_header_json(
        &self,
        _json: &str,
    ) -> Result<
        (
            HashMap<String, String>,
            HashMap<String, (TensorType, Vec<usize>, (usize, usize))>,
        ),
        String,
    > {
        Ok((HashMap::new(), HashMap::new()))
    }

    /// Lista modelos disponíveis no diretório
    pub fn list_available_models<P: AsRef<Path>>(models_dir: P) -> Vec<String> {
        let mut models = Vec::new();

        if let Ok(entries) = std::fs::read_dir(models_dir) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".safetensors") {
                        models.push(name.to_string());
                    }
                }
            }
        }

        models
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_models() {
        let models = SafeTensorsLoader::list_available_models(
            "d:\\stable-diffusion-webui\\models\\Stable-diffusion",
        );
        println!("Modelos encontrados:");
        for model in &models {
            println!("  - {}", model);
        }
        assert!(!models.is_empty(), "Nenhum modelo encontrado");
    }
}
