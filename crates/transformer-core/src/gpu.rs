//! GPU context and backend selection

use anyhow::{Result, anyhow};
use wgpu;

/// GPU backend types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendType {
    Vulkan,
    Metal,
    Dx12,
    WebGpu,
    Cpu,
}

/// GPU context for running computations
pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub backend: BackendType,
}

impl GpuContext {
    /// Create a new GPU context with auto-detection
    pub fn new() -> Result<Self> {
        pollster::block_on(Self::new_async())
    }

    /// Create GPU context with specific backend
    pub fn with_backend(backend: wgpu::Backends) -> Result<Self> {
        pollster::block_on(Self::with_backend_async(backend))
    }

    async fn new_async() -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow!("Failed to find a suitable GPU adapter"))?;

        let backend = Self::detect_backend(&adapter);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Transformer Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| anyhow!("Failed to create device: {}", e))?;

        Ok(Self {
            device,
            queue,
            backend,
        })
    }

    async fn with_backend_async(backend: wgpu::Backends) -> Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: backend,
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| anyhow!("Failed to find adapter for requested backend"))?;

        let backend_type = Self::detect_backend(&adapter);

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Transformer Device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .map_err(|e| anyhow!("Failed to create device: {}", e))?;

        Ok(Self {
            device,
            queue,
            backend: backend_type,
        })
    }

    fn detect_backend(adapter: &wgpu::Adapter) -> BackendType {
        let info = adapter.get_info();
        match info.backend {
            wgpu::Backend::Vulkan => BackendType::Vulkan,
            wgpu::Backend::Metal => BackendType::Metal,
            wgpu::Backend::Dx12 => BackendType::Dx12,
            wgpu::Backend::BrowserWebGpu => BackendType::WebGpu,
            _ => BackendType::Cpu,
        }
    }

    /// Get the backend type being used
    pub fn backend_name(&self) -> &str {
        match self.backend {
            BackendType::Vulkan => "Vulkan",
            BackendType::Metal => "Metal",
            BackendType::Dx12 => "DirectX 12",
            BackendType::WebGpu => "WebGPU",
            BackendType::Cpu => "CPU",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Ignore in CI - requires GPU
    fn test_gpu_context_creation() {
        // This test may fail in CI without GPU
        if let Ok(ctx) = GpuContext::new() {
            println!("Using backend: {}", ctx.backend_name());
        }
    }
}
