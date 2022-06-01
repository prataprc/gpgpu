use cgmath::{Deg, Point3, Vector3};
use log::info;
use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::{fs, path, time};

use gpgpu::{
    niw, primv::wireframe, Config, Context, Perspective, Render, Screen, Transforms,
};

const SSAA: f32 = 1.0;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "rotate", default_value = "0", use_delimiter = true)]
    rotate: Vec<f32>,

    #[structopt(long = "vertices")]
    vertices: path::PathBuf,

    #[structopt(long = "save")]
    save: Option<path::PathBuf>,
}

struct State {
    opts: Opt,
    render: Render,
    rotate_by: Vec<f32>,
    eye: Point3<f32>,
    center: Point3<f32>,
    up: Vector3<f32>,
    p: Perspective<Deg<f32>>,
    transforms: Transforms,
    wireframe: wireframe::Wireframe,
    next_frame: time::Instant,
    start_time: time::Instant,
    n_frames: u64,
}

impl AsMut<Render> for State {
    fn as_mut(&mut self) -> &mut Render {
        &mut self.render
    }
}

impl State {
    fn redraw(&mut self) {
        if self.next_frame > time::Instant::now() {
            return;
        }

        let mut target = self.render.to_color_target();

        let mut transforms = self.transforms;
        transforms
            .rotate_x_by(Deg(self.rotate_by[0]))
            .rotate_y_by(Deg(self.rotate_by[1]))
            .rotate_z_by(Deg(self.rotate_by[2]))
            .look_at_rh(self.eye, self.center, self.up)
            .perspective_by(self.p);

        let screen = self.render.as_screen();

        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("examples/wirecube:command-encoder"),
            };
            screen.device.create_command_encoder(&desc)
        };

        let context = Context {
            transforms: &transforms,
            device: &screen.device,
            queue: &screen.queue,
        };
        self.wireframe.redraw(&context, &mut encoder, &mut target).unwrap();

        self.render.submit(encoder).unwrap();

        self.rotate_by[0] += self.opts.rotate[0];
        self.rotate_by[1] += self.opts.rotate[1];
        self.rotate_by[2] += self.opts.rotate[2];

        self.next_frame = time::Instant::now() + time::Duration::from_millis(10);

        self.n_frames += 1;
    }
}

fn main() {
    env_logger::init();

    let mut opts = Opt::from_args();
    opts.rotate = match opts.rotate.as_slice() {
        [] => vec![0.0, 0.0, 0.0],
        [x] => vec![*x, 0.0, 0.0],
        [x, y] => vec![*x, *y, 0.0],
        [x, y, z] => vec![*x, *y, *z],
        [x, y, z, ..] => vec![*x, *y, *z],
    };

    let name = "example-triangle".to_string();
    let config = Config::default();

    let mut swin = {
        let wattrs = config.to_window_attributes().unwrap();
        niw::SingleWindow::<State, ()>::from_config(wattrs).unwrap()
    };

    let screen = pollster::block_on(Screen::new(
        name.clone(),
        swin.as_window(),
        Config::default(),
    ))
    .unwrap();

    let wireframe = {
        let data = fs::read(opts.vertices.clone()).unwrap();
        wireframe::Wireframe::from_bytes(&data, FORMAT, &screen.device).unwrap()
    };

    let mut render = Render::new_super_sampled(screen, SSAA, FORMAT);
    if let Some(loc) = opts.save.clone() {
        render.save_gif(loc, FORMAT);
    }

    let state = {
        let p = Perspective {
            fov: Deg(90.0),
            aspect: render.as_screen().to_aspect_ratio(),
            near: 0.1,
            far: 100.0,
        };

        render.start();
        State {
            opts: opts.clone(),
            render,
            rotate_by: opts.rotate.clone(),
            eye: Point3::new(0.0, 0.0, 3.0),
            center: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::unit_y(),
            p,
            transforms: Transforms::empty(),
            wireframe,
            next_frame: time::Instant::now(),
            start_time: time::Instant::now(),
            n_frames: 0,
        }
    };

    swin.on_win_keyboard_input(Box::new(on_win_keyboard_input))
        .on_win_resized(Box::new(on_win_resized))
        .on_redraw_requested(Box::new(on_redraw_requested));

    info!("Press Esc to exit");
    swin.run(state);
}

fn on_redraw_requested(
    _: &Window,
    state: &mut State,
    _event: &mut Event<()>,
) -> Option<ControlFlow> {
    state.redraw();
    None
}

fn on_win_resized(
    _: &Window,
    state: &mut State,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(_size) => {
                state.p.aspect = state.render.as_screen().to_aspect_ratio();
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}

fn on_win_keyboard_input(
    _: &Window,
    state: &mut State,
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
            } => {
                println!(
                    "frame rate {}/s total:{} frames",
                    state.n_frames / state.start_time.elapsed().as_secs(),
                    state.n_frames
                );
                None
            }
            _ => None,
        },
        _ => None,
    }
}
