# Linux Desktop App - Tiny Transformer

Native Linux application using Vulkan for GPU-accelerated transformer inference.

## Prerequisites

- Linux (Ubuntu, Fedora, Arch, etc.)
- Rust toolchain (1.70+)
- Vulkan drivers and runtime

## Installing Dependencies

### Ubuntu/Debian
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    mesa-vulkan-drivers \
    vulkan-tools \
    libvulkan-dev
```

### Fedora
```bash
sudo dnf install -y \
    gcc \
    pkg-config \
    mesa-vulkan-drivers \
    vulkan-tools \
    vulkan-loader-devel
```

### Arch Linux
```bash
sudo pacman -S \
    base-devel \
    vulkan-icd-loader \
    vulkan-tools \
    vulkan-headers
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

The application automatically uses Vulkan for GPU acceleration on Linux.

### Verifying Vulkan Support

Check Vulkan installation:
```bash
vulkaninfo | head -20
```

List available Vulkan devices:
```bash
vulkaninfo | grep deviceName
```

## Supported GPUs

- **AMD**: Open-source RADV driver or proprietary AMDGPU-PRO
- **NVIDIA**: Proprietary NVIDIA driver (version 450+)
- **Intel**: Mesa ANV driver (Intel HD/Iris Graphics)

## Performance

- **GPU Mode**: Uses Vulkan for acceleration
- **CPU Fallback**: Automatically falls back if Vulkan not available

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

## Troubleshooting

### Vulkan Not Found

If Vulkan is not detected:

1. Install Vulkan drivers (see Dependencies above)
2. Verify installation: `vulkaninfo`
3. Check that `/usr/share/vulkan/icd.d/` contains ICD configuration files

### GPU Not Detected

Ensure your GPU is recognized:
```bash
lspci | grep VGA
```

For NVIDIA GPUs, verify driver:
```bash
nvidia-smi
```

### Permission Errors

If you get permission errors accessing GPU:
```bash
sudo usermod -a -G video,render $USER
```
Then log out and back in.

### Build Errors

Install missing dependencies:
```bash
sudo apt-get install libxcb1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
```

## Distribution

### AppImage

Create a portable AppImage:
```bash
# Build release
cargo build --release

# Create AppImage (using linuxdeploy or similar tool)
# See: https://appimage.org/
```

### Flatpak

Package as Flatpak for distribution:
```bash
# See: https://flatpak.org/
```

### Snap

Package as Snap:
```bash
# See: https://snapcraft.io/
```

## Advanced Configuration

### Environment Variables

Set Vulkan device selection:
```bash
VK_ICD_FILENAMES=/path/to/device.json ./transformer-desktop
```

Enable Vulkan validation layers (debug):
```bash
VK_INSTANCE_LAYERS=VK_LAYER_KHRONOS_validation ./transformer-desktop
```

### Custom Model Loading

Modify `main.rs` to load custom models:
```rust
let model_data = std::fs::read("model.bin")?;
let model = TransformerModel::from_bytes(&model_data)?;
```

## System Requirements

- **OS**: Linux kernel 4.15+ (Ubuntu 18.04+, similar for other distros)
- **GPU**: Vulkan 1.0+ compatible
- **RAM**: 512 MB minimum
- **Disk**: 50 MB for application
