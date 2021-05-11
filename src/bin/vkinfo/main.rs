use colored::Colorize;
use prettytable::{cell, row};
use structopt::StructOpt;
use vulkano::instance::{
    InstanceExtensions, LayerProperties, MemoryHeap, MemoryType, PhysicalDevice,
    QueueFamily,
};

use cgi::vulkan::{self, PrettyRow};

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "vkinfo", version = "0.0.1")]
pub struct Opt {
    #[structopt(long = "debug")]
    debug: bool,
}

fn main() {
    use vulkano::instance::Instance;

    let opts = Opt::from_args();
    let force_color = false;

    let layers = vulkan::layers().unwrap();

    let mut extns = vulkan::extensions(None).unwrap();
    extns.sort_by_key(|e| e.name().to_string());
    for layer in layers.iter() {
        let name = layer.name().to_string();
        for extn in vulkan::extensions(Some(name.as_str())).unwrap().into_iter() {
            let ext_name = extn.name().to_string();
            match extns.binary_search_by_key(&ext_name, |e| e.name().to_string()) {
                Ok(off) => extns[off].add_layer(name.as_str()),
                Err(off) => extns.insert(off, extn),
            }
        }
    }

    let ok_layers = enable_layers(&layers);
    let inst_extns = enable_extensions(&extns);

    let instance = {
        let app_info = vulkano::app_info_from_cargo_toml!();
        Instance::new(Some(&app_info), &inst_extns, ok_layers).unwrap()
    };

    let pds: Vec<PhysicalDevice> = PhysicalDevice::enumerate(&instance).collect();
    for pd in pds.iter() {
        for extn in vulkan::device_extensions(pd.clone()).unwrap() {
            let name = extn.name().to_string();
            match extns.binary_search_by_key(&name, |e| e.name().to_string()) {
                Ok(off) => extns[off].add_physical_device(pd.index()),
                Err(off) => extns.push(extn),
            }
        }
    }

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

            let mut lists: Vec<Vec<vulkan::PhysicalDeviceLimit>> = pds
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

            table.set_format(vulkan::PhysicalDeviceLimit::to_format());
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

            let mut lists: Vec<Vec<vulkan::PhysicalDeviceFeature>> = pds
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

            table.set_format(vulkan::PhysicalDeviceFeature::to_format());
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

fn enable_layers(layers: &[LayerProperties]) -> Vec<&'static str> {
    layers
        .iter()
        .filter_map(|layer| match layer.name() {
            "VK_LAYER_LUNARG_parameter_validation" => {
                Some("VK_LAYER_LUNARG_parameter_validation")
            }
            "VK_LAYER_LUNARG_object_tracker" => Some("VK_LAYER_LUNARG_object_tracker"),
            "VK_LAYER_LUNARG_standard_validation" => {
                Some("VK_LAYER_LUNARG_standard_validation")
            }
            "VK_LAYER_LUNARG_core_validation" => Some("VK_LAYER_LUNARG_core_validation"),
            "VK_LAYER_GOOGLE_threading" => Some("VK_LAYER_GOOGLE_threading"),
            "VK_LAYER_GOOGLE_unique_objects" => Some("VK_LAYER_GOOGLE_unique_objects"),
            _ => None,
        })
        .collect()
}

fn enable_extensions(extns: &[vulkan::ExtensionProperties]) -> InstanceExtensions {
    let mut ie = InstanceExtensions::none();

    extns.iter().for_each(|extn| match extn.name() {
        "VK_KHR_surface" => ie.khr_surface = true,
        "VK_KHR_display" => ie.khr_display = true,
        "VK_KHR_xlib_surface" => ie.khr_xlib_surface = true,
        "VK_KHR_xcb_surface" => ie.khr_xcb_surface = true,
        "VK_KHR_wayland_surface" => ie.khr_wayland_surface = true,
        "VK_KHR_android_surface" => ie.khr_android_surface = true,
        "VK_KHR_win32_surface" => ie.khr_win32_surface = true,
        "VK_EXT_debug_utils" => ie.ext_debug_utils = true,
        "VK_MVK_ios_surface" => ie.mvk_ios_surface = true,
        "VK_MVK_macos_surface" => ie.mvk_macos_surface = true,
        "VK_MVK_moltenvk" => ie.mvk_moltenvk = true,
        "VK_NN_vi_surface" => ie.nn_vi_surface = true,
        "VK_EXT_swapchain_colorspace" => ie.ext_swapchain_colorspace = true,
        "VK_KHR_get_physical_device_properties2" => {
            ie.khr_get_physical_device_properties2 = true;
        }
        "VK_KHR_get_surface_capabilities2" => {
            ie.khr_get_surface_capabilities2 = true;
        }
        "VK_KHR_device_group_creation" => ie.khr_device_group_creation = true,
        "VK_KHR_external_fence_capabilities" => ie.khr_external_fence_capabilities = true,
        "VK_KHR_external_memory_capabilities" => {
            ie.khr_external_memory_capabilities = true;
        }
        "VK_KHR_external_semaphore_capabilities" => {
            ie.khr_external_semaphore_capabilities = true;
        }
        "VK_KHR_get_display_properties2" => ie.khr_get_display_properties2 = true,
        "VK_EXT_acquire_xlib_display" => ie.ext_acquire_xlib_display = true,
        "VK_EXT_debug_report" => ie.ext_debug_report = true,
        "VK_EXT_direct_mode_display" => ie.ext_direct_mode_display = true,
        "VK_EXT_display_surface_counter" => ie.ext_display_surface_counter = true,
        name => panic!("{} extension uknown", name),
    });

    ie
}
