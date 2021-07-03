use vulkano::{
    device::{DeviceExtensions, Features},
    instance::{
        ApplicationInfo, Instance, InstanceExtensions, LayerProperties, PhysicalDevice,
        Version,
    },
};

use std::sync::Arc;

use crate::{Error, Result};

pub struct Builder<'a> {
    // instance attributes
    app_info: ApplicationInfo<'a>,
    version: Version,
    layers: Vec<String>,
    extens: InstanceExtensions,
}

impl<'a> Builder<'a> {
    /// create new builder using cargo manifest for `application_info`, without enabling
    /// any of the instance-extensions and without enabling any of the layers. This
    /// method shall automatically detect the latest version from the driver's
    /// [FunctionPointers]. Later use one of the `with_*` methods to add more builder
    /// options.
    pub fn new() -> Result<Builder<'a>> {
        use vulkano::instance::loader::auto_loader;

        let funcptrs = err_at!(Vk, auto_loader())?;
        let version = err_at!(Vk, funcptrs.api_version())?;

        let builder = Builder {
            app_info: vulkano::app_info_from_cargo_toml!(),
            version,
            extens: InstanceExtensions::none(),
            layers: Vec::default(),
        };

        Ok(builder)
    }

    /// Similar to [new] method, but supply the [ApplicationInfo] and [Version].
    pub fn with(app_info: ApplicationInfo<'a>, version: Version) -> Builder<'a> {
        Builder {
            app_info,
            version,
            extens: InstanceExtensions::none(),
            layers: Vec::default(),
        }
    }

    /// Configure the [ApplicationInfo]
    pub fn with_app_info(mut self, app_info: ApplicationInfo<'a>) -> Self {
        self.app_info = app_info;
        self
    }

    /// List of layers to be enabled while creating vulkan-instance.
    pub fn with_layers<L>(mut self, layers: L) -> Self
    where
        L: IntoIterator<Item = &'a str>,
    {
        self.layers = layers.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// List of extensions to be enabled while creating vulkan-instance.
    pub fn with_extensions(mut self, extensions: InstanceExtensions) -> Self {
        self.extens = extensions;
        self
    }

    /// Finally call build, to obtain the [Vulkan] object.
    pub fn build(self) -> Result<Vulkan<'a>> {
        let instance = err_at!(
            Vk,
            Instance::new(
                Some(&self.app_info),
                self.version,
                &self.extens,
                self.layers.iter().map(|s| s.as_str()),
            )
        )?;
        let instance = Box::new(instance);

        let pds: Vec<PhysicalDevice> = unsafe {
            let inst = (instance.as_ref() as *const Arc<Instance>)
                .as_ref()
                .unwrap();
            PhysicalDevice::enumerate(inst).collect()
        };

        let val = Vulkan {
            layers: layers()?
                .into_iter()
                .filter(|l| self.layers.contains(&l.name().to_string()))
                .collect(),
            extens: self.extens,
            instance,
            phydevs: pds,
        };

        Ok(val)
    }
}

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
    /// Return the instance api-version.
    pub fn api_version(&self) -> vulkano::instance::Version {
        self.instance.api_version()
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

//TODO
//fn enable_layers(layers: &[LayerProperties]) -> Vec<&'static str> {
//    layers
//        .iter()
//        .filter_map(|layer| match layer.name() {
//            "VK_LAYER_LUNARG_parameter_validation" => {
//                Some("VK_LAYER_LUNARG_parameter_validation")
//            }
//            "VK_LAYER_LUNARG_object_tracker" => Some("VK_LAYER_LUNARG_object_tracker"),
//            "VK_LAYER_LUNARG_standard_validation" => {
//                Some("VK_LAYER_LUNARG_standard_validation")
//            }
//            "VK_LAYER_LUNARG_core_validation" => Some("VK_LAYER_LUNARG_core_validation"),
//            "VK_LAYER_GOOGLE_threading" => Some("VK_LAYER_GOOGLE_threading"),
//            "VK_LAYER_GOOGLE_unique_objects" => Some("VK_LAYER_GOOGLE_unique_objects"),
//            _ => None,
//        })
//        .collect()
//}

pub fn layers() -> Result<Vec<LayerProperties>> {
    Ok(err_at!(Vk, vulkano::instance::layers_list())?.collect())
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
