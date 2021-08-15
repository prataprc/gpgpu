use vulkano::instance::{
    Instance, InstanceExtensions, LayerProperties, MemoryHeap, MemoryType,
    PhysicalDevice, QueueFamily, Version,
};

use std::sync::Arc;

use crate::{Error, Result};

pub fn layers() -> Result<Vec<LayerProperties>> {
    Ok(err_at!(Vk, vulkano::instance::layers_list())?.collect())
}

/// Return the vulkan implementation available through this package.
pub fn api_version() -> Result<Version> {
    use vulkano::instance::loader::auto_loader;

    let funcptrs = err_at!(Vk, auto_loader())?;
    err_at!(Vk, funcptrs.api_version())
}

/// Variantes map to VkQueueFlagBits.
#[derive(Clone)]
pub enum QueueCapability {
    Graphics,
    Compute,
    Transfer,
    SparseBinding,
}

/// Similar to VkDeviceQueueCreateInfo. A single instance of QueueCreateInfo shall create
/// as many VkQueue objects as the number of priorities, in other-words each item in
/// priorities vector specify the priority for queue-count-index.
///
/// By default [Builder] creates a single queue with Graphics capabilities with priority
/// `1.0`. Refer to [Builder::with_queues] to learn how to configure/create queues
/// for [Vulkan] instances.
#[derive(Clone)]
pub struct QueueCreateInfo {
    pub cap: QueueCapability,
    pub stages: Vec<vulkano::sync::PipelineStage>,
    pub priorities: Vec<f32>,
}

impl Default for QueueCreateInfo {
    fn default() -> Self {
        QueueCreateInfo {
            cap: QueueCapability::Graphics,
            stages: Vec::default(),
            priorities: vec![1.0],
        }
    }
}

/// Builder blueprint for [Vulkan] type.
pub struct Builder<'a> {
    // instance attributes
    app_info: vulkano::instance::ApplicationInfo<'a>,
    version: Version,
    layers: Vec<String>,
    iextns: InstanceExtensions,
    // device attributes
    device_id: usize,
    queue_infos: Vec<QueueCreateInfo>,
    dextns: Option<vulkano::device::DeviceExtensions>,
    properties: vulkano::device::Properties,
    features: vulkano::device::Features,
}

impl<'a> Builder<'a> {
    /// Create new builder using cargo manifest for `application_info`, without enabling
    /// any of the instance-extensions and without enabling any of the layers. This
    /// method shall automatically detect the latest version from the driver's
    /// [FunctionPointers]. Subsequently use one of the `with_*` methods to add more
    /// builder options.
    pub fn new() -> Result<Builder<'a>> {
        use vulkano::device::{Features, Properties};

        let builder = Builder {
            // instance attributes
            app_info: vulkano::app_info_from_cargo_toml!(),
            version: crate::api_version()?,
            iextns: InstanceExtensions::none(),
            layers: Vec::default(),
            // device attributes
            device_id: 0,
            queue_infos: vec![QueueCreateInfo::default()],
            dextns: None,
            properties: Properties::default(),
            features: Features::none(),
        };

