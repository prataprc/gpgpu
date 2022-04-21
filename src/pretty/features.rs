// TODO: keep this module update with wgpu implementation and web-gpu spec.

use colored::Colorize;
use prettytable::{cell, row};

use crate::util::{format_bool, PrettyRow};

pub struct Feature {
    name: String,
    value: wgpu::Features,
    web: bool,
    native: bool,
    backends: Vec<wgpu::Backend>,
    adapters: Vec<bool>,
}

impl PrettyRow for Feature {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![
            Fy =>
            "Name", "Web", "Native",
            "Vulkan", "Metal", "Dx12", "Dx11", "Gl", "Web"
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        use wgpu::Backend::{BrowserWebGpu, Dx11, Dx12, Gl, Metal, Vulkan};

        let mut row = row![
            self.name,
            format_bool!(self.web),
            format_bool!(self.native),
            format_bool!(self.backends.contains(&Vulkan)),
            format_bool!(self.backends.contains(&Metal)),
            format_bool!(self.backends.contains(&Dx12)),
            format_bool!(self.backends.contains(&Dx11)),
            format_bool!(self.backends.contains(&Gl)),
            format_bool!(self.backends.contains(&BrowserWebGpu)),
        ];

        self.adapters.iter().for_each(|b| row.add_cell(cell![*b]));

        row
    }
}

