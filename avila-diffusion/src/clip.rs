//! CLIP - Text Encoder para embeddings
//!
//! Converte prompts de texto em vetores de 768 dimensões

use crate::safetensors_loader::{ModelInfo, Tensor};
use std::collections::HashMap;

const PAD_TOKEN_ID: usize = 0;
const CLS_TOKEN_ID: usize = 1;
const SEP_TOKEN_ID: usize = 2;
const UNK_TOKEN_ID: usize = 3;

const NUM_TRANSFORMER_LAYERS: usize = 4;
const NUM_HEADS: usize = 12;

/// Deterministic hash (FNV-1a) para strings
fn hash_string(input: &str) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;
    let mut hash = FNV_OFFSET;
    for byte in input.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9e3779b97f4a7c15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
    z ^ (z >> 31)
}

fn random_uniform(state: &mut u64, scale: f32) -> f32 {
    let value = splitmix64(state);
    let normalized = (value >> 40) as f32 / (1u32 << 24) as f32; // [0, 1)
    (normalized * 2.0 - 1.0) * scale
}

fn generate_matrix(seed: u64, rows: usize, cols: usize, scale: f32) -> Vec<f32> {
    let mut state = seed;
    (0..rows * cols)
        .map(|_| random_uniform(&mut state, scale))
        .collect()
}

fn generate_vector(seed: u64, size: usize, scale: f32) -> Vec<f32> {
    let mut state = seed;
    (0..size)
        .map(|_| random_uniform(&mut state, scale))
        .collect()
}

fn layer_norm(input: &[f32], gamma: &[f32], beta: &[f32]) -> Vec<f32> {
    let mean = input.iter().copied().sum::<f32>() / input.len() as f32;
    let variance = input
        .iter()
        .map(|x| {
            let diff = x - mean;
            diff * diff
        })
        .sum::<f32>()
        / input.len() as f32;
    let epsilon = 1e-5f32;
    let denom = (variance + epsilon).sqrt();

    input
        .iter()
        .enumerate()
        .map(|(i, x)| ((x - mean) / denom) * gamma[i] + beta[i])
        .collect()
}

fn gelu(x: f32) -> f32 {
    0.5 * x * (1.0 + (std::f32::consts::FRAC_2_SQRT_PI * (x + 0.044715 * x.powi(3))).tanh())
}

fn mat_vec_mul(matrix: &[f32], rows: usize, cols: usize, vector: &[f32]) -> Vec<f32> {
    let mut output = vec![0.0f32; rows];
    for r in 0..rows {
        let mut acc = 0.0;
        let base = r * cols;
        for c in 0..cols {
            acc += matrix[base + c] * vector[c];
        }
        output[r] = acc;
    }
    output
}

fn mat_vec_mul_bias(
    matrix: &[f32],
    rows: usize,
    cols: usize,
    vector: &[f32],
    bias: &[f32],
) -> Vec<f32> {
    let mut output = mat_vec_mul(matrix, rows, cols, vector);
    for (o, b) in output.iter_mut().zip(bias) {
        *o += *b;
    }
    output
}

fn sinusoidal_positional_embedding(position: usize, dim: usize) -> Vec<f32> {
    let mut embedding = vec![0.0f32; dim];
    for i in 0..(dim / 2) {
        let angle = position as f32 / (10000f32).powf((2 * i) as f32 / dim as f32);
        embedding[2 * i] = angle.sin();
        embedding[2 * i + 1] = angle.cos();
    }
    embedding
}

struct ClipTokenizer {
    max_length: usize,
    vocab_size: usize,
}

impl ClipTokenizer {
    fn new(max_length: usize, vocab_size: usize) -> Self {
        Self {
            max_length,
            vocab_size,
        }
    }

    fn token_to_id(&self, token: &str) -> usize {
        match token {
            "[PAD]" => PAD_TOKEN_ID,
            "[CLS]" => CLS_TOKEN_ID,
            "[SEP]" => SEP_TOKEN_ID,
            "[UNK]" => UNK_TOKEN_ID,
            _ => {
                let hash = hash_string(token);
                let available = self.vocab_size.saturating_sub(4).max(1);
                (hash as usize % available) + 4
            }
        }
    }

    fn normalize_token(token: &str) -> Option<String> {
        let cleaned: String = token
            .chars()
            .filter(|c| c.is_alphanumeric() || c == &'#')
            .collect();
        if cleaned.is_empty() {
            None
        } else {
            Some(cleaned.to_lowercase())
        }
    }

    fn tokenize(&self, prompt: &str) -> Vec<usize> {
        let mut ids = Vec::with_capacity(self.max_length);
        ids.push(CLS_TOKEN_ID);

        for token in prompt.split_whitespace() {
            if let Some(clean) = Self::normalize_token(token) {
                if ids.len() + 1 >= self.max_length {
                    break;
                }
                ids.push(self.token_to_id(&clean));
            }
        }

        if ids.len() < self.max_length {
            ids.push(SEP_TOKEN_ID);
        }

        while ids.len() < self.max_length {
            ids.push(PAD_TOKEN_ID);
        }

        ids
    }
}

