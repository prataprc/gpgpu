pub mod pretty;

#[cfg(target_os = "macos")]
pub fn backend() -> wgpu::Backends {
    wgpu::Backends::METAL
}

#[cfg(target_os = "linux")]
pub fn backend() -> wgpu::Backends {
    wgpu::Backends::VULKAN
}
