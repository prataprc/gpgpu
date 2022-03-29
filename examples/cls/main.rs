use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use gpgpu::{niw, util, Config, Error, Gpu};

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(short = "c")]
    color: Option<String>,
}

type Renderer = niw::Renderer<Gpu, State>;

struct State {
    color: wgpu::Color,
}

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    let name = "example-cls".to_string();
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
            color: util::html_to_color(
                &opts.color.clone().unwrap_or("#FFFFFF".to_string()),
            )
            .unwrap(),
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
    let surface_texture = r.gpu.get_current_texture().unwrap();
    let view = {
        let desc = wgpu::TextureViewDescriptor::default();
        surface_texture.texture.create_view(&desc)
    };
    let cmd_buffers = vec![r.gpu.clear_view(&view, r.state.color)];

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
