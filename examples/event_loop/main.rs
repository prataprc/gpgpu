use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::{ffi, process::exit};

use gpgpu::{niw, wg, Config, Error, Gpu, Result};

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(long = "config")]
    config_loc: Option<ffi::OsString>,
}

type Renderer = niw::Renderer<Gpu, ()>;

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

fn handle_events(_opts: Opt, config: Config) -> Result<()> {
    let name = "example-event-loop".to_string();
    let mut swin =
        niw::SingleWindow::<Gpu, (), ()>::from_config(config.to_window_attributes()?)?;

    let on_win_close_requested =
        |_: &Window, _: &mut Renderer, _: &mut Event<()>| -> Option<ControlFlow> { None };

    let on_win_keyboard_input =
        |_: &Window, _: &mut Renderer, event: &mut Event<()>| -> Option<ControlFlow> {
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

    swin.on_win_close_requested(Box::new(on_win_close_requested))
        .on_win_keyboard_input(Box::new(on_win_keyboard_input));

    let r = {
        let gpu = pollster::block_on(Gpu::new(
            name.clone(),
            swin.as_window(),
            Config::default(),
        ))
        .unwrap();
        let state = ();
        Renderer { gpu, state }
    };

    println!("Press Esc to exit");
    swin.run(r);
}
