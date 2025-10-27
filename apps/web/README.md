# Web App - Tiny Transformer

WebGPU-powered Tiny Transformer inference running in the browser via WebAssembly.

## Prerequisites

- Rust toolchain (1.70+)
- `wasm-pack` for building WASM modules
- A modern browser with WebGPU support (Chrome 113+, Edge 113+, or Firefox Nightly)

## Building

Install wasm-pack if you haven't already:
```bash
cargo install wasm-pack
```

Build the WASM module:
```bash
cd apps/web
wasm-pack build --target web --out-dir pkg
```

## Running

You need a local web server to serve the files (due to CORS restrictions). You can use any of these:

### Option 1: Python
```bash
python3 -m http.server 8080
```

### Option 2: Node.js http-server
```bash
npx http-server -p 8080
```

### Option 3: Rust simple-http-server
```bash
cargo install simple-http-server
simple-http-server -p 8080
```

Then open your browser to `http://localhost:8080`

## Usage

1. Click "Initialize Engine" to set up the WebGPU backend
2. Click "Create Model" to load the default transformer model
3. Enter token IDs (comma-separated numbers) in the input field
4. Click "Run Inference" to execute a forward pass
5. View the top predictions in the result box

## Features

- **WebGPU Backend**: Uses the browser's GPU for acceleration when available
- **CPU Fallback**: Automatically falls back to CPU if WebGPU is not supported
- **Real-time Inference**: Run transformer inference directly in the browser
- **Lightweight**: Optimized WASM binary for fast loading

## Browser Compatibility

| Browser | WebGPU Support | Status |
|---------|---------------|--------|
| Chrome 113+ | ✅ Yes | Fully supported |
| Edge 113+ | ✅ Yes | Fully supported |
| Firefox Nightly | ⚠️ Experimental | Enable via flags |
| Safari | 🚧 In Progress | Future support |

## Architecture

```
┌─────────────────┐
│  index.html     │  Web UI
└────────┬────────┘
         │
┌────────▼────────┐
│  lib.rs (WASM)  │  WASM Bindings
└────────┬────────┘
         │
┌────────▼────────┐
│ transformer-core│  Rust Core Engine
└────────┬────────┘
         │
┌────────▼────────┐
│   WGPU/WebGPU   │  GPU Abstraction
└─────────────────┘
```

## Troubleshooting

### WebGPU not available
If you see "WebGPU not available", ensure:
1. You're using a compatible browser version
2. GPU acceleration is enabled in browser settings
3. Your GPU drivers are up to date

### WASM loading fails
Make sure you're serving the files via HTTP/HTTPS, not opening them directly as files.

### Build errors
Ensure you have the latest wasm-pack:
```bash
cargo install wasm-pack --force
```
