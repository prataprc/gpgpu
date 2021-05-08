//! Vulkan toolkit.

use prettytable::{cell, row};
use vk_sys as vk;
use vulkano::{
    device::Device,
    instance::{
        Instance, LayerProperties, MemoryHeap, MemoryType, PhysicalDevice,
        PhysicalDeviceType,
    },
    VulkanObject,
};

use std::{
    ffi::{CStr, CString},
    ptr,
};

use crate::{Error, Result};

pub trait PrettyRow {
    fn to_format() -> prettytable::format::TableFormat;

    fn to_head() -> prettytable::Row;

    fn to_row(&self) -> prettytable::Row;
}

impl PrettyRow for LayerProperties {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Layer Name", "Description", "Vulkan Version", "Layer Version"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.name(),
            self.description(),
            self.vulkan_version(),
            self.implementation_version()
        ]
    }
}

impl<'a> PrettyRow for MemoryHeap<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Id", "Size", "DEVICE_LOCAL", "MULTI_INSTANCE"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.id(),
            self.size(),
            self.is_device_local(),
            self.is_multi_instance()
        ]
    }
}

impl<'a> PrettyRow for MemoryType<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Id", "Heap", "LOCAL", "VISIBLE", "CACHED", "COHERENT", "LAZY"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.id(),
            self.heap().id(),
            self.is_device_local(),
            self.is_host_visible(),
            self.is_host_cached(),
            self.is_host_coherent(),
            self.is_lazily_allocated(),
        ]
    }
}

impl<'a> PrettyRow for PhysicalDevice<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Index", "Name", "Type", "DeviceID", "VendorID", "API", "Driver"]
    }

    fn to_row(&self) -> prettytable::Row {
        let uuid = uuid::Uuid::from_slice(&self.uuid()[..]).unwrap();
        row![
            self.index(),
            format!("{}\nUUID:{}", self.name(), uuid.to_hyphenated()),
            physical_device_type_to_str(self.ty()),
            &format!("{:x}", self.pci_device_id()),
            &format!("{:x}", self.pci_vendor_id()),
            self.api_version(),
            self.driver_version(),
        ]
    }
}

fn physical_device_type_to_str(ty: PhysicalDeviceType) -> &'static str {
    match ty {
        PhysicalDeviceType::IntegratedGpu => "IntegratedGpu",
        PhysicalDeviceType::DiscreteGpu => "DiscreteGpu",
        PhysicalDeviceType::VirtualGpu => "VirtualGpu",
        PhysicalDeviceType::Cpu => "Cpu",
        PhysicalDeviceType::Other => "Other",
    }
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
        row![Fy => "Extension Name", "Version", "Core", "Devices", "Layers"]
    }

    fn to_row(&self) -> prettytable::Row {
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

pub fn extensions(layer: Option<&str>) -> Result<Vec<ExtensionProperties>> {
    use vulkano::instance::loader::auto_loader;

    let ptrs = err_at!(Vk, auto_loader())?;
    let entry_points = ptrs.entry_points();

    let (layer_cstr, _s) = match layer.clone() {
        Some(layer) => {
            let _s = err_at!(Invalid, CString::new(layer))?;
            (_s.as_c_str().as_ptr(), _s)
        }
        None => (ptr::null(), CString::new("").unwrap()),
    };

    let mut properties: Vec<ExtensionProperties> = unsafe {
        let mut num = 0;
        check_errors(entry_points.EnumerateInstanceExtensionProperties(
            layer_cstr,
            &mut num,
            ptr::null_mut(),
        ))?;

        let mut properties = Vec::with_capacity(num as usize);
        check_errors(entry_points.EnumerateInstanceExtensionProperties(
            layer_cstr,
            &mut num,
            properties.as_mut_ptr(),
        ))?;
        properties.set_len(num as usize);
        properties.into_iter().map(From::from).collect()
    };
    for prop in properties.iter_mut() {
        match layer {
            Some(layer) => prop.layers = vec![layer.to_string()],
            None => prop.core = true,
        }
    }

    Ok(properties)
}

pub fn device_extensions(pd: PhysicalDevice) -> Result<Vec<ExtensionProperties>> {
    let entry_points = pd.instance().pointers();
    let index = pd.index();

    let mut properties: Vec<ExtensionProperties> = unsafe {
        let mut num = 0;
        check_errors(entry_points.EnumerateDeviceExtensionProperties(
            pd.internal_object(),
            ptr::null(),
            &mut num,
            ptr::null_mut(),
        ))?;

        let mut properties = Vec::with_capacity(num as usize);
        check_errors(entry_points.EnumerateDeviceExtensionProperties(
            pd.internal_object(),
            ptr::null(),
            &mut num,
            properties.as_mut_ptr(),
        ))?;
        properties.set_len(num as usize);
        properties.into_iter().map(From::from).collect()
    };
    for props in properties.iter_mut() {
        props.physical_devices = vec![index];
    }

    Ok(properties)
}

pub struct PhysicalDeviceLimit {
    name: String,
    value: String,
}

impl PrettyRow for PhysicalDeviceLimit {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Limit-name", "Device"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.value]
    }
}