pub fn features() -> Vec<Feature> {
    use wgpu::Backend::{BrowserWebGpu, Dx11, Dx12, Gl, Metal, Vulkan};

    let native_features = wgpu::Features::all_native_mask();
    let webgpu_features = wgpu::Features::all_webgpu_mask();

    vec![
        Feature {
            name: "depth_clip_control".to_string(),
            value: wgpu::Features::DEPTH_CLIP_CONTROL,
            web: webgpu_features.contains(wgpu::Features::DEPTH_CLIP_CONTROL),
            native: native_features.contains(wgpu::Features::DEPTH_CLIP_CONTROL),
            backends: [Vulkan, Metal, Dx12, Dx11, Gl].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "texture_compression_bc".to_string(),
            value: wgpu::Features::TEXTURE_COMPRESSION_BC,
            web: webgpu_features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC),
            native: native_features.contains(wgpu::Features::TEXTURE_COMPRESSION_BC),
            backends: [Vulkan, Metal, Dx12, Dx11, Gl].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "indirect_first_instance".to_string(),
            value: wgpu::Features::INDIRECT_FIRST_INSTANCE,
            web: webgpu_features.contains(wgpu::Features::INDIRECT_FIRST_INSTANCE),
            native: native_features.contains(wgpu::Features::INDIRECT_FIRST_INSTANCE),
            backends: [Vulkan, Dx12, Metal].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "timestamp_query".to_string(),
            value: wgpu::Features::TIMESTAMP_QUERY,
            web: webgpu_features.contains(wgpu::Features::TIMESTAMP_QUERY),
            native: native_features.contains(wgpu::Features::TIMESTAMP_QUERY),
            backends: [Vulkan, Dx12].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "pipeline_statistics_query".to_string(),
            value: wgpu::Features::PIPELINE_STATISTICS_QUERY,
            web: webgpu_features.contains(wgpu::Features::PIPELINE_STATISTICS_QUERY),
            native: native_features.contains(wgpu::Features::PIPELINE_STATISTICS_QUERY),
            backends: [Vulkan, Dx12].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "mappable_primary_buffers".to_string(),
            value: wgpu::Features::MAPPABLE_PRIMARY_BUFFERS,
            web: webgpu_features.contains(wgpu::Features::MAPPABLE_PRIMARY_BUFFERS),
            native: native_features.contains(wgpu::Features::MAPPABLE_PRIMARY_BUFFERS),
            backends: [Vulkan, Metal, Dx12, Dx11, Gl, BrowserWebGpu].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "texture_binding_array".to_string(),
            value: wgpu::Features::TEXTURE_BINDING_ARRAY,
            web: webgpu_features.contains(wgpu::Features::TEXTURE_BINDING_ARRAY),
            native: native_features.contains(wgpu::Features::TEXTURE_BINDING_ARRAY),
            backends: [Vulkan, Metal, Dx12].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "buffer_binding_array".to_string(),
            value: wgpu::Features::BUFFER_BINDING_ARRAY,
            web: webgpu_features.contains(wgpu::Features::BUFFER_BINDING_ARRAY),
            native: native_features.contains(wgpu::Features::BUFFER_BINDING_ARRAY),
            backends: [Vulkan, Dx12].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "storage_resource_binding_array".to_string(),
            value: wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY,
            web: webgpu_features.contains(wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY),
            native: native_features.contains(wgpu::Features::STORAGE_RESOURCE_BINDING_ARRAY),
            backends: [Vulkan, Metal].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "sampled_texture_and_storage_buffer_array_non_uniform_indexing".to_string(),
            value: wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING,
            web: webgpu_features.contains(wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING),
            native: native_features.contains(wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING),
            backends: [Vulkan, Metal, Dx12].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "uniform_buffer_and_storage_texture_array_non_uniform_indexing".to_string(),
            value: wgpu::Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING,
            web: webgpu_features.contains(wgpu::Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING),
            native: native_features.contains(wgpu::Features::UNIFORM_BUFFER_AND_STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING),
            backends: [Vulkan, Metal, Dx12].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "partially_bound_binding_array".to_string(),
            value: wgpu::Features::PARTIALLY_BOUND_BINDING_ARRAY,
            web: webgpu_features.contains(wgpu::Features::PARTIALLY_BOUND_BINDING_ARRAY),
            native: native_features.contains(wgpu::Features::PARTIALLY_BOUND_BINDING_ARRAY),
            backends: [Vulkan, Metal, Dx12, Dx11, Gl, BrowserWebGpu].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "unsized_binding_array".to_string(),
            value: wgpu::Features::UNSIZED_BINDING_ARRAY,
            web: webgpu_features.contains(wgpu::Features::UNSIZED_BINDING_ARRAY),
            native: native_features.contains(wgpu::Features::UNSIZED_BINDING_ARRAY),
            backends: [Vulkan, Dx12].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "multi_draw_indirect".to_string(),
            value: wgpu::Features::MULTI_DRAW_INDIRECT,
            web: webgpu_features.contains(wgpu::Features::MULTI_DRAW_INDIRECT),
            native: native_features.contains(wgpu::Features::MULTI_DRAW_INDIRECT),
            backends: [Vulkan, Dx12].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "multi_draw_indirect_count".to_string(),
            value: wgpu::Features::MULTI_DRAW_INDIRECT_COUNT,
            web: webgpu_features.contains(wgpu::Features::MULTI_DRAW_INDIRECT_COUNT),
            native: native_features.contains(wgpu::Features::MULTI_DRAW_INDIRECT_COUNT),
            backends: [Vulkan, Dx12].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "push_constants".to_string(),
            value: wgpu::Features::PUSH_CONSTANTS,
            web: webgpu_features.contains(wgpu::Features::PUSH_CONSTANTS),
            native: native_features.contains(wgpu::Features::PUSH_CONSTANTS),
            backends: [Vulkan, Metal, Dx12, Dx11, Gl].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "address_mode_clamp_to_border".to_string(),
            value: wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER,
            web: webgpu_features.contains(wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER),
            native: native_features.contains(wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER),
            backends: [Vulkan, Metal, Dx12, Dx11, Gl, BrowserWebGpu].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "polygon_mode_line".to_string(),
            value: wgpu::Features::POLYGON_MODE_LINE,
            web: webgpu_features.contains(wgpu::Features::POLYGON_MODE_LINE),
            native: native_features.contains(wgpu::Features::POLYGON_MODE_LINE),
            backends: [Vulkan, Metal, Dx12].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "polygon_mode_point".to_string(),
            value: wgpu::Features::POLYGON_MODE_POINT,
            web: webgpu_features.contains(wgpu::Features::POLYGON_MODE_POINT),
            native: native_features.contains(wgpu::Features::POLYGON_MODE_POINT),
            backends: [Vulkan, Dx12].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "texture_compression_etc2".to_string(),
            value: wgpu::Features::TEXTURE_COMPRESSION_ETC2,
            web: webgpu_features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2),
            native: native_features.contains(wgpu::Features::TEXTURE_COMPRESSION_ETC2),
            backends: [Vulkan].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "texture_compression_astc_ldr".to_string(),
            value: wgpu::Features::TEXTURE_COMPRESSION_ASTC_LDR,
            web: webgpu_features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC_LDR),
            native: native_features.contains(wgpu::Features::TEXTURE_COMPRESSION_ASTC_LDR),
            backends: [Vulkan].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "texture_adapter_specific_format_features".to_string(),
            value: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
            web: webgpu_features.contains(wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES),
            native: native_features.contains(wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES),
            backends: [Vulkan].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "shader_float64".to_string(),
            value: wgpu::Features::SHADER_FLOAT64,
            web: webgpu_features.contains(wgpu::Features::SHADER_FLOAT64),
            native: native_features.contains(wgpu::Features::SHADER_FLOAT64),
            backends: [Vulkan].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "vertex_attribute_64bit".to_string(),
            value: wgpu::Features::VERTEX_ATTRIBUTE_64BIT,
            web: webgpu_features.contains(wgpu::Features::VERTEX_ATTRIBUTE_64BIT),
            native: native_features.contains(wgpu::Features::VERTEX_ATTRIBUTE_64BIT),
            backends: [Vulkan].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "conservative_rasterization".to_string(),
            value: wgpu::Features::CONSERVATIVE_RASTERIZATION,
            web: webgpu_features.contains(wgpu::Features::CONSERVATIVE_RASTERIZATION),
            native: native_features.contains(wgpu::Features::CONSERVATIVE_RASTERIZATION),
            backends: [Vulkan].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "vertex_writable_storage".to_string(),
            value: wgpu::Features::VERTEX_WRITABLE_STORAGE,
            web: webgpu_features.contains(wgpu::Features::VERTEX_WRITABLE_STORAGE),
            native: native_features.contains(wgpu::Features::VERTEX_WRITABLE_STORAGE),
            backends: [Vulkan, Metal, Dx12, Dx11, Gl, BrowserWebGpu].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "clear_commands".to_string(),
            value: wgpu::Features::CLEAR_COMMANDS,
            web: webgpu_features.contains(wgpu::Features::CLEAR_COMMANDS),
            native: native_features.contains(wgpu::Features::CLEAR_COMMANDS),
            backends: [Vulkan, Metal, Dx12, Dx11, Gl, BrowserWebGpu].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "spirv_shader_passthrough".to_string(),
            value: wgpu::Features::SPIRV_SHADER_PASSTHROUGH,
            web: webgpu_features.contains(wgpu::Features::SPIRV_SHADER_PASSTHROUGH),
            native: native_features.contains(wgpu::Features::SPIRV_SHADER_PASSTHROUGH),
            backends: [Vulkan].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "shader_primitive_index".to_string(),
            value: wgpu::Features::SHADER_PRIMITIVE_INDEX,
            web: webgpu_features.contains(wgpu::Features::SHADER_PRIMITIVE_INDEX),
            native: native_features.contains(wgpu::Features::SHADER_PRIMITIVE_INDEX),
            backends: [Vulkan].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "multiview".to_string(),
            value: wgpu::Features::MULTIVIEW,
            web: webgpu_features.contains(wgpu::Features::MULTIVIEW),
            native: native_features.contains(wgpu::Features::MULTIVIEW),
            backends: [Vulkan].to_vec(),
            adapters: Vec::default(),
        },
        Feature {
            name: "texture_format_16bit_norm".to_string(),
            value: wgpu::Features::TEXTURE_FORMAT_16BIT_NORM,
            web: webgpu_features.contains(wgpu::Features::TEXTURE_FORMAT_16BIT_NORM),
            native: native_features.contains(wgpu::Features::TEXTURE_FORMAT_16BIT_NORM),
            backends: [Vulkan, Metal, Dx12].to_vec(),
            adapters: Vec::default(),
        },
    ]
}

pub fn add_adapter_to_features(
    features: &mut Vec<Feature>,
    adapter_features: wgpu::Features,
) {
    for f in features.iter_mut() {
        f.adapters.push(adapter_features.contains(f.value))
    }
}
