use bytemuck::{Pod, Zeroable};
use cgmath::{Matrix4, Point3, Vector4};

use std::{fmt, path, result};

use crate::{widg, Error, Result, Transforms};

pub struct Wireframe {
    bg: wgpu::Color,
    primitive: Primitive,
    // wgpu cache objects
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    transform_buffer: wgpu::Buffer,
}

enum Primitive {
    Lines { vertices: Vec<Vertex> },
}

impl fmt::Display for Wireframe {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match &self.primitive {
            Primitive::Lines { vertices } => {
                for (i, v) in vertices.iter().enumerate() {
                    write!(f, "({:4})=> {:?}\n", i, v.position)?;
                }
            }
        }

        Ok(())
    }
}

impl Wireframe {
    pub fn from_file<P>(
        loc: P,
        format: wgpu::TextureFormat,
        device: &wgpu::Device,
    ) -> Result<Wireframe>
    where
        P: AsRef<path::Path>,
    {
        use std::fs;

        let data = err_at!(IOError, fs::read(loc))?;
        Self::from_bytes(&data, format, device)
    }

    pub fn from_bytes(
        data: &[u8],
        format: wgpu::TextureFormat,
        device: &wgpu::Device,
    ) -> Result<Wireframe> {
        use std::str::from_utf8;

        let txt = err_at!(IOError, from_utf8(data))?;
        let mut vertices: Vec<Vertex> = vec![];
        for line in txt.lines() {
            Vertex::from_text_line(line)?.map(|v| vertices.push(v));
        }

        let primitive = Primitive::Lines { vertices };

        let bind_group_layout = {
            let entry = Transforms::to_bind_group_layout_entry();
            let desc = wgpu::BindGroupLayoutDescriptor {
                label: Some("widg/wireframe:bind-group-layout"),
                entries: &[entry],
            };
            device.create_bind_group_layout(&desc)
        };

        let pipeline_layout = {
            let desc = wgpu::PipelineLayoutDescriptor {
                label: Some("widg/wireframe:pipeline-layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            };
            device.create_pipeline_layout(&desc)
        };

        let module = {
            let text = include_str!("wireframe.wgsl");
            let desc = wgpu::ShaderModuleDescriptor {
                label: Some("widg/wireframe:shader"),
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
            topology: wgpu::PrimitiveTopology::LineList,
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
                label: Some("widg/wireframe:pipeline"),
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

        let bind_group = {
            let desc = wgpu::BindGroupDescriptor {
                label: Some("widg/wireframe:bind-group"),
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: transform_buffer.as_entire_binding(),
                }],
            };
            device.create_bind_group(&desc)
        };

        let val = Wireframe {
            bg: wgpu::Color::BLACK,
            pipeline,
            primitive,
            transform_buffer,
            bind_group,
        };

        Ok(val)
    }
}

impl widg::Widget for Wireframe {
    fn render(
        &self,
        context: &widg::Context,
        encoder: &mut wgpu::CommandEncoder,
        target: &widg::ColorTarget,
    ) -> Result<()> {
        let num_vertices = self.num_vertices() as u32;
        let vertex_buffer = self.to_vertex_buffer(context.device);
        // overwrite the transform mvp buffer.
        {
            let content = context.transforms.to_bind_content();
            context
                .queue
                .write_buffer(&self.transform_buffer, 0, &content);
        }

        {
            let mut render_pass = {
                let desc = wgpu::RenderPassDescriptor {
                    label: Some("widg/wireframe:render-pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: target.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.bg),
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
            render_pass.draw(0..num_vertices, 0..1);
        }

        Ok(())
    }
}

impl Wireframe {
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

    fn to_vertex_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;

        match &self.primitive {
            Primitive::Lines { vertices } => {
                let contents: &[u8] = bytemuck::cast_slice(vertices);
                let desc = wgpu::util::BufferInitDescriptor {
                    label: Some("widg/wireframe:vertex-buffer"),
                    contents,
                    usage: wgpu::BufferUsages::VERTEX,
                };
                device.create_buffer_init(&desc)
            }
        }
    }
}

impl Wireframe {
    pub fn num_vertices(&self) -> usize {
        match &self.primitive {
            Primitive::Lines { vertices } => vertices.len(),
        }
    }

    pub fn as_vertices(&self) -> &[Vertex] {
        match &self.primitive {
            Primitive::Lines { vertices } => vertices,
        }
    }

    pub fn transform_mut(&mut self, mat: Matrix4<f32>) -> &mut Self {
        match &mut self.primitive {
            Primitive::Lines { vertices } => vertices
                .iter_mut()
                .for_each(|v| v.position = (mat * Vector4::from(v.position)).into()),
        };
        self
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        0 => Float32x4,
        1 => Float32x4,
    ];

    fn new(position: &[f32], color: &[f32]) -> Result<Vertex> {
        let position: [f32; 4] = match position {
            [x, y, z] => Point3::from((*x, *y, *z)).to_homogeneous().into(),
            [x, y] => Point3::from((*x, *y, 0.0)).to_homogeneous().into(),
            _ => err_at!(Invalid, msg: "invalid position {:?}", position)?,
        };
        let color: [f32; 4] = match color {
            [r, g, b, a] => [*r, *g, *b, *a],
            [r, g, b] => [*r, *g, *b, 1.0],
            [] => [1.0, 1.0, 1.0, 1.0],
            _ => err_at!(Invalid, msg: "invalid color {:?}", color)?,
        };

        Ok(Vertex { position, color })
    }

    fn from_position(position: &[f32]) -> Result<Vertex> {
        Self::new(position, &[1.0, 1.0, 1.0, 1.0])
    }

    fn from_text_line(txt: &str) -> Result<Option<Vertex>> {
        use crate::util;

        match txt.split(";").collect::<Vec<&str>>().as_slice() {
            ["", ""] | [""] | [] => Ok(None),
            [pos, ""] => Some(Vertex::from_position(&util::parse_csv(pos)?)).transpose(),
            [pos, color] => Some(Vertex::new(
                &util::parse_csv(pos)?,
                &util::parse_csv(color)?,
            ))
            .transpose(),
            [pos] => Some(Vertex::from_position(&util::parse_csv(pos)?)).transpose(),
            _ => err_at!(Invalid, msg: "invalid vertex {}", txt),
        }
    }
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
