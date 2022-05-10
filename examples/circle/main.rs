use log::info;
use structopt::StructOpt;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::{path, time};

use gpgpu::{
    dom::{self, circle, win, Domesticate},
    niw, Config, Context, Location, Render, Screen, Size, Style, Transforms,
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
        let wgpu::Extent3d { width, height, .. } = render.to_extent3d();
        domr.compute_layout(Some(width as f32), Some(height as f32))
            .unwrap();
        domr.print();

        render.start();
        State {
            render,
            next_frame: time::Instant::now(),
            domr,
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
            let scale_factor = state.render.to_scale_factor();
            state.domr.resize(Location::default(), scale_factor);

            let wgpu::Extent3d { width, height, .. } = state.render.to_extent3d();
            println!("width {} height {}", width, height);
            state
                .domr
                .compute_layout(Some(width as f32), Some(height as f32))
                .unwrap();
            state.domr.print();
        }
    }

    None
}

fn make_dom(opts: &Opt, render: &Render, format: wgpu::TextureFormat) -> dom::Dom {
    let wgpu::Extent3d { width, height, .. } = render.to_extent3d();
    let (width, height) = (width as f32, height as f32);

    let circles: Vec<dom::Node> = {
        let attrs = circle::Attributes {
            radius: opts.radius,
            width: opts.width,
            fill: opts.fill,
            ..circle::Attributes::default()
        };
        let device = render.as_device();
        (0..1)
            .map(|_| {
                let style = Style::default();
                dom::Node::from(circle::Circle::new(attrs, style, device, format))
            })
            .collect()
    };
    let mut win = {
        let size = Size { width, height };
        win::Win::new(size, circles)
    };
    win.set_size(width as f32, height as f32);
    win.resize(Location::default(), render.to_scale_factor());
    dom::Dom::new(win)
}
