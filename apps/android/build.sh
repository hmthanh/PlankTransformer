#!/bin/bash
set -e

echo "Building Rust library for Android..."

# Build for all Android architectures
TARGETS=("aarch64-linux-android" "armv7-linux-androideabi" "x86_64-linux-android" "i686-linux-android")

for TARGET in "${TARGETS[@]}"; do
    echo "Building for $TARGET..."
    cargo ndk --target $TARGET --platform 24 -- build --release --manifest-path ../../crates/transformer-ffi/Cargo.toml
done

# Create jniLibs directory structure
echo "Organizing libraries..."
mkdir -p app/src/main/jniLibs/{arm64-v8a,armeabi-v7a,x86_64,x86}

# Copy libraries to correct locations
cp ../../target/aarch64-linux-android/release/libtransformer_ffi.so app/src/main/jniLibs/arm64-v8a/
cp ../../target/armv7-linux-androideabi/release/libtransformer_ffi.so app/src/main/jniLibs/armeabi-v7a/
cp ../../target/x86_64-linux-android/release/libtransformer_ffi.so app/src/main/jniLibs/x86_64/
cp ../../target/i686-linux-android/release/libtransformer_ffi.so app/src/main/jniLibs/x86/

echo "✓ Build complete! Libraries copied to app/src/main/jniLibs/"
