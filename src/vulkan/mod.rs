//! Vulkan toolkit.

use prettytable::{cell, row};
use vk_sys as vk;
use vulkano::{
    device::Device,
    instance::{Instance, LayerProperties, PhysicalDevice, PhysicalDeviceType},
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
        row![Fy => "Limit-name", "Index"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.value]
    }
}

macro_rules! make_limits {
    ($(($limit:ident, $name:expr, $limit_name:ident),)*) => (
        vec![
            $(
                PhysicalDeviceLimit {
                    name: $name.to_string(),
                    value: format!("{:?}", $limit.$limit_name()),
                },
            )*
        ]
    );
}

pub fn physical_device_limits<'a>(pd: &PhysicalDevice<'a>) -> Vec<PhysicalDeviceLimit> {
    let l = pd.limits();
    make_limits![
        (l, "max_image_dimension_1d", max_image_dimension_1d),
        (l, "max_image_dimension_2d", max_image_dimension_2d),
        (l, "max_image_dimension_3d", max_image_dimension_3d),
        (l, "max_image_dimension_cube", max_image_dimension_cube),
        (l, "max_image_array_layers", max_image_array_layers),
        (l, "max_texel_buffer_elements", max_texel_buffer_elements),
        (l, "max_uniform_buffer_range", max_uniform_buffer_range),
        (l, "max_storage_buffer_range", max_storage_buffer_range),
        (l, "max_push_constants_size", max_push_constants_size),
        (
            l,
            "max_memory_allocation_count",
            max_memory_allocation_count
        ),
        (
            l,
            "max_sampler_allocation_count",
            max_sampler_allocation_count
        ),
        (l, "buffer_image_granularity", buffer_image_granularity),
        (l, "sparse_address_space_size", sparse_address_space_size),
        (l, "max_bound_descriptor_sets", max_bound_descriptor_sets),
        (
            l,
            "max_per_stage_descriptor_samplers",
            max_per_stage_descriptor_samplers
        ),
        (
            l,
            "max_per_stage_descriptor_uniform_buffers",
            max_per_stage_descriptor_uniform_buffers
        ),
        (
            l,
            "max_per_stage_descriptor_storage_buffers",
            max_per_stage_descriptor_storage_buffers
        ),
        (
            l,
            "max_per_stage_descriptor_sampled_images",
            max_per_stage_descriptor_sampled_images
        ),
        (
            l,
            "max_per_stage_descriptor_storage_images",
            max_per_stage_descriptor_storage_images
        ),
        (
            l,
            "max_per_stage_descriptor_input_attachments",
            max_per_stage_descriptor_input_attachments
        ),
        (l, "max_per_stage_resources", max_per_stage_resources),
        (
            l,
            "max_descriptor_set_samplers",
            max_descriptor_set_samplers
        ),
        (
            l,
            "max_descriptor_set_uniform_buffers",
            max_descriptor_set_uniform_buffers
        ),
        (
            l,
            "max_descriptor_set_uniform_buffers_dynamic",
            max_descriptor_set_uniform_buffers_dynamic
        ),
        (
            l,
            "max_descriptor_set_storage_buffers",
            max_descriptor_set_storage_buffers
        ),
        (
            l,
            "max_descriptor_set_storage_buffers_dynamic",
            max_descriptor_set_storage_buffers_dynamic
        ),
        (
            l,
            "max_descriptor_set_sampled_images",
            max_descriptor_set_sampled_images
        ),
        (
            l,
            "max_descriptor_set_storage_images",
            max_descriptor_set_storage_images
        ),
        (
            l,
            "max_descriptor_set_input_attachments",
            max_descriptor_set_input_attachments
        ),
        (
            l,
            "max_vertex_input_attributes",
            max_vertex_input_attributes
        ),
        (l, "max_vertex_input_bindings", max_vertex_input_bindings),
        (
            l,
            "max_vertex_input_attribute_offset",
            max_vertex_input_attribute_offset
        ),
        (
            l,
            "max_vertex_input_binding_stride",
            max_vertex_input_binding_stride
        ),
        (
            l,
            "max_vertex_output_components",
            max_vertex_output_components
        ),
        (
            l,
            "max_tessellation_generation_level",
            max_tessellation_generation_level
        ),
        (
            l,
            "max_tessellation_patch_size",
            max_tessellation_patch_size
        ),
        (
            l,
            "max_tessellation_control_per_vertex_input_components",
            max_tessellation_control_per_vertex_input_components
        ),
        (
            l,
            "max_tessellation_control_per_vertex_output_components",
            max_tessellation_control_per_vertex_output_components
        ),
        (
            l,
            "max_tessellation_control_per_patch_output_components",
            max_tessellation_control_per_patch_output_components
        ),
        (
            l,
            "max_tessellation_control_total_output_components",
            max_tessellation_control_total_output_components
        ),
        (
            l,
            "max_tessellation_evaluation_input_components",
            max_tessellation_evaluation_input_components
        ),
        (
            l,
            "max_tessellation_evaluation_output_components",
            max_tessellation_evaluation_output_components
        ),
        (
            l,
            "max_geometry_shader_invocations",
            max_geometry_shader_invocations
        ),
        (
            l,
            "max_geometry_input_components",
            max_geometry_input_components
        ),
        (
            l,
            "max_geometry_output_components",
            max_geometry_output_components
        ),
        (
            l,
            "max_geometry_output_vertices",
            max_geometry_output_vertices
        ),
        (
            l,
            "max_geometry_total_output_components",
            max_geometry_total_output_components
        ),
        (
            l,
            "max_fragment_input_components",
            max_fragment_input_components
        ),
        (
            l,
            "max_fragment_output_attachments",
            max_fragment_output_attachments
        ),
        (
            l,
            "max_fragment_dual_src_attachments",
            max_fragment_dual_src_attachments
        ),
        (
            l,
            "max_fragment_combined_output_resources",
            max_fragment_combined_output_resources
        ),
        (
            l,
            "max_compute_shared_memory_size",
            max_compute_shared_memory_size
        ),
        (
            l,
            "max_compute_work_group_count",
            max_compute_work_group_count
        ),
        (
            l,
            "max_compute_work_group_invocations",
            max_compute_work_group_invocations
        ),
        (
            l,
            "max_compute_work_group_size",
            max_compute_work_group_size
        ),
        (l, "sub_pixel_precision_bits", sub_pixel_precision_bits),
        (l, "sub_texel_precision_bits", sub_texel_precision_bits),
        (l, "mipmap_precision_bits", mipmap_precision_bits),
        (
            l,
            "max_draw_indexed_index_value",
            max_draw_indexed_index_value
        ),
        (l, "max_draw_indirect_count", max_draw_indirect_count),
        (l, "max_sampler_lod_bias", max_sampler_lod_bias),
        (l, "max_sampler_anisotropy", max_sampler_anisotropy),
        (l, "max_viewports", max_viewports),
        (l, "max_viewport_dimensions", max_viewport_dimensions),
        (l, "viewport_bounds_range", viewport_bounds_range),
        (l, "viewport_sub_pixel_bits", viewport_sub_pixel_bits),
        (l, "min_memory_map_alignment", min_memory_map_alignment),
        (
            l,
            "min_texel_buffer_offset_alignment",
            min_texel_buffer_offset_alignment
        ),
        (
            l,
            "min_uniform_buffer_offset_alignment",
            min_uniform_buffer_offset_alignment
        ),
        (
            l,
            "min_storage_buffer_offset_alignment",
            min_storage_buffer_offset_alignment
        ),
        (l, "min_texel_offset", min_texel_offset),
        (l, "max_texel_offset", max_texel_offset),
        (l, "min_texel_gather_offset", min_texel_gather_offset),
        (l, "max_texel_gather_offset", max_texel_gather_offset),
        (l, "min_interpolation_offset", min_interpolation_offset),
        (l, "max_interpolation_offset", max_interpolation_offset),
        (
            l,
            "sub_pixel_interpolation_offset_bits",
            sub_pixel_interpolation_offset_bits
        ),
        (l, "max_framebuffer_width", max_framebuffer_width),
        (l, "max_framebuffer_height", max_framebuffer_height),
        (l, "max_framebuffer_layers", max_framebuffer_layers),
        (
            l,
            "framebuffer_color_sample_counts",
            framebuffer_color_sample_counts
        ),
        (
            l,
            "framebuffer_depth_sample_counts",
            framebuffer_depth_sample_counts
        ),
        (
            l,
            "framebuffer_stencil_sample_counts",
            framebuffer_stencil_sample_counts
        ),
        (
            l,
            "framebuffer_no_attachments_sample_counts",
            framebuffer_no_attachments_sample_counts
        ),
        (l, "max_color_attachments", max_color_attachments),
        (
            l,
            "sampled_image_color_sample_counts",
            sampled_image_color_sample_counts
        ),
        (
            l,
            "sampled_image_integer_sample_counts",
            sampled_image_integer_sample_counts
        ),
        (
            l,
            "sampled_image_depth_sample_counts",
            sampled_image_depth_sample_counts
        ),
        (
            l,
            "sampled_image_stencil_sample_counts",
            sampled_image_stencil_sample_counts
        ),
        (
            l,
            "storage_image_sample_counts",
            storage_image_sample_counts
        ),
        (l, "max_sample_mask_words", max_sample_mask_words),
        (
            l,
            "timestamp_compute_and_graphics",
            timestamp_compute_and_graphics
        ),
        (l, "timestamp_period", timestamp_period),
        (l, "max_clip_distances", max_clip_distances),
        (l, "max_cull_distances", max_cull_distances),
        (
            l,
            "max_combined_clip_and_cull_distances",
            max_combined_clip_and_cull_distances
        ),
        (l, "discrete_queue_priorities", discrete_queue_priorities),
        (l, "point_size_range", point_size_range),
        (l, "line_width_range", line_width_range),
        (l, "point_size_granularity", point_size_granularity),
        (l, "line_width_granularity", line_width_granularity),
        (l, "strict_lines", strict_lines),
        (l, "standard_sample_locations", standard_sample_locations),
        (
            l,
            "optimal_buffer_copy_offset_alignment",
            optimal_buffer_copy_offset_alignment
        ),
        (
            l,
            "optimal_buffer_copy_row_pitch_alignment",
            optimal_buffer_copy_row_pitch_alignment
        ),
        (l, "non_coherent_atom_size", non_coherent_atom_size),
    ]
}

