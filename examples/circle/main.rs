use log::info;
use structopt::StructOpt;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::{path, time};

use gpgpu::{
    dom::{self, shape, win, Domesticate},
    niw,
    primv::circle,
    Config, Context, Render, Screen, Transforms,
};

const SSAA: f32 = 1.0;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(long = "radius", default_value = "200")]
    radius: f32,

    #[structopt(long = "width", default_value = "1")]
    width: f32,

    #[structopt(long = "fill")]
    fill: bool,

    #[structopt(long = "save")]
    save: Option<path::PathBuf>,
}

struct State {
    render: Render,
    next_frame: time::Instant,
    domr: dom::Dom,
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

        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("examples/circle:command-encoder"),
            };
            self.render.as_device().create_command_encoder(&desc)
        };

        let context = Context {
            transforms: &Transforms::empty(),
            device: self.render.as_device(),
            queue: self.render.as_queue(),
        };
        let mut target = self.render.to_color_target();
        self.domr
            .redraw(&context, &mut encoder, &mut target)
            .unwrap();

        self.render.submit(encoder).unwrap();

        self.next_frame = time::Instant::now() + time::Duration::from_millis(10);
    }
}

fn main() {
    env_logger::init();

    let opts = Opt::from_args();
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
        let mut domr = make_dom(&opts, &render, FORMAT);
        domr.compute_layout(render.to_extent3d().into()).unwrap();
        domr.print();

        render.start();
        State {
            render,
            next_frame: time::Instant::now(),
            domr,
        }
    };

    swin.on_win_scale_factor_changed(Box::new(on_win_scale_factor_changed))
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
    if let Event::WindowEvent { event, .. } = event {
        if let WindowEvent::Resized(size) = event {
            state.domr.resize((*size).into(), None);

            let extent = state.render.to_extent3d();
            info!("win_resized: extent:{:?}", extent);

            state.domr.compute_layout(extent.into()).unwrap();
            state.domr.print(); // TODO: remove this
        }
    }

    None
}

fn on_win_scale_factor_changed(
    _: &Window,
    state: &mut State,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    if let Event::WindowEvent { event, .. } = event {
        if let WindowEvent::ScaleFactorChanged { .. } = event {
            state.domr.resize(
                state.render.to_extent3d().into(),
                Some(state.render.to_scale_factor()),
            );

            let extent = state.render.to_extent3d();
            info!("win_scale_factor_changed extent:{:?}", extent);

            state.domr.compute_layout(extent.into()).unwrap();

            state.domr.print(); // TODO: remove this
        }
    }

    None
}

fn make_dom(opts: &Opt, render: &Render, format: wgpu::TextureFormat) -> dom::Dom {
    let shape: dom::Node = {
        let attrs = circle::Attributes {
            radius: opts.radius,
            width: opts.width,
            fill: opts.fill,
            ..circle::Attributes::default()
        };
        let device = render.as_device();
        shape::Shape::new_circle(circle::Circle::new(attrs, device, format)).into()
    };
    let mut win = win::Win::new(vec![shape]);
    win.resize(render.to_extent3d().into(), Some(render.to_scale_factor()));
    dom::Dom::new(win)
}