        Ok(builder)
    }

    /// Similar to [new] method, but supply the [ApplicationInfo] and [Version]. If
    /// requested [Version] is greater than the local vulkan version (driver), this
    /// call shall fail. Subsequently use one of the `with_*` methods to add more
    /// builder options.
    pub fn with(
        app_info: vulkano::instance::ApplicationInfo<'a>,
        version: Option<Version>,
    ) -> Result<Builder<'a>> {
        use vulkano::device::{Features, Properties};

        let local_ver = crate::api_version()?;
        let version = match version {
            Some(ver) if ver <= local_ver => ver,
            Some(ver) => err_at!(Vk, msg: "local_version {} < {}", local_ver, ver)?,
            None => local_ver,
        };

        Ok(Builder {
            // instance attributes
            app_info,
            version,
            iextns: InstanceExtensions::none(),
            layers: Vec::default(),
            // device attributes
            device_id: 0,
            queue_infos: vec![QueueCreateInfo::default()],
            dextns: None,
            properties: Properties::default(),
            features: Features::none(),
        })
    }

    /// Configure the [ApplicationInfo]
    pub fn with_app_info(
        mut self,
        app_info: vulkano::instance::ApplicationInfo<'a>,
    ) -> Self {
        self.app_info = app_info;
        self
    }

    /// List of layers to be enabled while creating vulkan-instance.
    pub fn with_layers<L>(mut self, layers: L) -> Self
    where
        L: IntoIterator<Item = String>,
    {
        self.layers = layers.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// List of instance-extensions to enable while creating vulkan-instance. If
    /// `extensions` is None, then all supported core extensions shall be enabled.
    ///
    /// For screen rendering enable `khr_surface` extension and platform specific
    /// extensions like `khr_android_surface`, `khr_wayland_surface`,
    /// `khr_win32_surface`, `khr_xcb_surface`, `khr_xlib_surface`, `mvk_ios_surface`,
    /// `mvk_macos_surface`, `nn_vi_surface` in `InstanceExtensions`.
    pub fn with_extensions(mut self, extensions: Option<InstanceExtensions>) -> Self {
        self.iextns =
            extensions.unwrap_or(InstanceExtensions::supported_by_core().unwrap());
        self
    }

    /// Create VkDevice object using supplied parameters. At present we don't have
    /// multi-device support. For requested [Features], device-extensions shall
    /// automatically be enabled event if they are not supplied in the `extensions` arg.
    ///
    /// By default, if this method is not used, the the first available physical device
    /// shall be used with default properties, with required-extensions supported by
    /// the physical-device, and with no-specific-feature.
    ///
    /// If `extensions` is None, then all extensions required and supported by the
    /// physical device shall be enabled.
    ///
    /// For screen rendering enable `khr_swapchain` extension, also enable the
    /// `khr_surface` extension in `InstanceExtensions` refer to [with_extensions]
    /// method for details.
    pub fn with_device(
        mut self,
        id: usize,
        extensions: Option<vulkano::device::DeviceExtensions>,
        properties: vulkano::device::Properties,
        features: vulkano::device::Features,
    ) -> Self {
        self.device_id = id;
        self.dextns = extensions;
        self.properties = properties;
        self.features = features;
        self
    }

    /// Create with queues. If not used, a single graphics queue with priority 1.0
    /// shall be created and used.
    pub fn with_queues(mut self, infos: Vec<QueueCreateInfo>) -> Self {
        self.queue_infos = infos;
        self
    }

    /// Finally call build, to obtain the [Vulkan] object. There are two variant
    /// of build, one to build for a platform dependant surface for which use
    /// `build_for_surface` method and second to rendering into image buffer.
    ///
    /// If not sure, use vulkano_win::required_extensions() for `surface` parameter.
    pub fn build_for_surface(self, surface: InstanceExtensions) -> Result<Vulkan<'a>> {
        use vulkano_win::VkSurfaceBuild;
        use winit::event_loop::EventLoop;
        use winit::window::WindowBuilder;

        use vulkano::device::{Device, DeviceExtensions, Queue};

        let instance = {
            let iextns = union_iextns(self.iextns.clone(), surface);
            let layers = self.layers.iter().map(|s| s.as_str());
            let res = Instance::new(Some(&self.app_info), self.version, &iextns, layers);
            Box::new(err_at!(Vk, res)?)
        };

        let pds: Vec<PhysicalDevice> = unsafe {
            let inst = (instance.as_ref() as *const Arc<Instance>)
                .as_ref()
                .unwrap();
            PhysicalDevice::enumerate(inst).collect()
        };
        let pd = pds[self.device_id];
        crate::validity::confirm_properties(&self, pd.properties().clone())?;
        let qfamilies: Vec<QueueFamily> = pd.queue_families().collect();

        let dextns = match self.dextns {
            Some(extensions) => extensions,
            None => DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::required_extensions(pd)
            },
        };
        let (dextns, device, queues) = {
            let qrs: Vec<(QueueFamily<'a>, f32)> = self
                .queue_infos
                .clone()
                .into_iter()
                .map(|info| make_queue_request(info, &qfamilies))
                .flatten()
                .map(|(id, p)| (pd.queue_family_by_id(id).unwrap(), p))
                .collect();
            let dextns = crate::validity::extensions_for_features(&self.features, dextns);
            let (device, queues) = err_at!(
                Vk,
                Device::new(pd, &self.features, &dextns, qrs.into_iter())
            )?;
            (dextns, device, queues.collect::<Vec<Arc<Queue>>>())
        };

        let event_loop = EventLoop::new();
        let target = crate::vulkan::Target::Surface {
            surface: err_at!(
                Vk,
                WindowBuilder::new().build_vk_surface(&event_loop, Arc::clone(&instance))
            )?,
            images: Vec::default(),
            event_loop,
            swapchain: None,
        };

        let layers = crate::layers()?
            .into_iter()
            .filter(|l| self.layers.contains(&l.name().to_string()))
            .collect();

        let val = Vulkan {
            // instance attribute
            layers,
            iextns: self.iextns,
            instance,
            phydevs: pds,
            // device attribute
            dextns,
            device,
            queues,
            target,
        };

        Ok(val)
    }

    /// Finally call build, to obtain the [Vulkan] object. There are two variant
    /// of build, one to render into image buffer, for which use `build_for_buffer`
    /// and second to build for a platform dependant surface.
    pub fn build_for_buffer(
        self,
        dimensions: [u32; 2],
        format: vulkano::format::Format,
    ) -> Result<Vulkan<'a>> {
        use vulkano::{
            device::{Device, DeviceExtensions, Queue},
            image::{AttachmentImage, ImageUsage},
        };

        let instance = {
            let iextns = self.iextns.clone();
            let layers = self.layers.iter().map(|s| s.as_str());
            let res = Instance::new(Some(&self.app_info), self.version, &iextns, layers);
            Box::new(err_at!(Vk, res)?)
        };

        let pds: Vec<PhysicalDevice> = unsafe {
            let inst = (instance.as_ref() as *const Arc<Instance>)
                .as_ref()
                .unwrap();
            PhysicalDevice::enumerate(inst).collect()
        };
        let pd = pds[self.device_id];
        crate::validity::confirm_properties(&self, pd.properties().clone())?;
        let qfamilies: Vec<QueueFamily> = pd.queue_families().collect();

        let dextns = match self.dextns {
            Some(extensions) => extensions,
            None => DeviceExtensions {
                khr_swapchain: true,
                ..DeviceExtensions::required_extensions(pd)
            },
        };
        let (dextns, device, queues) = {
            let qrs: Vec<(QueueFamily<'a>, f32)> = self
                .queue_infos
                .clone()
                .into_iter()
                .map(|info| make_queue_request(info, &qfamilies))
                .flatten()
                .map(|(id, p)| (pd.queue_family_by_id(id).unwrap(), p))
                .collect();
            let dextns = crate::validity::extensions_for_features(&self.features, dextns);
            let (device, queues) = err_at!(
                Vk,
                Device::new(pd, &self.features, &dextns, qrs.into_iter())
            )?;
            (dextns, device, queues.collect::<Vec<Arc<Queue>>>())
        };

        let target = {
            let usage = ImageUsage {
                color_attachment: true,
                transfer_source: true,
                ..ImageUsage::none()
            };
            crate::vulkan::Target::Bitmap {
                image: err_at!(
                    Vk,
                    AttachmentImage::with_usage(
                        device.clone(),
                        dimensions,
                        format,
                        usage
                    )
                )?,
            }
        };

        let layers = crate::layers()?
            .into_iter()
            .filter(|l| self.layers.contains(&l.name().to_string()))
            .collect();

        let val = Vulkan {
            // instance attribute
            layers,
            iextns: self.iextns,
            instance,
            phydevs: pds,
            // device attribute
            dextns,
            device,
            queues,
            target,
        };

        Ok(val)
    }

    pub(crate) fn to_properties(&self) -> &vulkano::device::Properties {
        &self.properties
    }
}