struct TransformerLayer {
    embedding_dim: usize,
    num_heads: usize,
    head_dim: usize,
    intermediate_dim: usize,
    w_q: Vec<f32>,
    w_k: Vec<f32>,
    w_v: Vec<f32>,
    w_o: Vec<f32>,
    w_ff1: Vec<f32>,
    b_ff1: Vec<f32>,
    w_ff2: Vec<f32>,
    b_ff2: Vec<f32>,
    ln1_gamma: Vec<f32>,
    ln1_beta: Vec<f32>,
    ln2_gamma: Vec<f32>,
    ln2_beta: Vec<f32>,
}

impl TransformerLayer {
    fn new(layer_index: usize, embedding_dim: usize, num_heads: usize) -> Self {
        let head_dim = embedding_dim / num_heads;
        let intermediate_dim = embedding_dim * 4;
        let base_seed = hash_string(&format!("clip_layer_{}", layer_index));

        let w_q = generate_matrix(base_seed ^ 0x01, embedding_dim, embedding_dim, 0.08);
        let w_k = generate_matrix(base_seed ^ 0x21, embedding_dim, embedding_dim, 0.08);
        let w_v = generate_matrix(base_seed ^ 0x41, embedding_dim, embedding_dim, 0.08);
        let w_o = generate_matrix(base_seed ^ 0x61, embedding_dim, embedding_dim, 0.08);

        let w_ff1 = generate_matrix(base_seed ^ 0x81, intermediate_dim, embedding_dim, 0.02);
        let b_ff1 = generate_vector(base_seed ^ 0xa1, intermediate_dim, 0.02);

        let w_ff2 = generate_matrix(base_seed ^ 0xc1, embedding_dim, intermediate_dim, 0.02);
        let b_ff2 = generate_vector(base_seed ^ 0xe1, embedding_dim, 0.02);

        let ln1_gamma = vec![1.0f32; embedding_dim];
        let ln1_beta = vec![0.0f32; embedding_dim];
        let ln2_gamma = vec![1.0f32; embedding_dim];
        let ln2_beta = vec![0.0f32; embedding_dim];

        Self {
            embedding_dim,
            num_heads,
            head_dim,
            intermediate_dim,
            w_q,
            w_k,
            w_v,
            w_o,
            w_ff1,
            b_ff1,
            w_ff2,
            b_ff2,
            ln1_gamma,
            ln1_beta,
            ln2_gamma,
            ln2_beta,
        }
    }

    fn apply(&self, hidden_states: &mut [Vec<f32>]) {
        let seq_len = hidden_states.len();

        let mut normed = Vec::with_capacity(seq_len);
        for token in hidden_states.iter() {
            normed.push(layer_norm(token, &self.ln1_gamma, &self.ln1_beta));
        }

        let mut queries = Vec::with_capacity(seq_len);
        let mut keys = Vec::with_capacity(seq_len);
        let mut values = Vec::with_capacity(seq_len);

        for token in normed.iter() {
            queries.push(mat_vec_mul(
                &self.w_q,
                self.embedding_dim,
                self.embedding_dim,
                token,
            ));
            keys.push(mat_vec_mul(
                &self.w_k,
                self.embedding_dim,
                self.embedding_dim,
                token,
            ));
            values.push(mat_vec_mul(
                &self.w_v,
                self.embedding_dim,
                self.embedding_dim,
                token,
            ));
        }

        let mut attention_context = vec![vec![0.0f32; self.embedding_dim]; seq_len];

        for head in 0..self.num_heads {
            let start = head * self.head_dim;
            let end = start + self.head_dim;

            for i in 0..seq_len {
                let q = &queries[i][start..end];
                let mut scores = vec![0.0f32; seq_len];
                for j in 0..seq_len {
                    let k = &keys[j][start..end];
                    let mut dot = 0.0f32;
                    for d in 0..self.head_dim {
                        dot += q[d] * k[d];
                    }
                    scores[j] = dot / (self.head_dim as f32).sqrt();
                }

                let max_score = scores.iter().copied().fold(f32::NEG_INFINITY, f32::max);
                let mut sum = 0.0f32;
                for score in scores.iter_mut() {
                    *score = (*score - max_score).exp();
                    sum += *score;
                }
                if sum > 0.0 {
                    for score in scores.iter_mut() {
                        *score /= sum;
                    }
                }

                for j in 0..seq_len {
                    let v = &values[j][start..end];
                    for d in 0..self.head_dim {
                        attention_context[i][start + d] += scores[j] * v[d];
                    }
                }
            }
        }

        for (hidden, context) in hidden_states.iter_mut().zip(attention_context.iter()) {
            let projection =
                mat_vec_mul(&self.w_o, self.embedding_dim, self.embedding_dim, context);
            for (h, p) in hidden.iter_mut().zip(projection.iter()) {
                *h += *p;
            }
        }

        let mut normed_post = Vec::with_capacity(seq_len);
        for token in hidden_states.iter() {
            normed_post.push(layer_norm(token, &self.ln2_gamma, &self.ln2_beta));
        }

        for (hidden, normed_token) in hidden_states.iter_mut().zip(normed_post.iter()) {
            let mut ff = mat_vec_mul_bias(
                &self.w_ff1,
                self.intermediate_dim,
                self.embedding_dim,
                normed_token,
                &self.b_ff1,
            );
            for val in ff.iter_mut() {
                *val = gelu(*val);
            }

            let ff_out = mat_vec_mul_bias(
                &self.w_ff2,
                self.embedding_dim,
                self.intermediate_dim,
                &ff,
                &self.b_ff2,
            );

            for (h, o) in hidden.iter_mut().zip(ff_out.iter()) {
                *h += *o;
            }
        }
    }
}

