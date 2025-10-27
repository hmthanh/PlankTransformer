# PlankTransformer Documentation

Complete guide to building, integrating, and deploying the PlankTransformer inference engine.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Core Components](#core-components)
3. [Platform Integration](#platform-integration)
4. [Build Instructions](#build-instructions)
5. [API Reference](#api-reference)
6. [Performance Tips](#performance-tips)
7. [Troubleshooting](#troubleshooting)

## Architecture Overview

PlankTransformer is designed as a modular, cross-platform transformer inference engine. The architecture consists of several layers:

### Layer 1: Core Engine (`transformer-core`)

The core engine implements the transformer model and inference logic. It's written in pure Rust and uses WGPU for GPU abstraction.

**Key Components:**
- Model representation and serialization
- Tensor operations
- GPU context management
- Inference pipeline

### Layer 2: FFI Bindings (`transformer-ffi`)

C-compatible FFI layer that enables integration with other languages:
- C/C++ compatible interface
- Swift/Objective-C (iOS)
- Java/Kotlin (Android)
- JavaScript (via WASM)

### Layer 3: Platform Applications

Platform-specific implementations that use the core engine:
- Web: WASM + WebGPU
- iOS: Swift + Metal
- Android: Kotlin + Vulkan
- Desktop: Native Rust CLI

## Core Components

### TransformerModel

The `TransformerModel` struct represents a tiny transformer with configurable parameters:

```rust
pub struct ModelConfig {
    pub vocab_size: usize,    // Size of vocabulary
    pub hidden_size: usize,   // Hidden layer dimension
    pub num_layers: usize,    // Number of transformer layers
    pub num_heads: usize,     // Number of attention heads
    pub seq_length: usize,    // Maximum sequence length
}
```

### GpuContext

Manages GPU device and queue, automatically selecting the best available backend:

- **Web**: WebGPU
- **Windows**: DirectX 12
- **Linux/Android**: Vulkan
- **macOS/iOS**: Metal

### Inference Pipeline

The forward pass consists of:
1. Token embedding lookup
2. Multi-layer transformer processing
   - Self-attention
   - Feed-forward network
3. Output projection and softmax

## Platform Integration

### Web Integration

The web integration uses WebAssembly and WebGPU:

```javascript
import init, { init as initEngine, create_default_model, forward } from './pkg/transformer_web.js';

// Initialize
await init();
await initEngine();

// Create model
create_default_model();

// Run inference
const input = new Float32Array([1, 2, 3]);
const output = forward(input);
```

### iOS Integration

iOS apps use FFI through Swift:

```swift
let transformer = TransformerBridge()

// Initialize
guard transformer.init() else {
    fatalError("Failed to initialize")
}

// Create model
transformer.createDefaultModel()

// Run inference
let input: [Float] = [1, 2, 3]
let output = transformer.forward(input: input)
```

### Android Integration

Android apps use JNI:

```kotlin
val transformer = TransformerBridge()
transformer.init()
transformer.createDefaultModel()

val input = floatArrayOf(1f, 2f, 3f)
val output = transformer.forward(input)
```

## Build Instructions

### Prerequisites by Platform

#### All Platforms
- Rust 1.70+ (`rustup`)

#### Web
- `wasm-pack`
- Modern browser with WebGPU

#### iOS
- macOS with Xcode
- iOS targets: `rustup target add aarch64-apple-ios`

#### Android
- Android NDK
- `cargo-ndk`
- Android targets

#### Desktop
- Platform-specific GPU drivers (see platform READMEs)

### Quick Build

```bash
# Build everything
./build-all.sh

# Test everything
./test-all.sh
```

### Platform-Specific Builds

#### Core Library
```bash
cd crates/transformer-core
cargo build --release
cargo test
```

#### Desktop
```bash
cd apps/desktop
cargo build --release
./target/release/transformer-desktop
```

#### Web
```bash
cd apps/web
wasm-pack build --target web --out-dir pkg
python3 -m http.server 8080
```

#### iOS
```bash
cd apps/ios
./build.sh
# Then open Xcode project
```

#### Android
```bash
cd apps/android
./build.sh
# Then open in Android Studio
```

## API Reference

### Rust API

#### Initialize GPU Context
```rust
use transformer_core::GpuContext;

let context = GpuContext::new()?;
println!("Backend: {}", context.backend_name());
```

#### Create Model
```rust
use transformer_core::{TransformerModel, ModelConfig};

let config = ModelConfig::default();
let model = TransformerModel::new(config);
```

#### Load Model from Bytes
```rust
let model_data = std::fs::read("model.bin")?;
let model = TransformerModel::from_bytes(&model_data)?;
```

#### Run Inference
```rust
use transformer_core::run_inference;

let input = vec![1.0, 2.0, 3.0];
let output = run_inference(&context, &model, &input)?;
```

### FFI API (C/Swift/Kotlin)

#### Initialize
```c
TransformerContext* ctx = transformer_init();
```

#### Create Model
```c
int result = transformer_create_default_model(ctx);
```

#### Run Inference
```c
float input[] = {1.0, 2.0, 3.0};
float output[1000];
int result = transformer_forward(ctx, input, 3, output, 1000);
```

#### Cleanup
```c
transformer_free(ctx);
```

### WASM API

All functions are exposed as JavaScript async functions or regular functions:

- `init()` - Initialize WASM module (async)
- `init()` - Initialize GPU backend (async)
- `create_default_model()` - Create model
- `load_model(data)` - Load from bytes
- `forward(input)` - Run inference
- `get_vocab_size()` - Get vocabulary size
- `get_model_info()` - Get model configuration

## Performance Tips

### Model Size Optimization

1. **Use smaller hidden dimensions**: Reduces memory and compute
2. **Fewer layers**: Faster inference, less accuracy
3. **Quantization**: Convert to int8 or fp16 (future feature)

### GPU Optimization

1. **Batch processing**: Process multiple inputs together
2. **Pre-warm GPU**: Run a dummy inference on startup
3. **Reuse context**: Don't recreate GPU context frequently

### Platform-Specific

#### Web
- Use WebGPU when available
- Minimize WASM bundle size with `wasm-opt`
- Cache compiled WASM module

#### Mobile
- Use static linking to reduce app size
- Lazy-load models
- Use Metal Performance Shaders (iOS) or NNAPI (Android) where applicable

#### Desktop
- Enable LTO (Link Time Optimization)
- Use release profile
- Consider CPU fallback for unsupported GPUs

## Troubleshooting

### Build Issues

#### "WGPU adapter not found"
- **Cause**: No compatible GPU or drivers
- **Solution**: Update GPU drivers, or use CPU fallback

#### "Cannot find wgpu crate"
- **Cause**: Dependencies not downloaded
- **Solution**: Run `cargo fetch`

#### WASM build fails
- **Cause**: Missing wasm-pack
- **Solution**: `cargo install wasm-pack`

### Runtime Issues

#### Inference returns all zeros
- **Cause**: Model not loaded properly
- **Solution**: Check model loading, verify model format

#### Slow performance
- **Cause**: Using CPU instead of GPU
- **Solution**: Verify GPU backend is active, check drivers

#### Out of memory
- **Cause**: Model too large or GPU memory limited
- **Solution**: Use smaller model, batch size, or CPU

### Platform-Specific Issues

#### Web: "WebGPU not supported"
- Use Chrome 113+ or Edge 113+
- Enable flags in Firefox Nightly

#### iOS: Build fails
- Install all targets: `rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios`
- Ensure Xcode Command Line Tools installed

#### Android: JNI error
- Verify NDK installed and `ANDROID_NDK_HOME` set
- Check library architecture matches device

## Contributing

See the main [README.md](README.md) for contribution guidelines.

## License

MIT OR Apache-2.0

## Support

- GitHub Issues: https://github.com/hmthanh/PlankTransformer/issues
- Documentation: https://planktransformer.dev/docs
