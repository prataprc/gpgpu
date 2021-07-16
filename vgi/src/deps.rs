// TODO: may be this is not required.

#[macro_export]
macro_rules! instance_extensions {
    ($(($field:ident, $name:expr),)*) => {{
        fn instance_extension_names(extn: InstanceExtensions) -> Vec<String> {
            let mut names: Vec<String> = vec![];
            $(
                if extn.$field {
                    names.push($name.to_string());
                }
            )*
            names
        }

        fn to_instance_extension(names: Vec<String>) -> InstanceExtensions {
            let def = InstanceExtensions::none();
            let mut extn = InstanceExtensions::none();
            for name in names.into_iter() {
                extn = match name {
                    $( $name => InstanceExtensions { $field: true, ..extn }, )*
                    name => {
                        panic!("non-exhaustive pattern for instance-extension {}", name)
                    }
                };
            }
        }
    }};
}

#[macro_export]
macro_rules! device_features {
    ($($field:ident,)*) => {{
        fn device_feature_names(features: Features) -> Vec<String> {
            let mut names: Vec<String> = vec![];
            $(
                if features.$field {
                    names.push(stringify!($field));
                }
            )*
            names
        }

        fn to_device_features(names: Vec<String>) -> Features {
            let def = Features::none();
            let mut features = Features::none();
            for name in names.into_iter() {
                features = match name {
                    $( stringify!($field) => Features { $field: true, ..features }, )*
                    name => panic!("non-exhaustive pattern for feature {}", name),
                };
            }
        }
    }};
}

pub fn dependency(
    mut iextens: InstanceExtensions,
    dextens: DeviceExtensions,
    mut features: Features,
) -> Result<(InstanceExtensions, DeviceExtensions, Features)> {
    use crate::api_version;

    let mut version = api_version()?;

    // check for instance extension dependency with other instance extensions and
    // device features

    let mut extensions: Vec<Extension> =
        get_registry_variant!(reg, Extensions, Extension)
            .into_iter()
            .map(|cc| cc.children)
            .flatten()
            .filter(|e| e.ext_type == Some("instance".to_string()))
            .collect();

    for extn in extensions.iter() {
        let extns = extn.requires.map(|e| e.split(",")).map(ToString::to_string)
            .unwrap_or(vec![]);
        iextens = iextens.union(to_instance_extension(extns));
        match &extn.requires_core {
            Some(rver) if rver > version => {
                err_at!(Vk, msg: "instance extension {} need version {}", extn.name, rver)?
            }
            None => (),
        }
        for child in extn.children().iter() {
            match child {
                ExtensionChild::Require { feature, extension, .. } => {
                    if let Some(extension) = extension.is_some() {
                        iextens = iextens.union(to_instance_extension(vec![extension]));
                    }
                    if let Some(featr) = feature {
                        let f = to_device_features(vec![featr]).intersection(features);
                        if f != feature {
                            err_at!(Vk, msg: "need {} for {}", feature, extn.name)?
                        }
                    };
                }
                ExtensionChild::Remove { .. } => (),
            }
        }
    }
}

extension_names![
    (ext_acquire_xlib_display, "VK_EXT_acquire_xlib_display"),
    (ext_debug_report, "VK_EXT_debug_report"),
    (ext_debug_utils, "VK_EXT_debug_utils"),
    (ext_direct_mode_display, "VK_EXT_direct_mode_display"),
    (ext_directfb_surface, "VK_EXT_directfb_surface"),
    (
        ext_display_surface_counter,
        "VK_EXT_display_surface_counter"
    ),
    (ext_headless_surface, "VK_EXT_headless_surface"),
    (ext_metal_surface, "VK_EXT_metal_surface"),
    (ext_swapchain_colorspace, "VK_EXT_swapchain_colorspace"),
    (ext_validation_features, "VK_EXT_validation_features"),
    (ext_validation_flags, "VK_EXT_validation_flags"),
    (khr_android_surface, "VK_KHR_android_surface"),
    (khr_device_group_creation, "VK_KHR_device_group_creation"),
    (khr_display, "VK_KHR_display"),
    (
        khr_external_fence_capabilities,
        "VK_KHR_external_fence_capabilities"
    ),
    (
        khr_external_memory_capabilities,
        "VK_KHR_external_memory_capabilities"
    ),
    (
        khr_external_semaphore_capabilities,
        "VK_KHR_external_semaphore_capabilities"
    ),
    (
        khr_get_display_properties2,
        "VK_KHR_get_display_properties2"
    ),
    (
        khr_get_physical_device_properties2,
        "VK_KHR_get_physical_device_properties2"
    ),
    (
        khr_get_surface_capabilities2,
        "VK_KHR_get_surface_capabilities2"
    ),
    (khr_surface, "VK_KHR_surface"),
    (
        khr_surface_protected_capabilities,
        "VK_KHR_surface_protected_capabilities"
    ),
    (khr_wayland_surface, "VK_KHR_wayland_surface"),
    (khr_win32_surface, "VK_KHR_win32_surface"),
    (khr_xcb_surface, "VK_KHR_xcb_surface"),
    (khr_xlib_surface, "VK_KHR_xlib_surface"),
    (fuchsia_imagepipe_surface, "VK_FUCHSIA_imagepipe_surface"),
    (
        ggp_stream_descriptor_surface,
        "VK_GGP_stream_descriptor_surface"
    ),
    (mvk_ios_surface, "VK_MVK_ios_surface"),
    (mvk_macos_surface, "VK_MVK_macos_surface"),
    (nn_vi_surface, "VK_NN_vi_surface"),
    (
        nv_external_memory_capabilities,
        "VK_NV_external_memory_capabilities"
    ),
];

