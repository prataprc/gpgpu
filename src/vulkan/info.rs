use colored::Colorize;
use prettytable::{cell, row, Row};
use vk_sys as vk;
use vulkano::{
    format::{Format, FormatProperties},
    image::{
        Extent, ImageAspects, ImageFormatProperties, ImageTiling, ImageType, ImageUsage,
    },
    instance::{
        LayerProperties, MemoryHeap, MemoryType, PhysicalDevice, PhysicalDeviceType,
        QueueFamily,
    },
    swapchain::{
        SupportedCompositeAlpha, SupportedPresentModes, SupportedSurfaceTransforms,
        Surface,
    },
    VulkanObject,
};

use std::{
    ffi::{CStr, CString},
    ptr,
};

use super::check_errors;
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
        row![Fy => "MemoryHeap", "Size", "DEVICE_LOCAL", "MULTI_INSTANCE"]
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

impl<'a> PrettyRow for QueueFamily<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "QueueId", "Count", "ImageTxGranularity", "Graphics", "Compute", "Sparse", "XTransfer"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.id(),
            self.queues_count(),
            format!("{:?}", self.min_image_transfer_granularity()),
            if self.supports_graphics() {
                "✓"
            } else {
                "✗"
            },
            if self.supports_compute() {
                "✓"
            } else {
                "✗"
            },
            if self.explicitly_supports_transfers() {
                "✓"
            } else {
                "✗"
            },
            if self.supports_sparse_binding() {
                "✓"
            } else {
                "✗"
            },
        ]
    }
}

impl<'a> PrettyRow for MemoryType<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "MemoryType", "Heap", "LOCAL", "VISIBLE", "CACHED", "COHERENT", "LAZY"]
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

pub struct LimitItem {
    name: String,
    value: String,
}

impl PrettyRow for LimitItem {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Limit-name", "-"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.value]
    }
}

macro_rules! make_limits {
    (method, $(($s:ident, $limit:ident, $field:ident),)*) => (
        vec![
            $(
                LimitItem {
                    name: stringify!($field).to_string(),
                    value: $s($limit.$field()),
                },
            )*
        ]
    );
    (dbg_method, $(($limit:ident, $field:ident),)*) => (
        vec![
            $(
                LimitItem {
                    name: stringify!($field).to_string(),
                    value: format!("{:?}", $limit.$field()),
                },
            )*
        ]
    );
    (field, $(($s:ident, $limit:ident, $field:ident),)*) => (
        vec![
            $(
                LimitItem {
                    name: stringify!($field).to_string(),
                    value: $s($limit.$field),
                },
            )*
        ]
    );
    (dbg_field, $(($limit:ident, $field:ident),)*) => (
        vec![
            $(
                LimitItem {
                    name: stringify!($field).to_string(),
                    value: format!("{:?}", $limit.$field),
                },
            )*
        ]
    );
}

