use colored::Colorize;
use structopt::StructOpt;
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    monitor::{MonitorHandle, VideoMode},
};

use cgi::{util, w, Result};

#[derive(StructOpt)]
pub struct Opt {
    #[structopt(long = "no-color")]
    no_color: bool,

    #[structopt(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clone, StructOpt)]
pub enum SubCommand {
    Monitors {
        #[structopt(long = "modes")]
        modes: bool,

        #[structopt(short = "n")]
        n: Option<usize>,
    },
    Events,
}

fn main() {
    let opts = Opt::from_args();

    let res = match &opts.subcmd {
        SubCommand::Monitors { .. } => info_monitors(opts),
        SubCommand::Events => {
            let evl = EventLoop::new();
            evl.run(event_handler)
        }
    };

    res.map_err(|err| println!("unexpected error: {}", err))
        .ok();
}

fn info_monitors(opts: Opt) -> Result<()> {
    let (modes, n) = match opts.subcmd {
        SubCommand::Monitors { modes, n } => (modes, n),
        _ => unreachable!(),
    };

    let evl = EventLoop::new();
    let monitors: Vec<MonitorHandle> = evl.available_monitors().collect();

    match n {
        Some(n) if modes => {
            let modes = monitors[n].video_modes().collect::<Vec<VideoMode>>();
            util::make_table(&modes).print_tty(!opts.no_color);
        }
        None if modes => match evl.primary_monitor() {
            Some(primary) => {
                let modes = primary.video_modes().collect::<Vec<VideoMode>>();
                util::make_table(&modes).print_tty(!opts.no_color);
            }
            None => println!("{}", "No primary monitor".red()),
        },
        _ => {
            match evl.primary_monitor() {
                Some(primary) => {
                    util::make_table(&vec![primary]).print_tty(!opts.no_color);
                }
                None => println!("{}", "No primary monitor".red()),
            }
            println!();
            util::make_table(&monitors).print_tty(!opts.no_color);
        }
    }

    Ok(())
}

fn event_handler(
    e: Event<()>,
    _target: &EventLoopWindowTarget<()>,
    _control: &mut ControlFlow,
) {
    let mut events = w::Events::default();
    events.append(e);
}
