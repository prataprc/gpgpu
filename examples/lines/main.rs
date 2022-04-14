use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::path;

use gpgpu::{niw, util, wireframe::Wireframe, Config, Error, Gpu};

mod render;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(short = "bg")]
    bg: Option<String>,

    #[structopt(short = "fg")]
    fg: Option<String>,

    #[structopt(short = "scale")]
    scale: Option<f32>,

    #[structopt(short = "vertices", default_value = "vertices.txt")]
    vertices: path::PathBuf,
}

type Renderer = niw::Renderer<Gpu, State>;

struct State {
    bg: wgpu::Color,
    fg: wgpu::Color,
    scale: f32,
    wireframe: Wireframe,
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
        let wireframe = Wireframe::from_bytes(fs::read(opts.vertices).unwrap()).unwrap();
        let state = State {
            bg: util::html_to_color(&opts.bg.clone().unwrap_or("#123456".to_string()))
                .unwrap(),
            fg: util::html_to_color(&opts.fg.clone().unwrap_or("#000000".to_string()))
                .unwrap(),
            scale: opts.fg.unwrap_or("1.0".to_string()).parse().unwrap(),
            wireframe,
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

    let cmd_buffers = vec![encoder.finish()];

    match r.gpu.render(cmd_buffers, surface_texture) {
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
