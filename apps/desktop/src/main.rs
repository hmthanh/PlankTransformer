//! Desktop application for Tiny Transformer
//! 
//! Cross-platform desktop app that runs on Windows (DirectX12), Linux (Vulkan), and macOS (Metal)

use transformer_core::{GpuContext, TransformerModel, ModelConfig};
use anyhow::Result;

fn main() -> Result<()> {
    println!("=== Tiny Transformer Desktop Demo ===\n");
    
    // Initialize GPU context
    println!("Initializing GPU backend...");
    let context = GpuContext::new()?;
    println!("✓ Using backend: {}\n", context.backend_name());
    
    // Create a default model
    println!("Creating default transformer model...");
    let config = ModelConfig {
        vocab_size: 1000,
        hidden_size: 128,
        num_layers: 2,
        num_heads: 4,
        seq_length: 32,
    };
    let model = TransformerModel::new(config.clone());
    println!("✓ Model created");
    println!("  - Vocab size: {}", config.vocab_size);
    println!("  - Hidden size: {}", config.hidden_size);
    println!("  - Layers: {}", config.num_layers);
    println!("  - Attention heads: {}\n", config.num_heads);
    
    // Run inference
    println!("Running inference...");
    let input = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    println!("Input tokens: {:?}", input);
    
    let output = transformer_core::run_inference(&context, &model, &input)?;
    
    // Find top 5 predictions
    let mut indexed: Vec<(usize, f32)> = output.iter()
        .enumerate()
        .map(|(i, &v)| (i, v))
        .collect();
    indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("\nTop 5 predictions:");
    for (rank, (token_id, prob)) in indexed.iter().take(5).enumerate() {
        println!("  {}. Token {}: {:.4}%", rank + 1, token_id, prob * 100.0);
    }
    
    println!("\n✓ Inference complete!");
    
    Ok(())
}