/// CLIP Text Encoder simplificado
pub struct CLIPTextEncoder {
    weights: HashMap<String, Tensor>,
    vocab_size: usize,
    embedding_dim: usize,
    max_length: usize,
    tokenizer: ClipTokenizer,
    layers: Vec<TransformerLayer>,
}

impl CLIPTextEncoder {
    /// Cria CLIP a partir do modelo carregado
    pub fn from_model(model: &ModelInfo) -> Result<Self, String> {
        println!("📝 Inicializando CLIP Text Encoder:");

        let mut weights = HashMap::new();

        // Extrair weights do CLIP
        for (key, tensor) in &model.tensors {
            if key.contains("cond_stage_model") || key.contains("text_model") {
                weights.insert(key.clone(), tensor.clone());
            }
        }

        println!("   Weights: {}", weights.len());

        // Parâmetros padrão do CLIP usado no SD 1.x
        let vocab_size = 49408;
        let embedding_dim = 768;
        let max_length = 77;

        println!("   Vocab size: {}", vocab_size);
        println!("   Embedding dim: {}", embedding_dim);

        let tokenizer = ClipTokenizer::new(max_length, vocab_size);

        let mut layers = Vec::with_capacity(NUM_TRANSFORMER_LAYERS);
        for layer_idx in 0..NUM_TRANSFORMER_LAYERS {
            layers.push(TransformerLayer::new(layer_idx, embedding_dim, NUM_HEADS));
        }

        Ok(Self {
            weights,
            vocab_size,
            embedding_dim,
            max_length,
            tokenizer,
            layers,
        })
    }

    /// Encode texto para embedding
    pub fn encode(&self, prompt: &str) -> Vec<f32> {
        println!("   📝 Encoding: \"{}\"", prompt);

        let token_ids = self.tokenizer.tokenize(prompt);
        let mut hidden_states: Vec<Vec<f32>> = token_ids
            .iter()
            .enumerate()
            .map(|(position, token_id)| {
                let token_seed = hash_string(&format!("token_{}", token_id));
                let mut embedding = generate_vector(token_seed, self.embedding_dim, 0.15);
                let positional = sinusoidal_positional_embedding(position, self.embedding_dim);
                for (e, p) in embedding.iter_mut().zip(positional.iter()) {
                    *e += *p;
                }
                embedding
            })
            .collect();

        for layer in &self.layers {
            layer.apply(&mut hidden_states);
        }

        let mut embedding = hidden_states
            .get(0)
            .cloned()
            .unwrap_or_else(|| vec![0.0f32; self.embedding_dim]);

        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for value in embedding.iter_mut() {
                *value /= norm;
            }
        }

        println!(
            "   ✅ Embedding: {} dims (norm={:.3})",
            embedding.len(),
            norm
        );

        embedding
    }

    /// Encode prompt negativo (para guidance)
    pub fn encode_negative(&self) -> Vec<f32> {
        self.encode("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clip_encode() {
        let clip = CLIPTextEncoder {
            weights: HashMap::new(),
            vocab_size: 49408,
            embedding_dim: 768,
            max_length: 77,
            tokenizer: ClipTokenizer::new(77, 49408),
            layers: (0..NUM_TRANSFORMER_LAYERS)
                .map(|idx| TransformerLayer::new(idx, 768, NUM_HEADS))
                .collect(),
        };

        let prompt = "a beautiful sunset over mountains";
        let embedding = clip.encode(prompt);

        assert_eq!(embedding.len(), 768);

        // Verificar normalização
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((norm - 1.0).abs() < 0.1);
    }
}
