pub fn render_pipeline(
    device: &wgpu::Device,
    format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
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
    let color_target_states = vec![wgpu::ColorTargetState {
        format: format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
    }];
    let vertex = wgpu::VertexState {
        module: &module,
        entry_point: "vs_main",
        buffers: &[Vertex::desc()],
    };

    let desc = wgpu::RenderPipelineDescriptor {
        label: Some("Triangle-Pipeline"),
        layout: Some(&pipeline_layout),
        vertex,
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: &module,
            entry_point: "fs_main",
            targets: color_target_states.as_slice(),
        }),
        multiview: None,
    };
    device.create_render_pipeline(&desc)
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