macro_rules! make_limits {
    ($(($limit:ident, $field:ident),)*) => (
        vec![
            $(
                PhysicalDeviceLimit {
                    name: stringify!($field).to_string(),
                    value: format!("{:?}", $limit.$field()),
                },
            )*
        ]
    );
}

pub fn physical_device_limits<'a>(pd: &PhysicalDevice<'a>) -> Vec<PhysicalDeviceLimit> {
    let l = pd.limits();
    make_limits![
        (l, max_image_dimension_1d),
        (l, max_image_dimension_2d),
        (l, max_image_dimension_3d),
        (l, max_image_dimension_cube),
        (l, max_image_array_layers),
        (l, max_texel_buffer_elements),
        (l, max_uniform_buffer_range),
        (l, max_storage_buffer_range),
        (l, max_push_constants_size),
        (l, max_memory_allocation_count),
        (l, max_sampler_allocation_count),
        (l, buffer_image_granularity),
        (l, sparse_address_space_size),
        (l, max_bound_descriptor_sets),
        (l, max_per_stage_descriptor_samplers),
        (l, max_per_stage_descriptor_uniform_buffers),
        (l, max_per_stage_descriptor_storage_buffers),
        (l, max_per_stage_descriptor_sampled_images),
        (l, max_per_stage_descriptor_storage_images),
        (l, max_per_stage_descriptor_input_attachments),
        (l, max_per_stage_resources),
        (l, max_descriptor_set_samplers),
        (l, max_descriptor_set_uniform_buffers),
        (l, max_descriptor_set_uniform_buffers_dynamic),
        (l, max_descriptor_set_storage_buffers),
        (l, max_descriptor_set_storage_buffers_dynamic),
        (l, max_descriptor_set_sampled_images),
        (l, max_descriptor_set_storage_images),
        (l, max_descriptor_set_input_attachments),
        (l, max_vertex_input_attributes),
        (l, max_vertex_input_bindings),
        (l, max_vertex_input_attribute_offset),
        (l, max_vertex_input_binding_stride),
        (l, max_vertex_output_components),
        (l, max_tessellation_generation_level),
        (l, max_tessellation_patch_size),
        (l, max_tessellation_control_per_vertex_input_components),
        (l, max_tessellation_control_per_vertex_output_components),
        (l, max_tessellation_control_per_patch_output_components),
        (l, max_tessellation_control_total_output_components),
        (l, max_tessellation_evaluation_input_components),
        (l, max_tessellation_evaluation_output_components),
        (l, max_geometry_shader_invocations),
        (l, max_geometry_input_components),
        (l, max_geometry_output_components),
        (l, max_geometry_output_vertices),
        (l, max_geometry_total_output_components),
        (l, max_fragment_input_components),
        (l, max_fragment_output_attachments),
        (l, max_fragment_dual_src_attachments),
        (l, max_fragment_combined_output_resources),
        (l, max_compute_shared_memory_size),
        (l, max_compute_work_group_count),
        (l, max_compute_work_group_invocations),
        (l, max_compute_work_group_size),
        (l, sub_pixel_precision_bits),
        (l, sub_texel_precision_bits),
        (l, mipmap_precision_bits),
        (l, max_draw_indexed_index_value),
        (l, max_draw_indirect_count),
        (l, max_sampler_lod_bias),
        (l, max_sampler_anisotropy),
        (l, max_viewports),
        (l, max_viewport_dimensions),
        (l, viewport_bounds_range),
        (l, viewport_sub_pixel_bits),
        (l, min_memory_map_alignment),
        (l, min_texel_buffer_offset_alignment),
        (l, min_uniform_buffer_offset_alignment),
        (l, min_storage_buffer_offset_alignment),
        (l, min_texel_offset),
        (l, max_texel_offset),
        (l, min_texel_gather_offset),
        (l, max_texel_gather_offset),
        (l, min_interpolation_offset),
        (l, max_interpolation_offset),
        (l, sub_pixel_interpolation_offset_bits),
        (l, max_framebuffer_width),
        (l, max_framebuffer_height),
        (l, max_framebuffer_layers),
        (l, framebuffer_color_sample_counts),
        (l, framebuffer_depth_sample_counts),
        (l, framebuffer_stencil_sample_counts),
        (l, framebuffer_no_attachments_sample_counts),
        (l, max_color_attachments),
        (l, sampled_image_color_sample_counts),
        (l, sampled_image_integer_sample_counts),
        (l, sampled_image_depth_sample_counts),
        (l, sampled_image_stencil_sample_counts),
        (l, storage_image_sample_counts),
        (l, max_sample_mask_words),
        (l, timestamp_compute_and_graphics),
        (l, timestamp_period),
        (l, max_clip_distances),
        (l, max_cull_distances),
        (l, max_combined_clip_and_cull_distances),
        (l, discrete_queue_priorities),
        (l, point_size_range),
        (l, line_width_range),
        (l, point_size_granularity),
        (l, line_width_granularity),
        (l, strict_lines),
        (l, standard_sample_locations),
        (l, optimal_buffer_copy_offset_alignment),
        (l, optimal_buffer_copy_row_pitch_alignment),
        (l, non_coherent_atom_size),
    ]
}

