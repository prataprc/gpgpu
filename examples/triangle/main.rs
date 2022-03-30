use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use gpgpu::{niw, util, Config, Error, Gpu};

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(short = "bg")]
    bg: Option<String>,

    #[structopt(short = "fg")]
    fg: Option<String>,

    #[structopt(short = "scale")]
    scale: Option<f32>,
}

type Renderer = niw::Renderer<Gpu, State>;

struct State {
    bg: wgpu::Color,
    fg: wgpu::Color,
    scale: f32,
}

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    let name = "example-triangle".to_string();
    let config = Config::default();

    let mut swin = {
        let wattrs = config.to_window_attributes().unwrap();
        niw::SingleWindow::<Gpu, State, ()>::from_config(wattrs).unwrap()
    };

    swin.on_win_close_requested(Box::new(on_win_close_requested))
        .on_win_keyboard_input(Box::new(on_win_keyboard_input))
        .on_win_resized(Box::new(on_win_resized))
        .on_win_scale_factor_changed(Box::new(on_win_scale_factor_changed))
        .on_main_events_cleared(Box::new(on_main_events_cleared))
        .on_redraw_requested(Box::new(on_redraw_requested));

    let r = {
        let gpu = pollster::block_on(Gpu::new(
            name.clone(),
            swin.as_window(),
            Config::default(),
        ))
        .unwrap();
        let state = State {
            bg: util::html_to_color(&opts.bg.clone().unwrap_or("#123456".to_string()))
                .unwrap(),
            fg: util::html_to_color(&opts.fg.clone().unwrap_or("#000000".to_string()))
                .unwrap(),
            scale: opts.fg.unwrap_or("1.0".to_string()).parse().unwrap(),
        };
        Renderer { gpu, state }
    };

    println!("Press Esc to exit");
    swin.run(r);
}

// RedrawRequested will only trigger once, unless we manually request it.
fn on_main_events_cleared(
    w: &Window,
    _r: &mut Renderer,
    _event: &mut Event<()>,
) -> Option<ControlFlow> {
    w.request_redraw();
    None
}

fn on_redraw_requested(
    _: &Window,
    r: &mut Renderer,
    _event: &mut Event<()>,
) -> Option<ControlFlow> {
    let module = {
        let desc = wgpu::ShaderModuleDescriptor {
            label: Some("Shader module"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        };
        r.gpu.device.create_shader_module(&desc)
    };
    let render_pipeline_layout = {
        let desc = wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };
        r.gpu.device.create_pipeline_layout(&desc)
    };
    let render_pipeline = {
        let targets = vec![wgpu::ColorTargetState {
            format: r.gpu.surface_config.format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        }];
        let desc = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
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
                // Setting this to anything other than Fill requires
                // Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,                         // 2.
                mask: !0,                         // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            fragment: Some(wgpu::FragmentState {
                module: &module,
                entry_point: "fs_main",
                targets: targets.as_slice(),
            }),
            multiview: None, // 5.
        };
        r.gpu.device.create_render_pipeline(&desc)
    };

    let surface_texture = r.gpu.get_current_texture().ok()?;
    let view = {
        let desc = wgpu::TextureViewDescriptor::default();
        surface_texture.texture.create_view(&desc)
    };

    let mut encoder = {
        let desc = wgpu::CommandEncoderDescriptor {
            label: Some("clear_screen"),
        };
        r.gpu.device.create_command_encoder(&desc)
    };
    {
        let mut render_pass = {
            let ops = wgpu::Operations {
                load: wgpu::LoadOp::Clear(r.state.bg.clone()),
                store: true,
            };
            let desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops,
                    },
                ],
                depth_stencil_attachment: None,
            };
            encoder.begin_render_pass(&desc)
        };
        render_pass.set_pipeline(&render_pipeline);
        render_pass.draw(0..3, 0..1);
    }

    let cmd_buffers = vec![encoder.finish()];

    match r.gpu.render(surface_texture, cmd_buffers) {
        Ok(_) => None,
        // Reconfigure the surface if lost
        Err(Error::SurfaceLost(_, _)) => {
            r.gpu.resize(r.gpu.size);
            None
        }
        // The system is out of memory, we should probably quit
        Err(Error::SurfaceOutOfMemory(_, _)) => Some(ControlFlow::Exit),
        // All other errors (Outdated, Timeout) should be resolved by the next frame
        Err(e) => {
            eprintln!("{:?}", e);
            None
        }
    }
}

fn on_win_resized(
    _: &Window,
    r: &mut Renderer,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => r.gpu.resize(*size),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}

fn on_win_scale_factor_changed(
    _: &Window,
    r: &mut Renderer,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // TODO Is this the right way to handle it, doc says the following:
                // After this event callback has been processed, the window will be
                // resized to whatever value is pointed to by the new_inner_size
                // reference. By default, this will contain the size suggested by the
                // OS, but it can be changed to any value.
                r.gpu.resize(**new_inner_size)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}

fn on_win_close_requested(
    _: &Window,
    _r: &mut Renderer,
    _: &mut Event<()>,
) -> Option<ControlFlow> {
    Some(ControlFlow::Exit)
}

fn on_win_keyboard_input(
    _: &Window,
    _r: &mut Renderer,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => Some(ControlFlow::Exit),
            _ => None,
        },
        _ => None,
    }
}
