use colored::Colorize;
use prettytable::{cell, row, Row, Table};
use vulkano::{
    descriptor::descriptor::ShaderStages,
    format::{Format, FormatProperties},
    image::{ImageAspects, SampleCounts},
    instance::{
        DriverId, LayerProperties, MemoryHeap, MemoryType, PhysicalDevice,
        PointClippingBehavior, QueueFamily, ShaderFloatControlsIndependence,
        SubgroupFeatures,
    },
};

use std::fmt;

#[macro_export]
macro_rules! format_unwrap_or {
    ($val:expr, $tos:ident, $def:expr) => {
        $val.as_ref().map(|x| $tos(x)).unwrap_or($def.to_string())
    };
}

#[macro_export]
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

pub trait PrettyRow {
    fn to_format() -> prettytable::format::TableFormat;

    fn to_head() -> prettytable::Row;

    fn to_row(&self) -> prettytable::Row;
}

pub fn make_table<R>(rows: &[R]) -> prettytable::Table
where
    R: PrettyRow,
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

impl<'a> PrettyRow for QueueFamily<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![
            Fy => "QueueId", "Count", "ImageTxGranularity", "Graphics", "Compute",
            "Sparse", "XTransfer"
        ]
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

// TODO: don't do this as table, there are way too many details.
impl<'a> PrettyRow for PhysicalDevice<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![
            "api_version",
            "driver_version",
            "vendor_id",
            "device_id",
            "device_type",
            "device_name",
            "driver_id",
            "driver_info",
            "driver_name",
            "driver_uuid",
            "device_uuid",
            "discrete_queue_priorities",
            "pci_function",
            "active_compute_unit_count",
            "advanced_blend_all_operations",
            "advanced_blend_correlated_overlap",
            "advanced_blend_independent_blend",
            "advanced_blend_max_color_attachments",
            "advanced_blend_non_premultiplied_dst_color",
            "advanced_blend_non_premultiplied_src_color",
            "allow_command_buffer_query_copies",
            "buffer_image_granularity",
            "compute_units_per_shader_array",
            "conformance_version",
            "conservative_point_and_line_rasterization",
            "conservative_rasterization_post_depth_coverage",
            "cooperative_matrix_supported_stages",
            "degenerate_lines_rasterized",
            "degenerate_triangles_rasterized",
            "denorm_behavior_independence",
            "device_luid",
            "device_luid_valid",
            "device_node_mask",
            "extra_primitive_overestimation_size_granularity",
            "filter_minmax_image_component_mapping",
            "filter_minmax_single_component_formats",
            "fragment_density_invocations",
            "fragment_shading_rate_non_trivial_combiner_ops",
            "fragment_shading_rate_strict_multiply_combiner",
            "fragment_shading_rate_with_conservative_rasterization",
            "fragment_shading_rate_with_custom_sample_locations",
            "fragment_shading_rate_with_fragment_shader_interlock",
            "fragment_shading_rate_with_sample_mask",
            "fragment_shading_rate_with_shader_depth_stencil_writes",
            "fragment_shading_rate_with_shader_sample_mask",
            "framebuffer_color_sample_counts",
            "framebuffer_depth_sample_counts",
            "framebuffer_integer_color_sample_counts",
            "framebuffer_no_attachments_sample_counts",
            "framebuffer_stencil_sample_counts",
            "fully_covered_fragment_shader_input_variable",
            "independent_resolve",
            "independent_resolve_none",
            "layered_shading_rate_attachments",
            "line_sub_pixel_precision_bits",
            "line_width_granularity",
            "line_width_range",
            "max_bound_descriptor_sets",
            "max_clip_distances",
            "max_color_attachments",
            "max_combined_clip_and_cull_distances",
            "max_compute_shared_memory_size",
            "max_compute_work_group_count",
            "max_compute_work_group_invocations",
            "max_compute_work_group_size",
            "max_compute_workgroup_subgroups",
            "max_cull_distances",
            "max_custom_border_color_samplers",
            "max_descriptor_set_acceleration_structures",
            "max_descriptor_set_inline_uniform_blocks",
            "max_descriptor_set_input_attachments",
            "max_descriptor_set_sampled_images",
            "max_descriptor_set_samplers",
            "max_descriptor_set_storage_buffers",
            "max_descriptor_set_storage_buffers_dynamic",
            "max_descriptor_set_storage_images",
            "max_descriptor_set_subsampled_samplers",
            "max_descriptor_set_uniform_buffers",
            "max_descriptor_set_uniform_buffers_dynamic",
            "max_descriptor_set_update_after_bind_acceleration_structures",
            "max_descriptor_set_update_after_bind_inline_uniform_blocks",
            "max_descriptor_set_update_after_bind_input_attachments",
            "max_descriptor_set_update_after_bind_sampled_images",
            "max_descriptor_set_update_after_bind_samplers",
            "max_descriptor_set_update_after_bind_storage_buffers",
            "max_descriptor_set_update_after_bind_storage_buffers_dynamic",
            "max_descriptor_set_update_after_bind_storage_images",
            "max_descriptor_set_update_after_bind_uniform_buffers",
            "max_descriptor_set_update_after_bind_uniform_buffers_dynamic",
            "max_discard_rectangles",
            "max_draw_indexed_index_value",
            "max_draw_indirect_count",
            "max_draw_mesh_tasks_count",
            "max_extra_primitive_overestimation_size",
            "max_fragment_combined_output_resources",
            "max_fragment_density_texel_size",
            "max_fragment_dual_src_attachments",
            "max_fragment_input_components",
            "max_fragment_output_attachments",
            "max_fragment_shading_rate_attachment_texel_size",
            "max_fragment_shading_rate_attachment_texel_size_aspect_ratio",
            "max_fragment_shading_rate_coverage_samples",
            "max_fragment_shading_rate_invocation_count",
            "max_fragment_shading_rate_rasterization_samples",
            "max_fragment_size",
            "max_fragment_size_aspect_ratio",
            "max_framebuffer_height",
            "max_framebuffer_layers",
            "max_framebuffer_width",
            "max_geometry_count",
            "max_geometry_input_components",
            "max_geometry_output_components",
            "max_geometry_output_vertices",
            "max_geometry_shader_invocations",
            "max_geometry_total_output_components",
            "max_graphics_shader_group_count",
            "max_image_array_layers",
            "max_image_dimension1_d",
            "max_image_dimension2_d",
            "max_image_dimension3_d",
            "max_image_dimension_cube",
            "max_indirect_commands_stream_count",
            "max_indirect_commands_stream_stride",
            "max_indirect_commands_token_count",
            "max_indirect_commands_token_offset",
            "max_indirect_sequence_count",
            "max_inline_uniform_block_size",
            "max_instance_count",
            "max_interpolation_offset",
            "max_memory_allocation_count",
            "max_memory_allocation_size",
            "max_mesh_multiview_view_count",
            "max_mesh_output_primitives",
            "max_mesh_output_vertices",
            "max_mesh_total_memory_size",
            "max_mesh_work_group_invocations",
            "max_mesh_work_group_size",
            "max_multiview_instance_index",
            "max_multiview_view_count",
            "max_per_set_descriptors",
            "max_per_stage_descriptor_acceleration_structures",
            "max_per_stage_descriptor_inline_uniform_blocks",
            "max_per_stage_descriptor_input_attachments",
            "max_per_stage_descriptor_sampled_images",
            "max_per_stage_descriptor_samplers",
            "max_per_stage_descriptor_storage_buffers",
            "max_per_stage_descriptor_storage_images",
            "max_per_stage_descriptor_uniform_buffers",
            "max_per_stage_descriptor_update_after_bind_acceleration_structures",
            "max_per_stage_descriptor_update_after_bind_inline_uniform_blocks",
            "max_per_stage_descriptor_update_after_bind_input_attachments",
            "max_per_stage_descriptor_update_after_bind_sampled_images",
            "max_per_stage_descriptor_update_after_bind_samplers",
            "max_per_stage_descriptor_update_after_bind_storage_buffers",
            "max_per_stage_descriptor_update_after_bind_storage_images",
            "max_per_stage_descriptor_update_after_bind_uniform_buffers",
            "max_per_stage_resources",
            "max_per_stage_update_after_bind_resources",
            "max_primitive_count",
            "max_push_constants_size",
            "max_push_descriptors",
            "max_ray_dispatch_invocation_count",
            "max_ray_hit_attribute_size",
            "max_ray_recursion_depth",
            "max_recursion_depth",
            "max_sample_location_grid_size",
            "max_sample_mask_words",
            "max_sampler_allocation_count",
            "max_sampler_anisotropy",
            "max_sampler_lod_bias",
            "max_sgpr_allocation",
            "max_shader_group_stride",
            "max_storage_buffer_range",
            "max_subgroup_size",
            "max_subsampled_array_layers",
            "max_task_output_count",
            "max_task_total_memory_size",
            "max_task_work_group_invocations",
            "max_task_work_group_size",
            "max_tessellation_control_per_patch_output_components",
            "max_tessellation_control_per_vertex_input_components",
            "max_tessellation_control_per_vertex_output_components",
            "max_tessellation_control_total_output_components",
            "max_tessellation_evaluation_input_components",
            "max_tessellation_evaluation_output_components",
            "max_tessellation_generation_level",
            "max_tessellation_patch_size",
            "max_texel_buffer_elements",
            "max_texel_gather_offset",
            "max_texel_offset",
            "max_timeline_semaphore_value_difference",
            "max_transform_feedback_buffer_data_size",
            "max_transform_feedback_buffer_data_stride",
            "max_transform_feedback_buffer_size",
            "max_transform_feedback_buffers",
            "max_transform_feedback_stream_data_size",
            "max_transform_feedback_streams",
            "max_triangle_count",
            "max_uniform_buffer_range",
            "max_update_after_bind_descriptors_in_all_pools",
            "max_vertex_attrib_divisor",
            "max_vertex_input_attribute_offset",
            "max_vertex_input_attributes",
            "max_vertex_input_binding_stride",
            "max_vertex_input_bindings",
            "max_vertex_output_components",
            "max_vgpr_allocation",
            "max_viewport_dimensions",
            "max_viewports",
            "mesh_output_per_primitive_granularity",
            "mesh_output_per_vertex_granularity",
            "min_acceleration_structure_scratch_offset_alignment",
            "min_fragment_density_texel_size",
            "min_fragment_shading_rate_attachment_texel_size",
            "min_imported_host_pointer_alignment",
            "min_indirect_commands_buffer_offset_alignment",
            "min_interpolation_offset",
            "min_memory_map_alignment",
            "min_sequences_count_buffer_offset_alignment",
            "min_sequences_index_buffer_offset_alignment",
            "min_sgpr_allocation",
            "min_storage_buffer_offset_alignment",
            "min_subgroup_size",
            "min_texel_buffer_offset_alignment",
            "min_texel_gather_offset",
            "min_texel_offset",
            "min_uniform_buffer_offset_alignment",
            "min_vertex_input_binding_stride_alignment",
            "min_vgpr_allocation",
            "mipmap_precision_bits",
            "non_coherent_atom_size",
            "optimal_buffer_copy_offset_alignment",
            "optimal_buffer_copy_row_pitch_alignment",
            "pci_bus",
            "pci_device",
            "pci_domain",
            "per_view_position_all_components",
            "pipeline_cache_uuid",
            "point_clipping_behavior",
            "point_size_granularity",
            "point_size_range",
            "primitive_fragment_shading_rate_with_multiple_viewports",
            "primitive_overestimation_size",
            "primitive_underestimation",
            "protected_no_fault",
            "quad_divergent_implicit_lod",
            "quad_operations_in_all_stages",
            "required_subgroup_size_stages",
            "residency_aligned_mip_size",
            "residency_non_resident_strict",
            "residency_standard2_d_block_shape",
            "residency_standard2_d_multisample_block_shape",
            "residency_standard3_d_block_shape",
            "robust_buffer_access_update_after_bind",
            "robust_storage_buffer_access_size_alignment",
            "robust_uniform_buffer_access_size_alignment",
            "rounding_mode_independence",
            "sample_location_coordinate_range",
            "sample_location_sample_counts",
            "sample_location_sub_pixel_bits",
            "sampled_image_color_sample_counts",
            "sampled_image_depth_sample_counts",
            "sampled_image_integer_sample_counts",
            "sampled_image_stencil_sample_counts",
            "sgpr_allocation_granularity",
            "sgprs_per_simd",
            "shader_arrays_per_engine_count",
            "shader_core_features",
            "shader_denorm_flush_to_zero_float16",
            "shader_denorm_flush_to_zero_float32",
            "shader_denorm_flush_to_zero_float64",
            "shader_denorm_preserve_float16",
            "shader_denorm_preserve_float32",
            "shader_denorm_preserve_float64",
            "shader_engine_count",
            "shader_group_base_alignment",
            "shader_group_handle_alignment",
            "shader_group_handle_capture_replay_size",
            "shader_group_handle_size",
            "shader_input_attachment_array_non_uniform_indexing_native",
            "shader_rounding_mode_rte_float16",
            "shader_rounding_mode_rte_float32",
            "shader_rounding_mode_rte_float64",
            "shader_rounding_mode_rtz_float16",
            "shader_rounding_mode_rtz_float32",
            "shader_rounding_mode_rtz_float64",
            "shader_sampled_image_array_non_uniform_indexing_native",
            "shader_signed_zero_inf_nan_preserve_float16",
            "shader_signed_zero_inf_nan_preserve_float32",
            "shader_signed_zero_inf_nan_preserve_float64",
            "shader_sm_count",
            "shader_storage_buffer_array_non_uniform_indexing_native",
            "shader_storage_image_array_non_uniform_indexing_native",
            "shader_uniform_buffer_array_non_uniform_indexing_native",
            "shader_warps_per_sm",
            "shading_rate_max_coarse_samples",
            "shading_rate_palette_size",
            "shading_rate_texel_size",
            "simd_per_compute_unit",
            "sparse_address_space_size",
            "standard_sample_locations",
            "storage_image_sample_counts",
            "storage_texel_buffer_offset_alignment_bytes",
            "storage_texel_buffer_offset_single_texel_alignment",
            "strict_lines",
            "sub_pixel_interpolation_offset_bits",
            "sub_pixel_precision_bits",
            "sub_texel_precision_bits",
            "subgroup_quad_operations_in_all_stages",
            "subgroup_size",
            "subgroup_supported_operations",
            "subgroup_supported_stages",
            "subsampled_coarse_reconstruction_early_access",
            "subsampled_loads",
            "supported_depth_resolve_modes",
            "supported_operations",
            "supported_stages",
            "supported_stencil_resolve_modes",
            "timestamp_compute_and_graphics",
            "timestamp_period",
            "transform_feedback_draw",
            "transform_feedback_queries",
            "transform_feedback_rasterization_stream_select",
            "transform_feedback_streams_lines_triangles",
            "uniform_texel_buffer_offset_alignment_bytes",
            "uniform_texel_buffer_offset_single_texel_alignment",
            "variable_sample_locations",
            "vgpr_allocation_granularity",
            "vgprs_per_simd",
            "viewport_bounds_range",
            "viewport_sub_pixel_bits",
            "wavefront_size",
            "wavefronts_per_simd"
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        let uuid_to_s = |id: &[u8; 16]| {
            uuid::Uuid::from_slice(&id[..])
                .unwrap()
                .to_hyphenated()
                .to_string()
        };

        let props = self.properties();
        let driver_uuid = props.driver_uuid.as_ref().map(uuid_to_s);
        let device_uuid = props.device_uuid.as_ref().map(uuid_to_s);
        let pcache_uuid = props.pipeline_cache_uuid.as_ref().map(uuid_to_s);

        row![
            format_unwrap_or!(props.api_version, tos, "-"),
            format_unwrap_or!(props.driver_version, tos, "-"),
            format_unwrap_or!(props.vendor_id, tos, "-"),
            format_unwrap_or!(props.device_id, tos, "-"),
            format_unwrap_or!(props.device_type, tod, "-"),
            format_unwrap_or!(props.device_name, tos, "-"),
            format_unwrap_or!(props.driver_id, driver_id_to_str, "-"),
            format_unwrap_or!(props.driver_info, tos, "-"),
            format_unwrap_or!(props.driver_name, tos, "-"),
            format_unwrap_or!(driver_uuid, tod, "-"),
            format_unwrap_or!(device_uuid, tod, "-"),
            format_unwrap_or!(props.discrete_queue_priorities, tos, "-"),
            format_unwrap_or!(props.pci_function, tos, "-"),
            format_unwrap_or!(props.active_compute_unit_count, tos, "-"),
            format_unwrap_or!(props.advanced_blend_all_operations, tos, "-"),
            format_unwrap_or!(props.advanced_blend_correlated_overlap, tos, "-"),
            format_unwrap_or!(props.advanced_blend_independent_blend, tos, "-"),
            format_unwrap_or!(props.advanced_blend_max_color_attachments, tos, "-"),
            format_unwrap_or!(props.advanced_blend_non_premultiplied_dst_color, tos, "-"),
            format_unwrap_or!(props.advanced_blend_non_premultiplied_src_color, tos, "-"),
            format_unwrap_or!(props.allow_command_buffer_query_copies, tos, "-"),
            format_unwrap_or!(props.buffer_image_granularity, tos, "-"),
            format_unwrap_or!(props.compute_units_per_shader_array, tos, "-"),
            format_unwrap_or!(props.conformance_version, tos, "-"),
            format_unwrap_or!(props.conservative_point_and_line_rasterization, tos, "-"),
            format_unwrap_or!(
                props.conservative_rasterization_post_depth_coverage,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.cooperative_matrix_supported_stages,
                shader_stages_to_str,
                "-"
            ),
            format_unwrap_or!(props.degenerate_lines_rasterized, tos, "-"),
            format_unwrap_or!(props.degenerate_triangles_rasterized, tos, "-"),
            format_unwrap_or!(
                props.denorm_behavior_independence,
                shader_float_control_to_str,
                "-"
            ),
            format_unwrap_or!(props.device_luid, tod, "-"),
            format_unwrap_or!(props.device_luid_valid, tos, "-"),
            format_unwrap_or!(props.device_node_mask, tos, "-"),
            format_unwrap_or!(
                props.extra_primitive_overestimation_size_granularity,
                tos,
                "-"
            ),
            format_unwrap_or!(props.filter_minmax_image_component_mapping, tos, "-"),
            format_unwrap_or!(props.filter_minmax_single_component_formats, tos, "-"),
            format_unwrap_or!(props.fragment_density_invocations, tos, "-"),
            format_unwrap_or!(
                props.fragment_shading_rate_non_trivial_combiner_ops,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.fragment_shading_rate_strict_multiply_combiner,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.fragment_shading_rate_with_conservative_rasterization,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.fragment_shading_rate_with_custom_sample_locations,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.fragment_shading_rate_with_fragment_shader_interlock,
                tos,
                "-"
            ),
            format_unwrap_or!(props.fragment_shading_rate_with_sample_mask, tos, "-"),
            format_unwrap_or!(
                props.fragment_shading_rate_with_shader_depth_stencil_writes,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.fragment_shading_rate_with_shader_sample_mask,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.framebuffer_color_sample_counts,
                sample_counts_to_str,
                "-"
            ),
            format_unwrap_or!(
                props.framebuffer_depth_sample_counts,
                sample_counts_to_str,
                "-"
            ),
            format_unwrap_or!(
                props.framebuffer_integer_color_sample_counts,
                sample_counts_to_str,
                "-"
            ),
            format_unwrap_or!(
                props.framebuffer_no_attachments_sample_counts,
                sample_counts_to_str,
                "-"
            ),
            format_unwrap_or!(
                props.framebuffer_stencil_sample_counts,
                sample_counts_to_str,
                "-"
            ),
            format_unwrap_or!(
                props.fully_covered_fragment_shader_input_variable,
                tos,
                "-"
            ),
            format_unwrap_or!(props.independent_resolve, tos, "-"),
            format_unwrap_or!(props.independent_resolve_none, tos, "-"),
            format_unwrap_or!(props.layered_shading_rate_attachments, tos, "-"),
            format_unwrap_or!(props.line_sub_pixel_precision_bits, tos, "-"),
            format_unwrap_or!(props.line_width_granularity, tos, "-"),
            format_unwrap_or!(props.line_width_range, tod, "-"),
            format_unwrap_or!(props.max_bound_descriptor_sets, tos, "-"),
            format_unwrap_or!(props.max_clip_distances, tos, "-"),
            format_unwrap_or!(props.max_color_attachments, tos, "-"),
            format_unwrap_or!(props.max_combined_clip_and_cull_distances, tos, "-"),
            format_unwrap_or!(props.max_compute_shared_memory_size, tos, "-"),
            format_unwrap_or!(props.max_compute_work_group_count, tod, "-"),
            format_unwrap_or!(props.max_compute_work_group_invocations, tos, "-"),
            format_unwrap_or!(props.max_compute_work_group_size, tod, "-"),
            format_unwrap_or!(props.max_compute_workgroup_subgroups, tos, "-"),
            format_unwrap_or!(props.max_cull_distances, tos, "-"),
            format_unwrap_or!(props.max_custom_border_color_samplers, tos, "-"),
            format_unwrap_or!(props.max_descriptor_set_acceleration_structures, tos, "-"),
            format_unwrap_or!(props.max_descriptor_set_inline_uniform_blocks, tos, "-"),
            format_unwrap_or!(props.max_descriptor_set_input_attachments, tos, "-"),
            format_unwrap_or!(props.max_descriptor_set_sampled_images, tos, "-"),
            format_unwrap_or!(props.max_descriptor_set_samplers, tos, "-"),
            format_unwrap_or!(props.max_descriptor_set_storage_buffers, tos, "-"),
            format_unwrap_or!(props.max_descriptor_set_storage_buffers_dynamic, tos, "-"),
            format_unwrap_or!(props.max_descriptor_set_storage_images, tos, "-"),
            format_unwrap_or!(props.max_descriptor_set_subsampled_samplers, tos, "-"),
            format_unwrap_or!(props.max_descriptor_set_uniform_buffers, tos, "-"),
            format_unwrap_or!(props.max_descriptor_set_uniform_buffers_dynamic, tos, "-"),
            format_unwrap_or!(
                props.max_descriptor_set_update_after_bind_acceleration_structures,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_descriptor_set_update_after_bind_inline_uniform_blocks,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_descriptor_set_update_after_bind_input_attachments,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_descriptor_set_update_after_bind_sampled_images,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_descriptor_set_update_after_bind_samplers,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_descriptor_set_update_after_bind_storage_buffers,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_descriptor_set_update_after_bind_storage_buffers_dynamic,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_descriptor_set_update_after_bind_storage_images,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_descriptor_set_update_after_bind_uniform_buffers,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_descriptor_set_update_after_bind_uniform_buffers_dynamic,
                tos,
                "-"
            ),
            format_unwrap_or!(props.max_discard_rectangles, tos, "-"),
            format_unwrap_or!(props.max_draw_indexed_index_value, tos, "-"),
            format_unwrap_or!(props.max_draw_indirect_count, tos, "-"),
            format_unwrap_or!(props.max_draw_mesh_tasks_count, tos, "-"),
            format_unwrap_or!(props.max_extra_primitive_overestimation_size, tos, "-"),
            format_unwrap_or!(props.max_fragment_combined_output_resources, tos, "-"),
            format_unwrap_or!(props.max_fragment_density_texel_size, tod, "-"),
            format_unwrap_or!(props.max_fragment_dual_src_attachments, tos, "-"),
            format_unwrap_or!(props.max_fragment_input_components, tos, "-"),
            format_unwrap_or!(props.max_fragment_output_attachments, tos, "-"),
            format_unwrap_or!(
                props.max_fragment_shading_rate_attachment_texel_size,
                tod,
                "-"
            ),
            format_unwrap_or!(
                props.max_fragment_shading_rate_attachment_texel_size_aspect_ratio,
                tod,
                "-"
            ),
            format_unwrap_or!(props.max_fragment_shading_rate_coverage_samples, tos, "-"),
            format_unwrap_or!(props.max_fragment_shading_rate_invocation_count, tod, "-"),
            format_unwrap_or!(
                props.max_fragment_shading_rate_rasterization_samples,
                tod,
                "-"
            ),
            format_unwrap_or!(props.max_fragment_size, tod, "-"),
            format_unwrap_or!(props.max_fragment_size_aspect_ratio, tos, "-"),
            format_unwrap_or!(props.max_framebuffer_height, tos, "-"),
            format_unwrap_or!(props.max_framebuffer_layers, tos, "-"),
            format_unwrap_or!(props.max_framebuffer_width, tos, "-"),
            format_unwrap_or!(props.max_geometry_count, tos, "-"),
            format_unwrap_or!(props.max_geometry_input_components, tos, "-"),
            format_unwrap_or!(props.max_geometry_output_components, tos, "-"),
            format_unwrap_or!(props.max_geometry_output_vertices, tos, "-"),
            format_unwrap_or!(props.max_geometry_shader_invocations, tos, "-"),
            format_unwrap_or!(props.max_geometry_total_output_components, tos, "-"),
            format_unwrap_or!(props.max_graphics_shader_group_count, tos, "-"),
            format_unwrap_or!(props.max_image_array_layers, tos, "-"),
            format_unwrap_or!(props.max_image_dimension1_d, tos, "-"),
            format_unwrap_or!(props.max_image_dimension2_d, tos, "-"),
            format_unwrap_or!(props.max_image_dimension3_d, tos, "-"),
            format_unwrap_or!(props.max_image_dimension_cube, tos, "-"),
            format_unwrap_or!(props.max_indirect_commands_stream_count, tos, "-"),
            format_unwrap_or!(props.max_indirect_commands_stream_stride, tos, "-"),
            format_unwrap_or!(props.max_indirect_commands_token_count, tos, "-"),
            format_unwrap_or!(props.max_indirect_commands_token_offset, tos, "-"),
            format_unwrap_or!(props.max_indirect_sequence_count, tos, "-"),
            format_unwrap_or!(props.max_inline_uniform_block_size, tos, "-"),
            format_unwrap_or!(props.max_instance_count, tos, "-"),
            format_unwrap_or!(props.max_interpolation_offset, tos, "-"),
            format_unwrap_or!(props.max_memory_allocation_count, tos, "-"),
            format_unwrap_or!(props.max_memory_allocation_size, tos, "-"),
            format_unwrap_or!(props.max_mesh_multiview_view_count, tos, "-"),
            format_unwrap_or!(props.max_mesh_output_primitives, tos, "-"),
            format_unwrap_or!(props.max_mesh_output_vertices, tos, "-"),
            format_unwrap_or!(props.max_mesh_total_memory_size, tos, "-"),
            format_unwrap_or!(props.max_mesh_work_group_invocations, tos, "-"),
            format_unwrap_or!(props.max_mesh_work_group_size, tod, "-"),
            format_unwrap_or!(props.max_multiview_instance_index, tos, "-"),
            format_unwrap_or!(props.max_multiview_view_count, tos, "-"),
            format_unwrap_or!(props.max_per_set_descriptors, tos, "-"),
            format_unwrap_or!(
                props.max_per_stage_descriptor_acceleration_structures,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_per_stage_descriptor_inline_uniform_blocks,
                tos,
                "-"
            ),
            format_unwrap_or!(props.max_per_stage_descriptor_input_attachments, tos, "-"),
            format_unwrap_or!(props.max_per_stage_descriptor_sampled_images, tos, "-"),
            format_unwrap_or!(props.max_per_stage_descriptor_samplers, tos, "-"),
            format_unwrap_or!(props.max_per_stage_descriptor_storage_buffers, tos, "-"),
            format_unwrap_or!(props.max_per_stage_descriptor_storage_images, tos, "-"),
            format_unwrap_or!(props.max_per_stage_descriptor_uniform_buffers, tos, "-"),
            format_unwrap_or!(
                props.max_per_stage_descriptor_update_after_bind_acceleration_structures,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_per_stage_descriptor_update_after_bind_input_attachments,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_per_stage_descriptor_update_after_bind_sampled_images,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_per_stage_descriptor_update_after_bind_samplers,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_per_stage_descriptor_update_after_bind_storage_buffers,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_per_stage_descriptor_update_after_bind_storage_images,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_per_stage_descriptor_update_after_bind_uniform_buffers,
                tos,
                "-"
            ),
            format_unwrap_or!(props.max_per_stage_resources, tos, "-"),
            format_unwrap_or!(props.max_per_stage_update_after_bind_resources, tos, "-"),
            format_unwrap_or!(props.max_primitive_count, tos, "-"),
            format_unwrap_or!(props.max_push_constants_size, tos, "-"),
            format_unwrap_or!(props.max_push_descriptors, tos, "-"),
            format_unwrap_or!(props.max_ray_dispatch_invocation_count, tos, "-"),
            format_unwrap_or!(props.max_ray_hit_attribute_size, tos, "-"),
            format_unwrap_or!(props.max_ray_recursion_depth, tos, "-"),
            format_unwrap_or!(props.max_recursion_depth, tos, "-"),
            format_unwrap_or!(props.max_sample_location_grid_size, tod, "-"),
            format_unwrap_or!(props.max_sample_mask_words, tos, "-"),
            format_unwrap_or!(props.max_sampler_allocation_count, tos, "-"),
            format_unwrap_or!(props.max_sampler_anisotropy, tos, "-"),
            format_unwrap_or!(props.max_sampler_lod_bias, tos, "-"),
            format_unwrap_or!(props.max_sgpr_allocation, tos, "-"),
            format_unwrap_or!(props.max_shader_group_stride, tos, "-"),
            format_unwrap_or!(props.max_storage_buffer_range, tos, "-"),
            format_unwrap_or!(props.max_subgroup_size, tos, "-"),
            format_unwrap_or!(props.max_subsampled_array_layers, tos, "-"),
            format_unwrap_or!(props.max_task_output_count, tos, "-"),
            format_unwrap_or!(props.max_task_total_memory_size, tos, "-"),
            format_unwrap_or!(props.max_task_work_group_invocations, tos, "-"),
            format_unwrap_or!(props.max_task_work_group_size, tod, "-"),
            format_unwrap_or!(
                props.max_tessellation_control_per_patch_output_components,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_tessellation_control_per_vertex_input_components,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_tessellation_control_per_vertex_output_components,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_tessellation_control_total_output_components,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_tessellation_evaluation_input_components,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.max_tessellation_evaluation_output_components,
                tos,
                "-"
            ),
            format_unwrap_or!(props.max_tessellation_generation_level, tos, "-"),
            format_unwrap_or!(props.max_tessellation_patch_size, tos, "-"),
            format_unwrap_or!(props.max_texel_buffer_elements, tos, "-"),
            format_unwrap_or!(props.max_texel_gather_offset, tos, "-"),
            format_unwrap_or!(props.max_texel_offset, tos, "-"),
            format_unwrap_or!(props.max_timeline_semaphore_value_difference, tos, "-"),
            format_unwrap_or!(props.max_transform_feedback_buffer_data_size, tos, "-"),
            format_unwrap_or!(props.max_transform_feedback_buffer_data_stride, tos, "-"),
            format_unwrap_or!(props.max_transform_feedback_buffer_size, tos, "-"),
            format_unwrap_or!(props.max_transform_feedback_buffers, tos, "-"),
            format_unwrap_or!(props.max_transform_feedback_stream_data_size, tos, "-"),
            format_unwrap_or!(props.max_transform_feedback_streams, tos, "-"),
            format_unwrap_or!(props.max_triangle_count, tos, "-"),
            format_unwrap_or!(props.max_uniform_buffer_range, tos, "-"),
            format_unwrap_or!(
                props.max_update_after_bind_descriptors_in_all_pools,
                tos,
                "-"
            ),
            format_unwrap_or!(props.max_vertex_attrib_divisor, tos, "-"),
            format_unwrap_or!(props.max_vertex_input_attribute_offset, tos, "-"),
            format_unwrap_or!(props.max_vertex_input_attributes, tos, "-"),
            format_unwrap_or!(props.max_vertex_input_binding_stride, tos, "-"),
            format_unwrap_or!(props.max_vertex_input_bindings, tos, "-"),
            format_unwrap_or!(props.max_vertex_output_components, tos, "-"),
            format_unwrap_or!(props.max_vgpr_allocation, tos, "-"),
            format_unwrap_or!(props.max_viewport_dimensions, tod, "-"),
            format_unwrap_or!(props.max_viewports, tos, "-"),
            format_unwrap_or!(props.mesh_output_per_primitive_granularity, tos, "-"),
            format_unwrap_or!(props.mesh_output_per_vertex_granularity, tos, "-"),
            format_unwrap_or!(
                props.min_acceleration_structure_scratch_offset_alignment,
                tos,
                "-"
            ),
            format_unwrap_or!(props.min_fragment_density_texel_size, tod, "-"),
            format_unwrap_or!(
                props.min_fragment_shading_rate_attachment_texel_size,
                tod,
                "-"
            ),
            format_unwrap_or!(props.min_imported_host_pointer_alignment, tos, "-"),
            format_unwrap_or!(
                props.min_indirect_commands_buffer_offset_alignment,
                tos,
                "-"
            ),
            format_unwrap_or!(props.min_interpolation_offset, tos, "-"),
            format_unwrap_or!(props.min_memory_map_alignment, tos, "-"),
            format_unwrap_or!(
                props.min_sequences_count_buffer_offset_alignment,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.min_sequences_index_buffer_offset_alignment,
                tos,
                "-"
            ),
            format_unwrap_or!(props.min_sgpr_allocation, tos, "-"),
            format_unwrap_or!(props.min_storage_buffer_offset_alignment, tos, "-"),
            format_unwrap_or!(props.min_subgroup_size, tos, "-"),
            format_unwrap_or!(props.min_texel_buffer_offset_alignment, tos, "-"),
            format_unwrap_or!(props.min_texel_gather_offset, tos, "-"),
            format_unwrap_or!(props.min_texel_offset, tos, "-"),
            format_unwrap_or!(props.min_uniform_buffer_offset_alignment, tos, "-"),
            format_unwrap_or!(props.min_vertex_input_binding_stride_alignment, tos, "-"),
            format_unwrap_or!(props.min_vgpr_allocation, tos, "-"),
            format_unwrap_or!(props.mipmap_precision_bits, tos, "-"),
            format_unwrap_or!(props.non_coherent_atom_size, tos, "-"),
            format_unwrap_or!(props.optimal_buffer_copy_offset_alignment, tos, "-"),
            format_unwrap_or!(props.optimal_buffer_copy_row_pitch_alignment, tos, "-"),
            format_unwrap_or!(props.pci_bus, tos, "-"),
            format_unwrap_or!(props.pci_device, tos, "-"),
            format_unwrap_or!(props.pci_domain, tos, "-"),
            format_unwrap_or!(props.per_view_position_all_components, tos, "-"),
            format_unwrap_or!(pcache_uuid, tod, "-"),
            format_unwrap_or!(props.point_clipping_behavior, point_clipping_to_str, "-"),
            format_unwrap_or!(props.point_size_granularity, tos, "-"),
            format_unwrap_or!(props.point_size_range, tod, "-"),
            format_unwrap_or!(
                props.primitive_fragment_shading_rate_with_multiple_viewports,
                tos,
                "-"
            ),
            format_unwrap_or!(props.primitive_overestimation_size, tos, "-"),
            format_unwrap_or!(props.primitive_underestimation, tos, "-"),
            format_unwrap_or!(props.protected_no_fault, tos, "-"),
            format_unwrap_or!(props.quad_divergent_implicit_lod, tos, "-"),
            format_unwrap_or!(props.quad_operations_in_all_stages, tos, "-"),
            format_unwrap_or!(
                props.required_subgroup_size_stages,
                shader_stages_to_str,
                "-"
            ),
            format_unwrap_or!(props.residency_aligned_mip_size, tos, "-"),
            format_unwrap_or!(props.residency_non_resident_strict, tos, "-"),
            format_unwrap_or!(props.residency_standard2_d_block_shape, tos, "-"),
            format_unwrap_or!(
                props.residency_standard2_d_multisample_block_shape,
                tos,
                "-"
            ),
            format_unwrap_or!(props.residency_standard3_d_block_shape, tos, "-"),
            format_unwrap_or!(props.robust_buffer_access_update_after_bind, tos, "-"),
            format_unwrap_or!(
                props.robust_storage_buffer_access_size_alignment,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.robust_uniform_buffer_access_size_alignment,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.rounding_mode_independence,
                shader_float_control_to_str,
                "-"
            ),
            format_unwrap_or!(props.sample_location_coordinate_range, tod, "-"),
            format_unwrap_or!(
                props.sample_location_sample_counts,
                sample_counts_to_str,
                "-"
            ),
            format_unwrap_or!(props.sample_location_sub_pixel_bits, tos, "-"),
            format_unwrap_or!(
                props.sampled_image_color_sample_counts,
                sample_counts_to_str,
                "-"
            ),
            format_unwrap_or!(
                props.sampled_image_depth_sample_counts,
                sample_counts_to_str,
                "-"
            ),
            format_unwrap_or!(
                props.sampled_image_integer_sample_counts,
                sample_counts_to_str,
                "-"
            ),
            format_unwrap_or!(
                props.sampled_image_stencil_sample_counts,
                sample_counts_to_str,
                "-"
            ),
            format_unwrap_or!(props.sgpr_allocation_granularity, tos, "-"),
            format_unwrap_or!(props.sgprs_per_simd, tos, "-"),
            format_unwrap_or!(props.shader_arrays_per_engine_count, tos, "-"),
            format_unwrap_or!(props.shader_core_features, tod, "-"),
            format_unwrap_or!(props.shader_denorm_flush_to_zero_float16, tos, "-"),
            format_unwrap_or!(props.shader_denorm_flush_to_zero_float32, tos, "-"),
            format_unwrap_or!(props.shader_denorm_flush_to_zero_float64, tos, "-"),
            format_unwrap_or!(props.shader_denorm_preserve_float16, tos, "-"),
            format_unwrap_or!(props.shader_denorm_preserve_float32, tos, "-"),
            format_unwrap_or!(props.shader_denorm_preserve_float64, tos, "-"),
            format_unwrap_or!(props.shader_engine_count, tos, "-"),
            format_unwrap_or!(props.shader_group_base_alignment, tos, "-"),
            format_unwrap_or!(props.shader_group_handle_alignment, tos, "-"),
            format_unwrap_or!(props.shader_group_handle_capture_replay_size, tos, "-"),
            format_unwrap_or!(props.shader_group_handle_size, tos, "-"),
            format_unwrap_or!(
                props.shader_input_attachment_array_non_uniform_indexing_native,
                tos,
                "-"
            ),
            format_unwrap_or!(props.shader_rounding_mode_rte_float16, tos, "-"),
            format_unwrap_or!(props.shader_rounding_mode_rte_float32, tos, "-"),
            format_unwrap_or!(props.shader_rounding_mode_rte_float64, tos, "-"),
            format_unwrap_or!(props.shader_rounding_mode_rtz_float16, tos, "-"),
            format_unwrap_or!(props.shader_rounding_mode_rtz_float32, tos, "-"),
            format_unwrap_or!(props.shader_rounding_mode_rtz_float64, tos, "-"),
            format_unwrap_or!(
                props.shader_sampled_image_array_non_uniform_indexing_native,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.shader_signed_zero_inf_nan_preserve_float16,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.shader_signed_zero_inf_nan_preserve_float32,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.shader_signed_zero_inf_nan_preserve_float64,
                tos,
                "-"
            ),
            format_unwrap_or!(props.shader_sm_count, tos, "-"),
            format_unwrap_or!(
                props.shader_storage_buffer_array_non_uniform_indexing_native,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.shader_storage_image_array_non_uniform_indexing_native,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.shader_uniform_buffer_array_non_uniform_indexing_native,
                tos,
                "-"
            ),
            format_unwrap_or!(props.shader_warps_per_sm, tos, "-"),
            format_unwrap_or!(props.shading_rate_max_coarse_samples, tos, "-"),
            format_unwrap_or!(props.shading_rate_palette_size, tos, "-"),
            format_unwrap_or!(props.shading_rate_texel_size, tod, "-"),
            format_unwrap_or!(props.simd_per_compute_unit, tos, "-"),
            format_unwrap_or!(props.sparse_address_space_size, tos, "-"),
            format_unwrap_or!(props.standard_sample_locations, tos, "-"),
            format_unwrap_or!(
                props.storage_image_sample_counts,
                sample_counts_to_str,
                "-"
            ),
            format_unwrap_or!(
                props.storage_texel_buffer_offset_alignment_bytes,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.storage_texel_buffer_offset_single_texel_alignment,
                tos,
                "-"
            ),
            format_unwrap_or!(props.strict_lines, tos, "-"),
            format_unwrap_or!(props.sub_pixel_interpolation_offset_bits, tos, "-"),
            format_unwrap_or!(props.sub_pixel_precision_bits, tos, "-"),
            format_unwrap_or!(props.sub_texel_precision_bits, tos, "-"),
            format_unwrap_or!(props.subgroup_quad_operations_in_all_stages, tod, "-"),
            format_unwrap_or!(props.subgroup_size, tos, "-"),
            format_unwrap_or!(props.subgroup_supported_operations, subgroup_to_str, "-"),
            format_unwrap_or!(props.subgroup_supported_stages, shader_stages_to_str, "-"),
            format_unwrap_or!(
                props.subsampled_coarse_reconstruction_early_access,
                tos,
                "-"
            ),
            format_unwrap_or!(props.subsampled_loads, tos, "-"),
            format_unwrap_or!(props.supported_depth_resolve_modes, tod, "-"),
            format_unwrap_or!(props.supported_operations, subgroup_to_str, "-"),
            format_unwrap_or!(props.supported_stages, shader_stages_to_str, "-"),
            format_unwrap_or!(props.supported_stencil_resolve_modes, tod, "-"),
            format_unwrap_or!(props.timestamp_compute_and_graphics, tos, "-"),
            format_unwrap_or!(props.timestamp_period, tos, "-"),
            format_unwrap_or!(props.transform_feedback_draw, tos, "-"),
            format_unwrap_or!(props.transform_feedback_queries, tos, "-"),
            format_unwrap_or!(
                props.transform_feedback_rasterization_stream_select,
                tos,
                "-"
            ),
            format_unwrap_or!(props.transform_feedback_streams_lines_triangles, tos, "-"),
            format_unwrap_or!(
                props.uniform_texel_buffer_offset_alignment_bytes,
                tos,
                "-"
            ),
            format_unwrap_or!(
                props.uniform_texel_buffer_offset_single_texel_alignment,
                tos,
                "-"
            ),
            format_unwrap_or!(props.variable_sample_locations, tos, "-"),
            format_unwrap_or!(props.vgpr_allocation_granularity, tos, "-"),
            format_unwrap_or!(props.vgprs_per_simd, tos, "-"),
            format_unwrap_or!(props.viewport_bounds_range, tod, "-"),
            format_unwrap_or!(props.viewport_sub_pixel_bits, tos, "-"),
            format_unwrap_or!(props.wavefront_size, tos, "-"),
            format_unwrap_or!(props.wavefronts_per_simd, tos, "-")
        ]
    }
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

