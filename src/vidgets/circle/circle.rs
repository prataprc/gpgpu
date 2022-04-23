use bytemuck::{Pod, Zeroable};

use crate::Transforms;

pub struct Circle {
    bg: wgpu::Color,
    fg: wgpu::Color,
    fill: bool,
    radius: f32,
    pipeline: wgpu::RenderPipeline,
    bind_group_0: wgpu::BindGroup,
    bind_group_1: wgpu::BindGroup,
    transform_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, Pod, Zeroable)]
struct UniformBuffer {
    fill: u32,
    radius: f32,
    bg: [f32; 4],
    fg: [f32; 4],
    padding: [f32; 2],
}

impl UniformBuffer {
    const SIZE: usize = 16 + 16 + 4 + 4 + 8;
}

impl<'a> From<&'a Circle> for UniformBuffer {
    fn from(val: &'a Circle) -> Self {
        UniformBuffer {
            bg: [
                val.bg.r as f32,
                val.bg.g as f32,
                val.bg.b as f32,
                val.bg.a as f32,
            ],
            fg: [
                val.fg.r as f32,
                val.fg.g as f32,
                val.fg.b as f32,
                val.fg.a as f32,
            ],
            fill: if val.fill { 1 } else { 0 },
            radius: val.radius,
            padding: [0_f32; 2],
        }
    }
}

impl Circle {
    pub fn new(
        radius: f32,
        format: wgpu::TextureFormat,
        device: &wgpu::Device,
    ) -> Circle {
        let bind_group_layout_0 = Transforms::to_bind_group_layout(device);
        let bind_group_layout_1 = Self::to_bind_group_layout(device);

        let pipeline_layout = {
            let desc = wgpu::PipelineLayoutDescriptor {
                label: Some("vidgets/circle:pipeline-layout"),
                bind_group_layouts: &[&bind_group_layout_0, &bind_group_layout_1],
                push_constant_ranges: &[],
            };
            device.create_pipeline_layout(&desc)
        };

        let module = {
            let text = include_str!("circle.wgsl");
            let desc = wgpu::ShaderModuleDescriptor {
                label: Some("vidgets/circle:shader"),
                source: wgpu::ShaderSource::Wgsl(text.into()),
            };
            device.create_shader_module(&desc)
        };

        let vertex = wgpu::VertexState {
            module: &module,
            entry_point: "vs_main",
            buffers: &[Vertex::to_vertex_buffer_layout()],
        };

        let primitive_state = wgpu::PrimitiveState {
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
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        };

        let pipeline = {
            let desc = wgpu::RenderPipelineDescriptor {
                label: Some("vidgets/circle:pipeline"),
                layout: Some(&pipeline_layout),
                vertex,
                primitive: primitive_state,
                depth_stencil: None,
                multisample,
                fragment: Some(fragment),
                multiview: None,
            };
            device.create_render_pipeline(&desc)
        };

        let transform_buffer = Self::to_transform_buffer(device);
        let bind_group_0 = {
            let desc = wgpu::BindGroupDescriptor {
                label: Some("vidgets/circle:bind-group-0"),
                layout: &bind_group_layout_0,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                }],
            };
            device.create_bind_group(&desc)
        };

        let uniform_buffer = Self::to_uniform_buffer(device);
        let bind_group_1 = {
            let desc = wgpu::BindGroupDescriptor {
                label: Some("vidgets/circle:bind-group-1"),
                layout: &bind_group_layout_1,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
            };
            device.create_bind_group(&desc)
        };

        Circle {
            bg: wgpu::Color::BLACK,
            fg: wgpu::Color::WHITE,
            fill: false,
            radius,
            pipeline,
            bind_group_0,
            bind_group_1,
            transform_buffer,
            uniform_buffer,
        }
    }

    pub fn set_fg(&mut self, fg: wgpu::Color) -> &mut Self {
        self.fg = fg;
        self
    }

    pub fn set_bg(&mut self, bg: wgpu::Color) -> &mut Self {
        self.bg = bg;
        self
    }

    pub fn set_fill(&mut self, fill: bool) -> &mut Self {
        self.fill = fill;
        self
    }

    pub fn render(
        &self,
        transf: &Transforms,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        color_view: &wgpu::TextureView,
    ) {
        use crate::vidgets;

        let vertex_buffer = Self::to_vertex_buffer(device);
        // overwrite the transform mvp buffer.
        {
            let content = transf.to_bind_content();
            queue.write_buffer(&self.transform_buffer, 0, &content);
        }
        // overwrite the transform mvp buffer.
        {
            let ub: UniformBuffer = self.into();
            let content: [u8; UniformBuffer::SIZE] = bytemuck::cast(ub);
            queue.write_buffer(&self.uniform_buffer, 0, &content.to_vec());
        }

        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("vidgets/circle:encoder"),
            };
            device.create_command_encoder(&desc)
        };

        {
            let mut render_pass = {
                let desc = wgpu::RenderPassDescriptor {
                    label: Some("vidgets/circle:render-pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &color_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(vidgets::CLEAR_COLOR),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                };
                encoder.begin_render_pass(&desc)
            };
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.bind_group_0, &[]);
            render_pass.set_bind_group(1, &self.bind_group_1, &[]);
            render_pass.draw(0..6, 0..1);
        }

        let cmd_buffers = vec![encoder.finish()];
        queue.submit(cmd_buffers.into_iter());
    }
}

impl Circle {
    fn to_transform_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::{util::DeviceExt, BufferUsages};

        let content = Transforms::empty().to_bind_content();
        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("transform-buffer"),
            contents: &content,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        };
        device.create_buffer_init(&desc)
    }

    fn to_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        use wgpu::ShaderStages;

        let desc = wgpu::BindGroupLayoutDescriptor {
            label: Some("vidgets/circle:bind-group-layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        };
        device.create_bind_group_layout(&desc)
    }

    fn to_uniform_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::{util::DeviceExt, BufferUsages};

        let contents = {
            let ub = UniformBuffer::default();
            let contents: [u8; UniformBuffer::SIZE] = bytemuck::cast(ub);
            contents.to_vec()
        };
        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("vidgets/circle:uniform-buffer"),
            contents: &contents,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        };
        device.create_buffer_init(&desc)
    }

    fn to_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::{util::DeviceExt, BufferUsages};

        let contents: &[u8] = bytemuck::cast_slice(&VERTICES);
        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("vidgets/circle:vertex-buffer"),
            contents,
            usage: BufferUsages::VERTEX,
        };
        device.create_buffer_init(&desc)
    }
}

const VERTICES: [Vertex; 6] = [
    // lower half triangle
    Vertex {
        position: [-1.0, -1.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0, 1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 1.0, 0.0],
    },
    // upper half triangle
    Vertex {
        position: [-1.0, -1.0, 0.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0, 0.0],
    },
];

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 4],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
        0 => Float32x4,
    ];
}

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
