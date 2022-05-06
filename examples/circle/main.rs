use cgmath::Deg;
use log::info;
use structopt::StructOpt;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::{path, time};

use gpgpu::{
    dom::circle, niw, Config, Context, Location, Render, Screen, Transforms, Widget,
};

const SSAA: f32 = 1.0;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "rotate", default_value = "0", use_delimiter = true)]
    rotate: Vec<f32>,

    #[structopt(long = "radius", default_value = "200")]
    radius: f32,

    #[structopt(long = "center", default_value = "0,0", use_delimiter = true)]
    center: Vec<f32>,

    #[structopt(long = "fill")]
    fill: bool,

    #[structopt(long = "save")]
    save: Option<path::PathBuf>,
}

struct State {
    opts: Opt,
    render: Render,
    rotate_by: Vec<f32>,
    transforms: Transforms,
    circle: circle::Circle,
    next_frame: time::Instant,
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

        let screen = self.render.as_screen();

        let mut transforms = self.transforms;
        transforms
            .rotate_x_by(Deg(self.rotate_by[0]))
            .rotate_y_by(Deg(self.rotate_by[1]))
            .rotate_z_by(Deg(self.rotate_by[2]));

        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("examples/circle:command-encoder"),
            };
            screen.device.create_command_encoder(&desc)
        };

        let context = Context {
            transforms: &transforms,
            device: &screen.device,
            queue: &screen.queue,
        };
        let target = self.render.to_color_target();
        self.circle.render(&context, &mut encoder, &target).unwrap();

        self.render.submit(encoder).unwrap();

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
    let center = Location {
        x: opts.center[0],
        y: opts.center[1],
    };

    let name = "example-circle".to_string();
    let config = Config::default();

    let mut swin = {
        let wattrs = config.to_window_attributes().unwrap();
        info!("winit::Window size {:?}", wattrs.inner_size);
        niw::SingleWindow::<State, ()>::from_config(wattrs).unwrap()
    };

    let screen = pollster::block_on(Screen::new(
        name.clone(),
        swin.as_window(),
        Config::default(),
    ))
    .unwrap();

    let mut render = Render::new_super_sampled(screen, SSAA, FORMAT);
    if let Some(loc) = opts.save.clone() {
        render.save_bmp(loc, FORMAT);
    }

    let state = {
        let mut circle = {
            let attrs = circle::Attributes {
                center,
                radius: opts.radius,
                fill: opts.fill,
                ..circle::Attributes::default()
            };
            circle::Circle::new(attrs, &render.as_screen().device, FORMAT)
        };
        circle.scale(render.to_scale_factor()).transform();

        render.start();
        State {
            opts: opts.clone(),
            render,
            rotate_by: opts.rotate.clone(),
            transforms: Transforms::empty(),
            circle,
            next_frame: time::Instant::now(),
        }
    };

    swin.on_win_scale_factor_changed(Box::new(on_win_scale_factor_changed))
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

fn on_win_scale_factor_changed(
    _: &Window,
    state: &mut State,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    if let Event::WindowEvent { event, .. } = event {
        if let WindowEvent::ScaleFactorChanged { .. } = event {
            state
                .circle
                .scale(state.render.to_scale_factor())
                .transform();
        }
    }

    None
}
