use vk_sys as vk;
use vulkano::{
    device::{DeviceExtensions, Features},
    instance::{Instance, InstanceExtensions, LayerProperties, PhysicalDevice},
};

use std::{ffi::CStr, sync::Arc};

use crate::{pp::PrettyRow, Error, Result};

/// Vulkan type roughly maps to instance/device object defined by the vulkan spec.
/// This type try to abstract the boiler plate code as much as possible by
/// providing convinient methods and related macros.
pub struct Vulkan<'a> {
    layers: Vec<LayerProperties>,
    extens: InstanceExtensions,
    instance: Box<Arc<Instance>>,
    phydevs: Vec<PhysicalDevice<'a>>,
}

impl<'a> Vulkan<'a> {
    pub fn version() -> vulkano::instance::Version {
        todo!()
    }

    pub fn new() -> Self {
        use vulkano::instance::Version;

        let layers = layers().unwrap();

        let extens = InstanceExtensions::supported_by_core().unwrap();
        let instance: Box<Arc<Instance>> = {
            let ok_layers = enable_layers(&layers);
            let app_info = vulkano::app_info_from_cargo_toml!();
            let ver = Version::major_minor(1, 1);
            Box::new(Instance::new(Some(&app_info), ver, &extens, ok_layers).unwrap())
        };

        let inst = unsafe {
            (instance.as_ref() as *const Arc<Instance>)
                .as_ref()
                .unwrap()
        };
        let pds: Vec<PhysicalDevice> = PhysicalDevice::enumerate(inst).collect();

        Vulkan {
            layers,
            extens,
            instance,
            phydevs: pds,
        }
    }

    pub fn to_instance(&self) -> Arc<Instance> {
        Arc::clone(&self.instance)
    }

    pub fn as_instance(&self) -> &Arc<Instance> {
        &self.instance
    }

    pub fn as_physical_devices(&self) -> &[PhysicalDevice<'a>] {
        &self.phydevs
    }

    pub fn as_layers(&self) -> &[LayerProperties] {
        &self.layers
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

pub struct ExtensionProperties {
    props: vk::ExtensionProperties,
    core: bool,
    layers: Vec<String>,
    physical_devices: Vec<usize>,
}

impl From<vk::ExtensionProperties> for ExtensionProperties {
    fn from(props: vk::ExtensionProperties) -> Self {
        ExtensionProperties {
            props,
            core: false,
            layers: Vec::default(),
            physical_devices: Vec::default(),
        }
    }
}

impl ExtensionProperties {
    #[inline]
    pub fn name(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.props.extensionName.as_ptr())
                .to_str()
                .unwrap()
        }
    }

    #[inline]
    pub fn version(&self) -> u32 {
        self.props.specVersion
    }

    #[inline]
    pub fn core(&self) -> String {
        if self.core {
            "✓".to_string()
        } else {
            "".to_string()
        }
    }

    #[inline]
    pub fn layers(&self) -> String {
        match self.layers.len() {
            0 => "-".to_string(),
            _ => self.layers.join("\n"),
        }
    }

    pub fn add_layer(&mut self, layer: &str) {
        self.layers.push(layer.to_string());
        self.layers.sort();
        self.layers.dedup();
    }

    pub fn add_physical_device(&mut self, index: usize) {
        self.physical_devices.push(index);
        self.physical_devices.sort();
        self.physical_devices.dedup();
    }
}

impl PrettyRow for ExtensionProperties {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        use prettytable::{cell, row};

        row![Fy => "Extension Name", "Version", "Core", "Devices", "Layers"]
    }

    fn to_row(&self) -> prettytable::Row {
        use prettytable::{cell, row};

        let devices = self
            .physical_devices
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(" ");
        row![
            self.name(),
            self.version(),
            self.core(),
            devices,
            self.layers()
        ]
    }
}

pub fn layers() -> Result<Vec<LayerProperties>> {
    use vulkano::instance::layers_list;

    Ok(err_at!(Vk, layers_list())?.collect())
}

// TODO: why are we even doing this ? How can a device extension is enabled when a device
// feature is not available.
pub fn extensions_for_features(
    features: &Features,
    mut extensions: DeviceExtensions,
) -> DeviceExtensions {
    if !features.descriptor_indexing {
        extensions.ext_descriptor_indexing = false
    }
    if !features.draw_indirect_count {
        extensions.khr_draw_indirect_count = false
    }
    if !features.sampler_filter_minmax {
        extensions.ext_sampler_filter_minmax = false
    }
    if !features.sampler_mirror_clamp_to_edge {
        extensions.khr_sampler_mirror_clamp_to_edge = false
    }
    if !features.shader_output_layer {
        extensions.ext_shader_viewport_index_layer = false
    }
    extensions
}

pub fn check_errors(result: vk::Result) -> Result<()> {
    if result & 0x80000000 > 0 {
        err_at!(Vk, msg: "fail vk-sys code {}", result)?
    }
    Ok(())
}
