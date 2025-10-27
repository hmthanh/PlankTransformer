# 🚀 PlankTransformer

**Cross-platform Tiny Transformer inference engine powered by Rust + WGPU**

A high-performance, GPU-accelerated transformer inference engine that runs everywhere: Web, iOS, Android, Windows, Linux, and macOS.

[![CI/CD](https://github.com/hmthanh/PlankTransformer/workflows/CI%2FCD/badge.svg)](https://github.com/hmthanh/PlankTransformer/actions)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)

## ✨ Features

- 🎯 **Single Codebase**: Write once, run everywhere using Rust
- ⚡ **GPU Accelerated**: Leverages WebGPU, Metal, Vulkan, and DirectX 12
- 📦 **Lightweight**: Optimized for small binary size and fast loading
- 🔧 **Easy Integration**: FFI bindings for Swift, Kotlin, JavaScript, and more
- 🧪 **Production Ready**: Comprehensive test coverage and CI/CD
- 🌐 **Zero Dependencies**: Self-contained with minimal external dependencies

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────┐
│                   Applications                       │
├──────────┬──────────┬──────────┬──────────┬─────────┤
│   Web    │   iOS    │ Android  │ Windows  │ Linux   │
│ (WASM)   │ (Swift)  │ (Kotlin) │  (CLI)   │ (CLI)   │
└────┬─────┴────┬─────┴────┬─────┴────┬─────┴────┬────┘
     │          │          │          │          │
     └──────────┴──────────┴──────────┴──────────┘
                         │
              ┌──────────▼──────────┐
              │  FFI Bindings Layer │
              │  (transformer-ffi)  │
              └──────────┬──────────┘
                         │
              ┌──────────▼──────────┐
              │    Core Engine      │
              │ (transformer-core)  │
              └──────────┬──────────┘
                         │
              ┌──────────▼──────────┐
              │    WGPU Layer       │
              └──────────┬──────────┘
                         │
        ┌────────────────┼────────────────┐
        │                │                │
    ┌───▼───┐      ┌────▼────┐      ┌───▼────┐
    │WebGPU │      │ Vulkan  │      │ Metal  │
    │  Web  │      │ Linux   │      │ macOS  │
    └───────┘      │ Android │      │  iOS   │
                   └────┬────┘      └────────┘
                        │
                   ┌────▼─────┐
                   │DirectX 12│
                   │ Windows  │
                   └──────────┘
```

## 🎯 Platform Support

| Platform | Backend | Status | App Type |
|----------|---------|--------|----------|
| **Web** | WebGPU | ✅ Supported | Browser WASM |
| **iOS** | Metal | ✅ Supported | Native App |
| **Android** | Vulkan | ✅ Supported | Native App |
| **Windows** | DirectX 12 | ✅ Supported | Desktop CLI |
| **Linux** | Vulkan | ✅ Supported | Desktop CLI |
| **macOS** | Metal | ✅ Supported | Desktop CLI |

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Platform-specific tools (see individual app READMEs)

### Building the Core Library

```bash
# Clone the repository
git clone https://github.com/hmthanh/PlankTransformer.git
cd PlankTransformer

# Build and test the core library
cargo build --release
cargo test
```

### Running Desktop App

```bash
cd apps/desktop
cargo run --release
```

### Building Web App

```bash
# Install wasm-pack
cargo install wasm-pack

# Build WASM module
cd apps/web
wasm-pack build --target web --out-dir pkg

# Serve with a local server
python3 -m http.server 8080
# Open http://localhost:8080
```

## 📱 Platform-Specific Guides

Each platform has its own detailed guide:

- **[Web (WASM)](apps/web/README.md)** - WebGPU-powered browser app
- **[iOS](apps/ios/README.md)** - Native iOS app with Swift
- **[Android](apps/android/README.md)** - Native Android app with Kotlin
- **[Desktop](apps/desktop/README.md)** - Cross-platform CLI app
  - [Windows Specific](apps/windows/README.md)
  - [Linux Specific](apps/linux/README.md)
  - [macOS Specific](apps/macos/README.md)

## 📦 Project Structure

```
PlankTransformer/
├── crates/
│   ├── transformer-core/    # Core inference engine
│   └── transformer-ffi/     # FFI bindings for C/Swift/Kotlin
├── apps/
│   ├── web/                 # Web app (WASM + WebGPU)
│   ├── ios/                 # iOS app (Swift + Metal)
│   ├── android/             # Android app (Kotlin + Vulkan)
│   ├── desktop/             # Desktop app (Rust)
│   ├── windows/             # Windows-specific docs
│   ├── linux/               # Linux-specific docs
│   └── macos/               # macOS-specific docs
├── .github/
│   └── workflows/           # CI/CD pipelines
├── Cargo.toml               # Workspace configuration
└── README.md                # This file
```

## 🔧 Development

### Running Tests

```bash
# Test core library
cd crates/transformer-core
cargo test

# Test FFI bindings
cd crates/transformer-ffi
cargo test
```

### Linting and Formatting

```bash
# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-targets --all-features
```

### Building for Specific Platforms

```bash
# Web (WASM)
cd apps/web
wasm-pack build --target web

# iOS
cd apps/ios
./build.sh

# Android
cd apps/android
./build.sh

# Desktop (current platform)
cd apps/desktop
cargo build --release
```

## 🧪 Example Usage

### Rust

```rust
use transformer_core::{GpuContext, TransformerModel, ModelConfig};

// Initialize GPU backend
let context = GpuContext::new()?;
println!("Using backend: {}", context.backend_name());

// Create or load model
let config = ModelConfig::default();
let model = TransformerModel::new(config);

// Run inference
let input = vec![1.0, 2.0, 3.0];
let output = transformer_core::run_inference(&context, &model, &input)?;
```

### JavaScript (Web)

```javascript
import init, { init as initEngine, create_default_model, forward } from './pkg/transformer_web.js';

// Initialize
await init();
const backend = await initEngine();
console.log('Backend:', backend);

// Create model
create_default_model();

// Run inference
const input = new Float32Array([1, 2, 3]);
const output = forward(input);
console.log('Top prediction:', output[0]);
```

### Swift (iOS)

```swift
let transformer = TransformerBridge()
transformer.createDefaultModel()

let input: [Float] = [1, 2, 3]
if let output = transformer.forward(input: input) {
    print("Top predictions:", output.prefix(5))
}
```

### Kotlin (Android)

```kotlin
val transformer = TransformerBridge()
transformer.init()
transformer.createDefaultModel()

val input = floatArrayOf(1f, 2f, 3f)
val output = transformer.forward(input)
println("Top predictions: ${output?.take(5)}")
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

### Development Setup

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

- [wgpu](https://github.com/gfx-rs/wgpu) - Cross-platform GPU abstraction
- [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) - WebAssembly bindings
- Rust community for excellent tooling and libraries

## 📞 Support

- 📧 Email: support@planktransformer.dev
- 💬 Discord: [Join our community](https://discord.gg/planktransformer)
- 🐛 Issues: [GitHub Issues](https://github.com/hmthanh/PlankTransformer/issues)

## 🗺️ Roadmap

- [ ] Add GPU kernel optimizations
- [ ] Support for quantized models
- [ ] Add more example models
- [ ] Performance benchmarks across platforms
- [ ] GUI applications for desktop platforms
- [ ] Model compression utilities
- [ ] Streaming inference support

---

Made with ❤️ by the PlankTransformer team
