# macOS Desktop App - Tiny Transformer

Native macOS application using Metal for GPU-accelerated transformer inference.

## Prerequisites

- macOS 10.13 (High Sierra) or later
- Xcode Command Line Tools
- Rust toolchain (1.70+)

## Installing Xcode Command Line Tools

```bash
xcode-select --install
```

## Building

```bash
cd apps/desktop
cargo build --release
```

The binary will be at: `target/release/transformer-desktop`

## Running

```bash
cargo run --release
```

Or run the binary directly:
```bash
./target/release/transformer-desktop
```

## GPU Backend

The application automatically uses Metal for GPU acceleration on macOS.

### Metal Support

Metal is available on:
- All Macs from 2012 onwards (macOS 10.13+)
- Apple Silicon (M1, M2, M3) - full support
- Intel Macs with compatible GPU

Check Metal support:
```bash
system_profiler SPDisplaysDataType | grep Metal
```

## Performance

- **GPU Mode**: Uses Metal for acceleration
- **Apple Silicon**: Optimized performance on M1/M2/M3
- **Intel Macs**: Uses discrete/integrated GPU via Metal

## Example Output

```
=== Tiny Transformer Desktop Demo ===

Initializing GPU backend...
✓ Using backend: Metal

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

### Metal Not Available

Metal should be available on all modern Macs. If you see errors:
1. Verify macOS version: `sw_vers`
2. Update to latest macOS version
3. Check GPU: `system_profiler SPDisplaysDataType`

### Build Errors

Ensure Xcode Command Line Tools are installed:
```bash
xcode-select -p
```

If not installed:
```bash
xcode-select --install
```

### Code Signing Issues

For running on your own Mac, no special signing is needed. For distribution:
```bash
codesign --sign "Developer ID Application: Your Name" ./target/release/transformer-desktop
```

## Creating a macOS App Bundle

Create a proper .app bundle for distribution:

```bash
#!/bin/bash
APP_NAME="TinyTransformer"
mkdir -p "$APP_NAME.app/Contents/MacOS"
mkdir -p "$APP_NAME.app/Contents/Resources"

cp target/release/transformer-desktop "$APP_NAME.app/Contents/MacOS/"

cat > "$APP_NAME.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>transformer-desktop</string>
    <key>CFBundleIdentifier</key>
    <string>com.transformer.demo</string>
    <key>CFBundleName</key>
    <string>$APP_NAME</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.13</string>
</dict>
</plist>
EOF
```

## Distribution

### Direct Distribution

Create a DMG for distribution:
```bash
# Using create-dmg or hdiutil
hdiutil create -volname "TinyTransformer" -srcfolder TinyTransformer.app -ov -format UDZO TinyTransformer.dmg
```

### Mac App Store

For App Store distribution:
1. Sign with Mac App Store certificate
2. Create app package with `productbuild`
3. Upload with Transporter or Application Loader

### Homebrew

Distribute via Homebrew:
```ruby
class TinyTransformer < Formula
  desc "Tiny Transformer inference engine"
  homepage "https://github.com/youruser/PlankTransformer"
  url "https://github.com/youruser/PlankTransformer/archive/v1.0.0.tar.gz"
  sha256 "..."

  def install
    system "cargo", "build", "--release"
    bin.install "target/release/transformer-desktop"
  end
end
```

## Universal Binary (Intel + Apple Silicon)

Build a universal binary:

```bash
# Install targets
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build both
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary
lipo -create \
    target/x86_64-apple-darwin/release/transformer-desktop \
    target/aarch64-apple-darwin/release/transformer-desktop \
    -output target/release/transformer-desktop-universal
```

## Advanced Configuration

### Custom Model Loading

Modify `main.rs` to load custom models:
```rust
let model_data = std::fs::read("model.bin")?;
let model = TransformerModel::from_bytes(&model_data)?;
```

### GUI Integration

Add a native macOS GUI:
- Use `cocoa` crate for native Cocoa APIs
- Use `iced` for cross-platform GUI with Metal backend
- Create SwiftUI wrapper (see ios/ for example)

## System Requirements

- **OS**: macOS 10.13 (High Sierra) or later
- **GPU**: Metal-compatible (all Macs from 2012+)
- **RAM**: 512 MB minimum
- **Disk**: 50 MB for application

## Optimizations for Apple Silicon

The app automatically benefits from Apple Silicon:
- Native ARM64 execution
- Unified memory architecture
- Metal Performance Shaders (MPS) optimization
