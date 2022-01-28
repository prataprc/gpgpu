use colored::Colorize;
use structopt::StructOpt;

use std::sync::Arc;

use cgi::{err_at, vk, Error, Result};

mod info;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(long = "debug")]
    debug: bool,

    #[structopt(long = "api-dump")]
    api_dump: bool,

    #[structopt(long = "no-color")]
    no_color: bool,

    #[structopt(long = "layers", default_value = "", use_delimiter = true)]
    layers: Vec<String>,

    #[structopt(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clone, StructOpt)]
pub enum SubCommand {
    Version,
    Layers {
        #[structopt(short = "v", help = "verbose")]
        verbose: bool,
    },
    Extensions,
    Devices,
    Surface,
    Formats,
    Device {
        #[structopt(short = "n", help = "list nth physical device")]
        phydev: Option<usize>,
    },
}

fn main() {
    use vulkano::instance::{
        debug::DebugCallback,
        {Instance, InstanceExtensions, Version},
    };

    let opts = Opt::from_args();

    let inst = {
        let inst_extns = InstanceExtensions::supported_by_core().unwrap();
        let mut layers = vec![];
        if opts.debug {
            layers.push("VK_LAYER_LUNARG_monitor");
            layers.push("VK_LAYER_KHRONOS_validation");
        }
        if opts.api_dump {
            layers.push("VK_LAYER_LUNARG_api_dump");
        };
        err_at!(
            Fatal,
            Instance::new(None, Version::V1_5, &inst_extns, layers.into_iter())
        )
        .unwrap()
    };

    let _debug_callback = match opts.debug {
        true => DebugCallback::errors_and_warnings(&inst, debug_callback),
        false => DebugCallback::errors_and_warnings(&inst, no_debug_callback),
    };

    // always/never color, whether or not output is piped to some other program
    colored::control::set_override(!opts.no_color);

    let res = match &opts.subcmd {
        SubCommand::Version => info_version(opts, inst),
        SubCommand::Layers { .. } => info_layers(opts),
        SubCommand::Extensions => info_extensions(opts, inst),
        SubCommand::Devices => info_devices(opts, inst),
        SubCommand::Surface => info_surface(opts),
        SubCommand::Formats => info_formats(opts),
        SubCommand::Device { .. } => info_device(opts),
    };

    res.map_err(|err| println!("unexpected error: {}", err))
        .ok();
}

fn info_version(_opts: Opt, inst: Arc<vulkano::instance::Instance>) -> Result<()> {
    use vulkano::{device::physical::PhysicalDevice, instance::loader::auto_loader};

    {
        let loader = err_at!(Fatal, auto_loader())?;
        let version = err_at!(Fatal, loader.api_version())?;
        println!("InstanceVersion  : {}", version);
    }

    {
        for pd in PhysicalDevice::enumerate(&inst) {
            println!("PhysicalDevice {} : {}", pd.index(), pd.api_version());
        }
    }

    Ok(())
}

fn info_layers(opts: Opt) -> Result<()> {
    match opts.subcmd {
        SubCommand::Layers { verbose } if verbose => {
            let layers = vk::layer_properties()?;
            vk::make_table(&layers).print_tty(!opts.no_color);
        }
        SubCommand::Layers { .. } => println!("{}", vk::layer_names()?.join("\n")),
        _ => unreachable!(),
    }

    Ok(())
}

fn info_extensions(opts: Opt, inst: Arc<vulkano::instance::Instance>) -> Result<()> {
    use vulkano::{device::physical::PhysicalDevice, instance::InstanceExtensions};

    match opts.subcmd {
        SubCommand::Extensions => {
            let inst_extns = err_at!(Fatal, InstanceExtensions::supported_by_core())?;

            let mut extensions = instance_extensions!(inst_extns);
            extensions.sort();

            println!("{}", "Instance Extensions".green());
            println!("-------------------");
            println!("{}", extensions.join("\n"));

            println!();

            println!("{}", "Device Extensions".green());
            println!("-----------------");
            for pd in PhysicalDevice::enumerate(&inst) {
                let extns = physical_device_extensions!(pd.supported_extensions());
                println!(
                    "Physical device {:2} supports {:3} extensions",
                    pd.index(),
                    extns.len()
                );
            }

            println!();
        }
        _ => unreachable!(),
    }

    Ok(())
}

