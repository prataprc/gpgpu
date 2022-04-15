use cgmath::{Deg, Point3, Vector3};
use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::{fs, path};

use gpgpu::{
    niw, util, wireframe::Wireframe, Config, Error, Perspective, Render, Screen,
    Transforms,
};

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "bg")]
    bg: Option<String>,

    #[structopt(long = "fg")]
    fg: Option<String>,

    #[structopt(long = "rotate", default_value = "0", use_delimiter = true)]
    rotate: Vec<f32>,

    #[structopt(long = "vertices")]
    vertices: path::PathBuf,
}

#[derive(Clone)]
struct State {
    bg: wgpu::Color,
    fg: wgpu::Color,
    rotate_by: [Option<Deg<f32>>; 3],
    eye: Point3<f32>,
    center: Point3<f32>,
    up: Vector3<f32>,
    p: Perspective<Deg<f32>>,
    transforms: Transforms,
    wireframe: Wireframe,
}

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    let name = "example-triangle".to_string();
    let config = Config::default();

    let mut swin = {
        let wattrs = config.to_window_attributes().unwrap();
        niw::SingleWindow::<Render<State>, ()>::from_config(wattrs).unwrap()
    };

    swin.on_win_close_requested(Box::new(on_win_close_requested))
        .on_win_keyboard_input(Box::new(on_win_keyboard_input))
        .on_win_resized(Box::new(on_win_resized))
        .on_win_scale_factor_changed(Box::new(on_win_scale_factor_changed))
        .on_main_events_cleared(Box::new(on_main_events_cleared))
        .on_redraw_requested(Box::new(on_redraw_requested));

    let r = {
        let screen = pollster::block_on(Screen::new(
            name.clone(),
            swin.as_window(),
            Config::default(),
        ))
        .unwrap();
        let size = screen.to_physical_size();
        let format = screen.to_texture_format();

        let p = Perspective {
            fov: Deg(90.0),
            aspect: screen.to_aspect_ratio(),
            near: (size.width as f32) / 2.0,
            far: 10000.0,
        };

        let mut wireframe = {
            let mut transforms = Transforms::empty();
            transforms.scale_by(swin.as_window().scale_factor() as f32);
            let data = fs::read(opts.vertices).unwrap();
            Wireframe::from_bytes(&data)
                .unwrap()
                .transform(transforms.model())
        };
        wireframe.set_color_format(format).prepare(&screen.device);

        let state = State {
            bg: util::html_to_color(&opts.bg.clone().unwrap_or("#123456".to_string()))
                .unwrap(),
            fg: util::html_to_color(&opts.fg.clone().unwrap_or("#000000".to_string()))
                .unwrap(),
            rotate_by: match opts.rotate.as_slice() {
                [] => [Some(Deg(0.0)), Some(Deg(0.0)), Some(Deg(0.0))],
                [x] => [Some(Deg(*x)), Some(Deg(0.0)), Some(Deg(0.0))],
                [x, y] => [Some(Deg(*x)), Some(Deg(*y)), Some(Deg(0.0))],
                [x, y, z] => [Some(Deg(*x)), Some(Deg(*y)), Some(Deg(*z))],
                [x, y, z, ..] => [Some(Deg(*x)), Some(Deg(*y)), Some(Deg(*z))],
            },
            eye: Point3::new(0.0, 0.0, size.width as f32 * 2.0),
            center: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::unit_y(),
            p,
            transforms: Transforms::empty(),
            wireframe,
        };
        Render::new(screen, state)
    };

    println!("Press Esc to exit");
    swin.run(r);
}

// RedrawRequested will only trigger once, unless we manually request it.
fn on_main_events_cleared(
    w: &Window,
    _r: &mut Render<State>,
    _event: &mut Event<()>,
) -> Option<ControlFlow> {
    w.request_redraw();
    None
}

fn on_redraw_requested(
    _: &Window,
    r: &mut Render<State>,
    _event: &mut Event<()>,
) -> Option<ControlFlow> {
    let state = r.as_state();

    let surface_texture = r.screen.get_current_texture().ok()?;
    let view = {
        let desc = wgpu::TextureViewDescriptor::default();
        surface_texture.texture.create_view(&desc)
    };

    let mut encoder = {
        let desc = wgpu::CommandEncoderDescriptor {
            label: Some("clear_screen"),
        };
        r.screen.device.create_command_encoder(&desc)
    };

    let mut transforms = state.transforms;
    transforms
        .rotate_by(state.rotate_by[0], state.rotate_by[1], state.rotate_by[2])
        .look_at_rh(state.eye, state.center, state.up)
        .perspective_by(state.p);

    state
        .wireframe
        .render(&transforms, &r.screen.device, &mut encoder, &view);

    {
        let mut new_state = r.to_state();
        new_state.wireframe.transform_mut(transforms.model());
        new_state.p.aspect = r.screen.to_aspect_ratio();
        r.set_state(new_state)
    }

    let cmd_buffers = vec![encoder.finish()];

    match r.screen.render(cmd_buffers, surface_texture) {
        Ok(_) => None,
        // Reconfigure the surface if lost
        Err(Error::SurfaceLost(_, _)) => {
            r.screen.resize(r.screen.to_physical_size());
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
    r: &mut Render<State>,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => r.screen.resize(*size),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}

fn on_win_scale_factor_changed(
    _: &Window,
    r: &mut Render<State>,
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
                r.screen.resize(**new_inner_size)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}

fn on_win_close_requested(
    _: &Window,
    _r: &mut Render<State>,
    _: &mut Event<()>,
) -> Option<ControlFlow> {
    Some(ControlFlow::Exit)
}

fn on_win_keyboard_input(
    _: &Window,
    _r: &mut Render<State>,
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
