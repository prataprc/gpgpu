use wgpu::Backend;

#[cfg(target_os = "macos")]
pub fn backend() -> Backend {
    Backend::Metal
}

#[cfg(target_os = "linux")]
pub fn backend() -> Backend {
    Backend::Vulkan
}

pub fn backend_to_string(backend: Backend) -> String {
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