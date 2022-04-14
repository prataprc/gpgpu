//! Package implement graphics as library on top of [wgpu] library.

mod backends;
mod gpu;
pub mod pretty;

pub use backends::{backend, backend_to_string, string_to_backend};
pub use gpu::Gpu;

pub trait VertexShader {
    fn to_shader_module() -> wgpu::ShaderModule;

    fn to_vertex_buffers_layout<'a>() -> &'a [wgpu::VertexBufferLayout<'a>];
}

pub trait FragmentShader {
    fn to_shader_module() -> wgpu::ShaderModule;
}

pub trait BindGroupInput {
    fn to_bind_group_layouts<'a>(d: &wgpu::Device) -> &'a [&'a wgpu::BindGroupLayout];
}

pub trait Model {
    type Vs: VertexShader;
    type Bg: BindGroupInput;
    type Fs: FragmentShader;

    fn to_primitive_state() -> wgpu::PrimitiveState;

    fn to_multisample_state() -> wgpu::MultisampleState;

    fn set_color_target_state(&mut self, n: usize, state: wgpu::ColorTargetState);

    fn render();
}
