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

const SSAA: f32 = 1.0;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

struct State {
    font: fonts::FontFile,
    render: Render,
    transforms: Transforms,
    clear: clear::Clear,
    frames: util::FrameRate,
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

    let font = fonts::FontFile::new(loc, 0, 24.0)?;

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

        let mut render = Render::new(screen);
        render.set_format(FORMAT);

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
    _state: &mut State,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::ScaleFactorChanged { .. } => (),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}
