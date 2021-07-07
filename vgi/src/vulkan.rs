use vulkano::{
    device::{Device, DeviceExtensions, Features, Properties, Queue},
    format::Format,
    image::{
        ImageCreateFlags, ImageFormatProperties, ImageTiling, ImageType, ImageUsage,
    },
    instance::{
        ApplicationInfo, Instance, InstanceExtensions, LayerProperties, MemoryHeap,
        MemoryType, PhysicalDevice, QueueFamily, Version,
    },
    sync::PipelineStage,
};

use std::sync::Arc;

use crate::{Error, Result};

/// Maps to VkQueueFlagBits.
#[derive(Clone)]
pub enum QueueCapability {
    Graphics,
    Compute,
    Transfer,
    SparseBinding,
}

/// Similar to VkDeviceQueueCreateInfo. A single instance of QueueCreateInfo shall create
/// as many VkQueue objects as then number of priorities, in other-words each item in
/// priorities vector specify the priority for queue-count-index.
#[derive(Clone)]
pub struct QueueCreateInfo {
    pub cap: QueueCapability,
    pub stages: Vec<PipelineStage>,
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

pub struct Builder<'a> {
    // instance attributes
    app_info: ApplicationInfo<'a>,
    version: Version,
    layers: Vec<String>,
    iextns: InstanceExtensions,
    // device attributes
    device_id: usize,
    queue_infos: Vec<QueueCreateInfo>,
    dextns: DeviceExtensions,
    properties: Properties,
    features: Features,
}

impl<'a> Builder<'a> {
    /// Create new builder using cargo manifest for `application_info`, without enabling
    /// any of the instance-extensions and without enabling any of the layers. This
    /// method shall automatically detect the latest version from the driver's
    /// [FunctionPointers]. Later use one of the `with_*` methods to add more builder
    /// options.
    pub fn new() -> Result<Builder<'a>> {
        use vulkano::instance::loader::auto_loader;

        let funcptrs = err_at!(Vk, auto_loader())?;
        let version = err_at!(Vk, funcptrs.api_version())?;

        let builder = Builder {
            // instance attributes
            app_info: vulkano::app_info_from_cargo_toml!(),
            version,
            iextns: InstanceExtensions::none(),
            layers: Vec::default(),
            // device attributes
            device_id: 0,
            queue_infos: vec![QueueCreateInfo::default()],
            dextns: DeviceExtensions::none(),
            properties: Properties::default(),
            features: Features::none(),
        };

