use wgpu::Backend;

#[cfg(target_os = "macos")]
pub fn wgpu_backend() -> Backend {
    Backend::Metal
}

#[cfg(target_os = "linux")]
pub fn wgpu_backend() -> Backend {
    Backend::Vulkan
}

pub fn wgpu_backend_to_string(backend: Backend) -> String {
    let s = match backend {
        Backend::Empty => "empty",
        Backend::Vulkan => "vulkan",
        Backend::Metal => "metal",
        Backend::Dx12 => "directx12",
        Backend::Dx11 => "directx11",
        Backend::Gl => "opengl",
        Backend::BrowserWebGpu => "web",
    };

    s.to_string()
}
