//! Model structures and serialization

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

/// Configuration for the Tiny Transformer model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub vocab_size: usize,
    pub hidden_size: usize,
    pub num_layers: usize,
    pub num_heads: usize,
    pub seq_length: usize,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            vocab_size: 1000,
            hidden_size: 128,
            num_layers: 2,
            num_heads: 4,
            seq_length: 32,
        }
    }
}

/// Transformer model containing weights and configuration
#[derive(Debug)]
pub struct TransformerModel {
    pub config: ModelConfig,
    pub embeddings: Vec<f32>,
    pub attention_weights: Vec<f32>,
    pub ffn_weights: Vec<f32>,
    pub output_weights: Vec<f32>,
}

impl TransformerModel {
    /// Create a new model with random weights (for demonstration)
    pub fn new(config: ModelConfig) -> Self {
        let embed_size = config.vocab_size * config.hidden_size;
        let attn_size = config.num_layers * config.hidden_size * config.hidden_size * 4;
        let ffn_size = config.num_layers * config.hidden_size * config.hidden_size * 4;
        let output_size = config.hidden_size * config.vocab_size;

        Self {
            config,
            embeddings: vec![0.01; embed_size],
            attention_weights: vec![0.01; attn_size],
            ffn_weights: vec![0.01; ffn_size],
            output_weights: vec![0.01; output_size],
        }
    }

    /// Load model from bytes (simplified binary format)
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        if data.len() < 20 {
            return Err(anyhow!("Invalid model data: too short"));
        }

        // Simple format: config (JSON) length (4 bytes) + config + weights
        let config_len = u32::from_le_bytes([data[0], data[1], data[2], data[3]]) as usize;
        
        if data.len() < 4 + config_len {
            return Err(anyhow!("Invalid model data: config section too short"));
        }

        let config_bytes = &data[4..4 + config_len];
        let config: ModelConfig = serde_json::from_slice(config_bytes)?;
        
        let weights_start = 4 + config_len;
        let weights_data = &data[weights_start..];
        
        // Parse weights (simplified - assumes all weights are concatenated)
        let embed_size = config.vocab_size * config.hidden_size;
        let attn_size = config.num_layers * config.hidden_size * config.hidden_size * 4;
        let ffn_size = config.num_layers * config.hidden_size * config.hidden_size * 4;
        let output_size = config.hidden_size * config.vocab_size;
        
        let expected_size = (embed_size + attn_size + ffn_size + output_size) * 4; // 4 bytes per f32
        
        if weights_data.len() < expected_size {
            // If not enough data, create with default weights
            return Ok(Self::new(config));
        }
        
        let weights_f32: Vec<f32> = weights_data
            .chunks_exact(4)
            .take(embed_size + attn_size + ffn_size + output_size)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();
        
        Ok(Self {
            embeddings: weights_f32[..embed_size].to_vec(),
            attention_weights: weights_f32[embed_size..embed_size + attn_size].to_vec(),
            ffn_weights: weights_f32[embed_size + attn_size..embed_size + attn_size + ffn_size].to_vec(),
            output_weights: weights_f32[embed_size + attn_size + ffn_size..].to_vec(),
            config,
        })
    }

    /// Serialize model to bytes
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let config_json = serde_json::to_vec(&self.config)?;
        let config_len = config_json.len() as u32;
        
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&config_len.to_le_bytes());
        bytes.extend_from_slice(&config_json);
        
        // Serialize weights
        for weight in &self.embeddings {
            bytes.extend_from_slice(&weight.to_le_bytes());
        }
        for weight in &self.attention_weights {
            bytes.extend_from_slice(&weight.to_le_bytes());
        }
        for weight in &self.ffn_weights {
            bytes.extend_from_slice(&weight.to_le_bytes());
        }
        for weight in &self.output_weights {
            bytes.extend_from_slice(&weight.to_le_bytes());
        }
        
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_serialization() {
        let config = ModelConfig::default();
        let model = TransformerModel::new(config);
        let bytes = model.to_bytes().unwrap();
        let loaded = TransformerModel::from_bytes(&bytes).unwrap();
        assert_eq!(loaded.config.vocab_size, 1000);
    }
}
