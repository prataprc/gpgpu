use cgmath::{Matrix4, Point3, Vector4};

use std::{fmt, path, result};

use crate::{Error, Result};

pub struct Wireframe {
    format: Option<wgpu::TextureFormat>,
    bg: wgpu::Color,
    pipeline: Option<wgpu::RenderPipeline>,
    primitive: Primitive,
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
    pub fn from_file<P>(loc: P) -> Result<Wireframe>
    where
        P: AsRef<path::Path>,
    {
        use std::fs;

        let data = err_at!(IOError, fs::read(loc))?;
        Self::from_bytes(&data)
    }

    pub fn from_bytes(data: &[u8]) -> Result<Wireframe> {
        use std::str::from_utf8;

        let txt = err_at!(IOError, from_utf8(data))?;
        let mut vertices: Vec<Vertex> = vec![];
        for line in txt.lines() {
            vertices.push(Vertex::from_text_line(line)?);
        }

        let primitive = Primitive::Lines { vertices };
        let val = Wireframe {
            format: None,
            bg: wgpu::Color::default(),
            pipeline: None,
            primitive,
        };

        Ok(val)
    }

    pub fn set_color_format(&mut self, format: wgpu::TextureFormat) -> &mut Self {
        self.format = Some(format);
        self
    }

    pub fn prepare(&mut self, device: &wgpu::Device) -> &mut Self {
        use crate::Transforms;

        let pipeline_layout = {
            let bind_group_layout = Transforms::to_bind_group_layout(device);
            let desc = wgpu::PipelineLayoutDescriptor {
                label: Some("wireframe-pipeline-layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            };
            device.create_pipeline_layout(&desc)
        };

        let module = {
            let text = include_str!("shader.wgsl");
            let desc = wgpu::ShaderModuleDescriptor {
                label: Some("Wireframe-Shader"),
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
            format: self.format.clone().unwrap(),
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        }];
        let fragment = wgpu::FragmentState {
            module: &module,
            entry_point: "fs_main",
            targets: color_target_states.as_slice(),
        };

        let desc = wgpu::RenderPipelineDescriptor {
            label: Some("Wireframe-Pipeline"),
            layout: Some(&pipeline_layout),
            vertex,
            primitive,
            depth_stencil: None,
            multisample,
            fragment: Some(fragment),
            multiview: None,
        };

        self.pipeline = Some(device.create_render_pipeline(&desc));

        self
    }

    pub fn render(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        view: &wgpu::TextureView,
    ) {
        let num_vertices = self.num_vertices() as u32;
        let vertex_buffer = self.to_vertex_buffer(device);
        let mut render_pass = {
            let desc = wgpu::RenderPassDescriptor {
                label: Some("Wireframe-render-pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
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
        render_pass.set_pipeline(self.pipeline.as_ref().unwrap());
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..num_vertices, 0..1);
    }

    pub fn write_back(&mut self) {
        todo!()
    }
}

impl Wireframe {
    pub fn num_vertices(&self) -> usize {
        match &self.primitive {
            Primitive::Lines { vertices } => vertices.len(),
        }
    }

    pub fn transform(&self, mat: Matrix4<f32>) -> Self {
        let primitive = match &self.primitive {
            Primitive::Lines { vertices } => Primitive::Lines {
                vertices: vertices
                    .iter()
                    .map(|v| Vertex {
                        position: (mat * Vector4::from(v.position)).into(),
                        color: v.color,
                    })
                    .collect(),
            },
        };
        Wireframe {
            format: self.format,
            bg: wgpu::Color::default(),
            pipeline: None,
            primitive,
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

impl Wireframe {
    fn to_vertex_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        use wgpu::util::DeviceExt;

        match &self.primitive {
            Primitive::Lines { vertices } => {
                let desc = wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(vertices),
                    usage: wgpu::BufferUsages::VERTEX,
                };
                device.create_buffer_init(&desc)
            }
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

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

    fn from_text_line(txt: &str) -> Result<Vertex> {
        use crate::util;

        match txt.split(";").collect::<Vec<&str>>().as_slice() {
            [pos, color] => Vertex::new(&util::parse_csv(pos)?, &util::parse_csv(color)?),
            [pos] => Vertex::from_position(&util::parse_csv(pos)?),
            _ => err_at!(Invalid, msg: "invalid vertex {}", txt),
        }
    }
}

impl Vertex {
    fn to_vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex> as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
