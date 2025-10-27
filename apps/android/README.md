# Android App - Tiny Transformer

Native Android application using Kotlin and Rust FFI via JNI for GPU-accelerated transformer inference.

## Prerequisites

- Android Studio (Arctic Fox or later)
- Rust toolchain with Android targets
- Android NDK 25+
- Cargo NDK tool

## Setup

### 1. Install Android targets and cargo-ndk

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
cargo install cargo-ndk
```

### 2. Set NDK path

Add to your `~/.bashrc` or `~/.zshrc`:
```bash
export ANDROID_NDK_HOME=$HOME/Android/Sdk/ndk/25.2.9519653
```

### 3. Build Rust library

```bash
cd apps/android
./build.sh
```

## Project Structure

```
apps/android/
├── build.sh                 # Build script for Rust library
├── app/
│   ├── src/main/
│   │   ├── java/com/transformer/demo/
│   │   │   ├── MainActivity.kt
│   │   │   └── TransformerBridge.kt  # JNI wrapper
│   │   ├── cpp/
│   │   │   └── transformer_jni.cpp   # JNI glue code
│   │   ├── jniLibs/                  # Compiled Rust libraries
│   │   └── res/
│   └── build.gradle
├── build.gradle
└── README.md
```

## Building

### Using Android Studio

1. Open the project in Android Studio
2. Build → Make Project (⌘+F9)
3. Run → Run 'app' (⌃+R)

### Using Gradle (CLI)

```bash
cd apps/android
./gradlew assembleDebug
```

Install on device:
```bash
./gradlew installDebug
```

## Features

- Native Android Material Design UI
- Rust library integration via JNI
- Vulkan GPU acceleration (automatic via WGPU)
- Real-time inference display
- Model info visualization
- Loading from assets

## Architecture

```
┌──────────────────┐
│  Jetpack Compose │  Android App UI
└────────┬─────────┘
         │
┌────────▼─────────┐
│TransformerBridge │  Kotlin JNI Wrapper
└────────┬─────────┘
         │
┌────────▼─────────┐
│transformer_jni.so│  JNI Glue Layer
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
│   WGPU/Vulkan    │  GPU Backend
└──────────────────┘
```

## Supported Architectures

- ARM64 (aarch64) - Modern Android devices
- ARMv7 - Older Android devices
- x86_64 - Emulators and some tablets
- x86 - Older emulators

## Usage

The app provides a Material Design interface to:
1. Initialize the transformer engine (Vulkan backend)
2. Load the default model
3. Input token IDs
4. Run inference and view predictions

## Testing on Emulator

The app works on Android emulators with Vulkan support. Ensure your emulator:
1. Uses Android 8.0 (API 26) or higher
2. Has hardware acceleration enabled
3. Uses a system image with Google APIs

## Troubleshooting

### Build Errors

If you encounter build errors:

1. Clean build: `./gradlew clean`
2. Rebuild Rust: `./build.sh`
3. Sync Gradle: File → Sync Project with Gradle Files

### JNI Errors

If you see `UnsatisfiedLinkError`:
1. Verify libraries are in `app/src/main/jniLibs/`
2. Check that all architectures are built
3. Ensure native library is loaded in Kotlin code

### Vulkan Not Available

If Vulkan is not available:
- The app will automatically fall back to CPU
- Check device compatibility: Many older devices don't support Vulkan
- Emulator: Use a newer API level (28+)

## Performance

- **Physical Device**: Uses Vulkan for GPU acceleration
- **Emulator**: May use CPU fallback depending on host GPU

## Code Signing

Configure signing in `app/build.gradle`:
```gradle
signingConfigs {
    release {
        storeFile file("keystore.jks")
        storePassword "password"
        keyAlias "key"
        keyPassword "password"
    }
}
```

## Deployment

### Google Play Store

1. Build release APK: `./gradlew assembleRelease`
2. Sign the APK with your keystore
3. Upload to Play Console

### Direct APK Distribution

The release APK will be located at:
```
app/build/outputs/apk/release/app-release.apk
```

## Minimum SDK

- Minimum: API 24 (Android 7.0)
- Target: API 34 (Android 14)
- Recommended: API 26+ for best Vulkan support