pub struct PhysicalDeviceFeature {
    name: String,
    supported: bool,
}

impl PrettyRow for PhysicalDeviceFeature {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Feature-name", "Device"]
    }

    fn to_row(&self) -> prettytable::Row {
        let supported = (if self.supported { "✓" } else { "✗" }).to_string();
        match self.supported {
            true => row![self.name, Fg -> supported],
            false => row![self.name, Fr -> supported],
        }
    }
}

macro_rules! make_features {
    ($(($features:ident, $field:ident),)*) => (
        vec![
            $(
                PhysicalDeviceFeature {
                    name: stringify!($field).to_string(),
                    supported: $features.$field,
                },
            )*
        ]
    );
}

pub fn physical_device_features<'a>(
    pd: &PhysicalDevice<'a>,
) -> Vec<PhysicalDeviceFeature> {
    let f = pd.supported_features();

    make_features![
        (f, robust_buffer_access),
        (f, full_draw_index_uint32),
        (f, image_cube_array),
        (f, independent_blend),
        (f, geometry_shader),
        (f, tessellation_shader),
        (f, sample_rate_shading),
        (f, dual_src_blend),
        (f, logic_op),
        (f, multi_draw_indirect),
        (f, draw_indirect_first_instance),
        (f, depth_clamp),
        (f, depth_bias_clamp),
        (f, fill_mode_non_solid),
        (f, depth_bounds),
        (f, wide_lines),
        (f, large_points),
        (f, alpha_to_one),
        (f, multi_viewport),
        (f, sampler_anisotropy),
        (f, texture_compression_etc2),
        (f, texture_compression_astc_ldr),
        (f, texture_compression_bc),
        (f, occlusion_query_precise),
        (f, pipeline_statistics_query),
        (f, vertex_pipeline_stores_and_atomics),
        (f, fragment_stores_and_atomics),
        (f, shader_tessellation_and_geometry_point_size),
        (f, shader_image_gather_extended),
        (f, shader_storage_image_extended_formats),
        (f, shader_storage_image_multisample),
        (f, shader_storage_image_read_without_format),
        (f, shader_storage_image_write_without_format),
        (f, shader_uniform_buffer_array_dynamic_indexing),
        (f, shader_sampled_image_array_dynamic_indexing),
        (f, shader_storage_buffer_array_dynamic_indexing),
        (f, shader_storage_image_array_dynamic_indexing),
        (f, shader_clip_distance),
        (f, shader_cull_distance),
        (f, shader_float64),
        (f, shader_int64),
        (f, shader_int16),
        (f, shader_resource_residency),
        (f, shader_resource_min_lod),
        (f, sparse_binding),
        (f, sparse_residency_buffer),
        (f, sparse_residency_image2d),
        (f, sparse_residency_image3d),
        (f, sparse_residency2_samples),
        (f, sparse_residency4_samples),
        (f, sparse_residency8_samples),
        (f, sparse_residency16_samples),
        (f, sparse_residency_aliased),
        (f, variable_multisample_rate),
        (f, inherited_queries),
        (f, buffer_device_address),
        (f, buffer_device_address_capture_replay),
        (f, buffer_device_address_multi_device),
        (f, variable_pointers_storage_buffer),
        (f, variable_pointers),
        (f, shader_buffer_int64_atomics),
        (f, shader_shared_int64_atomics),
        (f, storage_buffer_8bit),
        (f, storage_uniform_8bit),
        (f, storage_push_constant_8bit),
        (f, storage_buffer_16bit),
        (f, storage_uniform_16bit),
        (f, storage_push_constant_16bit),
        (f, storage_input_output_16bit),
        (f, shader_float16),
        (f, shader_int8),
    ]
}

fn check_errors(result: vk::Result) -> Result<()> {
    if result & 0x80000000 > 0 {
        err_at!(Vk, msg: "fail vk-sys code {}", result)?
    }
    Ok(())
}

pub struct Vulkan {
    instance: Instance,
    device: Device,
}

impl Vulkan {
    fn new(instance: Instance, device: Device) -> Self {
        Vulkan { instance, device }
    }
}