fn info_devices(_opts: Opt, inst: Arc<vulkano::instance::Instance>) -> Result<()> {
    use vulkano::device::physical::PhysicalDevice;

    for pd in PhysicalDevice::enumerate(&inst) {
        println!("PhysicalDevice {} : {}", pd.index(), pd.api_version());
    }

    Ok(())
}

fn info_surface(_opts: Opt) -> Result<()> {
    //use crate::info::ImageFormat;
    //use vgi::extensions_for_features;
    //use vulkano::format::Format;

    //let vobj: Vulkan = Builder::new()?
    //    .with_extensions(None) // enable core instance-extensions.
    //    .build_for_surface(vulkano_win::required_extensions())
    //    .unwrap();
    //let pds = vobj.to_physical_devices();
    //let pd = pds[DEFAULT_PHYDEV];

    //let event_loop = EventLoop::new();
    //let surface = WindowBuilder::new()
    //    .build_vk_surface(&event_loop, vobj.to_instance())
    //    .unwrap();

    //let caps = match pds.len() {
    //    0 => panic!("no physical device found"),
    //    _ => {
    //        let caps = surface.capabilities(pd).unwrap();
    //        let mut table = make_table(&info::surface_capabilities(caps.clone()));
    //        table.set_titles(row![Fy => "Surface-capability", "Value"]);
    //        table.print_tty(opts.color);
    //        caps
    //    }
    //};
    //println!();

    //let formats: Vec<Format> = caps
    //    .supported_formats
    //    .iter()
    //    .map(|(f, _)| f.clone())
    //    .collect();

    //make_table(&formats).print_tty(opts.color);
    //println!();

    //// format features
    //println!("{}", "Format supported features".red());
    //let rows: Vec<Row> = {
    //    formats
    //        .iter()
    //        .map(|f| (f, f.properties(pd)))
    //        .filter_map(|(f, p)| {
    //            let mut row = p.to_row();
    //            let mut iter = row.iter();
    //            iter.next();
    //            match iter.any(|c| c.get_content() != "-") {
    //                true => {
    //                    row.insert_cell(0, From::from(&format!("{:?}", f)));
    //                    Some(row)
    //                }
    //                false => None,
    //            }
    //        })
    //        .collect()
    //};
    //let mut table = Table::init(rows);
    //table.set_titles(FormatProperties::to_head());
    //table.set_format(FormatProperties::to_format());
    //table.print_tty(opts.color);
    //println!();

    //let (device, _iter) = {
    //    let features = pd.supported_features();
    //    let extens =
    //        extensions_for_features(&features, DeviceExtensions::supported_by_device(pd));
    //    let qfs = pd.queue_families().map(|q| (q, 1.0));
    //    Device::new(pd, &features, &extens, qfs).unwrap()
    //};
    //println!("{}", "ImageFormat properties".red());
    //let create_flags = ImageCreateFlags::none();
    //let mut image_formats = vec![];
    //for format in formats.into_iter() {
    //    for ty in info::image_type_list().into_iter() {
    //        for tiling in info::image_tiling_list().into_iter() {
    //            for usage in info::image_usage_list().into_iter() {
    //                let props = device.image_format_properties(
    //                    format,
    //                    ty,
    //                    tiling,
    //                    usage,
    //                    create_flags,
    //                );
    //                match props {
    //                    Ok(props) => image_formats
    //                        .push(ImageFormat::new(format, ty, tiling, usage, props)),
    //                    Err(_) => (),
    //                }
    //            }
    //        }
    //    }
    //}
    //filter_empty_rows(make_table(&image_formats)).print_tty(opts.color);
    //println!();

    //Ok(())
    todo!()
}

