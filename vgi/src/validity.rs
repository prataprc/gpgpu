use vulkano::device::{DeviceExtensions, Features, Properties};

use crate::{Builder, Error, Result};

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

// TODO: split this into properties, limits and more...
pub fn confirm_properties(val: &Builder, props: Properties) -> Result<()> {
    let p = val.to_properties().clone();

    if let Some(_val) = p.active_compute_unit_count {
        todo!()
    }
    if let Some(_val) = p.advanced_blend_all_operations {
        todo!()
    }
    if let Some(_val) = p.advanced_blend_correlated_overlap {
        todo!()
    }
    if let Some(_val) = p.advanced_blend_independent_blend {
        todo!()
    }
    if let Some(_val) = p.advanced_blend_max_color_attachments {
        todo!()
    }
    if let Some(_val) = p.advanced_blend_non_premultiplied_dst_color {
        todo!()
    }
    if let Some(_val) = p.advanced_blend_non_premultiplied_src_color {
        todo!()
    }
    if let Some(_val) = p.allow_command_buffer_query_copies {
        todo!()
    }
    if let Some(val) = p.api_version {
        if props.api_version.unwrap().lt(&val) {
            err_at!(Vk, msg: "api_version: {}", props.api_version.unwrap())?;
        }
    }
    if let Some(_val) = p.buffer_image_granularity {
        todo!()
    }
    if let Some(_val) = p.compute_units_per_shader_array {
        todo!()
    }
    if let Some(_val) = p.conformance_version {
        todo!()
    }
    if let Some(_val) = p.conservative_point_and_line_rasterization {
        todo!()
    }
    if let Some(_val) = p.conservative_rasterization_post_depth_coverage {
        todo!()
    }
    if let Some(_val) = p.cooperative_matrix_supported_stages {
        todo!()
    }
    if let Some(_val) = p.degenerate_lines_rasterized {
        todo!()
    }
    if let Some(_val) = p.degenerate_triangles_rasterized {
        todo!()
    }
    if let Some(_val) = p.denorm_behavior_independence {
        todo!()
    }
    if let Some(_val) = p.device_id {
        todo!()
    }
    if let Some(_val) = p.device_luid {
        todo!()
    }
    if let Some(_val) = p.device_luid_valid {
        todo!()
    }
    if let Some(_val) = p.device_name {
        todo!()
    }
    if let Some(_val) = p.device_node_mask {
        todo!()
    }
    if let Some(_val) = p.device_type {
        todo!()
    }
    if let Some(_val) = p.device_uuid {
        todo!()
    }
    if let Some(_val) = p.discrete_queue_priorities {
        todo!()
    }
    if let Some(_val) = p.driver_id {
        todo!()
    }
    if let Some(_val) = p.driver_info {
        todo!()
    }
    if let Some(_val) = p.driver_name {
        todo!()
    }
    if let Some(_val) = p.driver_uuid {
        todo!()
    }
    if let Some(_val) = p.driver_version {
        todo!()
    }
    if let Some(_val) = p.extra_primitive_overestimation_size_granularity {
        todo!()
    }
    if let Some(_val) = p.filter_minmax_image_component_mapping {
        todo!()
    }
    if let Some(_val) = p.filter_minmax_single_component_formats {
        todo!()
    }
    if let Some(_val) = p.fragment_density_invocations {
        todo!()
    }
    if let Some(_val) = p.fragment_shading_rate_non_trivial_combiner_ops {
        todo!()
    }
    if let Some(_val) = p.fragment_shading_rate_strict_multiply_combiner {
        todo!()
    }
    if let Some(_val) = p.fragment_shading_rate_with_conservative_rasterization {
        todo!()
    }
    if let Some(_val) = p.fragment_shading_rate_with_custom_sample_locations {
        todo!()
    }
    if let Some(_val) = p.fragment_shading_rate_with_fragment_shader_interlock {
        todo!()
    }
    if let Some(_val) = p.fragment_shading_rate_with_sample_mask {
        todo!()
    }
    if let Some(_val) = p.fragment_shading_rate_with_shader_depth_stencil_writes {
        todo!()
    }
    if let Some(_val) = p.fragment_shading_rate_with_shader_sample_mask {
        todo!()
    }
    if let Some(_val) = p.framebuffer_color_sample_counts {
        todo!()
    }
    if let Some(_val) = p.framebuffer_depth_sample_counts {
        todo!()
    }
    if let Some(_val) = p.framebuffer_integer_color_sample_counts {
        todo!()
    }
    if let Some(_val) = p.framebuffer_no_attachments_sample_counts {
        todo!()
    }
    if let Some(_val) = p.framebuffer_stencil_sample_counts {
        todo!()
    }
    if let Some(_val) = p.fully_covered_fragment_shader_input_variable {
        todo!()
    }
    if let Some(_val) = p.independent_resolve {
        todo!()
    }
    if let Some(_val) = p.independent_resolve_none {
        todo!()
    }
    if let Some(_val) = p.layered_shading_rate_attachments {
        todo!()
    }
    if let Some(_val) = p.line_sub_pixel_precision_bits {
        todo!()
    }
    if let Some(_val) = p.line_width_granularity {
        todo!()
    }
    if let Some(_val) = p.line_width_range {
        todo!()
    }
    if let Some(_val) = p.max_bound_descriptor_sets {
        todo!()
    }
    if let Some(_val) = p.max_clip_distances {
        todo!()
    }
    if let Some(_val) = p.max_color_attachments {
        todo!()
    }
    if let Some(_val) = p.max_combined_clip_and_cull_distances {
        todo!()
    }
    if let Some(_val) = p.max_compute_shared_memory_size {
        todo!()
    }
    if let Some(_val) = p.max_compute_work_group_count {
        todo!()
    }
    if let Some(_val) = p.max_compute_work_group_invocations {
        todo!()
    }
    if let Some(_val) = p.max_compute_work_group_size {
        todo!()
    }
    if let Some(_val) = p.max_compute_workgroup_subgroups {
        todo!()
    }
    if let Some(_val) = p.max_cull_distances {
        todo!()
    }
    if let Some(_val) = p.max_custom_border_color_samplers {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_acceleration_structures {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_inline_uniform_blocks {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_input_attachments {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_sampled_images {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_samplers {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_storage_buffers {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_storage_buffers_dynamic {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_storage_images {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_subsampled_samplers {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_uniform_buffers {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_uniform_buffers_dynamic {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_update_after_bind_acceleration_structures {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_update_after_bind_inline_uniform_blocks {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_update_after_bind_input_attachments {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_update_after_bind_sampled_images {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_update_after_bind_samplers {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_update_after_bind_storage_buffers {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_update_after_bind_storage_buffers_dynamic {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_update_after_bind_storage_images {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_update_after_bind_uniform_buffers {
        todo!()
    }
    if let Some(_val) = p.max_descriptor_set_update_after_bind_uniform_buffers_dynamic {
        todo!()
    }
    if let Some(_val) = p.max_discard_rectangles {
        todo!()
    }
    if let Some(_val) = p.max_draw_indexed_index_value {
        todo!()
    }
    if let Some(_val) = p.max_draw_indirect_count {
        todo!()
    }
    if let Some(_val) = p.max_draw_mesh_tasks_count {
        todo!()
    }
    if let Some(_val) = p.max_extra_primitive_overestimation_size {
        todo!()
    }
    if let Some(_val) = p.max_fragment_combined_output_resources {
        todo!()
    }
    if let Some(_val) = p.max_fragment_density_texel_size {
        todo!()
    }
    if let Some(_val) = p.max_fragment_dual_src_attachments {
        todo!()
    }
    if let Some(_val) = p.max_fragment_input_components {
        todo!()
    }
    if let Some(_val) = p.max_fragment_output_attachments {
        todo!()
    }
    if let Some(_val) = p.max_fragment_shading_rate_attachment_texel_size {
        todo!()
    }
    if let Some(_val) = p.max_fragment_shading_rate_attachment_texel_size_aspect_ratio {
        todo!()
    }
    if let Some(_val) = p.max_fragment_shading_rate_coverage_samples {
        todo!()
    }
    if let Some(_val) = p.max_fragment_shading_rate_invocation_count {
        todo!()
    }
    if let Some(_val) = p.max_fragment_shading_rate_rasterization_samples {
        todo!()
    }
    if let Some(_val) = p.max_fragment_size {
        todo!()
    }
    if let Some(_val) = p.max_fragment_size_aspect_ratio {
        todo!()
    }
    if let Some(_val) = p.max_framebuffer_height {
        todo!()
    }
    if let Some(_val) = p.max_framebuffer_layers {
        todo!()
    }
    if let Some(_val) = p.max_framebuffer_width {
        todo!()
    }
    if let Some(_val) = p.max_geometry_count {
        todo!()
    }
    if let Some(_val) = p.max_geometry_input_components {
        todo!()
    }
    if let Some(_val) = p.max_geometry_output_components {
        todo!()
    }
    if let Some(_val) = p.max_geometry_output_vertices {
        todo!()
    }
    if let Some(_val) = p.max_geometry_shader_invocations {
        todo!()
    }
    if let Some(_val) = p.max_geometry_total_output_components {
        todo!()
    }
    if let Some(_val) = p.max_graphics_shader_group_count {
        todo!()
    }
    if let Some(_val) = p.max_image_array_layers {
        todo!()
    }
    if let Some(_val) = p.max_image_dimension1_d {
        todo!()
    }
    if let Some(_val) = p.max_image_dimension2_d {
        todo!()
    }
    if let Some(_val) = p.max_image_dimension3_d {
        todo!()
    }
    if let Some(_val) = p.max_image_dimension_cube {
        todo!()
    }
    if let Some(_val) = p.max_indirect_commands_stream_count {
        todo!()
    }
    if let Some(_val) = p.max_indirect_commands_stream_stride {
        todo!()
    }
    if let Some(_val) = p.max_indirect_commands_token_count {
        todo!()
    }
    if let Some(_val) = p.max_indirect_commands_token_offset {
        todo!()
    }
    if let Some(_val) = p.max_indirect_sequence_count {
        todo!()
    }
    if let Some(_val) = p.max_inline_uniform_block_size {
        todo!()
    }
    if let Some(_val) = p.max_instance_count {
        todo!()
    }
    if let Some(_val) = p.max_interpolation_offset {
        todo!()
    }
    if let Some(_val) = p.max_memory_allocation_count {
        todo!()
    }
    if let Some(_val) = p.max_memory_allocation_size {
        todo!()
    }
    if let Some(_val) = p.max_mesh_multiview_view_count {
        todo!()
    }
    if let Some(_val) = p.max_mesh_output_primitives {
        todo!()
    }
    if let Some(_val) = p.max_mesh_output_vertices {
        todo!()
    }
    if let Some(_val) = p.max_mesh_total_memory_size {
        todo!()
    }
    if let Some(_val) = p.max_mesh_work_group_invocations {
        todo!()
    }
    if let Some(_val) = p.max_mesh_work_group_size {
        todo!()
    }
    if let Some(_val) = p.max_multiview_instance_index {
        todo!()
    }
    if let Some(_val) = p.max_multiview_view_count {
        todo!()
    }
    if let Some(_val) = p.max_per_set_descriptors {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_acceleration_structures {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_inline_uniform_blocks {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_input_attachments {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_sampled_images {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_samplers {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_storage_buffers {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_storage_images {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_uniform_buffers {
        todo!()
    }
    if let Some(_val) =
        p.max_per_stage_descriptor_update_after_bind_acceleration_structures
    {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
    {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_update_after_bind_input_attachments {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_update_after_bind_sampled_images {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_update_after_bind_samplers {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_update_after_bind_storage_buffers {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_update_after_bind_storage_images {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_descriptor_update_after_bind_uniform_buffers {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_resources {
        todo!()
    }
    if let Some(_val) = p.max_per_stage_update_after_bind_resources {
        todo!()
    }
    if let Some(_val) = p.max_primitive_count {
        todo!()
    }
    if let Some(_val) = p.max_push_constants_size {
        todo!()
    }
    if let Some(_val) = p.max_push_descriptors {
        todo!()
    }
    if let Some(_val) = p.max_ray_dispatch_invocation_count {
        todo!()
    }
    if let Some(_val) = p.max_ray_hit_attribute_size {
        todo!()
    }
    if let Some(_val) = p.max_ray_recursion_depth {
        todo!()
    }
    if let Some(_val) = p.max_recursion_depth {
        todo!()
    }
    if let Some(_val) = p.max_sample_location_grid_size {
        todo!()
    }
    if let Some(_val) = p.max_sample_mask_words {
        todo!()
    }
    if let Some(_val) = p.max_sampler_allocation_count {
        todo!()
    }
    if let Some(_val) = p.max_sampler_anisotropy {
        todo!()
    }
    if let Some(_val) = p.max_sampler_lod_bias {
        todo!()
    }
    if let Some(_val) = p.max_sgpr_allocation {
        todo!()
    }
    if let Some(_val) = p.max_shader_group_stride {
        todo!()
    }
    if let Some(_val) = p.max_storage_buffer_range {
        todo!()
    }
    if let Some(_val) = p.max_subgroup_size {
        todo!()
    }
    if let Some(_val) = p.max_subsampled_array_layers {
        todo!()
    }
    if let Some(_val) = p.max_task_output_count {
        todo!()
    }
    if let Some(_val) = p.max_task_total_memory_size {
        todo!()
    }
    if let Some(_val) = p.max_task_work_group_invocations {
        todo!()
    }
    if let Some(_val) = p.max_task_work_group_size {
        todo!()
    }
    if let Some(_val) = p.max_tessellation_control_per_patch_output_components {
        todo!()
    }
    if let Some(_val) = p.max_tessellation_control_per_vertex_input_components {
        todo!()
    }
    if let Some(_val) = p.max_tessellation_control_per_vertex_output_components {
        todo!()
    }
    if let Some(_val) = p.max_tessellation_control_total_output_components {
        todo!()
    }
    if let Some(_val) = p.max_tessellation_evaluation_input_components {
        todo!()
    }
    if let Some(_val) = p.max_tessellation_evaluation_output_components {
        todo!()
    }
    if let Some(_val) = p.max_tessellation_generation_level {
        todo!()
    }
    if let Some(_val) = p.max_tessellation_patch_size {
        todo!()
    }
    if let Some(_val) = p.max_texel_buffer_elements {
        todo!()
    }
    if let Some(_val) = p.max_texel_gather_offset {
        todo!()
    }
    if let Some(_val) = p.max_texel_offset {
        todo!()
    }
    if let Some(_val) = p.max_timeline_semaphore_value_difference {
        todo!()
    }
    if let Some(_val) = p.max_transform_feedback_buffer_data_size {
        todo!()
    }
    if let Some(_val) = p.max_transform_feedback_buffer_data_stride {
        todo!()
    }
    if let Some(_val) = p.max_transform_feedback_buffer_size {
        todo!()
    }
    if let Some(_val) = p.max_transform_feedback_buffers {
        todo!()
    }
    if let Some(_val) = p.max_transform_feedback_stream_data_size {
        todo!()
    }
    if let Some(_val) = p.max_transform_feedback_streams {
        todo!()
    }
    if let Some(_val) = p.max_triangle_count {
        todo!()
    }
    if let Some(_val) = p.max_uniform_buffer_range {
        todo!()
    }
    if let Some(_val) = p.max_update_after_bind_descriptors_in_all_pools {
        todo!()
    }
    if let Some(_val) = p.max_vertex_attrib_divisor {
        todo!()
    }
    if let Some(_val) = p.max_vertex_input_attribute_offset {
        todo!()
    }
    if let Some(_val) = p.max_vertex_input_attributes {
        todo!()
    }
    if let Some(_val) = p.max_vertex_input_binding_stride {
        todo!()
    }
    if let Some(_val) = p.max_vertex_input_bindings {
        todo!()
    }
    if let Some(_val) = p.max_vertex_output_components {
        todo!()
    }
    if let Some(_val) = p.max_vgpr_allocation {
        todo!()
    }
    if let Some(_val) = p.max_viewport_dimensions {
        todo!()
    }
    if let Some(_val) = p.max_viewports {
        todo!()
    }
    if let Some(_val) = p.mesh_output_per_primitive_granularity {
        todo!()
    }
    if let Some(_val) = p.mesh_output_per_vertex_granularity {
        todo!()
    }
    if let Some(_val) = p.min_acceleration_structure_scratch_offset_alignment {
        todo!()
    }
    if let Some(_val) = p.min_fragment_density_texel_size {
        todo!()
    }
    if let Some(_val) = p.min_fragment_shading_rate_attachment_texel_size {
        todo!()
    }
    if let Some(_val) = p.min_imported_host_pointer_alignment {
        todo!()
    }
    if let Some(_val) = p.min_indirect_commands_buffer_offset_alignment {
        todo!()
    }
    if let Some(_val) = p.min_interpolation_offset {
        todo!()
    }
    if let Some(_val) = p.min_memory_map_alignment {
        todo!()
    }
    if let Some(_val) = p.min_sequences_count_buffer_offset_alignment {
        todo!()
    }
    if let Some(_val) = p.min_sequences_index_buffer_offset_alignment {
        todo!()
    }
    if let Some(_val) = p.min_sgpr_allocation {
        todo!()
    }
    if let Some(_val) = p.min_storage_buffer_offset_alignment {
        todo!()
    }
    if let Some(_val) = p.min_subgroup_size {
        todo!()
    }
    if let Some(_val) = p.min_texel_buffer_offset_alignment {
        todo!()
    }
    if let Some(_val) = p.min_texel_gather_offset {
        todo!()
    }
    if let Some(_val) = p.min_texel_offset {
        todo!()
    }
    if let Some(_val) = p.min_uniform_buffer_offset_alignment {
        todo!()
    }
    if let Some(_val) = p.min_vertex_input_binding_stride_alignment {
        todo!()
    }
    if let Some(_val) = p.min_vgpr_allocation {
        todo!()
    }
    if let Some(_val) = p.mipmap_precision_bits {
        todo!()
    }
    if let Some(_val) = p.non_coherent_atom_size {
        todo!()
    }
    if let Some(_val) = p.optimal_buffer_copy_offset_alignment {
        todo!()
    }
    if let Some(_val) = p.optimal_buffer_copy_row_pitch_alignment {
        todo!()
    }
    if let Some(_val) = p.pci_bus {
        todo!()
    }
    if let Some(_val) = p.pci_device {
        todo!()
    }
    if let Some(_val) = p.pci_domain {
        todo!()
    }
    if let Some(_val) = p.pci_function {
        todo!()
    }
    if let Some(_val) = p.per_view_position_all_components {
        todo!()
    }
    if let Some(_val) = p.pipeline_cache_uuid {
        todo!()
    }
    if let Some(_val) = p.point_clipping_behavior {
        todo!()
    }
    if let Some(_val) = p.point_size_granularity {
        todo!()
    }
    if let Some(_val) = p.point_size_range {
        todo!()
    }
    if let Some(_val) = p.primitive_fragment_shading_rate_with_multiple_viewports {
        todo!()
    }
    if let Some(_val) = p.primitive_overestimation_size {
        todo!()
    }
    if let Some(_val) = p.primitive_underestimation {
        todo!()
    }
    if let Some(_val) = p.protected_no_fault {
        todo!()
    }
    if let Some(_val) = p.quad_divergent_implicit_lod {
        todo!()
    }
    if let Some(_val) = p.quad_operations_in_all_stages {
        todo!()
    }
    if let Some(_val) = p.required_subgroup_size_stages {
        todo!()
    }
    if let Some(_val) = p.residency_aligned_mip_size {
        todo!()
    }
    if let Some(_val) = p.residency_non_resident_strict {
        todo!()
    }
    if let Some(_val) = p.residency_standard2_d_block_shape {
        todo!()
    }
    if let Some(_val) = p.residency_standard2_d_multisample_block_shape {
        todo!()
    }
    if let Some(_val) = p.residency_standard3_d_block_shape {
        todo!()
    }
    if let Some(_val) = p.robust_buffer_access_update_after_bind {
        todo!()
    }
    if let Some(_val) = p.robust_storage_buffer_access_size_alignment {
        todo!()
    }
    if let Some(_val) = p.robust_uniform_buffer_access_size_alignment {
        todo!()
    }
    if let Some(_val) = p.rounding_mode_independence {
        todo!()
    }
    if let Some(_val) = p.sample_location_coordinate_range {
        todo!()
    }
    if let Some(_val) = p.sample_location_sample_counts {
        todo!()
    }
    if let Some(_val) = p.sample_location_sub_pixel_bits {
        todo!()
    }
    if let Some(_val) = p.sampled_image_color_sample_counts {
        todo!()
    }
    if let Some(_val) = p.sampled_image_depth_sample_counts {
        todo!()
    }
    if let Some(_val) = p.sampled_image_integer_sample_counts {
        todo!()
    }
    if let Some(_val) = p.sampled_image_stencil_sample_counts {
        todo!()
    }
    if let Some(_val) = p.sgpr_allocation_granularity {
        todo!()
    }
    if let Some(_val) = p.sgprs_per_simd {
        todo!()
    }
    if let Some(_val) = p.shader_arrays_per_engine_count {
        todo!()
    }
    if let Some(_val) = p.shader_core_features {
        todo!()
    }
    if let Some(_val) = p.shader_denorm_flush_to_zero_float16 {
        todo!()
    }
    if let Some(_val) = p.shader_denorm_flush_to_zero_float32 {
        todo!()
    }
    if let Some(_val) = p.shader_denorm_flush_to_zero_float64 {
        todo!()
    }
    if let Some(_val) = p.shader_denorm_preserve_float16 {
        todo!()
    }
    if let Some(_val) = p.shader_denorm_preserve_float32 {
        todo!()
    }
    if let Some(_val) = p.shader_denorm_preserve_float64 {
        todo!()
    }
    if let Some(_val) = p.shader_engine_count {
        todo!()
    }
    if let Some(_val) = p.shader_group_base_alignment {
        todo!()
    }
    if let Some(_val) = p.shader_group_handle_alignment {
        todo!()
    }
    if let Some(_val) = p.shader_group_handle_capture_replay_size {
        todo!()
    }
    if let Some(_val) = p.shader_group_handle_size {
        todo!()
    }
    if let Some(_val) = p.shader_input_attachment_array_non_uniform_indexing_native {
        todo!()
    }
    if let Some(_val) = p.shader_rounding_mode_rte_float16 {
        todo!()
    }
    if let Some(_val) = p.shader_rounding_mode_rte_float32 {
        todo!()
    }
    if let Some(_val) = p.shader_rounding_mode_rte_float64 {
        todo!()
    }
    if let Some(_val) = p.shader_rounding_mode_rtz_float16 {
        todo!()
    }
    if let Some(_val) = p.shader_rounding_mode_rtz_float32 {
        todo!()
    }
    if let Some(_val) = p.shader_rounding_mode_rtz_float64 {
        todo!()
    }
    if let Some(_val) = p.shader_sampled_image_array_non_uniform_indexing_native {
        todo!()
    }
    if let Some(_val) = p.shader_signed_zero_inf_nan_preserve_float16 {
        todo!()
    }
    if let Some(_val) = p.shader_signed_zero_inf_nan_preserve_float32 {
        todo!()
    }
    if let Some(_val) = p.shader_signed_zero_inf_nan_preserve_float64 {
        todo!()
    }
    if let Some(_val) = p.shader_sm_count {
        todo!()
    }
    if let Some(_val) = p.shader_storage_buffer_array_non_uniform_indexing_native {
        todo!()
    }
    if let Some(_val) = p.shader_storage_image_array_non_uniform_indexing_native {
        todo!()
    }
    if let Some(_val) = p.shader_uniform_buffer_array_non_uniform_indexing_native {
        todo!()
    }
    if let Some(_val) = p.shader_warps_per_sm {
        todo!()
    }
    if let Some(_val) = p.shading_rate_max_coarse_samples {
        todo!()
    }
    if let Some(_val) = p.shading_rate_palette_size {
        todo!()
    }
    if let Some(_val) = p.shading_rate_texel_size {
        todo!()
    }
    if let Some(_val) = p.simd_per_compute_unit {
        todo!()
    }
    if let Some(_val) = p.sparse_address_space_size {
        todo!()
    }
    if let Some(_val) = p.standard_sample_locations {
        todo!()
    }
    if let Some(_val) = p.storage_image_sample_counts {
        todo!()
    }
    if let Some(_val) = p.storage_texel_buffer_offset_alignment_bytes {
        todo!()
    }
    if let Some(_val) = p.storage_texel_buffer_offset_single_texel_alignment {
        todo!()
    }
    if let Some(_val) = p.strict_lines {
        todo!()
    }
    if let Some(_val) = p.sub_pixel_interpolation_offset_bits {
        todo!()
    }
    if let Some(_val) = p.sub_pixel_precision_bits {
        todo!()
    }
    if let Some(_val) = p.sub_texel_precision_bits {
        todo!()
    }
    if let Some(_val) = p.subgroup_quad_operations_in_all_stages {
        todo!()
    }
    if let Some(_val) = p.subgroup_size {
        todo!()
    }
    if let Some(_val) = p.subgroup_supported_operations {
        todo!()
    }
    if let Some(_val) = p.subgroup_supported_stages {
        todo!()
    }
    if let Some(_val) = p.subsampled_coarse_reconstruction_early_access {
        todo!()
    }
    if let Some(_val) = p.subsampled_loads {
        todo!()
    }
    if let Some(_val) = p.supported_depth_resolve_modes {
        todo!()
    }
    if let Some(_val) = p.supported_operations {
        todo!()
    }
    if let Some(_val) = p.supported_stages {
        todo!()
    }
    if let Some(_val) = p.supported_stencil_resolve_modes {
        todo!()
    }
    if let Some(_val) = p.timestamp_compute_and_graphics {
        todo!()
    }
    if let Some(_val) = p.timestamp_period {
        todo!()
    }
    if let Some(_val) = p.transform_feedback_draw {
        todo!()
    }
    if let Some(_val) = p.transform_feedback_queries {
        todo!()
    }
    if let Some(_val) = p.transform_feedback_rasterization_stream_select {
        todo!()
    }
    if let Some(_val) = p.transform_feedback_streams_lines_triangles {
        todo!()
    }
    if let Some(_val) = p.uniform_texel_buffer_offset_alignment_bytes {
        todo!()
    }
    if let Some(_val) = p.uniform_texel_buffer_offset_single_texel_alignment {
        todo!()
    }
    if let Some(_val) = p.variable_sample_locations {
        todo!()
    }
    if let Some(_val) = p.vendor_id {
        todo!()
    }
    if let Some(_val) = p.vgpr_allocation_granularity {
        todo!()
    }
    if let Some(_val) = p.vgprs_per_simd {
        todo!()
    }
    if let Some(_val) = p.viewport_bounds_range {
        todo!()
    }
    if let Some(_val) = p.viewport_sub_pixel_bits {
        todo!()
    }
    if let Some(_val) = p.wavefront_size {
        todo!()
    }
    if let Some(_val) = p.wavefronts_per_simd {
        todo!()
    }

    Ok(())
}
