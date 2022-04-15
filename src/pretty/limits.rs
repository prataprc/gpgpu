// TODO: keep this module update with wgpu implementation and web-gpu spec.

use prettytable::{cell, row};

use crate::util::PrettyRow;

pub struct Limit {
    name: String,
    default: u32,
    adapters: Vec<u32>,
}

impl PrettyRow for Limit {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![ Fy => "Name", "Default" ]
    }

    fn to_row(&self) -> prettytable::Row {
        let mut row = row![self.name, self.default];
        self.adapters.iter().for_each(|l| row.add_cell(cell![l]));
        row
    }
}

pub fn limits() -> Vec<Limit> {
    let defaults = wgpu::Limits::default();

    vec![
        Limit {
            name: "max_texture_dimension_1d".to_string(),
            default: defaults.max_texture_dimension_1d,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_texture_dimension_2d".to_string(),
            default: defaults.max_texture_dimension_2d,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_texture_dimension_3d".to_string(),
            default: defaults.max_texture_dimension_3d,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_texture_array_layers".to_string(),
            default: defaults.max_texture_array_layers,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_bind_groups".to_string(),
            default: defaults.max_bind_groups,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_dynamic_uniform_buffers_per_pipeline_layout".to_string(),
            default: defaults.max_dynamic_uniform_buffers_per_pipeline_layout,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_dynamic_storage_buffers_per_pipeline_layout".to_string(),
            default: defaults.max_dynamic_storage_buffers_per_pipeline_layout,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_sampled_textures_per_shader_stage".to_string(),
            default: defaults.max_sampled_textures_per_shader_stage,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_samplers_per_shader_stage".to_string(),
            default: defaults.max_samplers_per_shader_stage,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_storage_buffers_per_shader_stage".to_string(),
            default: defaults.max_storage_buffers_per_shader_stage,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_storage_textures_per_shader_stage".to_string(),
            default: defaults.max_storage_textures_per_shader_stage,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_uniform_buffers_per_shader_stage".to_string(),
            default: defaults.max_uniform_buffers_per_shader_stage,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_uniform_buffer_binding_size".to_string(),
            default: defaults.max_uniform_buffer_binding_size,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_storage_buffer_binding_size".to_string(),
            default: defaults.max_storage_buffer_binding_size,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_vertex_buffers".to_string(),
            default: defaults.max_vertex_buffers,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_vertex_attributes".to_string(),
            default: defaults.max_vertex_attributes,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_vertex_buffer_array_stride".to_string(),
            default: defaults.max_vertex_buffer_array_stride,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_push_constant_size".to_string(),
            default: defaults.max_push_constant_size,
            adapters: Vec::default(),
        },
        Limit {
            name: "min_uniform_buffer_offset_alignment".to_string(),
            default: defaults.min_uniform_buffer_offset_alignment,
            adapters: Vec::default(),
        },
        Limit {
            name: "min_storage_buffer_offset_alignment".to_string(),
            default: defaults.min_storage_buffer_offset_alignment,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_inter_stage_shader_components".to_string(),
            default: defaults.max_inter_stage_shader_components,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_compute_workgroup_storage_size".to_string(),
            default: defaults.max_compute_workgroup_storage_size,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_compute_invocations_per_workgroup".to_string(),
            default: defaults.max_compute_invocations_per_workgroup,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_compute_workgroup_size_x".to_string(),
            default: defaults.max_compute_workgroup_size_x,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_compute_workgroup_size_y".to_string(),
            default: defaults.max_compute_workgroup_size_y,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_compute_workgroup_size_z".to_string(),
            default: defaults.max_compute_workgroup_size_z,
            adapters: Vec::default(),
        },
        Limit {
            name: "max_compute_workgroups_per_dimension".to_string(),
            default: defaults.max_compute_workgroups_per_dimension,
            adapters: Vec::default(),
        },
    ]
}