device_features![
    acceleration_structure,
    acceleration_structure_capture_replay,
    acceleration_structure_host_commands,
    acceleration_structure_indirect_build,
    advanced_blend_coherent_operations,
    alpha_to_one,
    attachment_fragment_shading_rate,
    bresenham_lines,
    buffer_device_address,
    buffer_device_address_capture_replay,
    buffer_device_address_multi_device,
    compute_derivative_group_linear,
    compute_derivative_group_quads,
    compute_full_subgroups,
    conditional_rendering,
    constant_alpha_color_blend_factors,
    cooperative_matrix,
    cooperative_matrix_robust_buffer_access,
    corner_sampled_image,
    coverage_reduction_mode,
    custom_border_color_without_format,
    custom_border_colors,
    decode_mode_shared_exponent,
    dedicated_allocation_image_aliasing,
    depth_bias_clamp,
    depth_bounds,
    depth_clamp,
    depth_clip_enable,
    descriptor_binding_acceleration_structure_update_after_bind,
    descriptor_binding_inline_uniform_block_update_after_bind,
    descriptor_binding_partially_bound,
    descriptor_binding_sampled_image_update_after_bind,
    descriptor_binding_storage_buffer_update_after_bind,
    descriptor_binding_storage_image_update_after_bind,
    descriptor_binding_storage_texel_buffer_update_after_bind,
    descriptor_binding_uniform_buffer_update_after_bind,
    descriptor_binding_uniform_texel_buffer_update_after_bind,
    descriptor_binding_update_unused_while_pending,
    descriptor_binding_variable_descriptor_count,
    descriptor_indexing,
    device_coherent_memory,
    device_generated_commands,
    device_memory_report,
    diagnostics_config,
    draw_indirect_count,
    draw_indirect_first_instance,
    dual_src_blend,
    events,
    exclusive_scissor,
    extended_dynamic_state,
    fill_mode_non_solid,
    format_a4b4g4r4,
    format_a4r4g4b4,
    fragment_density_map,
    fragment_density_map_deferred,
    fragment_density_map_dynamic,
    fragment_density_map_non_subsampled_images,
    fragment_shader_barycentric,
    fragment_shader_pixel_interlock,
    fragment_shader_sample_interlock,
    fragment_shader_shading_rate_interlock,
    fragment_shading_rate_enums,
    fragment_stores_and_atomics,
    full_draw_index_uint32,
    geometry_shader,
    geometry_streams,
    host_query_reset,
    image_cube_array,
    image_footprint,
    image_view2_d_on3_d_image,
    image_view_format_reinterpretation,
    image_view_format_swizzle,
    imageless_framebuffer,
    independent_blend,
    index_type_uint8,
    inherited_conditional_rendering,
    inherited_queries,
    inline_uniform_block,
    large_points,
    logic_op,
    memory_priority,
    mesh_shader,
    multi_draw_indirect,
    multi_viewport,
    multisample_array_image,
    multiview,
    multiview_geometry_shader,
    multiview_tessellation_shader,
    mutable_comparison_samplers,
    mutable_descriptor_type,
    no_invocation_fragment_shading_rates,
    null_descriptor,
    occlusion_query_precise,
    performance_counter_multiple_query_pools,
    performance_counter_query_pools,
    pipeline_creation_cache_control,
    pipeline_executable_info,
    pipeline_fragment_shading_rate,
    pipeline_statistics_query,
    point_polygons,
    primitive_fragment_shading_rate,
    private_data,
    protected_memory,
    ray_query,
    ray_tracing_pipeline,
    ray_tracing_pipeline_shader_group_handle_capture_replay,
    ray_tracing_pipeline_shader_group_handle_capture_replay_mixed,
    ray_tracing_pipeline_trace_rays_indirect,
    ray_traversal_primitive_culling,
    rectangular_lines,
    representative_fragment_test,
    robust_buffer_access,
    robust_buffer_access2,
    robust_image_access,
    robust_image_access2,
    runtime_descriptor_array,
    sample_rate_shading,
    sampler_anisotropy,
    sampler_filter_minmax,
    sampler_mip_lod_bias,
    sampler_mirror_clamp_to_edge,
    sampler_ycbcr_conversion,
    scalar_block_layout,
    separate_depth_stencil_layouts,
    separate_stencil_mask_ref,
    shader_buffer_float32_atomic_add,
    shader_buffer_float32_atomics,
    shader_buffer_float64_atomic_add,
    shader_buffer_float64_atomics,
    shader_buffer_int64_atomics,
    shader_clip_distance,
    shader_cull_distance,
    shader_demote_to_helper_invocation,
    shader_device_clock,
    shader_draw_parameters,
    shader_float16,
    shader_float64,
    shader_image_float32_atomic_add,
    shader_image_float32_atomics,
    shader_image_gather_extended,
    shader_image_int64_atomics,
    shader_input_attachment_array_dynamic_indexing,
    shader_input_attachment_array_non_uniform_indexing,
    shader_int16,
    shader_int64,
    shader_int8,
    shader_integer_functions2,
    shader_output_layer,
    shader_output_viewport_index,
    shader_resource_min_lod,
    shader_resource_residency,
    shader_sample_rate_interpolation_functions,
    shader_sampled_image_array_dynamic_indexing,
    shader_sampled_image_array_non_uniform_indexing,
    shader_shared_float32_atomic_add,
    shader_shared_float32_atomics,
    shader_shared_float64_atomic_add,
    shader_shared_float64_atomics,
    shader_shared_int64_atomics,
    shader_sm_builtins,
    shader_storage_buffer_array_dynamic_indexing,
    shader_storage_buffer_array_non_uniform_indexing,
    shader_storage_image_array_dynamic_indexing,
    shader_storage_image_array_non_uniform_indexing,
    shader_storage_image_extended_formats,
    shader_storage_image_multisample,
    shader_storage_image_read_without_format,
    shader_storage_image_write_without_format,
    shader_storage_texel_buffer_array_dynamic_indexing,
    shader_storage_texel_buffer_array_non_uniform_indexing,
    shader_subgroup_clock,
    shader_subgroup_extended_types,
    shader_terminate_invocation,
    shader_tessellation_and_geometry_point_size,
    shader_uniform_buffer_array_dynamic_indexing,
    shader_uniform_buffer_array_non_uniform_indexing,
    shader_uniform_texel_buffer_array_dynamic_indexing,
    shader_uniform_texel_buffer_array_non_uniform_indexing,
    shader_zero_initialize_workgroup_memory,
    shading_rate_coarse_sample_order,
    shading_rate_image,
    smooth_lines,
    sparse_binding,
    sparse_image_float32_atomic_add,
    sparse_image_float32_atomics,
    sparse_image_int64_atomics,
    sparse_residency16_samples,
    sparse_residency2_samples,
    sparse_residency4_samples,
    sparse_residency8_samples,
    sparse_residency_aliased,
    sparse_residency_buffer,
    sparse_residency_image2_d,
    sparse_residency_image3_d,
    stippled_bresenham_lines,
    stippled_rectangular_lines,
    stippled_smooth_lines,
    storage_buffer16_bit_access,
    storage_buffer8_bit_access,
    storage_input_output16,
    storage_push_constant16,
    storage_push_constant8,
    subgroup_broadcast_dynamic_id,
    subgroup_size_control,
    supersample_fragment_shading_rates,
    task_shader,
    tessellation_isolines,
    tessellation_point_mode,
    tessellation_shader,
    texel_buffer_alignment,
    texture_compression_astc_hdr,
    texture_compression_astc_ldr,
    texture_compression_bc,
    texture_compression_etc2,
    timeline_semaphore,
    transform_feedback,
    triangle_fans,
    uniform_and_storage_buffer16_bit_access,
    uniform_and_storage_buffer8_bit_access,
    uniform_buffer_standard_layout,
    variable_multisample_rate,
    variable_pointers,
    variable_pointers_storage_buffer,
    vertex_attribute_access_beyond_stride,
    vertex_attribute_instance_rate_divisor,
    vertex_attribute_instance_rate_zero_divisor,
    vertex_pipeline_stores_and_atomics,
    vulkan_memory_model,
    vulkan_memory_model_availability_visibility_chains,
    vulkan_memory_model_device_scope,
    wide_lines,
    workgroup_memory_explicit_layout,
    workgroup_memory_explicit_layout16_bit_access,
    workgroup_memory_explicit_layout8_bit_access,
    workgroup_memory_explicit_layout_scalar_block_layout,
    ycbcr_image_arrays
]
