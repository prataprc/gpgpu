use colored::Colorize;
use structopt::StructOpt;

use std::sync::Arc;

use cgi::{err_at, util, vk, Error, Result};

mod debug;
mod info;

#[derive(StructOpt)]
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
        #[structopt(long = "extensions")]
        extensions: bool,
        #[structopt(long = "properties")]
        properties: bool,
        #[structopt(long = "features")]
        features: bool,
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
            layers.push("VK_LAYER_LUNARG_monitor".to_string());
            layers.push("VK_LAYER_KHRONOS_validation".to_string());
        }
        if opts.api_dump {
            layers.push("VK_LAYER_LUNARG_api_dump".to_string());
        };
        opts.layers
            .iter()
            .filter(|l| l.len() > 0)
            .for_each(|l| layers.push(l.clone()));
        layers.dedup();
        let layers = vk::check_layer_names(layers).unwrap();

        let (app_info, ver) = (vulkano::app_info_from_cargo_toml!(), Version::V1_5);
        let lyrs = layers.iter().map(|s| s.as_str());
        err_at!(
            Fatal,
            Instance::new(Some(&app_info), ver, &inst_extns, lyrs)
        )
        .unwrap()
    };

    let _debug_callback = match opts.debug {
        true => DebugCallback::errors_and_warnings(&inst, debug::debug_callback),
        false => DebugCallback::errors_and_warnings(&inst, debug::no_debug_callback),
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
        SubCommand::Device { .. } => info_device(opts, inst),
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
            util::make_table(&layers).print_tty(!opts.no_color);
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

fn info_device(opts: Opt, inst: Arc<vulkano::instance::Instance>) -> Result<()> {
    use vulkano::device::physical::{
        MemoryHeap, MemoryType, PhysicalDevice, QueueFamily,
    };

    let (device_no, is_extns, is_props, is_features) = match opts.subcmd {
        SubCommand::Device {
            phydev,
            extensions,
            properties,
            features,
        } => (phydev.unwrap_or(0), extensions, properties, features),
        _ => unreachable!(),
    };
    let phydev = PhysicalDevice::from_index(&inst, device_no).unwrap();
    let supported_extns = physical_device_extensions!(phydev.supported_extensions());
    let required_extns = physical_device_extensions!(phydev.required_extensions());
    let features = device_features!(phydev.supported_features());
    let ps = phydev.properties();
    let proplines = device_properties!(ps);

    if is_extns {
        println!("Supported extensions : {}", supported_extns.len());
        for extn in supported_extns.iter() {
            println!("  {}", extn);
        }
    } else if is_props {
        println!("Properties: {}", proplines.len());
        for line in proplines.iter() {
            println!("  {}", line);
        }
    } else if is_features {
        println!("Features: {}", features.len());
        for line in features.iter() {
            println!("  {}", line);
        }
    } else {
        println!("API Version              : {}", phydev.api_version());
        println!("Index                    : {}", phydev.index());
        println!("device_id                : {:x}", ps.device_id);
        println!("device_name              : {}", ps.device_name);
        println!("device_type              : {:?}", ps.device_type);
        println!("device_luid              : {:?}", ps.device_luid);
        println!("device_uuid              : {:?}", ps.device_uuid);
        println!(
            "discrete_queue_priorities: {}",
            ps.discrete_queue_priorities
        );
        println!("driver_id                : {:?}", ps.driver_id);
        println!("driver_info              : {:?}", ps.driver_info);
        println!("driver_name              : {:?}", ps.driver_name);
        println!("driver_uuid              : {:?}", ps.driver_uuid);
        println!("driver_version           : {:?}", ps.driver_version);
        println!("has_primary              : {:?}", ps.has_primary);
        println!("has_render               : {:?}", ps.has_render);
        println!("line_width_granularity   : {}", ps.line_width_granularity);
        println!("buffer_image_granularity : {}", ps.buffer_image_granularity);
        println!("conformance_version      : {:?}", ps.conformance_version);
        println!("Required extensions      : {}", required_extns.len());
        for extn in required_extns.iter() {
            println!("  {}", extn);
        }
        println!("Supported extensions     : {}", supported_extns.len());
        println!("Properties               : {}", proplines.len());
        println!("Features                 : {}", features.len());
        println!();

        util::make_table(&phydev.memory_types().collect::<Vec<MemoryType>>())
            .print_tty(!opts.no_color);
        println!();

        util::make_table(&phydev.memory_heaps().collect::<Vec<MemoryHeap>>())
            .print_tty(!opts.no_color);
        println!();

        util::make_table(&phydev.queue_families().collect::<Vec<QueueFamily>>())
            .print_tty(!opts.no_color);
        println!();

        util::make_table(&vk::queue_pipeline_stages(&phydev)).print_tty(!opts.no_color);
        println!();
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
