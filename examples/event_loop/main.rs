use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::{ffi, process::exit, sync::Arc};

use gpgpu::{niw, Config, Error, Render, Result, Screen};

#[derive(StructOpt, Clone)]
pub struct Opt {
    #[structopt(long = "config")]
    config_loc: Option<ffi::OsString>,

    #[structopt(long = "event")]
    event_name: Option<String>,
}

struct State {
    events_log: niw::EventsLog,
    opts: Opt,
}

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    let config = match &opts.config_loc {
        Some(loc) => match Config::from_file(loc) {
            Ok(config) => config,
            Err(err) => {
                println!("invalid config file {:?}: {}", loc, err);
                exit(1);
            }
        },
        None => Config::default(),
    };

    let res = handle_events(opts, config);

    res.map_err(|err: Error| println!("unexpected error: {}", err))
        .ok();
}

fn handle_events(opts: Opt, config: Config) -> Result<()> {
    let name = "example-event-loop".to_string();
    let mut swin = niw::SingleWindow::<Render<State>, ()>::from_config(
        config.to_window_attributes()?,
    )?;

    let on_win_close_requested = |_: &Window,
                                  _: &mut Render<State>,
                                  _: &mut Event<()>|
     -> Option<ControlFlow> { None };

    let on_win_keyboard_input = |_: &Window,
                                 _: &mut Render<State>,
                                 event: &mut Event<()>|
     -> Option<ControlFlow> {
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
    };

    let on_event = |_: &Window,
                    r: &mut Render<State>,
                    event: &mut Event<()>|
     -> Option<ControlFlow> {
        let state = r.as_state();
        match unsafe { (Arc::as_ptr(&state) as *mut State).as_mut() } {
            Some(state) => {
                state.events_log.append(event);
                match state.opts.event_name.as_ref() {
                    Some(event_name)
                        if &niw::to_event_name(event).to_string() == event_name =>
                    {
                        print!("\r{:?}", event);
                    }
                    _ => (),
                }
                None
            }
            None => unreachable!(),
        }
    };

    let on_loop_destroyed =
        |_: &Window, r: &mut Render<State>, _: &mut Event<()>| -> Option<ControlFlow> {
            let state = r.as_state();
            state.events_log.pretty_print();
            None
        };

    swin.on_win_close_requested(Box::new(on_win_close_requested))
        .on_win_keyboard_input(Box::new(on_win_keyboard_input))
        .on_loop_destroyed(Box::new(on_loop_destroyed))
        .on_event(Box::new(on_event));

    let r = {
        let screen = pollster::block_on(Screen::new(
            name.clone(),
            swin.as_window(),
            Config::default(),
        ))
        .unwrap();
        let state = State {
            events_log: niw::EventsLog::default(),
            opts: opts.clone(),
        };
        Render::new(screen, state)
    };

    println!("Press Esc to exit");
    swin.run(r);
}