fn make_queue_request<'a>(
    info: QueueCreateInfo,
    qfamilies: &[QueueFamily<'a>],
) -> Vec<(u32, f32)> {
    use std::cmp::min;

    for qf in qfamilies.iter() {
        let qf = match info.cap {
            QueueCapability::Graphics => {
                let ok1 = qf.supports_graphics();
                let ok2 = info
                    .stages
                    .clone()
                    .into_iter()
                    .all(|stage| qf.supports_stage(stage));

                if ok1 && ok2 {
                    qf
                } else {
                    continue;
                }
            }
            QueueCapability::Compute if qf.supports_compute() => qf,
            QueueCapability::Transfer if qf.explicitly_supports_transfers() => qf,
            QueueCapability::SparseBinding if qf.supports_sparse_binding() => qf,
            _ => continue,
        };
        return info.priorities
            [0..min(info.priorities.len(), qf.queues_count() as usize)]
            .to_vec()
            .into_iter()
            .map(|p| (qf.id(), p))
            .collect();
    }

    return vec![];
}

fn union_iextns(a: InstanceExtensions, b: InstanceExtensions) -> InstanceExtensions {
    InstanceExtensions {
        khr_android_surface: a.khr_android_surface || b.khr_android_surface,
        khr_device_group_creation: a.khr_device_group_creation
            || b.khr_device_group_creation,
        khr_display: a.khr_display || b.khr_display,
        khr_external_fence_capabilities: a.khr_external_fence_capabilities
            || b.khr_external_fence_capabilities,
        khr_external_memory_capabilities: a.khr_external_memory_capabilities
            || b.khr_external_memory_capabilities,
        khr_external_semaphore_capabilities: a.khr_external_semaphore_capabilities
            || b.khr_external_semaphore_capabilities,
        khr_get_display_properties2: a.khr_get_display_properties2
            || b.khr_get_display_properties2,
        khr_get_physical_device_properties2: a.khr_get_physical_device_properties2
            || b.khr_get_physical_device_properties2,
        khr_get_surface_capabilities2: a.khr_get_surface_capabilities2
            || b.khr_get_surface_capabilities2,
        khr_surface: a.khr_surface || b.khr_surface,
        khr_surface_protected_capabilities: a.khr_surface_protected_capabilities
            || b.khr_surface_protected_capabilities,
        khr_wayland_surface: a.khr_wayland_surface || b.khr_wayland_surface,
        khr_win32_surface: a.khr_win32_surface || b.khr_win32_surface,
        khr_xcb_surface: a.khr_xcb_surface || b.khr_xcb_surface,
        khr_xlib_surface: a.khr_xlib_surface || b.khr_xlib_surface,
        ext_acquire_xlib_display: a.ext_acquire_xlib_display
            || b.ext_acquire_xlib_display,
        ext_debug_report: a.ext_debug_report || b.ext_debug_report,
        ext_debug_utils: a.ext_debug_utils || b.ext_debug_utils,
        ext_direct_mode_display: a.ext_direct_mode_display || b.ext_direct_mode_display,
        ext_directfb_surface: a.ext_directfb_surface || b.ext_directfb_surface,
        ext_display_surface_counter: a.ext_display_surface_counter
            || b.ext_display_surface_counter,
        ext_headless_surface: a.ext_headless_surface || b.ext_headless_surface,
        ext_metal_surface: a.ext_metal_surface || b.ext_metal_surface,
        ext_swapchain_colorspace: a.ext_swapchain_colorspace
            || b.ext_swapchain_colorspace,
        ext_validation_features: a.ext_validation_features || b.ext_validation_features,
        ext_validation_flags: a.ext_validation_flags || b.ext_validation_flags,
        fuchsia_imagepipe_surface: a.fuchsia_imagepipe_surface
            || b.fuchsia_imagepipe_surface,
        ggp_stream_descriptor_surface: a.ggp_stream_descriptor_surface
            || b.ggp_stream_descriptor_surface,
        mvk_ios_surface: a.mvk_ios_surface || b.mvk_ios_surface,
        mvk_macos_surface: a.mvk_macos_surface || b.mvk_macos_surface,
        nn_vi_surface: a.nn_vi_surface || b.nn_vi_surface,
        nv_external_memory_capabilities: a.nv_external_memory_capabilities
            || b.nv_external_memory_capabilities,
        _unbuildable: a._unbuildable,
    }
}
/// Vulkan type roughly maps to instance/device object defined by the vulkan spec.
/// This type try to abstract the boiler plate code as much as possible by
/// providing convinient methods and related macros.
///
/// Note that this object shall be created in the main thread.
pub struct Vulkan<'a, W = winit::window::Window, T = ()>
where
    T: 'static,
{
    // instance objects
    layers: Vec<LayerProperties>,
    iextns: InstanceExtensions,
    instance: Box<Arc<Instance>>,
    phydevs: Vec<PhysicalDevice<'a>>,
    // device objects
    dextns: vulkano::device::DeviceExtensions,
    device: Arc<vulkano::device::Device>,
    queues: Vec<Arc<vulkano::device::Queue>>,
    // surface and swapchain objects, or bmp.
    target: Target<W, T>,
}

