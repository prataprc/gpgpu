use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::sync::Arc;

use gpgpu::{niw, Config, Render, Screen};

#[derive(StructOpt, Clone)]
pub struct Opt {
    #[structopt(long = "event")]
    event_name: Option<String>,
}

struct State {
    opts: Opt,
    render: Render,
    events_log: niw::EventsLog,
    color_texture: Arc<wgpu::Texture>,
}

const SSAA: f32 = 1.0;

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    let name = "example-event-loop".to_string();
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

    let color_texture =
        Arc::new(screen.like_surface_texture(SSAA, screen.to_texture_format()));

    let mut render = Render::new(screen);
    render.start();

    let state = State {
        opts: opts.clone(),
        render,
        events_log: niw::EventsLog::default(),
        color_texture,
    };

    swin.on_win_close_requested(Box::new(on_win_close_requested))
        .on_win_keyboard_input(Box::new(on_win_keyboard_input))
        .on_win_resized(Box::new(on_win_resized))
        .on_win_scale_factor_changed(Box::new(on_win_scale_factor_changed))
        .on_main_events_cleared(Box::new(on_main_events_cleared))
        .on_redraw_requested(Box::new(on_redraw_requested))
        .on_event(Box::new(on_event));

    println!("Press Esc to exit");
    swin.run(state);
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
                state.events_log.pretty_print();
                Some(ControlFlow::Exit)
            }
            _ => None,
        },
        _ => None,
    }
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
                state
                    .render
                    .as_screen()
                    .resize(**new_inner_size, Some(*scale_factor));
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}

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
    state
        .render
        .post_frame(Arc::clone(&state.color_texture))
        .unwrap();
    None
}

fn on_event(_: &Window, state: &mut State, event: &mut Event<()>) -> Option<ControlFlow> {
    state.events_log.append(event);
    match state.opts.event_name.as_ref() {
        Some(event_name) if &niw::to_event_name(event).to_string() == event_name => {
            print!("\r{:?}", event);
        }
        _ => (),
    }
    None
}
