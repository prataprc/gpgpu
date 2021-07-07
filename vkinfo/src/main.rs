use colored::Colorize;
use prettytable::{cell, row, Cell, Row, Table};
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
    vulkan::{Builder, Vulkan},
    Result,
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

    let res = if opts.surface {
        info_surface(opts)
    } else if opts.formats {
        info_formats(opts)
    } else {
        info_device(opts)
    };

    match res {
        Ok(_) => (),
        Err(err) => println!("unexpected error: {}", err),
    }
}

fn info_surface(_opts: Opt) -> Result<()> {
    use crate::info::{
        image_tiling_list, image_type_list, image_usage_list, surface_capabilities,
        ImageFormat,
    };
    use vgi::vulkan::extensions_for_features;
    use vulkano::format::Format;

    let force_color = false;

    let vobj: Vulkan = Builder::new()?.build().unwrap();
    let pds = vobj.to_physical_devices();
    let pd = pds[DEFAULT_PHYDEV];

    let event_loop = EventLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, vobj.to_instance())
        .unwrap();

    match pds.len() {
        0 => (),
        _ => {
            let caps = surface_capabilities(pd, &surface);
            let mut table = make_table(&caps);
            table.set_titles(row![Fy => "Surface-capability", "Value"]);
            table.print_tty(force_color);
        }
    }
    println!();

    let formats: Vec<Format> = surface
        .capabilities(pd)
        .unwrap()
        .supported_formats
        .iter()
        .map(|(f, _)| f.clone())
        .collect();

    make_table(&formats).print_tty(force_color);
    println!();

    // format features
    println!("{}", "Format supported features".red());
    let rows: Vec<Row> = {
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
        let extens =
            extensions_for_features(&features, DeviceExtensions::supported_by_device(pd));
        let qfs = pd.queue_families().map(|q| (q, 1.0));
        Device::new(pd, &features, &extens, qfs).unwrap()
    };
    println!("{}", "ImageFormat properties".red());
    let create_flags = ImageCreateFlags::none();
    let mut image_formats = vec![];
    for format in formats.into_iter() {
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

    Ok(())
}

fn info_formats(opts: Opt) -> Result<()> {
    use info::{
        format_list, image_tiling_list, image_type_list, image_usage_list, ImageFormat,
    };
    use vgi::vulkan::extensions_for_features;

    let force_color = false;
    let phydev = opts.phydev.unwrap_or(DEFAULT_PHYDEV);

    let vobj: Vulkan = Builder::new()?.build().unwrap();
    let pd = vobj.to_physical_devices()[phydev];

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
        let extens =
            extensions_for_features(&features, DeviceExtensions::supported_by_device(pd));
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

    Ok(())
}

fn info_device(_opts: Opt) -> Result<()> {
    use info::{device_extensions, instance_extensions, ChecklistItem};
    use vgi::{pp::transpose, vulkan::layers};

    let force_color = false;

    let layers = layers()?;
    let layer_names: Vec<&str> = layers.iter().map(|l| l.name()).collect();
    let vobj: Vulkan = Builder::new()?.with_layers(layer_names).build().unwrap();
    let layers = vobj.enabled_layers();
    let pds = vobj.to_physical_devices();

    println!("{}: {}", "API Version".yellow(), vobj.api_version());
    println!("{}: {}", "Number of physical devices".yellow(), pds.len());
    println!();

    println!("{}", "List of layers".yellow());
    make_table(&layers).print_tty(force_color);
    println!();

    println!("{}:", "Instance core extensions".yellow());
    make_table(&instance_extensions()).print_tty(force_color);
    println!();

    println!("{}:", "Device core extensions".yellow());
    let mut iter = pds.iter().map(|pd| device_extensions(pd.clone()));
    let mut table = match iter.next() {
        Some(dextns) => {
            let table = make_table(&dextns);
            iter.fold(table, |mut table, dextns| {
                for (r, c) in table.row_iter_mut().zip(make_table(&dextns).column_iter(1))
                {
                    r.add_cell(c.clone())
                }
                table
            })
        }
        None => make_table(&Vec::<ChecklistItem>::default()),
    };
    table.unset_titles();
    // add titles
    let mut head = row![Fy => "Extension"];
    pds.iter().for_each(|pd| {
        head.add_cell(Cell::from(&format!("device-{}", pd.index())).style_spec("Fy"))
    });
    table.set_titles(head);
    table.set_format(PhysicalDevice::to_format());
    // print
    table.print_tty(force_color);
    println!();

    println!("{}:", "Physical device properties".yellow());
    let mut table = make_table(&pds);
    table.unset_titles();
    let mut table = transpose(table);
    // add property column
    for (row, cell) in table
        .row_iter_mut()
        .zip(PhysicalDevice::to_head().into_iter())
    {
        row.insert_cell(0, cell.clone())
    }
    // add titles
    let mut head = row![Fy => "Property/Limit"];
    pds.iter().for_each(|pd| {
        head.add_cell(Cell::from(&format!("device-{}", pd.index())).style_spec("Fy"))
    });
    table.set_titles(head);
    table.set_format(PhysicalDevice::to_format());
    // print
    table.print_tty(force_color);
    println!();

    make_table_pdfeatures(&vobj).print_tty(force_color);
    println!();

    print_physical_devices(&pds, force_color);
    println!();

    Ok(())
}

fn make_table_pdfeatures(vobj: &Vulkan) -> prettytable::Table {
    use info::{device_features, ChecklistItem};

    let mut pds = vobj.to_physical_devices().to_vec();
    let mut table = prettytable::Table::new();

    let head = row![Fy => "Feature-name", format!("Device-0")];

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

            table.set_titles(head);
            table.set_format(ChecklistItem::to_format());
            table
        }
    }
}

fn print_physical_devices(pds: &[PhysicalDevice], force_color: bool) {
    for pd in pds {
        let name = pd.properties().device_name.as_ref().unwrap();
        let s = format!("Physical-device {{{}}} {}", pd.index(), name,).red();
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