macro_rules! make_features {
    ($(($features:ident, $field:ident, $name:expr, $ok:ident, $fail:ident),)*) => (
        $(
            (if $features.$field { &mut $ok } else { &mut $fail }).push($name);
        )*
    );
}

pub fn physical_device_features<'a>(
    pd: &PhysicalDevice<'a>,
) -> (Vec<&'static str>, Vec<&'static str>) {
    let f = pd.supported_features();
    let mut a = vec![];
    let mut b = vec![];

    make_features![
        (f, robust_buffer_access, "robust_buffer_access", a, b),
        (f, full_draw_index_uint32, "full_draw_index_uint32", a, b),
        (f, image_cube_array, "image_cube_array", a, b),
        (f, independent_blend, "independent_blend", a, b),
        (f, geometry_shader, "geometry_shader", a, b),
        (f, tessellation_shader, "tessellation_shader", a, b),
        (f, sample_rate_shading, "sample_rate_shading", a, b),
        (f, dual_src_blend, "dual_src_blend", a, b),
        (f, logic_op, "logic_op", a, b),
        (f, multi_draw_indirect, "multi_draw_indirect", a, b),
        (
            f,
            draw_indirect_first_instance,
            "draw_indirect_first_instance",
            a,
            b
        ),
        (f, depth_clamp, "depth_clamp", a, b),
        (f, depth_bias_clamp, "depth_bias_clamp", a, b),
        (f, fill_mode_non_solid, "fill_mode_non_solid", a, b),
        (f, depth_bounds, "depth_bounds", a, b),
        (f, wide_lines, "wide_lines", a, b),
        (f, large_points, "large_points", a, b),
        (f, alpha_to_one, "alpha_to_one", a, b),
        (f, multi_viewport, "multi_viewport", a, b),
        (f, sampler_anisotropy, "sampler_anisotropy", a, b),
        (
            f,
            texture_compression_etc2,
            "texture_compression_etc2",
            a,
            b
        ),
        (
            f,
            texture_compression_astc_ldr,
            "texture_compression_astc_ldr",
            a,
            b
        ),
        (f, texture_compression_bc, "texture_compression_bc", a, b),
        (f, occlusion_query_precise, "occlusion_query_precise", a, b),
        (
            f,
            pipeline_statistics_query,
            "pipeline_statistics_query",
            a,
            b
        ),
        (
            f,
            vertex_pipeline_stores_and_atomics,
            "vertex_pipeline_stores_and_atomics",
            a,
            b
        ),
        (
            f,
            fragment_stores_and_atomics,
            "fragment_stores_and_atomics",
            a,
            b
        ),
        (
            f,
            shader_tessellation_and_geometry_point_size,
            "shader_tessellation_and_geometry_point_size",
            a,
            b
        ),
        (
            f,
            shader_image_gather_extended,
            "shader_image_gather_extended",
            a,
            b
        ),
        (
            f,
            shader_storage_image_extended_formats,
            "shader_storage_image_extended_formats",
            a,
            b
        ),
        (
            f,
            shader_storage_image_multisample,
            "shader_storage_image_multisample",
            a,
            b
        ),
        (
            f,
            shader_storage_image_read_without_format,
            "shader_storage_image_read_without_format",
            a,
            b
        ),
        (
            f,
            shader_storage_image_write_without_format,
            "shader_storage_image_write_without_format",
            a,
            b
        ),
        (
            f,
            shader_uniform_buffer_array_dynamic_indexing,
            "shader_uniform_buffer_array_dynamic_indexing",
            a,
            b
        ),
        (
            f,
            shader_sampled_image_array_dynamic_indexing,
            "shader_sampled_image_array_dynamic_indexing",
            a,
            b
        ),
        (
            f,
            shader_storage_buffer_array_dynamic_indexing,
            "shader_storage_buffer_array_dynamic_indexing",
            a,
            b
        ),
        (
            f,
            shader_storage_image_array_dynamic_indexing,
            "shader_storage_image_array_dynamic_indexing",
            a,
            b
        ),
        (f, shader_clip_distance, "shader_clip_distance", a, b),
        (f, shader_cull_distance, "shader_cull_distance", a, b),
        (f, shader_float64, "shader_float64", a, b),
        (f, shader_int64, "shader_int64", a, b),
        (f, shader_int16, "shader_int16", a, b),
        (
            f,
            shader_resource_residency,
            "shader_resource_residency",
            a,
            b
        ),
        (f, shader_resource_min_lod, "shader_resource_min_lod", a, b),
        (f, sparse_binding, "sparse_binding", a, b),
        (f, sparse_residency_buffer, "sparse_residency_buffer", a, b),
        (
            f,
            sparse_residency_image2d,
            "sparse_residency_image2d",
            a,
            b
        ),
        (
            f,
            sparse_residency_image3d,
            "sparse_residency_image3d",
            a,
            b
        ),
        (
            f,
            sparse_residency2_samples,
            "sparse_residency2_samples",
            a,
            b
        ),
        (
            f,
            sparse_residency4_samples,
            "sparse_residency4_samples",
            a,
            b
        ),
        (
            f,
            sparse_residency8_samples,
            "sparse_residency8_samples",
            a,
            b
        ),
        (
            f,
            sparse_residency16_samples,
            "sparse_residency16_samples",
            a,
            b
        ),
        (
            f,
            sparse_residency_aliased,
            "sparse_residency_aliased",
            a,
            b
        ),
        (
            f,
            variable_multisample_rate,
            "variable_multisample_rate",
            a,
            b
        ),
        (f, inherited_queries, "inherited_queries", a, b),
        (f, buffer_device_address, "buffer_device_address", a, b),
        (
            f,
            buffer_device_address_capture_replay,
            "buffer_device_address_capture_replay",
            a,
            b
        ),
        (
            f,
            buffer_device_address_multi_device,
            "buffer_device_address_multi_device",
            a,
            b
        ),
        (
            f,
            variable_pointers_storage_buffer,
            "variable_pointers_storage_buffer",
            a,
            b
        ),
        (f, variable_pointers, "variable_pointers", a, b),
        (
            f,
            shader_buffer_int64_atomics,
            "shader_buffer_int64_atomics",
            a,
            b
        ),
        (
            f,
            shader_shared_int64_atomics,
            "shader_shared_int64_atomics",
            a,
            b
        ),
        (f, storage_buffer_8bit, "storage_buffer_8bit", a, b),
        (f, storage_uniform_8bit, "storage_uniform_8bit", a, b),
        (
            f,
            storage_push_constant_8bit,
            "storage_push_constant_8bit",
            a,
            b
        ),
        (f, storage_buffer_16bit, "storage_buffer_16bit", a, b),
        (f, storage_uniform_16bit, "storage_uniform_16bit", a, b),
        (
            f,
            storage_push_constant_16bit,
            "storage_push_constant_16bit",
            a,
            b
        ),
        (
            f,
            storage_input_output_16bit,
            "storage_input_output_16bit",
            a,
            b
        ),
        (f, shader_float16, "shader_float16", a, b),
        (f, shader_int8, "shader_int8", a, b),
    ];

    (a, b)
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
