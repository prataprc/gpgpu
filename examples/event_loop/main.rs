use structopt::StructOpt;
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoopWindowTarget},
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
    let mut h = niw::Handle::<()>::from_config(config.to_window_attributes()?)?;

    let on_win_close_requested =
        |_target: &EventLoopWindowTarget<()>| -> niw::HandlerRes<()> {
            niw::HandlerRes {
                control_flow: Some(ControlFlow::Exit),
                param: (),
            }
        };

    let on_win_keyboard_input = |input: niw::WinKeyboardInput,
                                 _target: &EventLoopWindowTarget<()>|
     -> niw::HandlerRes<()> {
        let control_flow = match input {
            niw::WinKeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => Some(ControlFlow::Exit),
            _ => None,
        };

        niw::HandlerRes {
            control_flow,
            param: (),
        }
    };

    h.on_win_close_requested(Some(Box::new(on_win_close_requested)))
        .on_win_keyboard_input(Some(Box::new(on_win_keyboard_input)));

    h.run();
}
