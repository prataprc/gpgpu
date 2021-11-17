use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    command_buffer::DynamicState,
    image::{attachment::AttachmentImage, view::ImageView, ImageUsage},
    pipeline::{vertex::SingleBufferDefinition, viewport::Viewport, GraphicsPipeline},
    render_pass::{Framebuffer, RenderPass, Subpass},
};

use std::sync::Arc;

use crate::{Error, Result};

pub struct Builder {
    pub device: Arc<vulkano::device::Device>,
    pub format: vulkano::format::Format,
    pub extent: [u32; 2], // width, height
}

pub struct Div {
    // context
    device: Arc<vulkano::device::Device>,
    // parameter
    extent: [u32; 2],
    format: vulkano::format::Format,
    render_pass: Arc<RenderPass>,
    frame_buffer: Framebuffer<((), Arc<ImageView<Arc<AttachmentImage>>>)>,
    pipeline: Arc<GraphicsPipeline<SingleBufferDefinition<DivVertex>>>,
    // output
    image: Arc<AttachmentImage>,
}

impl Builder {
    fn build(self) -> Result<Div> {
        let vertex_buffer = {
            let res = CpuAccessibleBuffer::from_iter(
                self.device.clone(),
                BufferUsage::vertex_buffer(),
                false,
                [
                    DivVertex {
                        position: [-1.0, 1.0],
                    },
                    DivVertex {
                        position: [1.0, 1.0],
                    },
                    DivVertex {
                        position: [-1.0, -1.0],
                    },
                ]
                .iter()
                .cloned(),
            );
            err_at!(Vk, res)
        };

        let vs = vs::Shader::load(self.device.clone()).unwrap();
        let fs = fs::Shader::load(self.device.clone()).unwrap();

        let render_pass = {
            let res = vulkano::ordered_passes_renderpass!(
                self.device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: self.format.clone(),
                        samples: 1,
                    }
                },
                passes: [
                    {
                        color: [color],
                        depth_stencil: {},
                        input: []
                    }
                ]
            );

            Arc::new(err_at!(Vk, res)?)
        };

        let pipeline = {
            let pipeline = GraphicsPipeline::start()
                .vertex_input_single_buffer()
                .vertex_shader(vs.main_entry_point(), ())
                .triangle_list()
                .viewports_dynamic_scissors_irrelevant(1)
                .fragment_shader(fs.main_entry_point(), ())
                .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
                .build(self.device.clone());

            Arc::new(err_at!(Vk, pipeline)?)
        };

        let dynamic_state = DynamicState {
            line_width: None,
            viewports: Some(vec![Viewport {
                origin: [0.0, 0.0],
                dimensions: [self.extent[0] as f32, self.extent[1] as f32],
                depth_range: 0.0..1.0,
            }]),
            scissors: None,
            compare_mask: None,
            write_mask: None,
            reference: None,
        };

        let (frame_buffer, color_attach) = {
            let usage = ImageUsage {
                color_attachment: true,
                transfer_source: true,
                ..ImageUsage::none()
            };

            let color_attach: Arc<AttachmentImage> = err_at!(
                Vk,
                AttachmentImage::with_usage(
                    self.device.clone(),
                    self.extent,
                    self.format,
                    usage
                )
            )?;

            let frame_buffer = {
                let view = err_at!(Vk, ImageView::new(color_attach.clone()))?;
                let builder =
                    err_at!(Vk, Framebuffer::start(render_pass.clone()).add(view))?;
                err_at!(Vk, builder.build())?
            };

            (frame_buffer, color_attach)
        };

        Ok(Div {
            // context
            device: self.device,
            // parameter
            extent: self.extent,
            format: self.format,
            render_pass,
            frame_buffer,
            pipeline,
            // output
            image: color_attach,
        })
    }
}

#[derive(Default, Debug, Clone)]
struct DivVertex {
    position: [f32; 2],
}
vulkano::impl_vertex!(DivVertex, position);

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
#version 450

layout(location = 0) in vec2 position;

void main() {
        gl_Position = vec4(position, 0.0, 1.0);
}
            "
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
#version 450

layout(location = 0) out vec4 f_color;

void main() {
        f_color = vec4(1.0, 0.0, 0.0, 1.0);
}
              "
    }
}
