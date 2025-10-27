# Desktop App - Tiny Transformer

Cross-platform desktop application for running Tiny Transformer inference with native GPU acceleration.

## Platform Support

| Platform | GPU Backend | Status |
|----------|-------------|--------|
| Windows | DirectX 12 | ✅ Supported |
| Linux | Vulkan | ✅ Supported |
| macOS | Metal | ✅ Supported |

## Prerequisites

- Rust toolchain (1.70+)
- Platform-specific GPU drivers:
  - **Windows**: DirectX 12 compatible GPU
  - **Linux**: Vulkan compatible GPU and drivers
  - **macOS**: Metal compatible GPU (macOS 10.13+)

## Building

### All Platforms
```bash
cd apps/desktop
cargo build --release
```

The binary will be located at `target/release/transformer-desktop` (or `.exe` on Windows).

### Platform-Specific Notes

#### Windows
No additional dependencies needed. DirectX 12 is included with Windows 10/11.

#### Linux
Install Vulkan drivers:

**Ubuntu/Debian:**
```bash
sudo apt-get install mesa-vulkan-drivers vulkan-tools
```

**Fedora:**
```bash
sudo dnf install mesa-vulkan-drivers vulkan-tools
```

**Arch:**
```bash
sudo pacman -S vulkan-icd-loader vulkan-tools
```

#### macOS
Xcode command line tools are required:
```bash
xcode-select --install
```

## Running

```bash
cargo run --release
```

Or run the binary directly:
```bash
./target/release/transformer-desktop
```

## Usage

The app will:
1. Auto-detect and initialize the best available GPU backend
2. Create a default transformer model
3. Run inference on sample input tokens
4. Display the top 5 predictions

## Example Output

```
=== Tiny Transformer Desktop Demo ===

Initializing GPU backend...
✓ Using backend: Vulkan

Creating default transformer model...
✓ Model created
  - Vocab size: 1000
  - Hidden size: 128
  - Layers: 2
  - Attention heads: 4

Running inference...
Input tokens: [1.0, 2.0, 3.0, 4.0, 5.0]

Top 5 predictions:
  1. Token 42: 2.1234%
  2. Token 17: 1.9876%
  3. Token 99: 1.8765%
  4. Token 3: 1.7654%
  5. Token 88: 1.6543%

✓ Inference complete!
```

## Architecture

```
┌─────────────────┐
│   main.rs       │  Desktop CLI App
└────────┬────────┘
         │
┌────────▼────────┐
│ transformer-core│  Rust Core Engine
└────────┬────────┘
         │
┌────────▼────────┐
│      WGPU       │  GPU Abstraction
└────────┬────────┘
         │
    ┌────┴─────┬─────────┬────────┐
    │          │         │        │
┌───▼───┐  ┌──▼───┐  ┌──▼───┐ ┌──▼────┐
│DirectX│  │Vulkan│  │Metal │ │  CPU  │
│  12   │  │      │  │      │ │Fallback│
└───────┘  └──────┘  └──────┘ └───────┘
```

## Advanced Usage

### Loading Custom Models

You can modify the code to load custom model weights:

```rust
use std::fs;

// Load model from file
let model_data = fs::read("path/to/model.bin")?;
let model = TransformerModel::from_bytes(&model_data)?;
```

### Custom Input

Modify the `input` vector in `main.rs` to test different token sequences:

```rust
let input = vec![10.0, 20.0, 30.0]; // Your custom tokens
```

## Troubleshooting

### GPU Backend Not Found

If you see an error about no suitable GPU adapter:
1. Ensure your GPU drivers are up to date
2. Check that your GPU supports the required API (DirectX 12, Vulkan, or Metal)
3. The app will try to fall back to CPU if no GPU is available

### Build Errors

Make sure you have the latest Rust toolchain:
```bash
rustup update
```

### Performance Issues

For best performance:
- Build in release mode: `cargo build --release`
- Ensure GPU drivers are up to date
- Close other GPU-intensive applications

## Cross-Compilation

### Building for Windows from Linux
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

### Building for Linux from macOS
```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

Note: Cross-compilation may require additional setup and toolchains.
