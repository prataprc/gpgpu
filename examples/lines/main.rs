use cgmath::{Deg, Point3, Vector3};
use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::{fs, path};

use gpgpu::{
    niw, util, wireframe::Wireframe, Config, Error, Gpu, Perspective, Transforms,
};

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "bg")]
    bg: Option<String>,

    #[structopt(long = "fg")]
    fg: Option<String>,

    #[structopt(long = "rotate")]
    rotate: Option<f32>,

    #[structopt(long = "vertices")]
    vertices: path::PathBuf,
}

type Renderer = niw::Renderer<Gpu, State>;

struct State {
    bg: wgpu::Color,
    fg: wgpu::Color,
    rotate_by: Deg<f32>,
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
        let p = Perspective {
            fov: Deg(90.0),
            aspect: (gpu.surface_config.width as f32)
                / (gpu.surface_config.height as f32),
            near: 100.0,
            far: 2000.0,
        };

        let mut wireframe = {
            let data = fs::read(opts.vertices).unwrap();
            Wireframe::from_bytes(&data).unwrap()
        };
        wireframe
            .set_color_format(gpu.surface_config.format)
            .prepare(&gpu.device);

        let state = State {
            bg: util::html_to_color(&opts.bg.clone().unwrap_or("#123456".to_string()))
                .unwrap(),
            fg: util::html_to_color(&opts.fg.clone().unwrap_or("#000000".to_string()))
                .unwrap(),
            rotate_by: Deg(opts.rotate.unwrap_or(0.0)),
            eye: Point3::new(0.0, 0.0, 300.0),
            center: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::unit_y(),
            p,
            transforms: Transforms::empty(),
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

    let mut transforms = r.state.transforms;
    transforms
        .rotate_by(
            Some(r.state.rotate_by),
            Some(r.state.rotate_by),
            Some(r.state.rotate_by),
        )
        .look_at_rh(r.state.eye, r.state.center, r.state.up)
        .perspective_by(r.state.p);

    r.state.wireframe.transform_mut(transforms.model());

    r.state
        .wireframe
        .render(&transforms, &r.gpu.device, &mut encoder, &view);

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
