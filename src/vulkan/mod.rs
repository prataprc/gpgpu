//! Vulkan toolkit.

mod info;

use vk_sys as vk;
use vulkano::instance::{Instance, InstanceExtensions, LayerProperties, PhysicalDevice};

use std::sync::Arc;

use crate::{Error, Result};

pub use info::{
    device_extensions, extensions, layers, physical_device_features,
    physical_device_limits, surface_capabilities, ChecklistItem, ExtensionProperties,
    LimitItem, PrettyRow,
};

pub fn check_errors(result: vk::Result) -> Result<()> {
    if result & 0x80000000 > 0 {
        err_at!(Vk, msg: "fail vk-sys code {}", result)?
    }
    Ok(())
}

// TODO: implement drop.
pub struct Vulkan<'a> {
    layers: Vec<LayerProperties>,
    extens: Vec<ExtensionProperties>,
    instance: Box<Arc<Instance>>,
    phys_devices: Vec<PhysicalDevice<'a>>,
}

impl<'a> Vulkan<'a> {
    pub fn new() -> Self {
        let layers = layers().unwrap();

        let mut extens = extensions(None).unwrap();
        extens.sort_by_key(|e| e.name().to_string());
        for layer in layers.iter() {
            let name = layer.name().to_string();
            for extn in extensions(Some(name.as_str())).unwrap().into_iter() {
                let ext_name = extn.name().to_string();
                match extens.binary_search_by_key(&ext_name, |e| e.name().to_string()) {
                    Ok(off) => extens[off].add_layer(name.as_str()),
                    Err(off) => extens.insert(off, extn),
                }
            }
        }

        let instance: Box<Arc<Instance>> = {
            let ok_layers = enable_layers(&layers);
            let inst_extns = enable_extensions(&extens);
            let app_info = vulkano::app_info_from_cargo_toml!();
            Box::new(Instance::new(Some(&app_info), &inst_extns, ok_layers).unwrap())
        };

        let inst = unsafe {
            (instance.as_ref() as *const Arc<Instance>)
                .as_ref()
                .unwrap()
        };
        let pds: Vec<PhysicalDevice> = PhysicalDevice::enumerate(inst).collect();

        for pd in pds.iter() {
            for extn in device_extensions(*pd).unwrap() {
                let name = extn.name().to_string();
                match extens.binary_search_by_key(&name, |e| e.name().to_string()) {
                    Ok(off) => extens[off].add_physical_device(pd.index()),
                    Err(_) => extens.push(extn),
                }
            }
        }

        Vulkan {
            layers,
            extens,
            instance,
            phys_devices: pds,
        }
    }

    pub fn to_instance(&self) -> Arc<Instance> {
        Arc::clone(&self.instance)
    }

    pub fn as_instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    pub fn as_physical_devices(&self) -> &[PhysicalDevice<'a>] {
        &self.phys_devices
    }

    pub fn as_layers(&self) -> &[LayerProperties] {
        &self.layers
    }

    pub fn as_extensions(&self) -> &[ExtensionProperties] {
        &self.extens
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

fn enable_extensions(extns: &[ExtensionProperties]) -> InstanceExtensions {
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
