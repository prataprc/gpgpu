mod info;

use colored::Colorize;
use structopt::StructOpt;
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoopWindowTarget},
    window::WindowBuilder,
};

use std::{ffi, process::exit};

use gpgpu::{err_at, niw, wg, Error, Result};

use info::{
    info_adapters, info_features, info_global_report, info_limits, info_texture_formats,
    info_window,
};

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "no-color")]
    no_color: bool,

    #[structopt(long = "config")]
    config_loc: Option<ffi::OsString>,

    #[structopt(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clone, StructOpt)]
pub enum SubCommand {
    /// List the wgpu backend available on this machine.
    Backend,
    /// List window attributes and monitors connected to this machine.
    Window {
        /// list video-modes available for the primary monitor.
        #[structopt(long = "modes")]
        modes: bool,

        /// list video-modes available for nth monitor connected to the machine.
        #[structopt(short = "n")]
        n: Option<usize>,
    },
    /// Generate a consolidated report of adapter, devices, features, limits etc.
    Report,
    /// List features, adapters and features supported for each adapter.
    Features,
    /// List limits, adapters and features supported for each adapter.
    Limits,
    /// List Texture formats.
    Formats,
    /// Start an event-loop using winit
    #[structopt(name = "event_loop")]
    EventLoop,
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

    let res = match &opts.subcmd {
        SubCommand::Report => handle_report(opts.clone(), &config),
        SubCommand::Backend => {
            println!("{:?} backend is used", wg::backend());
            Ok(())
        }
        SubCommand::Window { modes, n } => info_window(*modes, *n, &opts, &config),
        SubCommand::Features => handle_features(opts),
        SubCommand::Limits => handle_limits(opts),
        SubCommand::Formats => handle_formats(opts),
        SubCommand::EventLoop => handle_events(opts, config),
    };

    res.map_err(|err: Error| println!("unexpected error: {}", err))
        .ok();
}

fn handle_report(opts: Opt, config: &wg::Config) -> Result<()> {
    println!();
    println!("{}", "Monitors:".red());
    println!("{}", "---------".red());
    info_window(false, None, &opts, config)?;
    println!();

    println!("{}", "Global Memory Report:".red());
    println!("{}", "---------------------".red());
    info_global_report(&opts)?;
    println!();

    println!("{}", "Adapters:".red());
    println!("{}", "--------".red());
    info_adapters(&opts)?;
    println!();

    Ok(())
}

fn handle_features(opts: Opt) -> Result<()> {
    println!("{}", "Adapters:".red());
    println!("{}", "--------".red());
    info_adapters(&opts)?;
    println!();

    println!("{}", "Features:".red());
    println!("{}", "---------".red());
    info_features(&opts)?;
    println!();

    Ok(())
}

fn handle_limits(opts: Opt) -> Result<()> {
    println!("{}", "Adapters:".red());
    println!("{}", "--------".red());
    info_adapters(&opts)?;
    println!();

    println!("{}", "Limits:".red());
    println!("{}", "-------".red());
    info_limits(&opts)?;
    println!();

    Ok(())
}

fn handle_formats(opts: Opt) -> Result<()> {
    println!("{}", "TextureUsages:".red());
    println!("{}", "--------------".red());
    for item in wg::texture_usages().iter() {
        println!(" {}: {}", item.1, item.2)
    }
    println!();

    println!("{}", "TextureFormatFeatureFlags:".red());
    println!("{}", "--------------------------".red());
    for item in wg::texture_format_flags().iter() {
        println!(" {}: {}", item.1, item.2)
    }
    println!();

    println!("{}", "TextureFormats:".red());
    println!("{}", "---------------".red());
    info_texture_formats(&opts)?;
    println!();

    Ok(())
}

fn handle_events(_opts: Opt, config: wg::Config) -> Result<()> {
    let mut eloop = niw::Eloop::<()>::new();
    let window = {
        let mut wb = WindowBuilder::new();
        wb.window = config.to_window_attributes()?;
        err_at!(Fatal, wb.build(eloop.as_event_loop()))?
    };

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

    eloop
        .on_win_close_requested(Some(Box::new(on_win_close_requested)))
        .on_win_keyboard_input(Some(Box::new(on_win_keyboard_input)));

    eloop.run(window.id());
}