pub fn physical_device_limits<'a>(pd: &PhysicalDevice<'a>) -> Vec<LimitItem> {
    let l = pd.limits();
    make_limits![
        dbg_method,
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

pub struct ChecklistItem {
    pub(super) name: String,
    pub(super) supported: bool,
}

impl PrettyRow for ChecklistItem {
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

macro_rules! make_check_list {
    ($(($items:ident, $field:ident),)*) => (
        vec![
            $(
                ChecklistItem {
                    name: stringify!($field).to_string(),
                    supported: $items.$field,
                },
            )*
        ]
    );
}

pub fn device_features<'a>(pd: PhysicalDevice<'a>) -> Vec<ChecklistItem> {
    let f = pd.supported_features();

    make_check_list![
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

pub fn surface_capabilities<'a, W>(
    pd: PhysicalDevice<'a>,
    surface: &Surface<W>,
) -> Vec<LimitItem> {
    let c = surface.capabilities(pd).unwrap();

    let mut limits = make_limits![
        dbg_field,
        (c, min_image_count),
        (c, max_image_count),
        (c, current_extent),
        (c, min_image_extent),
        (c, max_image_extent),
        (c, max_image_array_layers),
        (c, current_transform),
        (c, supported_formats),
    ];
    limits.extend(make_limits![
        field,
        (surface_transforms, c, supported_transforms),
        (composite_alpha, c, supported_composite_alpha),
        (image_usage, c, supported_usage_flags),
        (present_modes, c, present_modes),
    ]);
    limits
}

macro_rules! make_list {
    ($(($items:ident, $field:ident),)*) => (
        vec![
            $(
                match $items.$field {
                    true => stringify!($field),
                    false => "",
                },
            )*
        ]
    );
    ($(($items:ident, $field:ident, $val:expr),)*) => (
        vec![
            $(
                match $items.$field {
                    true => $val,
                    false => "",
                },
            )*
        ]
    );
}

fn surface_transforms(val: SupportedSurfaceTransforms) -> String {
    let ss: Vec<&str> = make_list![
        (val, identity),
        (val, rotate90),
        (val, rotate180),
        (val, rotate270),
        (val, horizontal_mirror),
        (val, horizontal_mirror_rotate90),
        (val, horizontal_mirror_rotate180),
        (val, horizontal_mirror_rotate270),
        (val, inherit),
    ]
    .into_iter()
    .filter(|s| s.len() > 0)
    .collect();
    ss.join(", ")
}

fn composite_alpha(val: SupportedCompositeAlpha) -> String {
    let ss: Vec<&str> = make_list![
        (val, opaque),
        (val, pre_multiplied),
        (val, post_multiplied),
        (val, inherit),
    ]
    .into_iter()
    .filter(|s| s.len() > 0)
    .collect();
    ss.join(", ")
}

fn image_usage(val: ImageUsage) -> String {
    let ss: Vec<&str> = make_list![
        (val, transfer_source),
        (val, transfer_destination),
        (val, sampled),
        (val, storage),
        (val, color_attachment),
        (val, depth_stencil_attachment),
        (val, transient_attachment),
        (val, input_attachment),
    ]
    .into_iter()
    .filter(|s| s.len() > 0)
    .collect();
    ss.join(", ")
}

fn present_modes(val: SupportedPresentModes) -> String {
    let ss: Vec<&str> = make_list![
        (val, immediate),
        (val, mailbox),
        (val, fifo),
        (val, relaxed),
        (val, shared_demand),
        (val, shared_continuous),
    ]
    .into_iter()
    .filter(|s| s.len() > 0)
    .collect();
    ss.join(", ")
}

fn format_cell_content(a: bool, b: bool, c: bool) -> String {
    let mut s = String::default();
    match a {
        true => s.push_str(&"✓".green().to_string()),
        false => s.push_str(&"✗".red().to_string()),
    }
    match b {
        true => s.push_str(&"✓".green().to_string()),
        false => s.push_str(&"✗".red().to_string()),
    }
    match c {
        true => s.push_str(&"✓".green().to_string()),
        false => s.push_str(&"✗".red().to_string()),
    }
    s
}

macro_rules! format_props {
    ($val:ident, $($field:ident,)*) => (
        vec![
            $(
                match ($val.linear_tiling_features.$field, $val.optimal_tiling_features.$field, $val.buffer_features.$field) {
                    (false, false, false) => "-".to_string(),
                    (a, b, c) => format_cell_content(a, b, c).to_string(),
                },
            )*
        ]
    );
}

impl PrettyRow for FormatProperties {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy =>
            "Format", "sai", "sti", "sia", "utb", "stb", "stba",
            "vtb", "cra", "cab", "dsa", "bts", "btd", "sifl",
            "txs", "txd", "mcs", "ycl", "ycs", "ycc", "ycf",
            "disj", "ccs", "sifm", "sifc", "asvb", "fdm"
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        use std::iter::FromIterator;

        let cells = format_props![
            self,
            sampled_image,
            storage_image,
            storage_image_atomic,
            uniform_texel_buffer,
            storage_texel_buffer,
            storage_texel_buffer_atomic,
            vertex_buffer,
            color_attachment,
            color_attachment_blend,
            depth_stencil_attachment,
            blit_src,
            blit_dst,
            sampled_image_filter_linear,
            transfer_src,
            transfer_dst,
            midpoint_chroma_samples,
            sampled_image_ycbcr_conversion_linear_filter,
            sampled_image_ycbcr_conversion_separate_reconstruction_filter,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable,
            disjoint,
            cosited_chroma_samples,
            sampled_image_filter_minmax,
            img_sampled_image_filter_cubic,
            khr_acceleration_structure_vertex_buffer,
            ext_fragment_density_map,
        ];

        Row::from_iter(cells.into_iter())
    }
}

impl PrettyRow for Format {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Format", "DataType", "Size(Bytes)", "BlockDimn", "Planes", "Aspects" ]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            format!("{:?}", self),
            format!("{:?}", self.ty()),
            self.size()
                .map(|a| a.to_string())
                .unwrap_or("-".to_string()),
            format!("{:?}", self.block_dimensions()),
            self.planes(),
            image_aspects(self.aspects()),
        ]
    }
}

