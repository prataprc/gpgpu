use prettytable::{cell, row};
use vulkano::{
    format::Format,
    image::{ImageFormatProperties, ImageTiling, ImageType, ImageUsage},
    instance::PhysicalDevice,
    swapchain::{
        Capabilities, SupportedCompositeAlpha, SupportedPresentModes,
        SupportedSurfaceTransforms,
    },
};

use vgi::{make_list, pp::PrettyRow};

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

#[derive(Debug)]
pub struct ChecklistItem {
    pub(super) name: String,
    pub(super) supported: bool,
}

impl PrettyRow for ChecklistItem {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Name", "Supported"]
    }

    fn to_row(&self) -> prettytable::Row {
        match self.supported {
            true => row![self.name, Fg -> "✓"],
            false => row![self.name, Fr -> "✗"],
        }
    }
}

macro_rules! make_check_list {
    ($(($val:ident, $field:ident),)*) => (
        vec![
            $(
                ChecklistItem {
                    name: stringify!($field).to_string(),
                    supported: $val.$field,
                },
            )*
        ]
    );
}

pub fn device_features<'a>(pd: PhysicalDevice<'a>) -> Vec<ChecklistItem> {
    let f = pd.supported_features();

    make_check_list![
        (f, acceleration_structure),
        (f, acceleration_structure_capture_replay),
        (f, acceleration_structure_host_commands),
        (f, acceleration_structure_indirect_build),
        (f, advanced_blend_coherent_operations),
        (f, alpha_to_one),
        (f, attachment_fragment_shading_rate),
        (f, bresenham_lines),
        (f, buffer_device_address),
        (f, buffer_device_address_capture_replay),
        (f, buffer_device_address_multi_device),
        (f, compute_derivative_group_linear),
        (f, compute_derivative_group_quads),
        (f, compute_full_subgroups),
        (f, conditional_rendering),
        (f, constant_alpha_color_blend_factors),
        (f, cooperative_matrix),
        (f, cooperative_matrix_robust_buffer_access),
        (f, corner_sampled_image),
        (f, coverage_reduction_mode),
        (f, custom_border_color_without_format),
        (f, custom_border_colors),
        (f, decode_mode_shared_exponent),
        (f, dedicated_allocation_image_aliasing),
        (f, depth_bias_clamp),
        (f, depth_bounds),
        (f, depth_clamp),
        (f, depth_clip_enable),
        (
            f,
            descriptor_binding_acceleration_structure_update_after_bind
        ),
        (f, descriptor_binding_inline_uniform_block_update_after_bind),
        (f, descriptor_binding_partially_bound),
        (f, descriptor_binding_sampled_image_update_after_bind),
        (f, descriptor_binding_storage_buffer_update_after_bind),
        (f, descriptor_binding_storage_image_update_after_bind),
        (f, descriptor_binding_storage_texel_buffer_update_after_bind),
        (f, descriptor_binding_uniform_buffer_update_after_bind),
        (f, descriptor_binding_uniform_texel_buffer_update_after_bind),
        (f, descriptor_binding_update_unused_while_pending),
        (f, descriptor_binding_variable_descriptor_count),
        (f, descriptor_indexing),
        (f, device_coherent_memory),
        (f, device_generated_commands),
        (f, device_memory_report),
        (f, diagnostics_config),
        (f, draw_indirect_count),
        (f, draw_indirect_first_instance),
        (f, dual_src_blend),
        (f, events),
        (f, exclusive_scissor),
        (f, extended_dynamic_state),
        (f, fill_mode_non_solid),
        (f, format_a4b4g4r4),
        (f, format_a4r4g4b4),
        (f, fragment_density_map),
        (f, fragment_density_map_deferred),
        (f, fragment_density_map_dynamic),
        (f, fragment_density_map_non_subsampled_images),
        (f, fragment_shader_barycentric),
        (f, fragment_shader_pixel_interlock),
        (f, fragment_shader_sample_interlock),
        (f, fragment_shader_shading_rate_interlock),
        (f, fragment_shading_rate_enums),
        (f, fragment_stores_and_atomics),
        (f, full_draw_index_uint32),
        (f, geometry_shader),
        (f, geometry_streams),
        (f, host_query_reset),
        (f, image_cube_array),
        (f, image_footprint),
        (f, image_view2_d_on3_d_image),
        (f, image_view_format_reinterpretation),
        (f, image_view_format_swizzle),
        (f, imageless_framebuffer),
        (f, independent_blend),
        (f, index_type_uint8),
        (f, inherited_conditional_rendering),
        (f, inherited_queries),
        (f, inline_uniform_block),
        (f, large_points),
        (f, logic_op),
        (f, memory_priority),
        (f, mesh_shader),
        (f, multi_draw_indirect),
        (f, multi_viewport),
        (f, multisample_array_image),
        (f, multiview),
        (f, multiview_geometry_shader),
        (f, multiview_tessellation_shader),
        (f, mutable_comparison_samplers),
        (f, mutable_descriptor_type),
        (f, no_invocation_fragment_shading_rates),
        (f, null_descriptor),
        (f, occlusion_query_precise),
        (f, performance_counter_multiple_query_pools),
        (f, performance_counter_query_pools),
        (f, pipeline_creation_cache_control),
        (f, pipeline_executable_info),
        (f, pipeline_fragment_shading_rate),
        (f, pipeline_statistics_query),
        (f, point_polygons),
        (f, primitive_fragment_shading_rate),
        (f, private_data),
        (f, protected_memory),
        (f, ray_query),
        (f, ray_tracing_pipeline),
        (f, ray_tracing_pipeline_shader_group_handle_capture_replay),
        (
            f,
            ray_tracing_pipeline_shader_group_handle_capture_replay_mixed
        ),
        (f, ray_tracing_pipeline_trace_rays_indirect),
        (f, ray_traversal_primitive_culling),
        (f, rectangular_lines),
        (f, representative_fragment_test),
        (f, robust_buffer_access),
        (f, robust_buffer_access2),
        (f, robust_image_access),
        (f, robust_image_access2),
        (f, runtime_descriptor_array),
        (f, sample_rate_shading),
        (f, sampler_anisotropy),
        (f, sampler_filter_minmax),
        (f, sampler_mip_lod_bias),
        (f, sampler_mirror_clamp_to_edge),
        (f, sampler_ycbcr_conversion),
        (f, scalar_block_layout),
        (f, separate_depth_stencil_layouts),
        (f, separate_stencil_mask_ref),
        (f, shader_buffer_float32_atomic_add),
        (f, shader_buffer_float32_atomics),
        (f, shader_buffer_float64_atomic_add),
        (f, shader_buffer_float64_atomics),
        (f, shader_buffer_int64_atomics),
        (f, shader_clip_distance),
        (f, shader_cull_distance),
        (f, shader_demote_to_helper_invocation),
        (f, shader_device_clock),
        (f, shader_draw_parameters),
        (f, shader_float16),
        (f, shader_float64),
        (f, shader_image_float32_atomic_add),
        (f, shader_image_float32_atomics),
        (f, shader_image_gather_extended),
        (f, shader_image_int64_atomics),
        (f, shader_input_attachment_array_dynamic_indexing),
        (f, shader_input_attachment_array_non_uniform_indexing),
        (f, shader_int16),
        (f, shader_int64),
        (f, shader_int8),
        (f, shader_integer_functions2),
        (f, shader_output_layer),
        (f, shader_output_viewport_index),
        (f, shader_resource_min_lod),
        (f, shader_resource_residency),
        (f, shader_sample_rate_interpolation_functions),
        (f, shader_sampled_image_array_dynamic_indexing),
        (f, shader_sampled_image_array_non_uniform_indexing),
        (f, shader_shared_float32_atomic_add),
        (f, shader_shared_float32_atomics),
        (f, shader_shared_float64_atomic_add),
        (f, shader_shared_float64_atomics),
        (f, shader_shared_int64_atomics),
        (f, shader_sm_builtins),
        (f, shader_storage_buffer_array_dynamic_indexing),
        (f, shader_storage_buffer_array_non_uniform_indexing),
        (f, shader_storage_image_array_dynamic_indexing),
        (f, shader_storage_image_array_non_uniform_indexing),
        (f, shader_storage_image_extended_formats),
        (f, shader_storage_image_multisample),
        (f, shader_storage_image_read_without_format),
        (f, shader_storage_image_write_without_format),
        (f, shader_storage_texel_buffer_array_dynamic_indexing),
        (f, shader_storage_texel_buffer_array_non_uniform_indexing),
        (f, shader_subgroup_clock),
        (f, shader_subgroup_extended_types),
        (f, shader_terminate_invocation),
        (f, shader_tessellation_and_geometry_point_size),
        (f, shader_uniform_buffer_array_dynamic_indexing),
        (f, shader_uniform_buffer_array_non_uniform_indexing),
        (f, shader_uniform_texel_buffer_array_dynamic_indexing),
        (f, shader_uniform_texel_buffer_array_non_uniform_indexing),
        (f, shader_zero_initialize_workgroup_memory),
        (f, shading_rate_coarse_sample_order),
        (f, shading_rate_image),
        (f, smooth_lines),
        (f, sparse_binding),
        (f, sparse_image_float32_atomic_add),
        (f, sparse_image_float32_atomics),
        (f, sparse_image_int64_atomics),
        (f, sparse_residency16_samples),
        (f, sparse_residency2_samples),
        (f, sparse_residency4_samples),
        (f, sparse_residency8_samples),
        (f, sparse_residency_aliased),
        (f, sparse_residency_buffer),
        (f, sparse_residency_image2_d),
        (f, sparse_residency_image3_d),
        (f, stippled_bresenham_lines),
        (f, stippled_rectangular_lines),
        (f, stippled_smooth_lines),
        (f, storage_buffer16_bit_access),
        (f, storage_buffer8_bit_access),
        (f, storage_input_output16),
        (f, storage_push_constant16),
        (f, storage_push_constant8),
        (f, subgroup_broadcast_dynamic_id),
        (f, subgroup_size_control),
        (f, supersample_fragment_shading_rates),
        (f, task_shader),
        (f, tessellation_isolines),
        (f, tessellation_point_mode),
        (f, tessellation_shader),
        (f, texel_buffer_alignment),
        (f, texture_compression_astc_hdr),
        (f, texture_compression_astc_ldr),
        (f, texture_compression_bc),
        (f, texture_compression_etc2),
        (f, timeline_semaphore),
        (f, transform_feedback),
        (f, triangle_fans),
        (f, uniform_and_storage_buffer16_bit_access),
        (f, uniform_and_storage_buffer8_bit_access),
        (f, uniform_buffer_standard_layout),
        (f, variable_multisample_rate),
        (f, variable_pointers),
        (f, variable_pointers_storage_buffer),
        (f, vertex_attribute_access_beyond_stride),
        (f, vertex_attribute_instance_rate_divisor),
        (f, vertex_attribute_instance_rate_zero_divisor),
        (f, vertex_pipeline_stores_and_atomics),
        (f, vulkan_memory_model),
        (f, vulkan_memory_model_availability_visibility_chains),
        (f, vulkan_memory_model_device_scope),
        (f, wide_lines),
        (f, workgroup_memory_explicit_layout),
        (f, workgroup_memory_explicit_layout16_bit_access),
        (f, workgroup_memory_explicit_layout8_bit_access),
        (f, workgroup_memory_explicit_layout_scalar_block_layout),
        (f, ycbcr_image_arrays),
    ]
}

