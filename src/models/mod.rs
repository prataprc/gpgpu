pub mod triangle;

pub trait Shader {
    fn to_render_pipeline(&self, device: &wgpu::Device) -> wgpu::RenderPipeline;

    fn to_compute_pipeline(&self, device: &wgpu::Device) -> wgpu::ComputePipeline;
}

pub trait Model {
    fn to_pipeline(&self, device: &wgpu::Device) -> wgpu::RenderPipeline;

    fn to_vertex_buffers(&self, device: &wgpu::Device) -> Vec<(usize, wgpu::Buffer)>;

    fn draw(&self, device: &wgpu::Device, pass: &mut wgpu::RenderPass);
}
