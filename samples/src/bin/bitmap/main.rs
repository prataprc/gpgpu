use vulkano::{
    buffer::{cpu_access::CpuAccessibleBuffer, BufferUsage},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, DynamicState, PrimaryCommandBuffer,
        SubpassContents,
    },
    format::Format,
    image::view::ImageView,
    pipeline::{viewport::Viewport, GraphicsPipeline},
    render_pass::{Framebuffer, FramebufferAbstract, Subpass},
    sync::GpuFuture,
};

use std::sync::Arc;

use vgi::{layers, Builder};

fn main() {
    let width = 800;
    let height = 600;
    let format = Format::R8G8B8A8Srgb;
    let dimensions: [u32; 2] = [width, height]; // (width, height)

    // create the vulkan container
    let vko = {
        let layers = layers().expect("fail querying available layers");
        Builder::new()
            .unwrap()
            .with_layers(layers.iter().map(|l| l.name().to_string()))
            .with_extensions(None)
            .build_for_buffer(dimensions, format)
            .expect("fail creating Vulkan instance/device")
    };

    // We now create a buffer that will store the shape of our triangle.
    let vertex_buffer = {
        #[derive(Default, Debug, Clone)]
        struct Vertex {
            position: [f32; 2],
        }
        vulkano::impl_vertex!(Vertex, position);

        CpuAccessibleBuffer::from_iter(
            vko.to_device(),
            BufferUsage::vertex_buffer(),
            false,
            [
                Vertex {
                    position: [-0.5, -0.25],
                },
                Vertex {
                    position: [0.0, 0.5],
                },
                Vertex {
                    position: [0.25, -0.1],
                },
            ]
            .iter()
            .cloned(),
        )
        .unwrap()
    };

    let vs = vs::Shader::load(vko.to_device()).unwrap();
    let fs = fs::Shader::load(vko.to_device()).unwrap();

    // The next step is to create a *render pass*, which is an object that
    // describes where the output of the graphics pipeline will go. It describes
    // the layout of the images where the colors, depth and/or stencil information
    // will be written.

    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(
            vko.to_device(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: format,
                    samples: 1,
                }
            },
            pass: {
                // We use the attachment named `color` as the one and only
                // color attachment.
                color: [color],
                // No depth-stencil attachment is indicated with empty brackets.
                depth_stencil: {}
            }
        )
        .unwrap(),
    );

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(vko.to_device())
            .unwrap(),
    );

    let dynamic_state = DynamicState {
        line_width: None,
        viewports: Some(vec![Viewport {
            origin: [0.0, 0.0],
            dimensions: [width as f32, height as f32],
            depth_range: 0.0..1.0,
        }]),
        scissors: None,
        compare_mask: None,
        write_mask: None,
        reference: None,
    };

    let image = vko.to_image();
    let framebuffer = {
        let view = ImageView::new(image.clone()).unwrap();
        Arc::new(
            Framebuffer::start(render_pass.clone())
                .add(view)
                .unwrap()
                .build()
                .unwrap(),
        ) as Arc<dyn FramebufferAbstract + Send + Sync>
    };

    let queue = Arc::clone(vko.to_queues().iter().next().unwrap());
    let clear_values = vec![[0.0, 0.0, 1.0, 1.0].into()];

    let buf = CpuAccessibleBuffer::from_iter(
        vko.to_device(),
        BufferUsage::all(),
        false,
        (0..(width * height * 4)).map(|_| 0_u8),
    )
    .unwrap();

    let command_buffer1 = {
        let mut builder = AutoCommandBufferBuilder::primary(
            vko.to_device(),
            queue.family(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        builder
            .begin_render_pass(framebuffer.clone(), SubpassContents::Inline, clear_values)
            .unwrap()
            .draw(
                pipeline.clone(),
                &dynamic_state,
                vertex_buffer.clone(),
                (),
                (),
                vec![],
            )
            .unwrap()
            .end_render_pass()
            .unwrap();
        builder.build().unwrap()
    };
    let command_buffer2 = {
        let mut builder = AutoCommandBufferBuilder::primary(
            vko.to_device(),
            queue.family(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        builder
            .copy_image_to_buffer(image.clone(), buf.clone())
            .unwrap();
        builder.build().unwrap()
    };

    // Finish building the command buffer by calling `build`.
    command_buffer1
        .execute(queue.clone())
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();
    command_buffer2
        .execute(queue.clone())
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    {
        use image::{ImageBuffer, Rgba};
        let data = buf.read().unwrap();
        let image =
            ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, &data[..]).unwrap();
        image.save("image.png").unwrap();
    }
}

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
