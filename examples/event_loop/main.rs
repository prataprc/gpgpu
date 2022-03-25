use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use std::{ffi, process::exit};

use gpgpu::{niw, wg, Error, Result};

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(long = "config")]
    config_loc: Option<ffi::OsString>,
}

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    let config = match &opts.config_loc {
        Some(loc) => match wg::Config::from_file(loc) {
            Ok(config) => config,
            Err(err) => {
                println!("invalid config file {:?}: {}", loc, err);
                exit(1);
            }
        },
        None => wg::Config::default(),
    };

    println!("Press Esc to exit");
    let res = handle_events(opts, config);

    res.map_err(|err: Error| println!("unexpected error: {}", err))
        .ok();
}

fn handle_events(_opts: Opt, config: wg::Config) -> Result<()> {
    let mut win = niw::SingleWindow::<()>::from_config(config.to_window_attributes()?)?;

    let on_win_close_requested = |_: Event<()>| -> Option<ControlFlow> { None };

    let on_win_keyboard_input = |event: Event<()>| -> Option<ControlFlow> {
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

    win.on_win_close_requested(Some(Box::new(on_win_close_requested)))
        .on_win_keyboard_input(Some(Box::new(on_win_keyboard_input)));

    win.run();
}
