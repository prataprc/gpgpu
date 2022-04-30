use bytemuck::{Pod, Zeroable};

use crate::{dom, widg, Result, Transforms};

pub struct Circle {
    state: State,
    // wgpu buffers
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    transform_buffer: wgpu::Buffer,
    style_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
}

struct State {
    fill: bool,
    radius: f32,
    center: [f32; 2],
    style: dom::Style,
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, Pod, Zeroable)]
struct UniformBuffer {
    fill: u32,
    radius: f32,
    center: [f32; 2],
}

impl UniformBuffer {
    const SIZE: usize = 4 + 4 + 8;

    fn update_size(&mut self, size: wgpu::Extent3d) {
        let wgpu::Extent3d { width, height, .. } = size;
        let w = (width as f32) / 2.0;
        let h = (height as f32) / 2.0;

        self.radius = (self.radius * w).round();
        self.center = [(self.center[0] * w) + w, h - (self.center[1] * h)];
    }
}

impl<'a> From<&'a Circle> for UniformBuffer {
    fn from(val: &'a Circle) -> Self {
        UniformBuffer {
            fill: if val.state.fill { 1 } else { 0 },
            radius: val.state.radius,
            center: val.state.center,
        }
    }
}

impl Circle {
    pub fn new(
        device: &wgpu::Device,
        radius: f32,
        center: [f32; 2],
        target_format: wgpu::TextureFormat,
    ) -> Circle {
        use std::borrow::Cow;

        let bind_group_layout = Self::to_bind_group_layout(device);

        let pipeline_layout = {
            let desc = wgpu::PipelineLayoutDescriptor {
                label: Some("widg/circle:pipeline-layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            };
            device.create_pipeline_layout(&desc)
        };

        let module = {
            let text = Cow::Borrowed(include_str!("circle.wgsl"));
            let desc = wgpu::ShaderModuleDescriptor {
                label: Some("widg/circle:shader"),
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
                format: target_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            }],
        };

        let pipeline = {
            let desc = wgpu::RenderPipelineDescriptor {
                label: Some("widg/circle:pipeline"),
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
        let style_buffer = Self::to_style_buffer(device);
        let uniform_buffer = Self::to_uniform_buffer(device);

        let bind_group = {
            let desc = wgpu::BindGroupDescriptor {
                label: Some("widg/circle:bind-group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: transform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: style_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                ],
            };
            device.create_bind_group(&desc)
        };

        let style = dom::Style::default();
        Circle {
            state: State {
                fill: true,
                radius,
                center,
                style,
            },
            pipeline,
            bind_group,
            transform_buffer,
            style_buffer,
            uniform_buffer,
        }
    }

    pub fn set_fg(&mut self, fg: wgpu::Color) -> &mut Self {
        self.state.style.fg = Some(fg);
        self
    }

    pub fn set_bg(&mut self, bg: wgpu::Color) -> &mut Self {
        self.state.style.bg = Some(bg);
        self
    }

    pub fn set_fill(&mut self, fill: bool) -> &mut Self {
        self.state.fill = fill;
        self
    }
}

impl widg::Widget for Circle {
    fn render(
        &self,
        context: &widg::Context,
        encoder: &mut wgpu::CommandEncoder,
        target: &widg::ColorTarget,
    ) -> Result<()> {
        let vertex_buffer = Self::to_vertex_buffer(context.device);
        // overwrite the transform mvp buffer.
        {
            let content = context.transforms.to_bind_content();
            context
                .queue
                .write_buffer(&self.transform_buffer, 0, &content);
        }
        // overwrite the style buffer
        {
            let content = self.state.style.to_bind_content();
            context.queue.write_buffer(&self.style_buffer, 0, &content);
        }
        // overwrite the uniform buffer
        {
            let mut ub: UniformBuffer = self.into();
            ub.update_size(target.size);
            let content: [u8; UniformBuffer::SIZE] = bytemuck::cast(ub);
            context
                .queue
                .write_buffer(&self.uniform_buffer, 0, &content.to_vec());
        }

        let mut render_pass = {
            let desc = wgpu::RenderPassDescriptor {
                label: Some("widg/circle:render-pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: target.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(widg::CLEAR_COLOR),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            };
            encoder.begin_render_pass(&desc)
        };
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..6, 0..1);

        Ok(())
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

    fn to_style_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::{util::DeviceExt, BufferUsages};

        let content = dom::Style::default().to_bind_content();
        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("style-buffer"),
            contents: &content,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        };
        device.create_buffer_init(&desc)
    }

    fn to_uniform_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::{util::DeviceExt, BufferUsages};

        let contents = {
            let ub = UniformBuffer::default();
            let contents: [u8; UniformBuffer::SIZE] = bytemuck::cast(ub);
            contents.to_vec()
        };
        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("widg/circle:uniform-buffer"),
            contents: &contents,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        };
        device.create_buffer_init(&desc)
    }

    fn to_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::{util::DeviceExt, BufferUsages};

        let contents: &[u8] = bytemuck::cast_slice(&VERTICES);
        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("widg/circle:vertex-buffer"),
            contents,
            usage: BufferUsages::VERTEX,
        };
        device.create_buffer_init(&desc)
    }

    fn to_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        use wgpu::ShaderStages;

        let entry_0 = Transforms::to_bind_group_layout_entry(0);
        let entry_1 = dom::Style::to_bind_group_layout_entry(1);
        let desc = wgpu::BindGroupLayoutDescriptor {
            label: Some("widg/circle:bind-group-layout"),
            entries: &[
                entry_0,
                entry_1,
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        };
        device.create_bind_group_layout(&desc)
    }
}

const VERTICES: [Vertex; 6] = [
    // lower half triangle
    Vertex {
        position: [-1.0, -1.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0, 1.0],
    },
    // upper half triangle
    Vertex {
        position: [-1.0, -1.0, 0.0, 1.0],
    },
    Vertex {
        position: [1.0, 1.0, 0.0, 1.0],
    },
    Vertex {
        position: [-1.0, 1.0, 0.0, 1.0],
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
