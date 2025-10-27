#!/usr/bin/env python3
"""
Generate a sample model file for testing the Tiny Transformer.
This creates a binary model file that can be loaded by all platform apps.
"""

import struct
import json
import sys

def create_sample_model(output_path="model.bin"):
    """Create a sample model with default configuration"""
    
    # Model configuration
    config = {
        "vocab_size": 1000,
        "hidden_size": 128,
        "num_layers": 2,
        "num_heads": 4,
        "seq_length": 32
    }
    
    print(f"Creating sample model with config: {config}")
    
    # Calculate sizes
    embed_size = config["vocab_size"] * config["hidden_size"]
    attn_size = config["num_layers"] * config["hidden_size"] * config["hidden_size"] * 4
    ffn_size = config["num_layers"] * config["hidden_size"] * config["hidden_size"] * 4
    output_size = config["hidden_size"] * config["vocab_size"]
    
    total_weights = embed_size + attn_size + ffn_size + output_size
    
    print(f"Total weights: {total_weights:,}")
    print(f"Model size: {total_weights * 4 / 1024 / 1024:.2f} MB")
    
    # Serialize config to JSON
    config_json = json.dumps(config).encode('utf-8')
    config_len = len(config_json)
    
    # Create model file
    with open(output_path, 'wb') as f:
        # Write config length (4 bytes, little-endian)
        f.write(struct.pack('<I', config_len))
        
        # Write config JSON
        f.write(config_json)
        
        # Write weights (all initialized to 0.01 for demo)
        for _ in range(total_weights):
            f.write(struct.pack('<f', 0.01))
    
    print(f"✓ Model saved to: {output_path}")
    print(f"  File size: {(4 + config_len + total_weights * 4) / 1024 / 1024:.2f} MB")

if __name__ == "__main__":
    output_path = sys.argv[1] if len(sys.argv) > 1 else "model.bin"
    create_sample_model(output_path)