fn info_formats(_opts: Opt) -> Result<()> {
    //use info::ImageFormat;
    //use vgi::extensions_for_features;

    //let phydev = opts.phydev.unwrap_or(DEFAULT_PHYDEV);

    //let vobj: Vulkan = Builder::new()?
    //    .with_extensions(None) // enable core instance-extensions.
    //    .build_for_surface(vulkano_win::required_extensions())
    //    .unwrap();
    //let pd = vobj.to_physical_devices()[phydev];

    //// format attributes
    //make_table(&info::format_list()).print_tty(opts.color);
    //println!();

    //// format features
    //println!("{}", "Format supported features".red());
    //let rows: Vec<Row> = {
    //    let formats = info::format_list();
    //    formats
    //        .iter()
    //        .map(|f| (f, f.properties(pd)))
    //        .filter_map(|(f, p)| {
    //            let mut row = p.to_row();
    //            let mut iter = row.iter();
    //            iter.next();
    //            match iter.any(|c| c.get_content() != "-") {
    //                true => {
    //                    row.insert_cell(0, From::from(&format!("{:?}", f)));
    //                    Some(row)
    //                }
    //                false => None,
    //            }
    //        })
    //        .collect()
    //};
    //let mut table = Table::init(rows);
    //table.set_titles(FormatProperties::to_head());
    //table.set_format(FormatProperties::to_format());
    //table.print_tty(opts.color);
    //println!();

    //let (device, _iter) = {
    //    let features = pd.supported_features();
    //    let extens =
    //        extensions_for_features(&features, DeviceExtensions::supported_by_device(pd));
    //    let qfs = pd.queue_families().map(|q| (q, 1.0));
    //    Device::new(pd, &features, &extens, qfs).unwrap()
    //};

    //println!("{}", "ImageFormat properties".red());
    //let create_flags = ImageCreateFlags::none();
    //let mut image_formats = vec![];
    //for format in info::format_list().into_iter() {
    //    for ty in info::image_type_list().into_iter() {
    //        for tiling in info::image_tiling_list().into_iter() {
    //            for usage in info::image_usage_list().into_iter() {
    //                let props = device.image_format_properties(
    //                    format,
    //                    ty,
    //                    tiling,
    //                    usage,
    //                    create_flags,
    //                );
    //                match props {
    //                    Ok(props) => image_formats
    //                        .push(ImageFormat::new(format, ty, tiling, usage, props)),
    //                    Err(_) => (),
    //                }
    //            }
    //        }
    //    }
    //}
    //filter_empty_rows(make_table(&image_formats)).print_tty(opts.color);
    //println!();

    //Ok(())
    todo!()
}

fn info_device(_opts: Opt) -> Result<()> {
    //use info::{device_extensions, instance_extensions, ChecklistItem};
    //use vgi::{layers, pp::transpose};

    //let layers = layers()?;
    //let layer_names: Vec<String> = layers.iter().map(|l| l.name().to_string()).collect();
    //let vobj: Vulkan = Builder::new()?
    //    .with_extensions(None) // enable core instance-extensions.
    //    .with_layers(layer_names)
    //    .build_for_surface(vulkano_win::required_extensions())
    //    .unwrap();
    //let layers = vobj.enabled_layers();
    //let pds = vobj.to_physical_devices();

    //println!("{}: {}", "API Version".yellow(), vobj.api_version());
    //println!("{}: {}", "Number of physical devices".yellow(), pds.len());
    //println!();

    //println!("{}", "List of layers".yellow());
    //make_table(&layers).print_tty(opts.color);
    //println!();

    //println!("{}:", "Instance core extensions".yellow());
    //make_table(&instance_extensions()).print_tty(opts.color);
    //println!();

    //println!("{}:", "Device supported extensions".yellow());
    //let mut iter = pds.iter().map(|pd| device_extensions(pd.clone()));
    //let mut table = match iter.next() {
    //    Some(dextns) => {
    //        let table = make_table(&dextns);
    //        iter.fold(table, |mut table, dextns| {
    //            for (r, c) in table.row_iter_mut().zip(make_table(&dextns).column_iter(1))
    //            {
    //                r.add_cell(c.clone())
    //            }
    //            table
    //        })
    //    }
    //    None => make_table(&Vec::<ChecklistItem>::default()),
    //};
    //table.unset_titles();
    //// add titles
    //let mut head = row![Fy => "Extension"];
    //pds.iter().for_each(|pd| {
    //    head.add_cell(Cell::from(&format!("device-{}", pd.index())).style_spec("Fy"))
    //});
    //table.set_titles(head);
    //table.set_format(PhysicalDevice::to_format());
    //// print
    //table.print_tty(opts.color);
    //println!();

    //println!("{}:", "Physical device properties".yellow());
    //let mut table = make_table(&pds);
    //table.unset_titles();
    //let mut table = transpose(table);
    //// add property column
    //for (row, cell) in table
    //    .row_iter_mut()
    //    .zip(PhysicalDevice::to_head().into_iter())
    //{
    //    row.insert_cell(0, cell.clone())
    //}
    //// add titles
    //let mut head = row![Fy => "Property/Limit"];
    //pds.iter().for_each(|pd| {
    //    head.add_cell(Cell::from(&format!("device-{}", pd.index())).style_spec("Fy"))
    //});
    //table.set_titles(head);
    //table.set_format(PhysicalDevice::to_format());
    //// print
    //table.print_tty(opts.color);
    //println!();

    //make_table_pdfeatures(&vobj).print_tty(opts.color);
    //println!();

    //print_physical_devices(&pds, &opts);
    //println!();

    //Ok(())
    todo!()
}

