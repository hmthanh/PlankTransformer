# Windows Desktop App - Tiny Transformer

Native Windows application using DirectX 12 for GPU-accelerated transformer inference.

## Prerequisites

- Windows 10/11 with DirectX 12 support
- Rust toolchain (1.70+)
- Visual Studio 2019+ with C++ tools (optional, for development)

## Building

```bash
cd apps/desktop
cargo build --release
```

The executable will be at: `target/release/transformer-desktop.exe`

## Running

```bash
cargo run --release
```

Or run the executable directly:
```bash
.\target\release\transformer-desktop.exe
```

## GPU Backend

The application automatically uses DirectX 12 for GPU acceleration on Windows. The WGPU library handles the DirectX 12 API calls.

### Requirements for DirectX 12

- Windows 10 version 1903 or later (recommended)
- DirectX 12 compatible GPU
- Latest GPU drivers

### Checking DirectX 12 Support

Run `dxdiag` from the Windows Run dialog (Win+R) to check:
1. System → Operating System should be Windows 10/11
2. Display → Feature Levels should show "12_0" or higher

## Performance

- **GPU Mode**: Uses DirectX 12 for acceleration
- **CPU Fallback**: Automatically falls back if no compatible GPU

## Example Output

```
=== Tiny Transformer Desktop Demo ===

Initializing GPU backend...
✓ Using backend: DirectX 12

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

## Troubleshooting

### DirectX 12 Not Available

If you see errors about DirectX 12:
1. Update Windows to the latest version
2. Update GPU drivers from manufacturer's website
3. Verify GPU supports DirectX 12

### Build Errors

Ensure you have the MSVC toolchain:
```bash
rustup default stable-msvc
```

### Missing DLL Errors

If the executable fails to run due to missing DLLs:
1. Install Visual C++ Redistributable from Microsoft
2. Or build with static CRT: Add to Cargo.toml:
   ```toml
   [target.x86_64-pc-windows-msvc]
   rustflags = ["-C", "target-feature=+crt-static"]
   ```

## Distribution

The release executable is self-contained and can be distributed as-is. Consider packaging it with:
- Installer (e.g., WiX, NSIS)
- Portable ZIP archive

## Advanced Configuration

### Custom Model Loading

Modify `main.rs` to load custom models:
```rust
let model_data = std::fs::read("model.bin")?;
let model = TransformerModel::from_bytes(&model_data)?;
```

### GUI Integration

The current app is CLI-based. To add a GUI:
- Use `winit` + `egui` for immediate mode GUI
- Use `iced` for Elm-style GUI
- Use native Windows APIs via `windows-rs`
