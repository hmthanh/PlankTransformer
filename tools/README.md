# Tools

Utility scripts for working with PlankTransformer models.

## generate_model.py

Python script to generate sample model files for testing.

### Usage

```bash
# Generate model.bin in current directory
python3 tools/generate_model.py

# Generate with custom path
python3 tools/generate_model.py /path/to/output/model.bin
```

This creates a binary model file compatible with all platform apps.

### Model Format

The binary format is:
```
[4 bytes] Config length (uint32, little-endian)
[N bytes] Config JSON
[M bytes] Weights (float32, little-endian)
```

Where weights are concatenated in order:
1. Embeddings
2. Attention weights
3. Feed-forward weights
4. Output projection weights