//fn make_table_pdfeatures(vobj: &Vulkan) -> prettytable::Table {
//    use info::{device_features, ChecklistItem};
//
//    let mut pds = vobj.to_physical_devices().to_vec();
//    let mut table = prettytable::Table::new();
//
//    let head = row![Fy => "Feature-name", format!("Device-0")];
//
//    match pds.len() {
//        0 => table,
//        _ => {
//            for l in device_features(pds.remove(0)).iter() {
//                table.add_row(l.to_row());
//            }
//            for (mut i, pd) in pds.into_iter().enumerate() {
//                i += 1;
//                for l in device_features(pd).iter() {
//                    table
//                        .get_mut_row(i)
//                        .unwrap()
//                        .add_cell(l.to_row().get_cell(1).unwrap().clone())
//                }
//            }
//
//            table.set_titles(head);
//            table.set_format(ChecklistItem::to_format());
//            table
//        }
//    }
//}
//
//fn print_physical_devices(pds: &[PhysicalDevice], opts: &Opt) {
//    for pd in pds {
//        let name = pd.properties().device_name.as_ref().unwrap();
//        let s = format!("Physical-device {{{}}} {}", pd.index(), name,).red();
//        println!("{}", s);
//
//        let mut heap_table = prettytable::Table::new();
//        let mut type_table = prettytable::Table::new();
//
//        heap_table.set_titles(MemoryHeap::to_head());
//        type_table.set_titles(MemoryType::to_head());
//
//        let heaps: Vec<MemoryHeap> = pd.memory_heaps().collect();
//        let types: Vec<MemoryType> = pd.memory_types().collect();
//        let queues: Vec<QueueFamily> = pd.queue_families().collect();
//
//        make_table(&heaps).print_tty(opts.color);
//        println!();
//        make_table(&types).print_tty(opts.color);
//        println!();
//        make_table(&queues).print_tty(opts.color);
//        println!();
//    }
//}
//
//fn filter_empty_rows(table: Table) -> Table {
//    let mut packed_table = table.clone();
//    for (i, row) in table.row_iter().enumerate() {
//        if row.is_empty() {
//            packed_table.remove_row(i)
//        }
//    }
//    packed_table
//}

// TODO: debug-callback not happening !!
fn debug_callback(msg: &vulkano::instance::debug::Message) {
    let layer = format!("[{}]", msg.layer_prefix.unwrap_or_else(|| "--NA--")).cyan();
    let typ = format!("<{}>", dbg_msg_type!(msg.ty).join(","));
    let severity = match dbg_msg_severity!(msg.severity).as_str() {
        "NONE" => format!("[NONE]").truecolor(100, 100, 100),
        "VERB" => format!("[VERB]").white(),
        "INFO" => format!("[INFO]").blue(),
        "WARN" => format!("[WARN]").yellow(),
        "EROR" => format!("[EROR]").red(),
        _ => unreachable!(),
    };

    println!("{} {:17} {} {}", layer, typ, severity, msg.description);
}

fn no_debug_callback(_msg: &vulkano::instance::debug::Message) {
    ()
}
