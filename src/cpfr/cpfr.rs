use crate::Result;

pub struct Cpfr {
    bind_group_layout: wgpu::BindGroupLayout,
    pipeline: wgpu::RenderPipeline,
}

impl Cpfr {
    fn to_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        let sample_type = wgpu::TextureSampleType::Float { filterable: true };
        let ty = wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering);
        let desc = wgpu::BindGroupLayoutDescriptor {
            label: Some("cpfr-bind-group-layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    // This should match the filterable field of the
                    // corresponding Texture entry above.
                    ty,
                    count: None,
                },
            ],
        };

        device.create_bind_group_layout(&desc)
    }

    fn to_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;

        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("cpfr-vertex-buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        };
        device.create_buffer_init(&desc)
    }
}

impl Cpfr {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Result<Cpfr> {
        let bind_group_layout = Self::to_bind_group_layout(device);

        let pipeline_layout = {
            let desc = wgpu::PipelineLayoutDescriptor {
                label: Some("cpfr-pipeline-layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            };
            device.create_pipeline_layout(&desc)
        };

        let module = {
            let text = include_str!("cpfr.wgsl");
            let desc = wgpu::ShaderModuleDescriptor {
                label: Some("cpfr-shader"),
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

        let color_target_states = vec![wgpu::ColorTargetState {
            format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        }];
        let fragment = wgpu::FragmentState {
            module: &module,
            entry_point: "fs_main",
            targets: color_target_states.as_slice(),
        };

        let desc = wgpu::RenderPipelineDescriptor {
            label: Some("cpfr-pipeline"),
            layout: Some(&pipeline_layout),
            vertex,
            primitive,
            depth_stencil: None,
            multisample,
            fragment: Some(fragment),
            multiview: None,
        };

        let pipeline = device.create_render_pipeline(&desc);

        let val = Cpfr {
            bind_group_layout,
            pipeline,
        };

        Ok(val)
    }

    pub fn render(
        &self,
        frame: &wgpu::Texture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color_view: &wgpu::TextureView,
    ) {
        let vertex_buffer = Self::to_vertex_buffer(device);

        let frame_view = {
            let desc = wgpu::TextureViewDescriptor::default();
            frame.create_view(&desc)
        };
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
            device.create_sampler(&desc)
        };

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cpfr-bind-group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&frame_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&frame_samp),
                },
            ],
        });

        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("cpfr-command-encoder"),
            };
            device.create_command_encoder(&desc)
        };

        {
            let mut render_pass = {
                let desc = wgpu::RenderPassDescriptor {
                    label: Some("cpfr-render-pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: color_view,
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
        }

        let cmd_buffers = vec![encoder.finish()];
        queue.submit(cmd_buffers.into_iter());
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
