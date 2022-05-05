use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

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
}

impl AsMut<Render> for State {
    fn as_mut(&mut self) -> &mut Render {
        &mut self.render
    }
}

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

    let mut render = Render::new(screen);
    render.start();

    let state = State {
        opts: opts.clone(),
        render,
        events_log: niw::EventsLog::default(),
    };

    swin.on_win_keyboard_input(Box::new(on_win_keyboard_input))
        .on_redraw_requested(Box::new(on_redraw_requested))
        .on_event(Box::new(on_event));

    println!("Press Esc to exit");
    swin.run(state);
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

fn on_redraw_requested(
    _: &Window,
    state: &mut State,
    _event: &mut Event<()>,
) -> Option<ControlFlow> {
    let screen = state.render.as_screen();

    let encoder = {
        let desc = wgpu::CommandEncoderDescriptor {
            label: Some("examples/event_loop:command-encoder"),
        };
        screen.device.create_command_encoder(&desc)
    };

    state.render.submit(encoder).unwrap();
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