fn image_aspects(val: ImageAspects) -> String {
    let ss: Vec<&str> = make_list![
        (val, color),
        (val, depth),
        (val, stencil),
        (val, metadata),
        (val, plane0),
        (val, plane1),
        (val, plane2),
        (val, memory_plane0),
        (val, memory_plane1),
        (val, memory_plane2),
    ]
    .into_iter()
    .filter(|s| s.len() > 0)
    .collect();
    ss.join(", ")
}

pub struct ImageFormat {
    format: Format,
    ty: ImageType,
    tiling: ImageTiling,
    usage: ImageUsage,
    props: ImageFormatProperties,
}

impl PrettyRow for ImageFormat {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy =>
            "Format", "Type", "Tiling", "Usage",
            "max_extent", "max_mip_levels", "max_array_layers", "sample_counts",
            "max_resource_size"
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        let sample_val = self.props.sample_counts;
        let sample_counts: Vec<&str> = make_list![
            (sample_val, sample1, "1"),
            (sample_val, sample2, "2"),
            (sample_val, sample4, "4"),
            (sample_val, sample8, "8"),
            (sample_val, sample16, "16"),
            (sample_val, sample32, "32"),
            (sample_val, sample64, "64"),
        ]
        .into_iter()
        .filter(|s| s.len() > 0)
        .collect();

        row![
            format!("{:?}", self.format),
            format!("{:?}", self.ty),
            format!("{:?}", self.tiling),
            image_usage(self.usage),
            format!("{:?}", self.props.max_extent),
            format!("{:?}", self.props.max_mip_levels),
            self.props.max_array_layers,
            sample_counts.join(","),
            self.props.max_resource_size,
        ]
    }
}

impl ImageFormat {
    pub fn new(
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsage,
        props: ImageFormatProperties,
    ) -> Self {
        ImageFormat {
            format,
            ty,
            tiling,
            usage,
            props,
        }
    }
}