fn shader_stages_to_str(val: &ShaderStages) -> String {
    let mut outs = vec![];
    if val.vertex {
        outs.push("vs")
    }
    if val.tessellation_control {
        outs.push("tcs")
    }
    if val.tessellation_evaluation {
        outs.push("tes")
    }
    if val.geometry {
        outs.push("gs")
    }
    if val.fragment {
        outs.push("fs")
    }
    if val.compute {
        outs.push("cs")
    }
    outs.join("|")
}

fn shader_float_control_to_str(val: &ShaderFloatControlsIndependence) -> String {
    match val {
        ShaderFloatControlsIndependence::Float32Only => "float32_only",
        ShaderFloatControlsIndependence::All => "all",
        ShaderFloatControlsIndependence::None => "none",
    }
    .to_string()
}

fn driver_id_to_str(val: &DriverId) -> String {
    match val {
        DriverId::AMDProprietary => "AMDProprietary",
        DriverId::AMDOpenSource => "AMDOpenSource",
        DriverId::MesaRADV => "MesaRADV",
        DriverId::NvidiaProprietary => "NvidiaProprietary",
        DriverId::IntelProprietaryWindows => "IntelProprietaryWindows",
        DriverId::IntelOpenSourceMesa => "IntelOpenSourceMesa",
        DriverId::ImaginationProprietary => "ImaginationProprietary",
        DriverId::QualcommProprietary => "QualcommProprietary",
        DriverId::ARMProprietary => "ARMProprietary",
        DriverId::GoogleSwiftshader => "GoogleSwiftshader",
        DriverId::GGPProprietary => "GGPProprietary",
        DriverId::BroadcomProprietary => "BroadcomProprietary",
        DriverId::MesaLLVMpipe => "MesaLLVMpipe",
        DriverId::MoltenVK => "MoltenVK",
    }
    .to_string()
}

