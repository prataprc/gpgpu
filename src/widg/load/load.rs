use crate::{widg, Error, Result};

pub struct Load {
    source: Option<wgpu::TextureView>,
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
}

impl Load {
    pub fn new(
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
    ) -> Result<Load> {
        let bind_group_layout = Self::to_bind_group_layout(device);

        let pipeline_layout = {
            let desc = wgpu::PipelineLayoutDescriptor {
                label: Some("widgets/load:pipeline-layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            };
            device.create_pipeline_layout(&desc)
        };

        let module = {
            let text = include_str!("load.wgsl");
            let desc = wgpu::ShaderModuleDescriptor {
                label: Some("widgets/load:shader"),
                source: wgpu::ShaderSource::Wgsl(text.into()),
            };
            device.create_shader_module(&desc)
        };

        let vertex = wgpu::VertexState {
            module: &module,
            entry_point: "vs_main",
            buffers: &[Vertex::to_vertex_buffer_layout()],
        };

        let primitive = wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        };

        let multisample = wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        };

        let fragment = wgpu::FragmentState {
            module: &module,
            entry_point: "fs_main",
            targets: &[wgpu::ColorTargetState {
                format: target_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        };

        let pipeline = {
            let desc = wgpu::RenderPipelineDescriptor {
                label: Some("widgets/load:pipeline"),
                layout: Some(&pipeline_layout),
                vertex,
                primitive,
                depth_stencil: None,
                multisample,
                fragment: Some(fragment),
                multiview: None,
            };
            device.create_render_pipeline(&desc)
        };

        let val = Load {
            source: None,
            bind_group_layout,
            pipeline,
        };

        Ok(val)
    }

    pub fn set_source(&mut self, src: wgpu::TextureView) -> Option<wgpu::TextureView> {
        self.source.replace(src)
    }
}

impl widg::Widget for Load {
    fn render(
        &self,
        context: &widg::Context,
        encoder: &mut wgpu::CommandEncoder,
        target: &widg::ColorTarget,
    ) -> Result<()> {
        let source = match self.source.as_ref() {
            Some(source) => source,
            None => err_at!(Fatal, msg: "set source frame-view for loading")?,
        };

        let vertex_buffer = Self::to_vertex_buffer(context.device);
        let bind_group = {
            let frame_samp = {
                let desc = wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Linear,
                    min_filter: wgpu::FilterMode::Linear,
                    mipmap_filter: wgpu::FilterMode::Linear,
                    ..Default::default()
                };
                context.device.create_sampler(&desc)
            };
            let desc = wgpu::BindGroupDescriptor {
                label: Some("widgets/load:bind-group"),
                layout: &self.bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(source),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(&frame_samp),
                    },
                ],
            };
            context.device.create_bind_group(&desc)
        };

        let mut render_pass = {
            let desc = wgpu::RenderPassDescriptor {
                label: Some("widgets/load:render-pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: target.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            };
            encoder.begin_render_pass(&desc)
        };

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.draw(0..Vertex::NUM_VERTICES, 0..1);

        Ok(())
    }
}

impl Load {
    fn to_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        use wgpu::{
            BindingType, SamplerBindingType, TextureSampleType, TextureViewDimension,
        };

        let desc = wgpu::BindGroupLayoutDescriptor {
            label: Some("widgets/load:bind-group-layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: TextureViewDimension::D2,
                        sample_type: TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        };

        device.create_bind_group_layout(&desc)
    }

    fn to_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;

        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("widgets/load:vertex-buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        };
        device.create_buffer_init(&desc)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 4],
    tex_coord: [f32; 2],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x4,
        1 => Float32x2,
    ];
    const NUM_VERTICES: u32 = 6;
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 01.0, 1.0, 01.0],
        tex_coord: [0.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0, 01.0],
        tex_coord: [0.0, 1.0],
    },
    Vertex {
        position: [01.0, 01.0, 1.0, 01.0],
        tex_coord: [1.0, 0.0],
    },
    Vertex {
        position: [01.0, 01.0, 1.0, 01.0],
        tex_coord: [1.0, 0.0],
    },
    Vertex {
        position: [-1.0, -1.0, 1.0, 01.0],
        tex_coord: [0.0, 1.0],
    },
    Vertex {
        position: [01.0, -1.0, 1.0, 01.0],
        tex_coord: [1.0, 1.0],
    },
];

impl Vertex {
    fn to_vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
