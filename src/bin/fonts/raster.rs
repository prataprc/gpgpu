use log::info;
use winit::{
    dpi,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::sync::Arc;

use crate::Opt;
use gpgpu::{
    fonts, niw, util,
    widg::{self, clear, Widget},
    Config, Render, Result, Screen, Transforms,
};

const SSAA: f32 = 1.0;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

struct State {
    font: fonts::FontFile,
    render: Render,
    transforms: Transforms,
    clear: clear::Clear,
    color_attach: Arc<wgpu::Texture>,
    frames: util::FrameRate,
}

impl State {
    fn resize(&mut self, size: dpi::PhysicalSize<u32>, scale_factor: Option<f64>) {
        let screen = self.render.as_screen();
        screen.resize(size, scale_factor);
        self.color_attach = Arc::new(screen.like_surface_texture(SSAA, Some(FORMAT)));
    }

    fn redraw(&mut self) {
        if !self.frames.is_redraw() {
            return;
        }

        let size = self.render.as_screen().to_extent3d(SSAA as u32);
        let screen = self.render.as_screen();

        let view = {
            let desc = wgpu::TextureViewDescriptor::default();
            self.color_attach.create_view(&desc)
        };

        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("font-app:command-encoder"),
            };
            screen.device.create_command_encoder(&desc)
        };
        let context = widg::Context {
            transforms: &self.transforms,
            device: &screen.device,
            queue: &screen.queue,
        };
        let target = widg::ColorTarget {
            size,
            format: FORMAT,
            view: &view,
        };
        self.clear.render(&context, &mut encoder, &target).unwrap();

        screen.queue.submit(vec![encoder.finish()]);

        self.render
            .post_frame(Arc::clone(&self.color_attach))
            .unwrap();

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
        let color_attach = Arc::new(screen.like_surface_texture(SSAA, Some(FORMAT)));

        let mut render = Render::new(screen);
        render.start();

        let clear = clear::Clear::new(wgpu::Color::WHITE);

        State {
            font,
            render,
            transforms: Transforms::empty(),
            clear,
            color_attach,
            frames: util::FrameRate::new(),
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
                state.resize(*size, None);
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
            WindowEvent::ScaleFactorChanged {
                new_inner_size,
                scale_factor,
            } => {
                state.resize(**new_inner_size, Some(*scale_factor));
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
                println!(
                    "frame rate {}/s total:{} frames",
                    state.frames.rate(),
                    state.frames.total(),
                );
                state.render.stop().ok();
                Some(ControlFlow::Exit)
            }
            _ => None,
        },
        _ => None,
    }
}