pub(crate) enum Target<W, T>
where
    T: 'static,
{
    Surface {
        // surface, swapchain and event-loop
        surface: Arc<vulkano::swapchain::Surface<W>>,
        swapchain: Option<Arc<vulkano::swapchain::Swapchain<W>>>,
        images: Vec<Arc<vulkano::image::swapchain::SwapchainImage<W>>>,
        // window, events
        event_loop: winit::event_loop::EventLoop<T>,
    },
    Bitmap {
        image: Arc<vulkano::image::AttachmentImage>,
    },
}

impl<W, T> Target<W, T>
where
    T: 'static,
{
    fn to_surface(&self) -> Arc<vulkano::swapchain::Surface<W>> {
        match self {
            Target::Surface { surface, .. } => Arc::clone(surface),
            Target::Bitmap { .. } => panic!("vulkan target not a surface"),
        }
    }

    fn to_swapchain(&self) -> Arc<vulkano::swapchain::Swapchain<W>> {
        match self {
            Target::Surface {
                swapchain: Some(swpc),
                ..
            } => Arc::clone(swpc),
            Target::Surface { .. } => panic!("swapchain yet to be built"),
            Target::Bitmap { .. } => panic!("vulkan target not a surface"),
        }
    }

    #[allow(dead_code)]
    fn to_swapimages(&self) -> Vec<Arc<vulkano::image::swapchain::SwapchainImage<W>>> {
        match self {
            Target::Surface { images, .. } => images.iter().map(Arc::clone).collect(),
            Target::Bitmap { .. } => panic!("vulkan target not a surface"),
        }
    }

    #[allow(dead_code)]
    fn to_image(&self) -> Arc<vulkano::image::AttachmentImage> {
        match self {
            Target::Bitmap { image } => Arc::clone(image),
            Target::Surface { .. } => panic!("vulkan target not a bitmap"),
        }
    }
}

