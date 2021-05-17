use colored::Colorize;
use prettytable::{cell, row, Row, Table};
use structopt::StructOpt;
use vulkano::{
    device::{Device, DeviceExtensions},
    format::FormatProperties,
    image::ImageCreateFlags,
    instance::{MemoryHeap, MemoryType, PhysicalDevice, QueueFamily},
};
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

use vgi::{
    pp::{make_table, PrettyRow},
    vulkan::Vulkan,
};

mod info;

const DEFAULT_PHYDEV: usize = 0;

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "vkinfo", version = "0.0.1")]
pub struct Opt {
    #[structopt(long = "debug")]
    debug: bool,

    #[structopt(long = "surface")]
    surface: bool,

    #[structopt(long = "formats")]
    formats: bool,

    #[structopt(long = "phydev")]
    phydev: Option<usize>,
}

fn main() {
    let opts = Opt::from_args();

    if opts.surface {
        info_surface(opts)
    } else if opts.formats {
        info_formats(opts)
    } else {
        info_device(opts)
    }
}

fn info_surface(_opts: Opt) {
    use crate::info::surface_capabilities;

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
            let caps = surface_capabilities(pds[DEFAULT_PHYDEV], &surface);
            let mut table = make_table(&caps);
            table.set_titles(row![Fy => "Surface-capability", "Value"]);
            table.print_tty(force_color);
        }
    }
}

fn info_formats(opts: Opt) {
    use info::{
        format_list, image_tiling_list, image_type_list, image_usage_list, ImageFormat,
    };

    let force_color = false;
    let phydev = opts.phydev.unwrap_or(DEFAULT_PHYDEV);

    let vobj = Vulkan::new();
    let pd = vobj.as_physical_devices()[phydev];

    // format attributes
    make_table(&format_list()).print_tty(force_color);
    println!();

    // format features
    println!("{}", "Format supported features".red());
    let rows: Vec<Row> = {
        let formats = format_list();
        formats
            .iter()
            .map(|f| (f, f.properties(pd)))
            .filter_map(|(f, p)| {
                let mut row = p.to_row();
                let mut iter = row.iter();
                iter.next();
                match iter.any(|c| c.get_content() != "-") {
                    true => {
                        row.insert_cell(0, From::from(&format!("{:?}", f)));
                        Some(row)
                    }
                    false => None,
                }
            })
            .collect()
    };
    let mut table = Table::init(rows);
    table.set_titles(FormatProperties::to_head());
    table.set_format(FormatProperties::to_format());
    table.print_tty(force_color);
    println!();

    let (device, _iter) = {
        let features = pd.supported_features();
        let extens = DeviceExtensions::supported_by_device(pd);
        let qfs = pd.queue_families().map(|q| (q, 1.0));
        Device::new(pd, &features, &extens, qfs).unwrap()
    };

    println!("{}", "ImageFormat properties".red());
    let create_flags = ImageCreateFlags::none();
    let mut image_formats = vec![];
    for format in format_list().into_iter() {
        for ty in image_type_list().into_iter() {
            for tiling in image_tiling_list().into_iter() {
                for usage in image_usage_list().into_iter() {
                    let props = device.image_format_properties(
                        format,
                        ty,
                        tiling,
                        usage,
                        create_flags,
                    );
                    match props {
                        Ok(props) => image_formats
                            .push(ImageFormat::new(format, ty, tiling, usage, props)),
                        Err(_) => (),
                    }
                }
            }
        }
    }
    filter_empty_rows(make_table(&image_formats)).print_tty(force_color);
    println!();
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
    make_table_pdfeatures(&vobj).print_tty(force_color);
    println!();

    print_physical_devices(&pds, force_color);
    println!();
}

fn make_table_pdlimits(pds: &[PhysicalDevice]) -> prettytable::Table {
    use info::{physical_device_limits, LimitItem};

    let mut table = prettytable::Table::new();

    match pds.len() {
        0 => table,
        _ => {
            let titles = row![
                Fy => "Limit-name", format!("Device-{}", pds[DEFAULT_PHYDEV].index())
            ];

            let mut lists: Vec<Vec<LimitItem>> =
                pds.iter().map(|pd| physical_device_limits(&pd)).collect();
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

            table.set_format(LimitItem::to_format());
            table
        }
    }
}

fn make_table_pdfeatures(vobj: &Vulkan) -> prettytable::Table {
    use info::{device_features, ChecklistItem};

    let mut pds = vobj.as_physical_devices().to_vec();
    let mut table = prettytable::Table::new();

    let titles = row![Fy => "Feature-name", format!("Device-0")];

    match pds.len() {
        0 => table,
        _ => {
            for l in device_features(pds.remove(0)).iter() {
                table.add_row(l.to_row());
            }
            for (mut i, pd) in pds.into_iter().enumerate() {
                i += 1;
                for l in device_features(pd).iter() {
                    table
                        .get_mut_row(i)
                        .unwrap()
                        .add_cell(l.to_row().get_cell(1).unwrap().clone())
                }
            }

            table.set_titles(titles);
            table.set_format(ChecklistItem::to_format());
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

fn filter_empty_rows(table: Table) -> Table {
    let mut packed_table = table.clone();
    for (i, row) in table.row_iter().enumerate() {
        if row.is_empty() {
            packed_table.remove_row(i)
        }
    }
    packed_table
}
