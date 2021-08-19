use std::sync::Arc;

fn main() {
    use vgi::{layers, Builder};
    use vulkano::buffer::{cpu_access::CpuAccessibleBuffer, BufferUsage};

    // create the vulkan container
    let mut vk = {
        let layers = layers().expect("fail querying available layers");
        Builder::new()
            .unwrap()
            .with_layers(layers.iter().map(|l| l.name().to_string()))
            .with_extensions(None)
            .build_for_surface(vulkano_win::required_extensions())
            .expect("fail creating Vulkan instance/device")
    };

    // create the swapchain
    vk.create_swapchain(None)
        .expect("fail creating the swap chain");

    // We now create a buffer that will store the shape of our triangle.
    let vertex_buffer = {
        #[derive(Default, Debug, Clone)]
        struct Vertex {
            position: [f32; 2],
        }
        vulkano::impl_vertex!(Vertex, position);

        CpuAccessibleBuffer::from_iter(
            vk.to_device(),
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

    mod vs {
        vulkano_shaders::shader! { ty: "vertex", path: "./src/bin/triangle/vs.glsl" }
    }
    mod fs {
        vulkano_shaders::shader! { ty: "fragment", path: "src/bin/triangle/fs.glsl" }
    }

    let vs = vs::Shader::load(vk.to_device()).unwrap();
    let fs = fs::Shader::load(vk.to_device()).unwrap();

    // The next step is to create a *render pass*, which is an object that describes where the
    // output of the graphics pipeline will go. It describes the layout of the images
    // where the colors, depth and/or stencil information will be written.

    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(
            vk.to_device(),
            attachments: {
                // `color` is a custom name we give to the first and only attachment.
                color: {
                    // `load: Clear` means that we ask the GPU to clear the content of this
                    // attachment at the start of the drawing.
                    load: Clear,
                    // `store: Store` means that we ask the GPU to store the output of the draw
                    // in the actual image. We could also ask it to discard the result.
                    store: Store,
                    // `format: <ty>` indicates the type of the format of the image. This has to
                    // be one of the types of the `vulkano::format` module (or alternatively one
                    // of your structs that implements the `FormatDesc` trait). Here we use the
                    // same format as the swapchain.
                    format: vk.to_swapchain().format(),
                    // TODO:
                    samples: 1,
                }
            },
            pass: {
                // We use the attachment named `color` as the one and only color attachment.
                color: [color],
                // No depth-stencil attachment is indicated with empty brackets.
                depth_stencil: {}
            }
        )
        .unwrap(),
    );
}