pub struct SwapchainCreateInfo {
    // swapchain parameters
    num_images: u32,
    format: vulkano::format::Format,
    color_space: vulkano::swapchain::ColorSpace,
    dimensions: [u32; 2],
    layers: u32,
    usage: vulkano::image::ImageUsage,
    sharing_mode: vulkano::sync::SharingMode,
    transform: vulkano::swapchain::SurfaceTransform,
    composite_alpha: vulkano::swapchain::CompositeAlpha,
    present_mode: vulkano::swapchain::PresentMode,
    fullscreen_exclusive: vulkano::swapchain::FullscreenExclusive,
    clipped: bool,
}

impl<'a, W, T> Vulkan<'a, W, T>
where
    T: 'static,
{
    /// Return enabled layers for instance.
    pub fn enabled_layers(&self) -> Vec<LayerProperties> {
        self.layers.clone()
    }

    /// Return instance extensions that are enabled/disabled.
    pub fn instance_extensions(&self) -> InstanceExtensions {
        self.iextns.clone()
    }

    /// Return device extensions that are enabled/disabled.
    pub fn device_extensions(&self) -> vulkano::device::DeviceExtensions {
        self.dextns.clone()
    }

    /// Return the instance api-version.
    pub fn api_version(&self) -> vulkano::instance::Version {
        self.instance.api_version()
    }

    /// Return the list of memory-heaps available for this device instance, depends
    /// on the physical-device used to create this device.
    pub fn memory_heaps(&self) -> Vec<MemoryHeap> {
        self.device.physical_device().memory_heaps().collect()
    }

    /// Return the list of memory-types available for this device instance, depends
    /// on the physical-device used to create this device.
    pub fn memory_types(&self) -> Vec<MemoryType> {
        self.device.physical_device().memory_types().collect()
    }

    /// Return the list of queue-families available for this device instance, depends
    /// on the physical-device used to create this device.
    pub fn queue_families(&self) -> Vec<QueueFamily> {
        self.device.physical_device().queue_families().collect()
    }

    /// Return the list of queue-families created for this device instance.
    pub fn active_queue_families(&self) -> Vec<QueueFamily> {
        self.device.active_queue_families().collect()
    }

    /// Return the properties of physical-device used to create this device.
    pub fn properties(&self) -> &vulkano::device::Properties {
        self.device.physical_device().properties()
    }

    /// Return the features supported by physical-device used to create this device.
    pub fn supported_features(&self) -> &vulkano::device::Features {
        self.device.physical_device().supported_features()
    }

    /// Return the format properties supported for this device.
    pub fn format_properties(
        &self,
        format: vulkano::format::Format,
    ) -> Result<vulkano::format::FormatProperties> {
        Ok(format.properties(self.device.physical_device()))
    }

    /// Return the image format properties supported for this device.
    pub fn image_format_properties(
        &self,
        format: vulkano::format::Format,
        ty: vulkano::image::ImageType,
        tiling: vulkano::image::ImageTiling,
        usage: vulkano::image::ImageUsage,
        create_flags: vulkano::image::ImageCreateFlags,
    ) -> Result<vulkano::image::ImageFormatProperties> {
        err_at!(
            Vk,
            self.device
                .image_format_properties(format, ty, tiling, usage, create_flags)
        )
    }

    /// Return the physical device used to create the device instance.
    pub fn to_physical_device(&'a self) -> PhysicalDevice<'a> {
        self.device.physical_device()
    }

    /// Return the instance object used to create this device.
    pub fn to_instance(&self) -> Arc<Instance> {
        Arc::clone(&self.instance)
    }

    /// Return the physical-device used to create this device.
    pub fn to_physical_devices(&self) -> Vec<PhysicalDevice<'a>> {
        self.phydevs.clone()
    }

    /// Return the underlying device reference as Arc<T>
    pub fn to_device(&self) -> Arc<vulkano::device::Device> {
        self.device.clone()
    }

    /// Return the queue objects created for this device
    pub fn to_queues(&self) -> Vec<Arc<vulkano::device::Queue>> {
        self.queues.clone()
    }

    pub fn to_swapchain(&self) -> Arc<vulkano::swapchain::Swapchain<W>> {
        self.target.to_swapchain()
    }

    pub fn to_image(&self) -> Arc<vulkano::image::AttachmentImage> {
        self.target.to_image()
    }
}