fn sample_counts_to_str(val: &SampleCounts) -> String {
    let mut outs = vec![];
    if val.sample1 {
        outs.push("1")
    }
    if val.sample2 {
        outs.push("2")
    }
    if val.sample4 {
        outs.push("4")
    }
    if val.sample8 {
        outs.push("8")
    }
    if val.sample16 {
        outs.push("16")
    }
    if val.sample32 {
        outs.push("32")
    }
    if val.sample64 {
        outs.push("64")
    }
    outs.join("|")
}

fn point_clipping_to_str(val: &PointClippingBehavior) -> String {
    match val {
        PointClippingBehavior::AllClipPlanes => "AllClipPlanes",
        PointClippingBehavior::UserClipPlanesOnly => "UserClipPlanesOnly",
    }
    .to_string()
}

fn subgroup_to_str(val: &SubgroupFeatures) -> String {
    let mut outs = vec![];

    if val.basic {
        outs.push("basic")
    }
    if val.vote {
        outs.push("vote")
    }
    if val.arithmetic {
        outs.push("arithmetic")
    }
    if val.ballot {
        outs.push("ballot")
    }
    if val.shuffle {
        outs.push("shuffle")
    }
    if val.shuffle_relative {
        outs.push("shuffle_relative")
    }
    if val.clustered {
        outs.push("clustered")
    }
    if val.quad {
        outs.push("quad")
    }
    outs.join("|")
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

#[inline]
pub fn tos<T: fmt::Display>(val: T) -> String {
    val.to_string()
}

#[inline]
fn tod<T: fmt::Debug>(val: T) -> String {
    format!("{:?}", val)
}

pub fn transpose(mut table: Table) -> Table {
    let format = table.get_format().clone();

    let mut rows: Vec<Row> = vec![];
    for row in table.into_iter() {
        for (i, cell) in row.into_iter().enumerate() {
            if i < rows.len() {
                rows[i].add_cell(cell.clone())
            } else {
                rows.push(Row::new(vec![cell.clone()]));
            }
        }
    }

    let mut transpose = Table::init(rows);
    transpose.set_format(format);

    transpose
}