pub fn add_adapter_to_limits(limits: &mut Vec<Limit>, adapter_limits: wgpu::Limits) {
    for limit in limits.iter_mut() {
        match limit.name.as_str() {
            "max_texture_dimension_1d" => {
                limit.adapters.push(adapter_limits.max_texture_dimension_1d)
            }
            "max_texture_dimension_2d" => {
                limit.adapters.push(adapter_limits.max_texture_dimension_2d)
            }
            "max_texture_dimension_3d" => {
                limit.adapters.push(adapter_limits.max_texture_dimension_3d)
            }
            "max_texture_array_layers" => {
                limit.adapters.push(adapter_limits.max_texture_array_layers)
            }
            "max_bind_groups" => limit.adapters.push(adapter_limits.max_bind_groups),
            "max_dynamic_uniform_buffers_per_pipeline_layout" => limit
                .adapters
                .push(adapter_limits.max_dynamic_uniform_buffers_per_pipeline_layout),
            "max_dynamic_storage_buffers_per_pipeline_layout" => limit
                .adapters
                .push(adapter_limits.max_dynamic_storage_buffers_per_pipeline_layout),
            "max_sampled_textures_per_shader_stage" => limit
                .adapters
                .push(adapter_limits.max_sampled_textures_per_shader_stage),
            "max_samplers_per_shader_stage" => limit
                .adapters
                .push(adapter_limits.max_samplers_per_shader_stage),
            "max_storage_buffers_per_shader_stage" => limit
                .adapters
                .push(adapter_limits.max_storage_buffers_per_shader_stage),
            "max_storage_textures_per_shader_stage" => limit
                .adapters
                .push(adapter_limits.max_storage_textures_per_shader_stage),
            "max_uniform_buffers_per_shader_stage" => limit
                .adapters
                .push(adapter_limits.max_uniform_buffers_per_shader_stage),
            "max_uniform_buffer_binding_size" => limit
                .adapters
                .push(adapter_limits.max_uniform_buffer_binding_size),
            "max_storage_buffer_binding_size" => limit
                .adapters
                .push(adapter_limits.max_storage_buffer_binding_size),
            "max_vertex_buffers" => {
                limit.adapters.push(adapter_limits.max_vertex_buffers)
            }
            "max_vertex_attributes" => {
                limit.adapters.push(adapter_limits.max_vertex_attributes)
            }
            "max_vertex_buffer_array_stride" => limit
                .adapters
                .push(adapter_limits.max_vertex_buffer_array_stride),
            "max_push_constant_size" => {
                limit.adapters.push(adapter_limits.max_push_constant_size)
            }
            "min_uniform_buffer_offset_alignment" => limit
                .adapters
                .push(adapter_limits.min_uniform_buffer_offset_alignment),
            "min_storage_buffer_offset_alignment" => limit
                .adapters
                .push(adapter_limits.min_storage_buffer_offset_alignment),
            "max_inter_stage_shader_components" => limit
                .adapters
                .push(adapter_limits.max_inter_stage_shader_components),
            "max_compute_workgroup_storage_size" => limit
                .adapters
                .push(adapter_limits.max_compute_workgroup_storage_size),
            "max_compute_invocations_per_workgroup" => limit
                .adapters
                .push(adapter_limits.max_compute_invocations_per_workgroup),
            "max_compute_workgroup_size_x" => limit
                .adapters
                .push(adapter_limits.max_compute_workgroup_size_x),
            "max_compute_workgroup_size_y" => limit
                .adapters
                .push(adapter_limits.max_compute_workgroup_size_y),
            "max_compute_workgroup_size_z" => limit
                .adapters
                .push(adapter_limits.max_compute_workgroup_size_z),
            "max_compute_workgroups_per_dimension" => limit
                .adapters
                .push(adapter_limits.max_compute_workgroups_per_dimension),
            _ => unreachable!(),
        }
    }
}