impl<'a, T> Vulkan<'a, winit::window::Window, T> {
    /// Returns swapchain create parameters
    pub fn default_swapchain_create_info(&self) -> Result<SwapchainCreateInfo> {
        use vulkano::{
            format::Format,
            swapchain::{FullscreenExclusive, PresentMode, SurfaceTransform},
        };

        let (caps, dimensions, _qf) = match &self.target {
            Target::Surface { surface, .. } => {
                // Query capabilities of the surface. When we create the swapchain
                // we can only pass values that are allowed by the capabilities.
                let caps = err_at!(Vk, surface.capabilities(self.to_physical_device()))?;

                // The dimensions of the window, only used to initially setup the
                // swapchain.
                //
                // NOTE: On some drivers the swapchain dimensions are specified by
                // `caps.current_extent` and the swapchain size must use these dimensions.
                //
                // These dimensions are always the same as the window dimensions.
                // However, other drivers don't specify a value, i.e.
                // `caps.current_extent` is `None`. These drivers will allow anything,
                // but the only sensible value is the window dimensions.
                //
                // Both of these cases need the swapchain to use the window dimensions,
                // so we just use that.
                let dimensions: [u32; 2] = surface.window().inner_size().into();

                let qf = {
                    self.device
                        .physical_device()
                        .queue_families()
                        .find(|&q| {
                            // take the first queue that supports drawing to our window.
                            q.supports_graphics()
                                && surface.is_supported(q).unwrap_or(false)
                        })
                        .unwrap();
                };
                (caps, dimensions, qf)
            }
            Target::Bitmap { .. } => err_at!(Vk, msg: "vulkan target not surface")?,
        };

        // The alpha mode indicates how the alpha value of the final image will behave.
        // For example, you can choose whether the window will be opaque or transparent.
        let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();

        let (format, color_space) = match caps.supported_formats.into_iter().next() {
            Some((Format::R8G8B8A8Unorm, cs)) => (Format::R8G8B8A8Unorm, cs),
            Some((Format::B8G8R8A8Unorm, cs)) => (Format::B8G8R8A8Unorm, cs),
            Some((format, cs)) => (format, cs),
            None => err_at!(Vk, msg: "no image-formats supported by surface")?,
        };

        Ok(SwapchainCreateInfo {
            num_images: caps.min_image_count,
            format,
            color_space,
            dimensions,
            layers: 1,
            usage: vulkano::image::ImageUsage::color_attachment(),
            sharing_mode: self.queues.iter().next().unwrap().into(),
            transform: SurfaceTransform::Identity,
            composite_alpha,
            present_mode: PresentMode::Fifo,
            fullscreen_exclusive: FullscreenExclusive::Default,
            clipped: true,
        })
    }

