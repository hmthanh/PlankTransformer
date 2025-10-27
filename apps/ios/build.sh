#!/bin/bash
set -e

echo "Building Rust library for iOS..."

# Build for iOS device (ARM64)
echo "Building for iOS device (arm64)..."
cargo build --release --target aarch64-apple-ios --manifest-path ../../crates/transformer-ffi/Cargo.toml

# Build for iOS simulator (ARM64)
echo "Building for iOS simulator (arm64)..."
cargo build --release --target aarch64-apple-ios-sim --manifest-path ../../crates/transformer-ffi/Cargo.toml

# Build for iOS simulator (x86_64, for Intel Macs)
echo "Building for iOS simulator (x86_64)..."
cargo build --release --target x86_64-apple-ios --manifest-path ../../crates/transformer-ffi/Cargo.toml

# Create output directory
mkdir -p lib

# Create universal library for simulator
echo "Creating universal simulator library..."
lipo -create \
    ../../target/aarch64-apple-ios-sim/release/libtransformer_ffi.a \
    ../../target/x86_64-apple-ios/release/libtransformer_ffi.a \
    -output lib/libtransformer_ffi_sim.a

# Copy device library
cp ../../target/aarch64-apple-ios/release/libtransformer_ffi.a lib/libtransformer_ffi_device.a

# Create XCFramework
echo "Creating XCFramework..."
rm -rf lib/TransformerFFI.xcframework

xcodebuild -create-xcframework \
    -library lib/libtransformer_ffi_device.a \
    -headers headers \
    -library lib/libtransformer_ffi_sim.a \
    -headers headers \
    -output lib/TransformerFFI.xcframework

echo "✓ Build complete! XCFramework created at lib/TransformerFFI.xcframework"
