use colored::Colorize;
use prettytable::{cell, row};

use crate::util::{format_bool, PrettyRow};

pub struct TextureFormatInfo {
    name: String,
    #[allow(dead_code)]
    value: wgpu::TextureFormat,
    info: wgpu_types::TextureFormatInfo,
}

impl PrettyRow for TextureFormatInfo {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![
            Fy =>
            "Name", "Components", "Size", "Dimension", "SRGB", "SampleType",
            "Features", "Usages", "Flags", "Filter"
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        let gff = &self.info.guaranteed_format_features;
        let usages = texture_usages()
            .iter()
            .filter_map(|(u, s, _)| {
                if gff.allowed_usages.contains(*u) {
                    Some(s.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>()
            .join(",");

        let flags =
            texture_format_flags()
                .iter()
                .filter_map(|(u, s, _)| {
                    if gff.flags.contains(*u) {
                        Some(s.to_string())
                    } else {
                        None
                    }
                })
                .collect::<Vec<String>>()
                .join(",");

        row![
            self.name,
            self.info.components,
            self.info.block_size,
            format!("{:?}", self.info.block_dimensions),
            self.info.srgb,
            format!("{:?}", self.info.sample_type),
            format_bool!(!self.info.required_features.is_empty()),
            usages,
            flags,
            gff.filterable,
        ]
    }
}

pub fn texture_usages() -> Vec<(wgpu_types::TextureUsages, &'static str, &'static str)> {
    vec![
        (wgpu_types::TextureUsages::COPY_DST, "D", "COPY_DST"),
        (wgpu_types::TextureUsages::COPY_SRC, "C", "COPY_SRC"),
        (wgpu_types::TextureUsages::RENDER_ATTACHMENT, "R", "RENDER_ATTACHMENT"),
        (wgpu_types::TextureUsages::STORAGE_BINDING, "S", "STORAGE_BINDING"),
        (wgpu_types::TextureUsages::TEXTURE_BINDING, "T", "TEXTURE_BINDING"),
    ]
}

pub fn texture_format_flags(
) -> Vec<(wgpu_types::TextureFormatFeatureFlags, &'static str, &'static str)> {
    vec![
        (wgpu_types::TextureFormatFeatureFlags::STORAGE_ATOMICS, "A", "STORAGE_ATOMICS"),
        (
            wgpu_types::TextureFormatFeatureFlags::STORAGE_READ_WRITE,
            "RW",
            "STORAGE_READ_WRITE",
        ),
    ]
}

pub fn texture_formats_info() -> Vec<TextureFormatInfo> {
    vec![
        TextureFormatInfo {
            name: "Astc10x10RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc10x10RgbaUnorm,
            info: wgpu::TextureFormat::Astc10x10RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc10x10RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc10x10RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc10x10RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc10x5RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc10x5RgbaUnorm,
            info: wgpu::TextureFormat::Astc10x5RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc10x5RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc10x5RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc10x5RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc10x6RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc10x6RgbaUnorm,
            info: wgpu::TextureFormat::Astc10x6RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc10x6RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc10x6RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc10x6RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc10x8RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc10x8RgbaUnorm,
            info: wgpu::TextureFormat::Astc10x8RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc10x8RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc10x8RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc10x8RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc12x10RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc12x10RgbaUnorm,
            info: wgpu::TextureFormat::Astc12x10RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc12x10RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc12x10RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc12x10RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc12x12RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc12x12RgbaUnorm,
            info: wgpu::TextureFormat::Astc12x12RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc12x12RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc12x12RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc12x12RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc4x4RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc4x4RgbaUnorm,
            info: wgpu::TextureFormat::Astc4x4RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc4x4RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc4x4RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc4x4RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc5x4RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc5x4RgbaUnorm,
            info: wgpu::TextureFormat::Astc5x4RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc5x4RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc5x4RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc5x4RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc5x5RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc5x5RgbaUnorm,
            info: wgpu::TextureFormat::Astc5x5RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc5x5RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc5x5RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc5x5RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc6x5RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc6x5RgbaUnorm,
            info: wgpu::TextureFormat::Astc6x5RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc6x5RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc6x5RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc6x5RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc6x6RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc6x6RgbaUnorm,
            info: wgpu::TextureFormat::Astc6x6RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc6x6RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc6x6RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc6x6RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc8x5RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc8x5RgbaUnorm,
            info: wgpu::TextureFormat::Astc8x5RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc8x5RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc8x5RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc8x5RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc8x6RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc8x6RgbaUnorm,
            info: wgpu::TextureFormat::Astc8x6RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc8x6RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc8x6RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc8x6RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Astc8x8RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Astc8x8RgbaUnorm,
            info: wgpu::TextureFormat::Astc8x8RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Astc8x8RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Astc8x8RgbaUnormSrgb,
            info: wgpu::TextureFormat::Astc8x8RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Bc1RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Bc1RgbaUnorm,
            info: wgpu::TextureFormat::Bc1RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Bc1RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Bc1RgbaUnormSrgb,
            info: wgpu::TextureFormat::Bc1RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Bc2RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Bc2RgbaUnorm,
            info: wgpu::TextureFormat::Bc2RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Bc2RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Bc2RgbaUnormSrgb,
            info: wgpu::TextureFormat::Bc2RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Bc3RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Bc3RgbaUnorm,
            info: wgpu::TextureFormat::Bc3RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Bc3RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Bc3RgbaUnormSrgb,
            info: wgpu::TextureFormat::Bc3RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Bc4RSnorm".to_string(),
            value: wgpu::TextureFormat::Bc4RSnorm,
            info: wgpu::TextureFormat::Bc4RSnorm.describe(),
        },
        TextureFormatInfo {
            name: "Bc4RUnorm".to_string(),
            value: wgpu::TextureFormat::Bc4RUnorm,
            info: wgpu::TextureFormat::Bc4RUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Bc5RgSnorm".to_string(),
            value: wgpu::TextureFormat::Bc5RgSnorm,
            info: wgpu::TextureFormat::Bc5RgSnorm.describe(),
        },
        TextureFormatInfo {
            name: "Bc5RgUnorm".to_string(),
            value: wgpu::TextureFormat::Bc5RgUnorm,
            info: wgpu::TextureFormat::Bc5RgUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Bc6hRgbSfloat".to_string(),
            value: wgpu::TextureFormat::Bc6hRgbSfloat,
            info: wgpu::TextureFormat::Bc6hRgbSfloat.describe(),
        },
        TextureFormatInfo {
            name: "Bc6hRgbUfloat".to_string(),
            value: wgpu::TextureFormat::Bc6hRgbUfloat,
            info: wgpu::TextureFormat::Bc6hRgbUfloat.describe(),
        },
        TextureFormatInfo {
            name: "Bc7RgbaUnorm".to_string(),
            value: wgpu::TextureFormat::Bc7RgbaUnorm,
            info: wgpu::TextureFormat::Bc7RgbaUnorm.describe(),
        },
        TextureFormatInfo {
            name: "Bc7RgbaUnormSrgb".to_string(),
            value: wgpu::TextureFormat::Bc7RgbaUnormSrgb,
            info: wgpu::TextureFormat::Bc7RgbaUnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Bgra8Unorm".to_string(),
            value: wgpu::TextureFormat::Bgra8Unorm,
            info: wgpu::TextureFormat::Bgra8Unorm.describe(),
        },
        TextureFormatInfo {
            name: "Bgra8UnormSrgb".to_string(),
            value: wgpu::TextureFormat::Bgra8UnormSrgb,
            info: wgpu::TextureFormat::Bgra8UnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Depth24Plus".to_string(),
            value: wgpu::TextureFormat::Depth24Plus,
            info: wgpu::TextureFormat::Depth24Plus.describe(),
        },
        TextureFormatInfo {
            name: "Depth24PlusStencil8".to_string(),
            value: wgpu::TextureFormat::Depth24PlusStencil8,
            info: wgpu::TextureFormat::Depth24PlusStencil8.describe(),
        },
        TextureFormatInfo {
            name: "Depth32Float".to_string(),
            value: wgpu::TextureFormat::Depth32Float,
            info: wgpu::TextureFormat::Depth32Float.describe(),
        },
        TextureFormatInfo {
            name: "EacR11Snorm".to_string(),
            value: wgpu::TextureFormat::EacR11Snorm,
            info: wgpu::TextureFormat::EacR11Snorm.describe(),
        },
        TextureFormatInfo {
            name: "EacR11Unorm".to_string(),
            value: wgpu::TextureFormat::EacR11Unorm,
            info: wgpu::TextureFormat::EacR11Unorm.describe(),
        },
        TextureFormatInfo {
            name: "EacRg11Snorm".to_string(),
            value: wgpu::TextureFormat::EacRg11Snorm,
            info: wgpu::TextureFormat::EacRg11Snorm.describe(),
        },
        TextureFormatInfo {
            name: "EacRg11Unorm".to_string(),
            value: wgpu::TextureFormat::EacRg11Unorm,
            info: wgpu::TextureFormat::EacRg11Unorm.describe(),
        },
        TextureFormatInfo {
            name: "Etc2Rgb8A1Unorm".to_string(),
            value: wgpu::TextureFormat::Etc2Rgb8A1Unorm,
            info: wgpu::TextureFormat::Etc2Rgb8A1Unorm.describe(),
        },
        TextureFormatInfo {
            name: "Etc2Rgb8A1UnormSrgb".to_string(),
            value: wgpu::TextureFormat::Etc2Rgb8A1UnormSrgb,
            info: wgpu::TextureFormat::Etc2Rgb8A1UnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Etc2Rgb8Unorm".to_string(),
            value: wgpu::TextureFormat::Etc2Rgb8Unorm,
            info: wgpu::TextureFormat::Etc2Rgb8Unorm.describe(),
        },
        TextureFormatInfo {
            name: "Etc2Rgb8UnormSrgb".to_string(),
            value: wgpu::TextureFormat::Etc2Rgb8UnormSrgb,
            info: wgpu::TextureFormat::Etc2Rgb8UnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "Etc2Rgba8Unorm".to_string(),
            value: wgpu::TextureFormat::Etc2Rgba8Unorm,
            info: wgpu::TextureFormat::Etc2Rgba8Unorm.describe(),
        },
        TextureFormatInfo {
            name: "Etc2Rgba8UnormSrgb".to_string(),
            value: wgpu::TextureFormat::Etc2Rgba8UnormSrgb,
            info: wgpu::TextureFormat::Etc2Rgba8UnormSrgb.describe(),
        },
        TextureFormatInfo {
            name: "R16Float".to_string(),
            value: wgpu::TextureFormat::R16Float,
            info: wgpu::TextureFormat::R16Float.describe(),
        },
        TextureFormatInfo {
            name: "R16Sint".to_string(),
            value: wgpu::TextureFormat::R16Sint,
            info: wgpu::TextureFormat::R16Sint.describe(),
        },
        TextureFormatInfo {
            name: "R16Snorm".to_string(),
            value: wgpu::TextureFormat::R16Snorm,
            info: wgpu::TextureFormat::R16Snorm.describe(),
        },
        TextureFormatInfo {
            name: "R16Uint".to_string(),
            value: wgpu::TextureFormat::R16Uint,
            info: wgpu::TextureFormat::R16Uint.describe(),
        },
        TextureFormatInfo {
            name: "R16Unorm".to_string(),
            value: wgpu::TextureFormat::R16Unorm,
            info: wgpu::TextureFormat::R16Unorm.describe(),
        },
        TextureFormatInfo {
            name: "R32Float".to_string(),
            value: wgpu::TextureFormat::R32Float,
            info: wgpu::TextureFormat::R32Float.describe(),
        },
        TextureFormatInfo {
            name: "R32Sint".to_string(),
            value: wgpu::TextureFormat::R32Sint,
            info: wgpu::TextureFormat::R32Sint.describe(),
        },
        TextureFormatInfo {
            name: "R32Uint".to_string(),
            value: wgpu::TextureFormat::R32Uint,
            info: wgpu::TextureFormat::R32Uint.describe(),
        },
        TextureFormatInfo {
            name: "R8Sint".to_string(),
            value: wgpu::TextureFormat::R8Sint,
            info: wgpu::TextureFormat::R8Sint.describe(),
        },
        TextureFormatInfo {
            name: "R8Snorm".to_string(),
            value: wgpu::TextureFormat::R8Snorm,
            info: wgpu::TextureFormat::R8Snorm.describe(),
        },
        TextureFormatInfo {
            name: "R8Uint".to_string(),
            value: wgpu::TextureFormat::R8Uint,
            info: wgpu::TextureFormat::R8Uint.describe(),
        },
        TextureFormatInfo {
            name: "R8Unorm".to_string(),
            value: wgpu::TextureFormat::R8Unorm,
            info: wgpu::TextureFormat::R8Unorm.describe(),
        },
        TextureFormatInfo {
            name: "Rg11b10Float".to_string(),
            value: wgpu::TextureFormat::Rg11b10Float,
            info: wgpu::TextureFormat::Rg11b10Float.describe(),
        },
        TextureFormatInfo {
            name: "Rg16Float".to_string(),
            value: wgpu::TextureFormat::Rg16Float,
            info: wgpu::TextureFormat::Rg16Float.describe(),
        },
        TextureFormatInfo {
            name: "Rg16Sint".to_string(),
            value: wgpu::TextureFormat::Rg16Sint,
            info: wgpu::TextureFormat::Rg16Sint.describe(),
        },
        TextureFormatInfo {
            name: "Rg16Snorm".to_string(),
            value: wgpu::TextureFormat::Rg16Snorm,
            info: wgpu::TextureFormat::Rg16Snorm.describe(),
        },
        TextureFormatInfo {
            name: "Rg16Uint".to_string(),
            value: wgpu::TextureFormat::Rg16Uint,
            info: wgpu::TextureFormat::Rg16Uint.describe(),
        },
        TextureFormatInfo {
            name: "Rg16Unorm".to_string(),
            value: wgpu::TextureFormat::Rg16Unorm,
            info: wgpu::TextureFormat::Rg16Unorm.describe(),
        },
        TextureFormatInfo {
            name: "Rg32Float".to_string(),
            value: wgpu::TextureFormat::Rg32Float,
            info: wgpu::TextureFormat::Rg32Float.describe(),
        },
        TextureFormatInfo {
            name: "Rg32Sint".to_string(),
            value: wgpu::TextureFormat::Rg32Sint,
            info: wgpu::TextureFormat::Rg32Sint.describe(),
        },
        TextureFormatInfo {
            name: "Rg32Uint".to_string(),
            value: wgpu::TextureFormat::Rg32Uint,
            info: wgpu::TextureFormat::Rg32Uint.describe(),
        },
        TextureFormatInfo {
            name: "Rg8Sint".to_string(),
            value: wgpu::TextureFormat::Rg8Sint,
            info: wgpu::TextureFormat::Rg8Sint.describe(),
        },
        TextureFormatInfo {
            name: "Rg8Snorm".to_string(),
            value: wgpu::TextureFormat::Rg8Snorm,
            info: wgpu::TextureFormat::Rg8Snorm.describe(),
        },
        TextureFormatInfo {
            name: "Rg8Uint".to_string(),
            value: wgpu::TextureFormat::Rg8Uint,
            info: wgpu::TextureFormat::Rg8Uint.describe(),
        },
        TextureFormatInfo {
            name: "Rg8Unorm".to_string(),
            value: wgpu::TextureFormat::Rg8Unorm,
            info: wgpu::TextureFormat::Rg8Unorm.describe(),
        },
        TextureFormatInfo {
            name: "Rgb10a2Unorm".to_string(),
            value: wgpu::TextureFormat::Rgb10a2Unorm,
            info: wgpu::TextureFormat::Rgb10a2Unorm.describe(),
        },
        TextureFormatInfo {
            name: "Rgb9e5Ufloat".to_string(),
            value: wgpu::TextureFormat::Rgb9e5Ufloat,
            info: wgpu::TextureFormat::Rgb9e5Ufloat.describe(),
        },
        TextureFormatInfo {
            name: "Rgba16Float".to_string(),
            value: wgpu::TextureFormat::Rgba16Float,
            info: wgpu::TextureFormat::Rgba16Float.describe(),
        },
        TextureFormatInfo {
            name: "Rgba16Sint".to_string(),
            value: wgpu::TextureFormat::Rgba16Sint,
            info: wgpu::TextureFormat::Rgba16Sint.describe(),
        },
        TextureFormatInfo {
            name: "Rgba16Snorm".to_string(),
            value: wgpu::TextureFormat::Rgba16Snorm,
            info: wgpu::TextureFormat::Rgba16Snorm.describe(),
        },
        TextureFormatInfo {
            name: "Rgba16Uint".to_string(),
            value: wgpu::TextureFormat::Rgba16Uint,
            info: wgpu::TextureFormat::Rgba16Uint.describe(),
        },
        TextureFormatInfo {
            name: "Rgba16Unorm".to_string(),
            value: wgpu::TextureFormat::Rgba16Unorm,
            info: wgpu::TextureFormat::Rgba16Unorm.describe(),
        },
        TextureFormatInfo {
            name: "Rgba32Float".to_string(),
            value: wgpu::TextureFormat::Rgba32Float,
            info: wgpu::TextureFormat::Rgba32Float.describe(),
        },
        TextureFormatInfo {
            name: "Rgba32Sint".to_string(),
            value: wgpu::TextureFormat::Rgba32Sint,
            info: wgpu::TextureFormat::Rgba32Sint.describe(),
        },
        TextureFormatInfo {
            name: "Rgba32Uint".to_string(),
            value: wgpu::TextureFormat::Rgba32Uint,
            info: wgpu::TextureFormat::Rgba32Uint.describe(),
        },
        TextureFormatInfo {
            name: "Rgba8Sint".to_string(),
            value: wgpu::TextureFormat::Rgba8Sint,
            info: wgpu::TextureFormat::Rgba8Sint.describe(),
        },
        TextureFormatInfo {
            name: "Rgba8Snorm".to_string(),
            value: wgpu::TextureFormat::Rgba8Snorm,
            info: wgpu::TextureFormat::Rgba8Snorm.describe(),
        },
        TextureFormatInfo {
            name: "Rgba8Uint".to_string(),
            value: wgpu::TextureFormat::Rgba8Uint,
            info: wgpu::TextureFormat::Rgba8Uint.describe(),
        },
        TextureFormatInfo {
            name: "Rgba8Unorm".to_string(),
            value: wgpu::TextureFormat::Rgba8Unorm,
            info: wgpu::TextureFormat::Rgba8Unorm.describe(),
        },
        TextureFormatInfo {
            name: "Rgba8UnormSrgb".to_string(),
            value: wgpu::TextureFormat::Rgba8UnormSrgb,
            info: wgpu::TextureFormat::Rgba8UnormSrgb.describe(),
        },
    ]
}