pub fn instance_extensions() -> Vec<ChecklistItem> {
    use vulkano::instance::InstanceExtensions;

    let iextns = InstanceExtensions::supported_by_core().unwrap();

    make_check_list![
        (iextns, khr_android_surface),
        (iextns, khr_device_group_creation),
        (iextns, khr_display),
        (iextns, khr_external_fence_capabilities),
        (iextns, khr_external_memory_capabilities),
        (iextns, khr_external_semaphore_capabilities),
        (iextns, khr_get_display_properties2),
        (iextns, khr_get_physical_device_properties2),
        (iextns, khr_get_surface_capabilities2),
        (iextns, khr_surface),
        (iextns, khr_surface_protected_capabilities),
        (iextns, khr_wayland_surface),
        (iextns, khr_win32_surface),
        (iextns, khr_xcb_surface),
        (iextns, khr_xlib_surface),
        (iextns, ext_acquire_xlib_display),
        (iextns, ext_debug_report),
        (iextns, ext_debug_utils),
        (iextns, ext_direct_mode_display),
        (iextns, ext_directfb_surface),
        (iextns, ext_display_surface_counter),
        (iextns, ext_headless_surface),
        (iextns, ext_metal_surface),
        (iextns, ext_swapchain_colorspace),
        (iextns, ext_validation_features),
        (iextns, ext_validation_flags),
        (iextns, fuchsia_imagepipe_surface),
        (iextns, ggp_stream_descriptor_surface),
        (iextns, mvk_ios_surface),
        (iextns, mvk_macos_surface),
        (iextns, nn_vi_surface),
        (iextns, nv_external_memory_capabilities),
    ]
}

