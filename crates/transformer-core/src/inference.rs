//! Inference engine

use crate::{GpuContext, TransformerModel};
use anyhow::Result;

/// Run a forward pass through the transformer model
pub fn forward_pass(
    _context: &GpuContext,
    model: &TransformerModel,
    input: &[f32],
) -> Result<Vec<f32>> {
    // Create input tensor
    let input_len = input.len();
    let hidden_size = model.config.hidden_size;
    
    // Simple embedding lookup (simplified)
    let mut embedded = vec![0.0f32; input_len * hidden_size];
    for (i, &token_id) in input.iter().enumerate() {
        let token_id = token_id as usize % model.config.vocab_size;
        let start = token_id * hidden_size;
        let end = start + hidden_size;
        if end <= model.embeddings.len() {
            embedded[i * hidden_size..(i + 1) * hidden_size]
                .copy_from_slice(&model.embeddings[start..end]);
        }
    }

    // Run through transformer layers (simplified CPU version for now)
    let mut hidden_states = embedded;
    
    for _ in 0..model.config.num_layers {
        // Self-attention (simplified)
        hidden_states = self_attention(&hidden_states, hidden_size);
        
        // Feed-forward network (simplified)
        hidden_states = feed_forward(&hidden_states, hidden_size);
    }

    // Output projection (take first token for simplicity)
    let output = if hidden_states.len() >= hidden_size {
        project_output(&hidden_states[..hidden_size], &model.output_weights, model.config.vocab_size)
    } else {
        vec![0.0; model.config.vocab_size]
    };

    Ok(output)
}

/// Simplified self-attention
fn self_attention(hidden_states: &[f32], hidden_size: usize) -> Vec<f32> {
    let seq_len = hidden_states.len() / hidden_size;
    let mut output = vec![0.0; hidden_states.len()];
    
    for i in 0..seq_len {
        for j in 0..hidden_size {
            // Simple average attention (placeholder)
            let mut sum = 0.0;
            for k in 0..seq_len {
                sum += hidden_states[k * hidden_size + j];
            }
            output[i * hidden_size + j] = sum / seq_len as f32;
        }
    }
    
    output
}

/// Simplified feed-forward network
fn feed_forward(hidden_states: &[f32], _hidden_size: usize) -> Vec<f32> {
    let mut output = vec![0.0; hidden_states.len()];
    
    for i in 0..hidden_states.len() {
        // Simple activation function
        output[i] = hidden_states[i].max(0.0); // ReLU
    }
    
    output
}

/// Project hidden states to vocabulary
fn project_output(hidden_state: &[f32], weights: &[f32], vocab_size: usize) -> Vec<f32> {
    let hidden_size = hidden_state.len();
    let mut output = vec![0.0; vocab_size];
    
    for i in 0..vocab_size {
        let start = i * hidden_size;
        let end = start + hidden_size;
        if end <= weights.len() {
            for j in 0..hidden_size {
                output[i] += hidden_state[j] * weights[start + j];
            }
        }
    }
    
    // Simple softmax
    let max = output.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    let mut sum = 0.0;
    for val in output.iter_mut() {
        *val = (*val - max).exp();
        sum += *val;
    }
    for val in output.iter_mut() {
        *val /= sum;
    }
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ModelConfig;

    #[test]
    #[ignore] // Ignore in CI - requires GPU
    fn test_forward_pass() {
        let config = ModelConfig {
            vocab_size: 100,
            hidden_size: 32,
            num_layers: 1,
            num_heads: 2,
            seq_length: 10,
        };
        
        let model = TransformerModel::new(config);
        let input = vec![1.0, 2.0, 3.0];
        
        // Test CPU fallback
        if let Ok(context) = GpuContext::new() {
            let output = forward_pass(&context, &model, &input).unwrap();
            assert_eq!(output.len(), 100);
            
            // Check softmax property (sum to 1)
            let sum: f32 = output.iter().sum();
            assert!((sum - 1.0).abs() < 0.01);
        }
    }
}
