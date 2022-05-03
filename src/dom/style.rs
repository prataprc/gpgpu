use bytemuck::{Pod, Zeroable};

pub const DEFAULT_FONT_SIZE: f32 = 15.0; // in pixels.

#[derive(Default, Copy, Clone)]
pub struct Style {
    pub font_size: f32, // in pixels
    pub fg: wgpu::Color,
    pub bg: wgpu::Color,
    pub border: Border,
    pub flex: stretch::style::Style,
}

#[derive(Default, Copy, Clone)]
pub struct StyleStyle {
    pub style: Style,
    pub computed: Style,
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

#[derive(Default, Copy, Clone)]
pub struct Border {
    pub style: StyleBorder,
    pub width: stretch::geometry::Rect<stretch::style::Dimension>,
    pub color: wgpu::Color,
    pub radius: stretch::geometry::Rect<stretch::style::Dimension>,
}

impl Border {
    fn scale(&self, factor: f32) -> Border {
        Border {
            width: scale_rect(self.width, factor),
            radius: scale_rect(self.radius, factor),
            ..self.clone()
        }
    }
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

impl StyleStyle {
    pub fn to_bind_content(&self) -> Vec<u8> {
        let ub = UniformBuffer {
            fg: to_rgba8unorm_color(self.computed.fg),
            bg: to_rgba8unorm_color(self.computed.bg),
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

impl StyleStyle {
    pub fn new(style: Style) -> StyleStyle {
        StyleStyle {
            style,
            computed: style,
        }
    }

    pub fn set_fg(&mut self, fg: wgpu::Color) -> &mut Self {
        self.style.fg = fg;
        self.computed.fg = fg;
        self
    }

    pub fn set_bg(&mut self, bg: wgpu::Color) -> &mut Self {
        self.style.bg = bg;
        self.computed.bg = bg;
        self
    }

    pub fn scale(&mut self, factor: f32) -> &mut Self {
        self.computed = {
            let flex = {
                let flex = self.style.flex;
                stretch::style::Style {
                    position: scale_rect(flex.position, factor),
                    margin: scale_rect(flex.margin, factor),
                    padding: scale_rect(flex.padding, factor),
                    border: scale_rect(flex.border, factor),
                    flex_basis: scale_dimension(flex.flex_basis, factor),
                    size: scale_size(flex.size, factor),
                    min_size: scale_size(flex.min_size, factor),
                    max_size: scale_size(flex.max_size, factor),
                    ..flex
                }
            };
            Style {
                font_size: self.style.font_size * factor,
                border: self.style.border.scale(factor),
                flex,
                ..self.style
            }
        };
        self
    }

    pub fn with_aspect_ratio(&mut self, aspect_ratio: f32) -> &mut Self {
        self.style.flex.aspect_ratio = stretch::number::Number::Defined(aspect_ratio);
        self.computed.flex.aspect_ratio = stretch::number::Number::Defined(aspect_ratio);
        self
    }
}

pub fn to_rgba8unorm_color(color: wgpu::Color) -> [f32; 4] {
    // downgrade from f64 representation of wgpu::Color.
    let wgpu::Color { r, g, b, a } = color;
    [r as f32, g as f32, b as f32, a as f32]
}

fn scale_rect(
    rect: stretch::geometry::Rect<stretch::style::Dimension>,
    factor: f32,
) -> stretch::geometry::Rect<stretch::style::Dimension> {
    stretch::geometry::Rect {
        start: scale_dimension(rect.start, factor),
        end: scale_dimension(rect.end, factor),
        top: scale_dimension(rect.top, factor),
        bottom: scale_dimension(rect.bottom, factor),
    }
}

fn scale_size(
    size: stretch::geometry::Size<stretch::style::Dimension>,
    factor: f32,
) -> stretch::geometry::Size<stretch::style::Dimension> {
    stretch::geometry::Size {
        width: scale_dimension(size.width, factor),
        height: scale_dimension(size.height, factor),
    }
}

fn scale_dimension(
    dimen: stretch::style::Dimension,
    factor: f32,
) -> stretch::style::Dimension {
    match dimen {
        stretch::style::Dimension::Points(val) => {
            stretch::style::Dimension::Points(val * factor)
        }
        val => val,
    }
}