    pub fn create_swapchain(&mut self, info: Option<SwapchainCreateInfo>) -> Result<()> {
        use std::cmp;
        use vulkano::swapchain::Swapchain;

        let device = Arc::clone(&self.device);
        let info = match info {
            Some(info) => info,
            None => self.default_swapchain_create_info()?,
        };

        let max_image_count = err_at!(
            Vk,
            self.target
                .to_surface()
                .capabilities(self.to_physical_device())
        )?
        .max_image_count
        .unwrap_or(info.num_images);

        match &mut self.target {
            Target::Surface {
                surface,
                swapchain,
                images,
                ..
            } => {
                let res = Swapchain::start(device, Arc::clone(surface))
                    .num_images(cmp::min(info.num_images, max_image_count))
                    .format(info.format)
                    .color_space(info.color_space)
                    .dimensions(info.dimensions)
                    .layers(info.layers)
                    .usage(info.usage)
                    .sharing_mode(info.sharing_mode)
                    .transform(info.transform)
                    .composite_alpha(info.composite_alpha)
                    .present_mode(info.present_mode)
                    .fullscreen_exclusive(info.fullscreen_exclusive)
                    .clipped(info.clipped)
                    .build();
                let (swapchain_n, images_n) = err_at!(Vk, res)?;
                *swapchain = Some(swapchain_n);
                *images = images_n;
            }
            Target::Bitmap { .. } => err_at!(Vk, msg: "vulkan target not a surface")?,
        };

        Ok(())
    }

    pub fn recreate_swapchain(&mut self, _info: SwapchainCreateInfo) {
        todo!()
    }

    pub unsafe fn wait(&self) -> Result<()> {
        err_at!(Vk, self.device.wait())
    }
}
