use crate::wg::{self, Shader};

pub struct ColorShader {
    module: wgpu::ShaderModule,
    pipeline_layout: wgpu::PipelineLayout,
    color_target_states: Vec<wgpu::ColorTargetState>,
}

impl ColorShader {
    pub fn new(device: &wgpu::Device) -> ColorShader {
        let module = {
            let text = include_str!("triangle.wgsl");
            let desc = wgpu::ShaderModuleDescriptor {
                label: Some("Triangle-Shader"),
                source: wgpu::ShaderSource::Wgsl(text.into()),
            };
            device.create_shader_module(&desc)
        };
        let pipeline_layout = {
            let desc = wgpu::PipelineLayoutDescriptor {
                label: Some("Triangle-Pipeline-Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            };
            device.create_pipeline_layout(&desc)
        };
        ColorShader {
            module,
            pipeline_layout,
            color_target_states: Vec::default(),
        }
    }

    pub fn set_color_target_states<I>(&mut self, states: I) -> &mut Self
    where
        I: Iterator<Item = wgpu::ColorTargetState>,
    {
        self.color_target_states = states.collect();
        self
    }
}

impl wg::Shader for ColorShader {
    fn to_render_pipeline(&self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        let vertex = wgpu::VertexState {
            module: &self.module,
            entry_point: "vs_main",
            buffers: &[Vertex::desc()],
        };

        let desc = wgpu::RenderPipelineDescriptor {
            label: Some("Triangle-Pipeline"),
            layout: Some(&self.pipeline_layout),
            fragment: Some(wgpu::FragmentState {
                module: &self.module,
                entry_point: "fs_main",
                targets: self.color_target_states.as_slice(),
            }),
            ..wg::render_pipeline_desc(vertex)
        };
        device.create_render_pipeline(&desc)
    }

    fn to_compute_pipeline(&self, _: &wgpu::Device) -> wgpu::ComputePipeline {
        panic!("Model triangle does not support compute pipeline")
    }
}

pub struct Triangle<S>
where
    S: wg::Shader,
{
    shader: S,
    vertices: Vec<Vertex>,
}

impl<S> Triangle<S>
where
    S: wg::Shader,
{
    pub fn with_shader(shader: S) -> Triangle<S> {
        Triangle {
            shader,
            vertices: Vec::default(),
        }
    }

    pub fn set_vertices(&mut self, vertices: &[Vertex]) -> &mut Self {
        self.vertices = vertices.to_vec();
        self
    }
}

impl wg::Model for Triangle<ColorShader> {
    fn to_pipeline(&self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        self.shader.to_render_pipeline(device)
    }

    fn to_vertex_buffers(&self, device: &wgpu::Device) -> Vec<(usize, wgpu::Buffer)> {
        let vertex_buffer = Vertex::to_buffer(device, self.vertices.as_slice());
        vec![vertex_buffer].into_iter().enumerate().collect()
    }

    fn draw(&self, _: &wgpu::Device, pass: &mut wgpu::RenderPass) {
        pass.draw(0..3, 0..1);
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    pub fn to_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;

        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        };
        device.create_buffer_init(&desc)
    }
}
