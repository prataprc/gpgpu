use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState, SubpassContents};
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass};
use vulkano::image::view::ImageView;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::swapchain;
use vulkano::swapchain::{
    AcquireError, ColorSpace, FullscreenExclusive, PresentMode, Surface, SurfaceTransform,
    Swapchain, SwapchainCreationError,
};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};

use vulkano_win::VkSurfaceBuild;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use std::{mem::MaybeUninit, sync::Arc};

use crate::Opt;

struct Vk<'a> {
    instance: Arc<Instance>,
    physical: PhysicalDevice<'a>,
    device: MaybeUninit<Arc<Device>>,
    queue: MaybeUninit<Arc<Queue>>,
}

impl<'a> Vk<'a> {
    fn new(instance: Arc<Instance>, physical: PhysicalDevice<'a>) -> Self {
        Vk {
            instance,
            physical,
            device: unsafe { MaybeUninit::uninit().assume_init() },
            queue: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }

    fn create_device_queue(&mut self, surface: &Arc<Surface<Window>>) {
        let queue_family = self
            .physical
            .queue_families()
            .find(|&q| {
                // We take the first queue that supports drawing to our window.
                q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
            })
            .unwrap();

        let device_ext = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::none()
        };
        let (device, mut queues) = Device::new(
            self.physical,
            self.physical.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();
        self.device = MaybeUninit::new(device);
        self.queue = MaybeUninit::new(queues.next().unwrap());
    }

    fn clone_device(&self) -> Arc<Device> {
        unsafe { Arc::clone(self.device.assume_init_ref()) }
    }

    fn clone_queue(&self) -> Arc<Queue> {
        unsafe { Arc::clone(self.queue.assume_init_ref()) }
    }

    fn as_queue(&self) -> &Queue {
        unsafe { self.queue.assume_init_ref() }
    }

    fn print_info(&self) {
        let (name, ty) = (self.physical.name(), self.physical.ty());
        println!("Using device: {} (type: {:?})", name, ty);
    }
}

fn get_physical_device_optimal<'a>(instance: *const Arc<Instance>) -> PhysicalDevice<'a> {
    let instance = unsafe { instance.as_ref().unwrap() };
    PhysicalDevice::enumerate(&instance).next().unwrap()
}

pub fn main_loop(_opts: Opt) {
    let mut vk = {
        let required_extensions = vulkano_win::required_extensions();
        let instance = Instance::new(None, &required_extensions, None).unwrap();
        let physical = get_physical_device_optimal(&instance as *const Arc<Instance>);
        Vk::new(instance, physical)
    };

    vk.print_info();

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, vk.instance.clone())
        .unwrap();

    vk.create_device_queue(&surface);

    let (mut swapchain, images) = {
        let caps = surface.capabilities(vk.physical).unwrap();
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;
        let dimensions: [u32; 2] = surface.window().inner_size().into();

        Swapchain::new(
            vk.clone_device(),
            surface.clone(),
            caps.min_image_count,
            format,
            dimensions,
            1,
            ImageUsage::color_attachment(),
            &vk.clone_queue(),
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            FullscreenExclusive::Default,
            true,
            ColorSpace::SrgbNonLinear,
        )
        .unwrap()
    };

    let vertex_buffer = {
        #[derive(Default, Debug, Clone)]
        struct Vertex {
            position: [f32; 2],
        }
        vulkano::impl_vertex!(Vertex, position);

        CpuAccessibleBuffer::from_iter(
            vk.clone_device(),
            BufferUsage::all(),
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

    let vs = vs::Shader::load(vk.clone_device()).unwrap();
    let fs = fs::Shader::load(vk.clone_device()).unwrap();

    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(
            vk.clone_device(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
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
            .build(vk.clone_device())
            .unwrap(),
    );

    let mut dynamic_state = DynamicState {
        line_width: None,
        viewports: None,
        scissors: None,
        compare_mask: None,
        write_mask: None,
        reference: None,
    };

    let mut framebuffers =
        window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);
    let mut recreate_swapchain = false;
    let mut previous_frame_end = Some(sync::now(vk.clone_device()).boxed());

    event_loop.run(move |event, _, controlf| {
        println!("{:?}", event);
        let newcf = match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                recreate_swapchain = true;
                ControlFlow::Wait
            }
            Event::RedrawEventsCleared => {
                previous_frame_end.as_mut().unwrap().cleanup_finished();
                if recreate_swapchain {
                    let dimensions: [u32; 2] = surface.window().inner_size().into();
                    let (new_swapchain, new_images) =
                        match swapchain.recreate_with_dimensions(dimensions) {
                            Ok(r) => r,
                            Err(SwapchainCreationError::UnsupportedDimensions) => return,
                            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                        };

                    swapchain = new_swapchain;
                    framebuffers = window_size_dependent_setup(
                        &new_images,
                        render_pass.clone(),
                        &mut dynamic_state,
                    );
                    recreate_swapchain = false;
                }

                let (image_num, suboptimal, acquire_future) =
                    match swapchain::acquire_next_image(swapchain.clone(), None) {
                        Ok(r) => r,
                        Err(AcquireError::OutOfDate) => {
                            recreate_swapchain = true;
                            return;
                        }
                        Err(e) => panic!("Failed to acquire next image: {:?}", e),
                    };

                if suboptimal {
                    recreate_swapchain = true;
                }
                let clear_values = vec![[0.0, 0.0, 1.0, 1.0].into()];
                let mut builder = {
                    let device = vk.clone_device();
                    let qf = vk.as_queue().family();
                    AutoCommandBufferBuilder::primary_one_time_submit(device, qf).unwrap()
                };

                builder
                    .begin_render_pass(
                        framebuffers[image_num].clone(),
                        SubpassContents::Inline,
                        clear_values,
                    )
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

                let command_buffer = builder.build().unwrap();

                let future = previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(vk.clone_queue(), command_buffer)
                    .unwrap()
                    .then_swapchain_present(vk.clone_queue(), swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();

                match future {
                    Ok(future) => {
                        previous_frame_end = Some(future.boxed());
                    }
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end = Some(sync::now(vk.clone_device()).boxed());
                    }
                    Err(e) => {
                        println!("Failed to flush future: {:?}", e);
                        previous_frame_end = Some(sync::now(vk.clone_device()).boxed());
                    }
                }
                ControlFlow::Wait
            }
            _ => ControlFlow::Wait,
        };
        *controlf = newcf;
    });
}

/// This method is called once during initialization,
/// then again whenever the window is resized
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimensions = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]);

    images
        .iter()
        .map(|image| {
            let view = ImageView::new(image.clone()).unwrap();
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(view)
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>()
}