pub fn device_extensions<'a>(pd: PhysicalDevice<'a>) -> Vec<ChecklistItem> {
    use vulkano::device::DeviceExtensions;

    let dextns = DeviceExtensions::supported_by_device(pd);

    make_check_list![
        (dextns, khr_16bit_storage),
        (dextns, khr_8bit_storage),
        (dextns, khr_acceleration_structure),
        (dextns, khr_bind_memory2),
        (dextns, khr_buffer_device_address),
        (dextns, khr_copy_commands2),
        (dextns, khr_create_renderpass2),
        (dextns, khr_dedicated_allocation),
        (dextns, khr_deferred_host_operations),
        (dextns, khr_depth_stencil_resolve),
        (dextns, khr_descriptor_update_template),
        (dextns, khr_device_group),
        (dextns, khr_display_swapchain),
        (dextns, khr_draw_indirect_count),
        (dextns, khr_driver_properties),
        (dextns, khr_external_fence),
        (dextns, khr_external_fence_fd),
        (dextns, khr_external_fence_win32),
        (dextns, khr_external_memory),
        (dextns, khr_external_memory_fd),
        (dextns, khr_external_memory_win32),
        (dextns, khr_external_semaphore),
        (dextns, khr_external_semaphore_fd),
        (dextns, khr_external_semaphore_win32),
        (dextns, khr_fragment_shading_rate),
        (dextns, khr_get_memory_requirements2),
        (dextns, khr_image_format_list),
        (dextns, khr_imageless_framebuffer),
        (dextns, khr_incremental_present),
        (dextns, khr_maintenance1),
        (dextns, khr_maintenance2),
        (dextns, khr_maintenance3),
        (dextns, khr_multiview),
        (dextns, khr_performance_query),
        (dextns, khr_pipeline_executable_properties),
        (dextns, khr_pipeline_library),
        (dextns, khr_portability_subset),
        (dextns, khr_push_descriptor),
        (dextns, khr_ray_query),
        (dextns, khr_ray_tracing_pipeline),
        (dextns, khr_relaxed_block_layout),
        (dextns, khr_sampler_mirror_clamp_to_edge),
        (dextns, khr_sampler_ycbcr_conversion),
        (dextns, khr_separate_depth_stencil_layouts),
        (dextns, khr_shader_atomic_int64),
        (dextns, khr_shader_clock),
        (dextns, khr_shader_draw_parameters),
        (dextns, khr_shader_float16_int8),
        (dextns, khr_shader_float_controls),
        (dextns, khr_shader_non_semantic_info),
        (dextns, khr_shader_subgroup_extended_types),
        (dextns, khr_shader_terminate_invocation),
        (dextns, khr_shared_presentable_image),
        (dextns, khr_spirv_1_4),
        (dextns, khr_storage_buffer_storage_class),
        (dextns, khr_swapchain),
        (dextns, khr_swapchain_mutable_format),
        (dextns, khr_timeline_semaphore),
        (dextns, khr_uniform_buffer_standard_layout),
        (dextns, khr_variable_pointers),
        (dextns, khr_vulkan_memory_model),
        (dextns, khr_win32_keyed_mutex),
        (dextns, khr_workgroup_memory_explicit_layout),
        (dextns, khr_zero_initialize_workgroup_memory),
        (dextns, ext_4444_formats),
        (dextns, ext_astc_decode_mode),
        (dextns, ext_blend_operation_advanced),
        (dextns, ext_buffer_device_address),
        (dextns, ext_calibrated_timestamps),
        (dextns, ext_conditional_rendering),
        (dextns, ext_conservative_rasterization),
        (dextns, ext_custom_border_color),
        (dextns, ext_debug_marker),
        (dextns, ext_depth_clip_enable),
        (dextns, ext_depth_range_unrestricted),
        (dextns, ext_descriptor_indexing),
        (dextns, ext_device_memory_report),
        (dextns, ext_discard_rectangles),
        (dextns, ext_display_control),
        (dextns, ext_extended_dynamic_state),
        (dextns, ext_external_memory_dma_buf),
        (dextns, ext_external_memory_host),
        (dextns, ext_filter_cubic),
        (dextns, ext_fragment_density_map),
        (dextns, ext_fragment_density_map2),
        (dextns, ext_fragment_shader_interlock),
        (dextns, ext_full_screen_exclusive),
        (dextns, ext_global_priority),
        (dextns, ext_hdr_metadata),
        (dextns, ext_host_query_reset),
        (dextns, ext_image_drm_format_modifier),
        (dextns, ext_image_robustness),
        (dextns, ext_index_type_uint8),
        (dextns, ext_inline_uniform_block),
        (dextns, ext_line_rasterization),
        (dextns, ext_memory_budget),
        (dextns, ext_memory_priority),
        (dextns, ext_pci_bus_info),
        (dextns, ext_pipeline_creation_cache_control),
        (dextns, ext_pipeline_creation_feedback),
        (dextns, ext_post_depth_coverage),
        (dextns, ext_private_data),
        (dextns, ext_queue_family_foreign),
        (dextns, ext_robustness2),
        (dextns, ext_sample_locations),
        (dextns, ext_sampler_filter_minmax),
        (dextns, ext_scalar_block_layout),
        (dextns, ext_separate_stencil_usage),
        (dextns, ext_shader_atomic_float),
        (dextns, ext_shader_demote_to_helper_invocation),
        (dextns, ext_shader_image_atomic_int64),
        (dextns, ext_shader_stencil_export),
        (dextns, ext_shader_subgroup_ballot),
        (dextns, ext_shader_subgroup_vote),
        (dextns, ext_shader_viewport_index_layer),
        (dextns, ext_subgroup_size_control),
        (dextns, ext_texel_buffer_alignment),
        (dextns, ext_texture_compression_astc_hdr),
        (dextns, ext_tooling_info),
        (dextns, ext_transform_feedback),
        (dextns, ext_validation_cache),
        (dextns, ext_vertex_attribute_divisor),
        (dextns, ext_ycbcr_image_arrays),
        (dextns, amd_buffer_marker),
        (dextns, amd_device_coherent_memory),
        (dextns, amd_display_native_hdr),
        (dextns, amd_draw_indirect_count),
        (dextns, amd_gcn_shader),
        (dextns, amd_gpu_shader_half_float),
        (dextns, amd_gpu_shader_int16),
        (dextns, amd_memory_overallocation_behavior),
        (dextns, amd_mixed_attachment_samples),
        (dextns, amd_pipeline_compiler_control),
        (dextns, amd_rasterization_order),
        (dextns, amd_shader_ballot),
        (dextns, amd_shader_core_properties),
        (dextns, amd_shader_core_properties2),
        (dextns, amd_shader_explicit_vertex_parameter),
        (dextns, amd_shader_fragment_mask),
        (dextns, amd_shader_image_load_store_lod),
        (dextns, amd_shader_info),
        (dextns, amd_shader_trinary_minmax),
        (dextns, amd_texture_gather_bias_lod),
        (dextns, android_external_memory_android_hardware_buffer),
        (dextns, ggp_frame_token),
        (dextns, google_decorate_string),
        (dextns, google_display_timing),
        (dextns, google_hlsl_functionality1),
        (dextns, google_user_type),
        (dextns, img_filter_cubic),
        (dextns, img_format_pvrtc),
        (dextns, intel_performance_query),
        (dextns, intel_shader_integer_functions2),
        (dextns, nvx_image_view_handle),
        (dextns, nvx_multiview_per_view_attributes),
        (dextns, nv_acquire_winrt_display),
        (dextns, nv_clip_space_w_scaling),
        (dextns, nv_compute_shader_derivatives),
        (dextns, nv_cooperative_matrix),
        (dextns, nv_corner_sampled_image),
        (dextns, nv_coverage_reduction_mode),
        (dextns, nv_dedicated_allocation),
        (dextns, nv_dedicated_allocation_image_aliasing),
        (dextns, nv_device_diagnostic_checkpoints),
        (dextns, nv_device_diagnostics_config),
        (dextns, nv_device_generated_commands),
        (dextns, nv_external_memory),
        (dextns, nv_external_memory_win32),
        (dextns, nv_fill_rectangle),
        (dextns, nv_fragment_coverage_to_color),
        (dextns, nv_fragment_shader_barycentric),
        (dextns, nv_fragment_shading_rate_enums),
        (dextns, nv_framebuffer_mixed_samples),
        (dextns, nv_geometry_shader_passthrough),
        (dextns, nv_glsl_shader),
        (dextns, nv_mesh_shader),
        (dextns, nv_ray_tracing),
        (dextns, nv_representative_fragment_test),
        (dextns, nv_sample_mask_override_coverage),
        (dextns, nv_scissor_exclusive),
        (dextns, nv_shader_image_footprint),
        (dextns, nv_shader_sm_builtins),
        (dextns, nv_shader_subgroup_partitioned),
        (dextns, nv_shading_rate_image),
        (dextns, nv_viewport_array2),
        (dextns, nv_viewport_swizzle),
        (dextns, nv_win32_keyed_mutex),
        (dextns, qcom_render_pass_shader_resolve),
        (dextns, qcom_render_pass_store_ops),
        (dextns, qcom_render_pass_transform),
        (dextns, qcom_rotated_copy_commands),
        (dextns, valve_mutable_descriptor_type),
    ]
}

pub fn surface_capabilities(c: Capabilities) -> Vec<LimitItem> {
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
