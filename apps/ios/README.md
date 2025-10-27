# iOS App - Tiny Transformer

Native iOS application using Swift and Rust FFI for GPU-accelerated transformer inference.

## Prerequisites

- macOS with Xcode 14+
- Rust toolchain with iOS targets
- CocoaPods (optional, for dependencies)

## Setup

### 1. Install Rust iOS targets

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
```

### 2. Build Rust library

```bash
cd apps/ios
./build.sh
```

This will build the Rust static library for all iOS architectures and create a universal library.

## Project Structure

```
apps/ios/
├── build.sh                 # Build script for Rust library
├── TransformerDemo/        # Xcode project
│   ├── TransformerDemo/
│   │   ├── ContentView.swift
│   │   ├── TransformerBridge.swift  # Swift FFI wrapper
│   │   └── Info.plist
│   └── TransformerDemo.xcodeproj
├── headers/
│   └── transformer.h       # C header for FFI
└── README.md
```

## Building

### Using Xcode

1. Open `TransformerDemo.xcodeproj` in Xcode
2. Select your target device or simulator
3. Press ⌘+R to build and run

### Using xcodebuild (CLI)

```bash
cd TransformerDemo
xcodebuild -scheme TransformerDemo -destination 'platform=iOS Simulator,name=iPhone 14'
```

## Features

- Native SwiftUI interface
- Rust static library integration via FFI
- Metal GPU acceleration (automatic via WGPU)
- Real-time inference display
- Model info visualization

## Architecture

```
┌──────────────────┐
│   SwiftUI View   │  iOS App UI
└────────┬─────────┘
         │
┌────────▼─────────┐
│TransformerBridge │  Swift FFI Wrapper
└────────┬─────────┘
         │
┌────────▼─────────┐
│ transformer-ffi  │  Rust FFI Layer
└────────┬─────────┘
         │
┌────────▼─────────┐
│transformer-core  │  Core Engine
└────────┬─────────┘
         │
┌────────▼─────────┐
│   WGPU/Metal     │  GPU Backend
└──────────────────┘
```

## Usage

The app provides a simple interface to:
1. Initialize the transformer engine (Metal backend)
2. Load the default model
3. Input token IDs
4. Run inference and view predictions

## Troubleshooting

### Build Errors

If you encounter build errors:

1. Clean build folder: `rm -rf target/`
2. Rebuild: `./build.sh`
3. Clean Xcode build: Product → Clean Build Folder (⇧⌘K)

### Linker Errors

Ensure the Rust library is properly linked in Xcode:
1. Select project in Xcode
2. Go to Build Phases → Link Binary With Libraries
3. Verify `libtransformer_ffi.a` is present

### Simulator vs Device

The build script creates a universal library supporting both simulator and device. If you only need one:

**Simulator only:**
```bash
cargo build --release --target aarch64-apple-ios-sim
```

**Device only:**
```bash
cargo build --release --target aarch64-apple-ios
```

## Performance

- **Device**: Uses Metal for GPU acceleration
- **Simulator**: May fall back to CPU depending on host GPU

## Code Signing

For running on physical devices, configure code signing in Xcode:
1. Select project target
2. Go to Signing & Capabilities
3. Select your development team

## Deployment

### TestFlight / App Store

1. Archive the app in Xcode
2. Follow standard iOS app distribution process
3. Ensure the Rust library is included in the bundle

### Enterprise Distribution

The Rust library can be distributed as part of the app bundle or as a separate framework.
