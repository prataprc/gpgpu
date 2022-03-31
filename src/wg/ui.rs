pub struct Triangle {
    bind_group_layouts: Vec<wgpu::BindGroupLayout>,
    push_constants: Vec<wgpu::PushConstantRange>,
    module: Option<wgpu::ShaderModule>,
    pipeline_layout: Option<wgpu::PipelineLayout>,
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle {
            bind_group_layouts: Vec::default(),
            push_constants: Vec::default(),
            module: None,
            pipeline_layout: None,
        }
    }

    pub fn set_bind_group_layouts<I>(&mut self, layouts: I) -> &mut Self
    where
        I: Iterator<Item = wgpu::BindGroupLayout>,
    {
        self.bind_group_layouts = layouts.collect();
        self
    }

    pub fn set_push_constants<I>(&mut self, constants: I) -> &mut Self
    where
        I: Iterator<Item = wgpu::PushConstantRange>,
    {
        self.push_constants = constants.collect();
        self
    }

    pub fn finalize(&mut self, device: &wgpu::Device) -> &Self {
        let bind_group_layouts = self
            .bind_group_layouts
            .iter()
            .collect::<Vec<&wgpu::BindGroupLayout>>();
        let push_constants = self.push_constants.clone();

        self.pipeline_layout = {
            let desc = wgpu::PipelineLayoutDescriptor {
                label: Some("Triangle-Pipeline-Layout"),
                bind_group_layouts: bind_group_layouts.as_slice(),
                push_constant_ranges: push_constants.as_slice(),
            };
            Some(device.create_pipeline_layout(&desc))
        };

        self.module = {
            let desc = wgpu::ShaderModuleDescriptor {
                label: Some("Triangle-Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("triangle.wgsl").into()),
            };
            Some(device.create_shader_module(&desc))
        };

        self
    }
}

impl Triangle {
    pub fn create_pipeline(&self) -> TrianglePipeline {
        let module = self.module.as_ref().unwrap();
        let layout = self.pipeline_layout.as_ref();

        let desc = wgpu::RenderPipelineDescriptor {
            label: Some("Triangle-Pipeline"),
            layout,
            vertex: wgpu::VertexState {
                module: &module,
                entry_point: "vs_main",
                buffers: &[],
            },
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
                module,
                entry_point: "fs_main",
                targets: &[],
            }),
            multiview: None,
        };

        TrianglePipeline {
            desc,
            color_target_states: Vec::default(),
            pipeline: None,
        }
    }
}

pub struct TrianglePipeline<'a> {
    desc: wgpu::RenderPipelineDescriptor<'a>,
    color_target_states: Vec<wgpu::ColorTargetState>,
    pipeline: Option<wgpu::RenderPipeline>,
}

impl<'a> TrianglePipeline<'a> {
    pub fn set_color_target_states<I>(&mut self, states: I) -> &mut Self
    where
        I: Iterator<Item = wgpu::ColorTargetState>,
    {
        self.color_target_states = states.collect();
        self
    }

    pub fn finalize(&mut self, device: &wgpu::Device) -> &Self {
        let color_target_states = self.color_target_states.clone();

        let mut desc = self.desc.clone();
        desc.fragment.as_mut().unwrap().targets = color_target_states.as_slice();
        self.pipeline = Some(device.create_render_pipeline(&desc));
        self
    }

    pub fn as_render_pipeline(&self) -> &wgpu::RenderPipeline {
        self.pipeline.as_ref().unwrap()
    }
}
