use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::sync::Arc;

use gpgpu::{niw, util, widg::Clear, Config, Render, Screen};

const SSAA: f32 = 2.0;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(short = "c")]
    color: Option<String>,
}

struct State {
    color: wgpu::Color,
    color_texture: Arc<wgpu::Texture>,
    render: Render,
}

impl State {
    fn redraw(&mut self) {
        let view = {
            let desc = wgpu::TextureViewDescriptor::default();
            self.color_texture.create_view(&desc)
        };
        let screen = self.render.as_screen();
        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("examples/cls:command-encoder"),
            };
            screen.device.create_command_encoder(&desc)
        };

        let clear = Clear::new(self.color);
        clear
            .render(&mut encoder, &screen.device, &screen.queue, &view)
            .unwrap();
        screen.queue.submit(vec![encoder.finish()]);

        self.render
            .post_frame(Arc::clone(&self.color_texture))
            .unwrap();
    }
}

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    let name = "example-cls".to_string();
    let config = Config::default();

    let mut swin = {
        let wattrs = config.to_window_attributes().unwrap();
        niw::SingleWindow::<State, ()>::from_config(wattrs).unwrap()
    };

    swin.on_win_close_requested(Box::new(on_win_close_requested))
        .on_win_keyboard_input(Box::new(on_win_keyboard_input))
        .on_win_resized(Box::new(on_win_resized))
        .on_win_scale_factor_changed(Box::new(on_win_scale_factor_changed))
        .on_main_events_cleared(Box::new(on_main_events_cleared))
        .on_redraw_requested(Box::new(on_redraw_requested));

    let state = {
        let screen = pollster::block_on(Screen::new(
            name.clone(),
            swin.as_window(),
            Config::default(),
        ))
        .unwrap();

        let color_texture =
            Arc::new(screen.like_surface_texture(SSAA, screen.to_texture_format()));

        let mut render = Render::new(screen);
        render.start();

        State {
            color: util::html_to_color(
                &opts.color.clone().unwrap_or("#FFFFFF".to_string()),
            )
            .unwrap(),
            render,
            color_texture,
        }
    };

    println!("Press Esc to exit");
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
            WindowEvent::Resized(size) => state.render.as_screen().resize(*size, None),
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
                let screen = state.render.as_screen();
                screen.resize(**new_inner_size, Some(*scale_factor));
                state.color_texture = Arc::new(
                    screen.like_surface_texture(SSAA, screen.to_texture_format()),
                );
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}

fn on_win_close_requested(
    _: &Window,
    _: &mut State,
    _: &mut Event<()>,
) -> Option<ControlFlow> {
    Some(ControlFlow::Exit)
}

fn on_win_keyboard_input(
    _: &Window,
    _state: &mut State,
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