        Ok(builder)
    }

    /// Similar to [new] method, but supply the [ApplicationInfo] and [Version].
    pub fn with(app_info: ApplicationInfo<'a>, version: Version) -> Builder<'a> {
        Builder {
            // instance attributes
            app_info,
            version,
            iextns: InstanceExtensions::none(),
            layers: Vec::default(),
            // device attributes
            device_id: 0,
            queue_infos: vec![QueueCreateInfo::default()],
            dextns: DeviceExtensions::none(),
            properties: Properties::default(),
            features: Features::none(),
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

    /// List of extensions to enable while creating vulkan-instance.
    ///
    /// For screen rendering enable `khr_surface` extension and platform specific
    /// extensions like `khr_android_surface`, `khr_wayland_surface`,
    /// `khr_win32_surface`, `khr_xcb_surface`, `khr_xlib_surface`, `mvk_ios_surface`,
    /// `mvk_macos_surface`, `nn_vi_surface` in `InstanceExtensions`.
    pub fn with_extensions(mut self, extensions: InstanceExtensions) -> Self {
        self.iextns = extensions;
        self
    }

    /// Create VkDevice object using supplied parameters. At preset we don't have
    /// multi-device support. For requested [Features], device-extensions shall
    /// automatically be enabled event if they are not supplied in the `extensions` arg.
    ///
    /// By default if this method is not used, the the first available physical device
    /// shall be used with default properties and no-specific-feature requested and
    /// no-specific-device-extension enabled.
    ///
    /// For screen rendering enable `khr_swapchain` extension, also enable the
    /// `khr_surface` extension in `InstanceExtensions` refer to [with_extensions]
    /// method for details.
    pub fn with_device(
        mut self,
        id: usize,
        extensions: DeviceExtensions,
        properties: Properties,
        features: Features,
    ) -> Self {
        self.device_id = id;
        self.dextns = extensions;
        self.properties = properties;
        self.features = features;
        self
    }

    /// Create with queues. If not used a single graphics queue with priority 1.0 shall
    /// be used.
    pub fn with_queues(mut self, infos: Vec<QueueCreateInfo>) -> Self {
        self.queue_infos = infos;
        self
    }

    /// Finally call build, to obtain the [Vulkan] object.
    pub fn build(self) -> Result<Vulkan<'a>> {
        let instance = err_at!(
            Vk,
            Instance::new(
                Some(&self.app_info),
                self.version,
                &self.iextns,
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
        let pd = pds[self.device_id];
        self.confirm_properties(pd.properties().clone())?;
        let qfamilies: Vec<QueueFamily> = pd.queue_families().collect();

        let (dextns, device, queues) = {
            let qrs: Vec<(QueueFamily<'a>, f32)> = self
                .queue_infos
                .clone()
                .into_iter()
                .map(|info| make_queue_request(info, &qfamilies))
                .flatten()
                .map(|(id, p)| (pd.queue_family_by_id(id).unwrap(), p))
                .collect();
            let dextns = extensions_for_features(&self.features, self.dextns);
            let (device, queues) = err_at!(
                Vk,
                Device::new(pd, &self.features, &dextns, qrs.into_iter())
            )?;
            (dextns, device, queues.collect::<Vec<Arc<Queue>>>())
        };

        let val = Vulkan {
            // instance attribute
            layers: layers()?
                .into_iter()
                .filter(|l| self.layers.contains(&l.name().to_string()))
                .collect(),
            iextns: self.iextns,
            instance,
            phydevs: pds,
            // device attribute
            dextns,
            device,
            queues,
        };

        Ok(val)
    }

    fn confirm_properties(&self, props: Properties) -> Result<()> {
        let p = self.properties.clone();

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
        if let Some(_val) = p.max_descriptor_set_update_after_bind_acceleration_structures
        {
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
        if let Some(_val) = p.max_descriptor_set_update_after_bind_storage_buffers_dynamic
        {
            todo!()
        }
        if let Some(_val) = p.max_descriptor_set_update_after_bind_storage_images {
            todo!()
        }
        if let Some(_val) = p.max_descriptor_set_update_after_bind_uniform_buffers {
            todo!()
        }
        if let Some(_val) = p.max_descriptor_set_update_after_bind_uniform_buffers_dynamic
        {
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
        if let Some(_val) = p.max_fragment_shading_rate_attachment_texel_size_aspect_ratio
        {
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
        if let Some(_val) =
            p.max_per_stage_descriptor_update_after_bind_inline_uniform_blocks
        {
            todo!()
        }
        if let Some(_val) = p.max_per_stage_descriptor_update_after_bind_input_attachments
        {
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
}

/// Vulkan type roughly maps to instance/device object defined by the vulkan spec.
/// This type try to abstract the boiler plate code as much as possible by
/// providing convinient methods and related macros.
pub struct Vulkan<'a> {
    // instance attributes
    layers: Vec<LayerProperties>,
    iextns: InstanceExtensions,
    instance: Box<Arc<Instance>>,
    phydevs: Vec<PhysicalDevice<'a>>,
    // device attributes
    dextns: DeviceExtensions,
    device: Arc<Device>,
    queues: Vec<Arc<Queue>>,
}

impl<'a> Vulkan<'a> {
    /// Return enabled layers for instance.
    pub fn enabled_layers(&self) -> Vec<LayerProperties> {
        self.layers.clone()
    }

    /// Return instance extensions that are enabled/disabled.
    pub fn instance_extensions(&self) -> InstanceExtensions {
        self.iextns.clone()
    }

    /// Return device extensions that are enabled/disabled.
    pub fn device_extensions(&self) -> DeviceExtensions {
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

    /// Return the list of queue-families available for this device instance, depends
    /// on the physical-device used to create this device.
    pub fn queue_families(&self) -> Vec<QueueFamily> {
        self.device.physical_device().queue_families().collect()
    }

    /// Return the list of queue-families created for this device instance.
    pub fn active_queue_families(&self) -> Vec<QueueFamily> {
        self.device.active_queue_families().collect()
    }

    /// Return the list of memory-types available for this device instance, depends
    /// on the physical-device used to create this device.
    pub fn memory_types(&self) -> Vec<MemoryType> {
        self.device.physical_device().memory_types().collect()
    }

    /// Return the properties of physical-device used to create this device.
    pub fn properties(&self) -> &Properties {
        self.device.physical_device().properties()
    }

    /// Return the features supported by physical-device used to create this device.
    pub fn supported_features(&self) -> &Features {
        self.device.physical_device().supported_features()
    }

    /// Return the image format properties supported for this device.
    pub fn image_format_properties(
        &self,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsage,
        create_flags: ImageCreateFlags,
    ) -> Result<ImageFormatProperties> {
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

    /// Return the queue objects created for this device
    pub fn to_queues(&self) -> Vec<Arc<Queue>> {
        self.queues.clone()
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
