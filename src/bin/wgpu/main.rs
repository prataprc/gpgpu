mod info;

use colored::Colorize;
use structopt::StructOpt;
use winit::{
    event::{ElementState, KeyboardInput, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoopWindowTarget},
};

use cgi::{wg, Error, Result};

use info::{
    info_adapters, info_features, info_global_report, info_limits, info_monitors,
};

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "no-color")]
    no_color: bool,

    #[structopt(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clone, StructOpt)]
pub enum SubCommand {
    /// List the wgpu backend available on this machine.
    Backend,
    /// List monitors connected to this machine.
    Monitors {
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
    /// Start an event-loop using winit
    Events,
}

fn main() {
    let opts = Opt::from_args();

    let res = match &opts.subcmd {
        SubCommand::Report => handle_report(opts.clone()),
        SubCommand::Backend => {
            println!("{:?} backend is used", wg::backend());
            Ok(())
        }
        SubCommand::Events => handle_events(opts),
        SubCommand::Monitors { modes, n } => info_monitors(*modes, *n, opts.no_color),
        SubCommand::Features => handle_features(opts),
        SubCommand::Limits => handle_limits(opts),
    };

    res.map_err(|err: Error| println!("unexpected error: {}", err))
        .ok();
}

fn handle_report(opts: Opt) -> Result<()> {
    println!();
    println!("{}", "Monitors:".red());
    println!("{}", "---------".red());
    info_monitors(false, None, opts.no_color)?;
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

fn handle_events(_opts: Opt) -> Result<()> {
    let mut wloop = cgi::win::WinLoop::<()>::new();

    let on_win_close_requested =
        |_target: &EventLoopWindowTarget<()>| -> cgi::win::HandlerRes<()> {
            cgi::win::HandlerRes {
                control_flow: Some(ControlFlow::Exit),
                param: (),
            }
        };

    let on_win_keyboard_input = |input: cgi::win::WinKeyboardInput,
                                 _target: &EventLoopWindowTarget<()>|
     -> cgi::win::HandlerRes<()> {
        let control_flow = match input {
            cgi::win::WinKeyboardInput {
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

        cgi::win::HandlerRes {
            control_flow,
            param: (),
        }
    };

    wloop
        .on_win_close_requested(Some(Box::new(on_win_close_requested)))
        .on_win_keyboard_input(Some(Box::new(on_win_keyboard_input)));

    wloop.run();
}
