use cgmath::{Deg, Point3, Vector3};
use log::info;
use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::{fs, path, sync::Arc, time};

use gpgpu::{niw, vidgets::Wireframe, Config, Perspective, Render, Screen, Transforms};

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "rotate", default_value = "0", use_delimiter = true)]
    rotate: Vec<f32>,

    #[structopt(long = "vertices")]
    vertices: path::PathBuf,
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
    wireframe: Wireframe,
    next_frame: time::Instant,
    color_texture: Arc<wgpu::Texture>,
}

impl State {
    fn redraw(&mut self) {
        if self.next_frame > time::Instant::now() {
            return;
        }

        let view = {
            let desc = wgpu::TextureViewDescriptor::default();
            self.color_texture.create_view(&desc)
        };

        let mut transforms = self.transforms;
        transforms
            .rotate_x_by(Deg(self.rotate_by[0]))
            .rotate_y_by(Deg(self.rotate_by[1]))
            .rotate_z_by(Deg(self.rotate_by[2]))
            .look_at_rh(self.eye, self.center, self.up)
            .perspective_by(self.p);

        self.wireframe.render(
            &transforms,
            &self.render.as_screen().device,
            &self.render.as_screen().queue,
            &view,
        );

        self.render
            .post_frame(Arc::clone(&self.color_texture))
            .unwrap();

        self.rotate_by[0] += self.opts.rotate[0];
        self.rotate_by[1] += self.opts.rotate[1];
        self.rotate_by[2] += self.opts.rotate[2];

        self.next_frame = time::Instant::now() + time::Duration::from_millis(10);
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

    let state = {
        let screen = pollster::block_on(Screen::new(
            name.clone(),
            swin.as_window(),
            Config::default(),
        ))
        .unwrap();
        let format = screen.to_texture_format();
        let p = Perspective {
            fov: Deg(90.0),
            aspect: screen.to_aspect_ratio(),
            near: 0.1,
            far: 100.0,
        };
        let wireframe = {
            let data = fs::read(opts.vertices.clone()).unwrap();
            Wireframe::from_bytes(&data, format, &screen.device).unwrap()
        };

        let color_texture = Arc::new(screen.like_surface_texture());

        let mut render = Render::new(screen);
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
            color_texture,
        }
    };

    swin.on_win_close_requested(Box::new(on_win_close_requested))
        .on_win_keyboard_input(Box::new(on_win_keyboard_input))
        .on_win_resized(Box::new(on_win_resized))
        .on_win_scale_factor_changed(Box::new(on_win_scale_factor_changed))
        .on_main_events_cleared(Box::new(on_main_events_cleared))
        .on_redraw_requested(Box::new(on_redraw_requested));

    info!("Press Esc to exit");
    swin.run(state);
}

// RedrawRequested will only trigger once, unless we manually request it.
fn on_main_events_cleared(
    w: &Window,
    _state: &mut State,
    _event: &mut Event<()>,
) -> Option<ControlFlow> {
    w.request_redraw();
    None
}

fn on_redraw_requested(
    _: &Window,
    state: &mut State,
    _event: &mut Event<()>,
) -> Option<ControlFlow> {
    //let surface_texture = state.render.as_screen().get_current_texture().unwrap();
    //let surface_view = {
    //    let desc = wgpu::TextureViewDescriptor::default();
    //    surface_texture.texture.create_view(&desc)
    //};
    //let mut transforms = state.transforms;
    //transforms
    //    .rotate_x_by(Deg(state.rotate_by[0]))
    //    .rotate_y_by(Deg(state.rotate_by[1]))
    //    .rotate_z_by(Deg(state.rotate_by[2]))
    //    .look_at_rh(state.eye, state.center, state.up)
    //    .perspective_by(state.p);
    //state.wireframe.render(
    //    &transforms,
    //    &state.render.as_screen().device,
    //    &state.render.as_screen().queue,
    //    &surface_view,
    //);
    //surface_texture.present();

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
            WindowEvent::Resized(size) => {
                state.p.aspect = state.render.as_screen().to_aspect_ratio();
                state.render.as_screen().resize(*size);
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}

fn on_win_scale_factor_changed(
    _: &Window,
    state: &mut State,
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
                state.render.as_screen().resize(**new_inner_size)
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}

fn on_win_close_requested(
    _: &Window,
    state: &mut State,
    _: &mut Event<()>,
) -> Option<ControlFlow> {
    state.render.stop().ok();
    Some(ControlFlow::Exit)
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
                state.render.stop().ok();
                Some(ControlFlow::Exit)
            }
            _ => None,
        },
        _ => None,
    }
}
