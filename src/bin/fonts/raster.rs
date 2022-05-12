use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use gpgpu::{
    fonts, niw, util, widg::clear, Config, Context, Render, Result, Screen, Transforms,
    Widget,
};

use crate::Opt;

const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

struct State {
    font: fonts::FontFile,
    render: Render,
    frames: util::FrameRate,
}

impl AsRef<Render> for State {
    fn as_ref(&self) -> &Render {
        &self.render
    }
}

impl AsMut<Render> for State {
    fn as_mut(&mut self) -> &mut Render {
        &mut self.render
    }
}

impl State {
    fn redraw(&mut self) {
        if !self.frames.is_redraw() {
            return;
        }

        let screen = self.render.as_screen();

        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("font-app:command-encoder"),
            };
            screen.device.create_command_encoder(&desc)
        };
        let context = Context {
            transforms: &self.transforms,
            device: &screen.device,
            queue: &screen.queue,
        };
        let target = self.render.to_color_target();
        self.clear.render(&context, &mut encoder, &target).unwrap();

        self.render.submit(encoder).unwrap();

        self.frames.next_frame_after(10_000 /*micros*/);
    }
}

pub fn handle_raster(opts: Opt) -> Result<()> {
    use crate::SubCommand;

    let (loc, _ch) = match opts.subcmd.clone() {
        SubCommand::Raster { loc, ch } => (loc, ch),
        _ => unreachable!(),
    };

    let font = fonts::FontFile::new(loc)?;

    let name = "font-app".to_string();
    let mut config = gpgpu::Config::default();
    {
        let mont = niw::get_monitor_info()?;
        let size = {
            let size = mont.to_logical_size();
            vec![(size.width as f64) * 0.8, (size.height as f64) * 0.9]
        };
        info!("{}", mont);
        info!("window size: {}x{}", size[0], size[1]);
        config.winit.inner_size = Some(size);
    };

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

        let mut render = Render::new(screen, FORMAT);

        let clear = clear::Clear::new(wgpu::Color::WHITE);

        render.start();
        State {
            font,
            render,
            transforms: Transforms::empty(),
            clear,
            frames: util::FrameRate::new(),
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

//fn make_dom(opts: &Opt, render: &Render, format: wgpu::TextureFormat) -> dom::Dom {
//    let circles: Vec<dom::Node> = {
//        let attrs = circle::Attributes {
//            radius: opts.radius,
//            width: opts.width,
//            fill: opts.fill,
//            ..circle::Attributes::default()
//        };
//        let device = render.as_device();
//        (0..1)
//            .map(|_| {
//                let style = Style::default();
//                dom::Node::from(circle::Circle::new(attrs, style, device, format))
//            })
//            .collect()
//    };
//    let mut win = win::Win::new(circles);
//    win.resize(Location::default(), render.to_scale_factor());
//    dom::Dom::new(win)
//}
