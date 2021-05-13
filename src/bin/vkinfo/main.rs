use colored::Colorize;
use prettytable::{cell, row};
use structopt::StructOpt;
use vulkano::instance::{MemoryHeap, MemoryType, PhysicalDevice, QueueFamily};
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use cgi::vulkan::{self, PrettyRow, Vulkan};

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "vkinfo", version = "0.0.1")]
pub struct Opt {
    #[structopt(long = "debug")]
    debug: bool,

    #[structopt(long = "surface")]
    surface: bool,
}

fn main() {
    let opts = Opt::from_args();

    if opts.surface {
        info_surface(opts)
    } else {
        info_device(opts)
    }
}

fn info_surface(_opts: Opt) {
    let force_color = false;

    let vobj = Vulkan::new();
    let pds = vobj.as_physical_devices();

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, vobj.to_instance())
        .unwrap();

    match pds.len() {
        0 => (),
        _ => {
            let caps = vulkan::surface_capabilities(pds[0], &surface);
            let mut table = make_table(&caps);
            table.set_titles(row![Fy => "Surface-capability", "Value"]);
            table.print_tty(force_color);
        }
    }
}

fn info_device(_opts: Opt) {
    let force_color = false;

    let vobj = Vulkan::new();
    let layers = vobj.as_layers();
    let extns = vobj.as_extensions();
    let pds = vobj.as_physical_devices();

    println!("{}: {}", "Number of physical devices".yellow(), pds.len());
    println!();
    make_table(&pds).print_tty(force_color);
    println!();

    make_table(&layers).print_tty(force_color);
    println!();
    make_table(&extns).print_tty(force_color);
    println!();

    make_table_pdlimits(&pds).print_tty(force_color);
    println!();
    make_table_pdfeatures(&pds).print_tty(force_color);
    println!();

    print_physical_devices(&pds, force_color);
    println!();
}

fn make_table<R>(rows: &[R]) -> prettytable::Table
where
    R: vulkan::PrettyRow,
{
    let mut table = prettytable::Table::new();

    match rows.len() {
        0 => table,
        _ => {
            table.set_titles(R::to_head());
            rows.iter().for_each(|r| {
                table.add_row(r.to_row());
            });
            table.set_format(R::to_format());
            table
        }
    }
}

fn make_table_pdlimits(pds: &[PhysicalDevice]) -> prettytable::Table {
    let mut table = prettytable::Table::new();

    match pds.len() {
        0 => table,
        _ => {
            let titles = row![Fy => "Limit-name", format!("Device-{}", pds[0].index()) ];

            let mut lists: Vec<Vec<vulkan::LimitItem>> = pds
                .iter()
                .map(|pd| vulkan::physical_device_limits(&pd))
                .collect();
            let list = lists.remove(0);

            for l in list.iter() {
                table.add_row(l.to_row());
            }

            for list in lists.into_iter() {
                for (i, l) in list.iter().enumerate() {
                    table
                        .get_mut_row(i)
                        .unwrap()
                        .add_cell(l.to_row().get_cell(1).unwrap().clone())
                }
            }

            table.set_titles(titles);

            table.set_format(vulkan::LimitItem::to_format());
            table
        }
    }
}

fn make_table_pdfeatures(pds: &[PhysicalDevice]) -> prettytable::Table {
    let mut table = prettytable::Table::new();

    match pds.len() {
        0 => table,
        _ => {
            let titles =
                row![Fy => "Feature-name", format!("Device-{}", pds[0].index()) ];

            let mut lists: Vec<Vec<vulkan::ChecklistItem>> = pds
                .iter()
                .map(|pd| vulkan::physical_device_features(&pd))
                .collect();
            let list = lists.remove(0);

            for l in list.iter() {
                table.add_row(l.to_row());
            }

            for list in lists.into_iter() {
                for (i, l) in list.iter().enumerate() {
                    table
                        .get_mut_row(i)
                        .unwrap()
                        .add_cell(l.to_row().get_cell(1).unwrap().clone())
                }
            }

            table.set_titles(titles);

            table.set_format(vulkan::ChecklistItem::to_format());
            table
        }
    }
}

fn print_physical_devices(pds: &[PhysicalDevice], force_color: bool) {
    for pd in pds {
        let s = format!("Physical-device {{{}}} {}", pd.index(), pd.name()).red();
        println!("{}", s);

        let mut heap_table = prettytable::Table::new();
        let mut type_table = prettytable::Table::new();

        heap_table.set_titles(MemoryHeap::to_head());
        type_table.set_titles(MemoryType::to_head());

        let heaps: Vec<MemoryHeap> = pd.memory_heaps().collect();
        let types: Vec<MemoryType> = pd.memory_types().collect();
        let queues: Vec<QueueFamily> = pd.queue_families().collect();

        make_table(&heaps).print_tty(force_color);
        println!();
        make_table(&types).print_tty(force_color);
        println!();
        make_table(&queues).print_tty(force_color);
        println!();
    }
}
