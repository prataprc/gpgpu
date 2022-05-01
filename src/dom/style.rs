use bytemuck::{Pod, Zeroable};

pub const DEFAULT_FONT_SIZE: f32 = 15.0; // in pixels.

#[derive(Default, Clone)]
pub struct Border {
    style: StyleBorder,
    width: stretch::geometry::Rect<stretch::style::Dimension>,
    color: wgpu::Color,
    radius: stretch::geometry::Rect<stretch::style::Dimension>,
}

#[derive(Default, Clone)]
pub struct Style {
    pub font_size: f32, // in pixels
    pub fg: wgpu::Color,
    pub bg: wgpu::Color,
    pub border: Border,
    pub flex: stretch::style::Style,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct UniformBuffer {
    fg: [f32; 4],
    bg: [f32; 4],
}

impl UniformBuffer {
    const SIZE: usize = (4 * 4) + (4 * 4);
}

impl Style {
    pub fn to_bind_content(&self) -> Vec<u8> {
        let ub = UniformBuffer {
            fg: {
                let wgpu::Color { r, g, b, a } = self.fg;
                [r as f32, g as f32, b as f32, a as f32]
            },
            bg: {
                let wgpu::Color { r, g, b, a } = self.bg;
                [r as f32, g as f32, b as f32, a as f32]
            },
        };

        let contents: [u8; UniformBuffer::SIZE] = bytemuck::cast(ub);
        contents.to_vec()
    }

    pub fn to_bind_group_layout_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
        use wgpu::ShaderStages;

        wgpu::BindGroupLayoutEntry {
            binding,
            visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum StyleBorder {
    /// Specifies no border
    None,
    /// Specifies a solid border
    Solid,
    /// Specifies a dotted border
    Dotted,
    /// Specifies a dashed border
    Dashed,
    /// The same as "none", except in border conflict resolution for table elements
    Hidden,
    /// Inherits this property from its parent element.
    Inherit,
}

impl Default for StyleBorder {
    fn default() -> StyleBorder {
        StyleBorder::None
    }
}
