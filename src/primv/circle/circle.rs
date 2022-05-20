use bytemuck::{Pod, Zeroable};
use cgmath::Point2;

use crate::{BoxVertex, ColorTarget, Context, Extent, Result, Transforms};

pub struct Circle {
    scale_factor: f32, // default is crate::SCALE_FACTOR
    attrs: Attributes,
    computed_attrs: Attributes,
    // wgpu items
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    transform_buffer: wgpu::Buffer,
    uniform_buffer: wgpu::Buffer,
}

/// measurements are in pixels.
#[derive(Copy, Clone, Debug)]
pub struct Attributes {
    pub origin: Point2<f32>, // top-left position in screen-coordinates
    pub radius: f32,         // in pixels
    pub width: f32,          // in pixels
    pub fill: bool,
    pub fg: wgpu::Color,
    pub bg: wgpu::Color,
}

impl Default for Attributes {
    fn default() -> Attributes {
        Attributes {
            origin: (0.0, 0.0).into(),
            radius: 1.0,
            width: 1.0,
            fill: false,
            fg: wgpu::Color::WHITE,
            bg: wgpu::Color::BLACK,
        }
    }
}

impl Attributes {
    fn computed(&self, scale_factor: f32) -> Self {
        Attributes {
            origin: self.origin * scale_factor,
            radius: self.radius * scale_factor,
            width: self.width * scale_factor,
            ..*self
        }
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug, Pod, Zeroable)]
struct UniformBuffer {
    fg: [f32; 4],
    bg: [f32; 4],
    center: [f32; 2],
    radius: f32,
    width: f32,
    fill: u32,
    _padding: [f32; 3],
}

impl UniformBuffer {
    const SIZE: usize = 4 * 4 + 4 * 4 + 4 * 2 + 4 + 4 + 4 + 4 * 3;
}

impl Circle {
    pub fn new(
        attrs: Attributes,
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
    ) -> Circle {
        use std::borrow::Cow;

        let bind_group_layout = Self::to_bind_group_layout(device);

        let pipeline_layout = {
            let desc = wgpu::PipelineLayoutDescriptor {
                label: Some("primv/circle:pipeline-layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            };
            device.create_pipeline_layout(&desc)
        };

        let module = {
            let text = Cow::Borrowed(include_str!("circle.wgsl"));
            let desc = wgpu::ShaderModuleDescriptor {
                label: Some("primv/circle:shader"),
                source: wgpu::ShaderSource::Wgsl(text.into()),
            };
            device.create_shader_module(&desc)
        };

        let vertex = wgpu::VertexState {
            module: &module,
            entry_point: "vs_main",
            buffers: &[BoxVertex::to_vertex_buffer_layout()],
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
                label: Some("primv/circle:pipeline"),
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
        let uniform_buffer = Self::to_uniform_buffer(device);

        let bind_group = {
            let desc = wgpu::BindGroupDescriptor {
                label: Some("primv/circle:bind-group"),
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: transform_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                ],
            };
            device.create_bind_group(&desc)
        };

        Circle {
            scale_factor: crate::DEFAULT_SCALE_FACTOR,
            attrs,
            computed_attrs: attrs,
            // wgpu items
            pipeline,
            bind_group,
            transform_buffer,
            uniform_buffer,
        }
    }

    pub fn print(&self, prefix: &str) {
        println!(
            "{}primv::Circle({},{})",
            prefix, self.attrs.radius, self.attrs.width
        );
    }
}

impl Circle {
    pub fn to_extent(&self) -> Extent {
        let diameter = self.computed_attrs.radius * 2.0;
        Extent {
            width: diameter,
            height: diameter,
        }
    }

    pub fn resize(&mut self, _: Extent, scale_factor: Option<f32>) -> &mut Self {
        if let Some(scale_factor) = scale_factor {
            self.scale_factor = scale_factor;
            self.computed_attrs = self.attrs.computed(self.scale_factor);
        }
        self
    }

    pub fn redraw(
        &mut self,
        context: &Context,
        encoder: &mut wgpu::CommandEncoder,
        target: &mut ColorTarget,
    ) -> Result<()> {
        // debug!("Settings view port for circle {:?}", target.view_port);

        let vertex_buffer = self.to_vertex_buffer(&context.device);
        // overwrite the transform mvp buffer.
        {
            let content = context.transforms.to_bind_content();
            context
                .queue
                .write_buffer(&self.transform_buffer, 0, &content);
        }
        // overwrite the uniform buffer
        {
            use crate::to_rgba8unorm_color;
            use cgmath::Vector2;

            let ca = &self.computed_attrs;
            let ub = UniformBuffer {
                center: (ca.origin + Vector2::from((ca.radius, ca.radius))).into(),
                radius: ca.radius,
                width: ca.width,
                fill: if ca.fill { 1 } else { 0 },
                fg: to_rgba8unorm_color(ca.fg),
                bg: to_rgba8unorm_color(ca.bg),
                _padding: Default::default(),
            };
            let content: [u8; UniformBuffer::SIZE] = bytemuck::cast(ub);
            context
                .queue
                .write_buffer(&self.uniform_buffer, 0, &content.to_vec());
        }

        let mut render_pass = {
            let desc = wgpu::RenderPassDescriptor {
                label: Some("primv/circle:render-pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &target.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            };
            encoder.begin_render_pass(&desc)
        };
        target.view_port.set_viewport(&mut render_pass);
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

    fn to_uniform_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::{util::DeviceExt, BufferUsages};

        let contents = {
            let ub = UniformBuffer::default();
            let contents: [u8; UniformBuffer::SIZE] = bytemuck::cast(ub);
            contents.to_vec()
        };
        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("primv/circle:uniform-buffer"),
            contents: &contents,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        };
        device.create_buffer_init(&desc)
    }

    fn to_vertex_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::{util::DeviceExt, BufferUsages};

        let vertices = [
            BoxVertex {
                position: [-1.0, 1.0, 0.0, 1.0],
            },
            BoxVertex {
                position: [-1.0, -1.0, 0.0, 1.0],
            },
            BoxVertex {
                position: [1.0, 1.0, 0.0, 1.0],
            },
            BoxVertex {
                position: [1.0, 1.0, 0.0, 1.0],
            },
            BoxVertex {
                position: [-1.0, -1.0, 0.0, 1.0],
            },
            BoxVertex {
                position: [1.0, -1.0, 0.0, 1.0],
            },
        ];
        let contents: &[u8] = bytemuck::cast_slice(&vertices);
        let desc = wgpu::util::BufferInitDescriptor {
            label: Some("primv/circle:vertex-buffer"),
            contents,
            usage: BufferUsages::VERTEX,
        };
        device.create_buffer_init(&desc)
    }

    fn to_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
        use wgpu::ShaderStages;

        let desc = wgpu::BindGroupLayoutDescriptor {
            label: Some("primv/circle:bind-group-layout"),
            entries: &[
                Transforms::to_bind_group_layout_entry(0),
                // uniform-buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
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
