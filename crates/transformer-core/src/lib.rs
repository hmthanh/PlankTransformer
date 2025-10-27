//! Tiny Transformer Core Engine
//! 
//! This crate provides a minimal transformer inference engine using WGPU for GPU acceleration.
//! It supports WebGPU, Vulkan, Metal, and DirectX12 backends.

mod model;
mod tensor;
mod gpu;
mod inference;

pub use model::{TransformerModel, ModelConfig};
pub use tensor::Tensor;
pub use gpu::{GpuContext, BackendType};
pub use inference::forward_pass;

use anyhow::Result;

/// Initialize the transformer engine with auto-detection of best available GPU backend
pub fn init() -> Result<GpuContext> {
    GpuContext::new()
}

/// Load a model from bytes
pub fn load_model(data: &[u8]) -> Result<TransformerModel> {
    TransformerModel::from_bytes(data)
}

/// Run inference on the model
pub fn run_inference(
    context: &GpuContext,
    model: &TransformerModel,
    input: &[f32],
) -> Result<Vec<f32>> {
    forward_pass(context, model, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_config() {
        let config = ModelConfig {
            vocab_size: 1000,
            hidden_size: 128,
            num_layers: 2,
            num_heads: 4,
            seq_length: 32,
        };
        assert_eq!(config.vocab_size, 1000);
    }
}