pub fn format_list() -> Vec<Format> {
    vec![
        Format::R4G4UnormPack8,
        Format::R4G4B4A4UnormPack16,
        Format::B4G4R4A4UnormPack16,
        Format::R5G6B5UnormPack16,
        Format::B5G6R5UnormPack16,
        Format::R5G5B5A1UnormPack16,
        Format::B5G5R5A1UnormPack16,
        Format::A1R5G5B5UnormPack16,
        Format::R8Unorm,
        Format::R8Snorm,
        Format::R8Uscaled,
        Format::R8Sscaled,
        Format::R8Uint,
        Format::R8Sint,
        Format::R8Srgb,
        Format::R8G8Unorm,
        Format::R8G8Snorm,
        Format::R8G8Uscaled,
        Format::R8G8Sscaled,
        Format::R8G8Uint,
        Format::R8G8Sint,
        Format::R8G8Srgb,
        Format::R8G8B8Unorm,
        Format::R8G8B8Snorm,
        Format::R8G8B8Uscaled,
        Format::R8G8B8Sscaled,
        Format::R8G8B8Uint,
        Format::R8G8B8Sint,
        Format::R8G8B8Srgb,
        Format::B8G8R8Unorm,
        Format::B8G8R8Snorm,
        Format::B8G8R8Uscaled,
        Format::B8G8R8Sscaled,
        Format::B8G8R8Uint,
        Format::B8G8R8Sint,
        Format::B8G8R8Srgb,
        Format::R8G8B8A8Unorm,
        Format::R8G8B8A8Snorm,
        Format::R8G8B8A8Uscaled,
        Format::R8G8B8A8Sscaled,
        Format::R8G8B8A8Uint,
        Format::R8G8B8A8Sint,
        Format::R8G8B8A8Srgb,
        Format::B8G8R8A8Unorm,
        Format::B8G8R8A8Snorm,
        Format::B8G8R8A8Uscaled,
        Format::B8G8R8A8Sscaled,
        Format::B8G8R8A8Uint,
        Format::B8G8R8A8Sint,
        Format::B8G8R8A8Srgb,
        Format::A8B8G8R8UnormPack32,
        Format::A8B8G8R8SnormPack32,
        Format::A8B8G8R8UscaledPack32,
        Format::A8B8G8R8SscaledPack32,
        Format::A8B8G8R8UintPack32,
        Format::A8B8G8R8SintPack32,
        Format::A8B8G8R8SrgbPack32,
        Format::A2R10G10B10UnormPack32,
        Format::A2R10G10B10SnormPack32,
        Format::A2R10G10B10UscaledPack32,
        Format::A2R10G10B10SscaledPack32,
        Format::A2R10G10B10UintPack32,
        Format::A2R10G10B10SintPack32,
        Format::A2B10G10R10UnormPack32,
        Format::A2B10G10R10SnormPack32,
        Format::A2B10G10R10UscaledPack32,
        Format::A2B10G10R10SscaledPack32,
        Format::A2B10G10R10UintPack32,
        Format::A2B10G10R10SintPack32,
        Format::R16Unorm,
        Format::R16Snorm,
        Format::R16Uscaled,
        Format::R16Sscaled,
        Format::R16Uint,
        Format::R16Sint,
        Format::R16Sfloat,
        Format::R16G16Unorm,
        Format::R16G16Snorm,
        Format::R16G16Uscaled,
        Format::R16G16Sscaled,
        Format::R16G16Uint,
        Format::R16G16Sint,
        Format::R16G16Sfloat,
        Format::R16G16B16Unorm,
        Format::R16G16B16Snorm,
        Format::R16G16B16Uscaled,
        Format::R16G16B16Sscaled,
        Format::R16G16B16Uint,
        Format::R16G16B16Sint,
        Format::R16G16B16Sfloat,
        Format::R16G16B16A16Unorm,
        Format::R16G16B16A16Snorm,
        Format::R16G16B16A16Uscaled,
        Format::R16G16B16A16Sscaled,
        Format::R16G16B16A16Uint,
        Format::R16G16B16A16Sint,
        Format::R16G16B16A16Sfloat,
        Format::R32Uint,
        Format::R32Sint,
        Format::R32Sfloat,
        Format::R32G32Uint,
        Format::R32G32Sint,
        Format::R32G32Sfloat,
        Format::R32G32B32Uint,
        Format::R32G32B32Sint,
        Format::R32G32B32Sfloat,
        Format::R32G32B32A32Uint,
        Format::R32G32B32A32Sint,
        Format::R32G32B32A32Sfloat,
        Format::R64Uint,
        Format::R64Sint,
        Format::R64Sfloat,
        Format::R64G64Uint,
        Format::R64G64Sint,
        Format::R64G64Sfloat,
        Format::R64G64B64Uint,
        Format::R64G64B64Sint,
        Format::R64G64B64Sfloat,
        Format::R64G64B64A64Uint,
        Format::R64G64B64A64Sint,
        Format::R64G64B64A64Sfloat,
        Format::B10G11R11UfloatPack32,
        Format::E5B9G9R9UfloatPack32,
        Format::D16Unorm,
        Format::X8_D24UnormPack32,
        Format::D32Sfloat,
        Format::S8Uint,
        Format::D16Unorm_S8Uint,
        Format::D24Unorm_S8Uint,
        Format::D32Sfloat_S8Uint,
        Format::BC1_RGBUnormBlock,
        Format::BC1_RGBSrgbBlock,
        Format::BC1_RGBAUnormBlock,
        Format::BC1_RGBASrgbBlock,
        Format::BC2UnormBlock,
        Format::BC2SrgbBlock,
        Format::BC3UnormBlock,
        Format::BC3SrgbBlock,
        Format::BC4UnormBlock,
        Format::BC4SnormBlock,
        Format::BC5UnormBlock,
        Format::BC5SnormBlock,
        Format::BC6HUfloatBlock,
        Format::BC6HSfloatBlock,
        Format::BC7UnormBlock,
        Format::BC7SrgbBlock,
        Format::ETC2_R8G8B8UnormBlock,
        Format::ETC2_R8G8B8SrgbBlock,
        Format::ETC2_R8G8B8A1UnormBlock,
        Format::ETC2_R8G8B8A1SrgbBlock,
        Format::ETC2_R8G8B8A8UnormBlock,
        Format::ETC2_R8G8B8A8SrgbBlock,
        Format::EAC_R11UnormBlock,
        Format::EAC_R11SnormBlock,
        Format::EAC_R11G11UnormBlock,
        Format::EAC_R11G11SnormBlock,
        Format::ASTC_4x4UnormBlock,
        Format::ASTC_4x4SrgbBlock,
        Format::ASTC_5x4UnormBlock,
        Format::ASTC_5x4SrgbBlock,
        Format::ASTC_5x5UnormBlock,
        Format::ASTC_5x5SrgbBlock,
        Format::ASTC_6x5UnormBlock,
        Format::ASTC_6x5SrgbBlock,
        Format::ASTC_6x6UnormBlock,
        Format::ASTC_6x6SrgbBlock,
        Format::ASTC_8x5UnormBlock,
        Format::ASTC_8x5SrgbBlock,
        Format::ASTC_8x6UnormBlock,
        Format::ASTC_8x6SrgbBlock,
        Format::ASTC_8x8UnormBlock,
        Format::ASTC_8x8SrgbBlock,
        Format::ASTC_10x5UnormBlock,
        Format::ASTC_10x5SrgbBlock,
        Format::ASTC_10x6UnormBlock,
        Format::ASTC_10x6SrgbBlock,
        Format::ASTC_10x8UnormBlock,
        Format::ASTC_10x8SrgbBlock,
        Format::ASTC_10x10UnormBlock,
        Format::ASTC_10x10SrgbBlock,
        Format::ASTC_12x10UnormBlock,
        Format::ASTC_12x10SrgbBlock,
        Format::ASTC_12x12UnormBlock,
        Format::ASTC_12x12SrgbBlock,
        Format::G8B8R8_3PLANE420Unorm,
        Format::G8B8R8_2PLANE420Unorm,
    ]
}

pub fn image_type_list() -> Vec<ImageType> {
    vec![ImageType::Dim1d, ImageType::Dim2d, ImageType::Dim3d]
}

pub fn image_tiling_list() -> Vec<ImageTiling> {
    vec![ImageTiling::Optimal, ImageTiling::Linear]
}

pub fn image_usage_list() -> Vec<ImageUsage> {
    let none = ImageUsage::none();
    vec![
        ImageUsage::all(),
        ImageUsage::color_attachment(),
        ImageUsage::depth_stencil_attachment(),
        ImageUsage::transient_color_attachment(),
        ImageUsage::transient_depth_stencil_attachment(),
        ImageUsage {
            input_attachment: true,
            ..none
        },
        ImageUsage {
            sampled: true,
            ..none
        },
        ImageUsage {
            storage: true,
            ..none
        },
        ImageUsage {
            transfer_source: true,
            ..none
        },
        ImageUsage {
            transfer_destination: true,
            ..none
        },
    ]
}

pub fn format_extent(extent: Extent) -> String {
    match extent {
        Extent::E1D(val) => format!("{:?}", val),
        Extent::E2D(val) => format!("{:?}", val),
        Extent::E3D(val) => format!("{:?}", val),
    }
}
