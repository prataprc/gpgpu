use bytemuck::{Pod, Zeroable};

use crate::{Location, Resize, Size};

pub const DEFAULT_FONT_SIZE: f32 = 15.0; // in pixels.

#[derive(Copy, Clone, Debug)]
pub struct Style {
    pub font_size: stretch::number::Number, // in pixels
    pub border: Border,
    pub flex_style: stretch::style::Style,
    pub fg: wgpu::Color,
    pub bg: wgpu::Color,
    scale_factor: f32,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            font_size: stretch::number::Number::Defined(DEFAULT_FONT_SIZE),
            fg: wgpu::Color::WHITE,
            bg: wgpu::Color::BLACK,
            border: Border::default(),
            flex_style: stretch::style::Style::default(),
            scale_factor: crate::SCALE_FACTOR,
        }
    }
}

impl Resize for Style {
    fn resize(&mut self, _: Size, scale_factor: Option<f32>) {
        if let Some(scale_factor) = scale_factor {
            self.scale_factor = scale_factor;
        }
    }

    fn computed(&self) -> Self {
        let factor = self.scale_factor;
        let flex_style = {
            let flex = self.flex_style;
            stretch::style::Style {
                margin: scale_rect(flex.margin, factor),
                padding: scale_rect(flex.padding, factor),
                border: scale_rect(flex.border, factor),
                flex_basis: scale_dimension(flex.flex_basis, factor),
                size: scale_size(flex.size, factor),
                min_size: scale_size(flex.min_size, factor),
                max_size: scale_size(flex.max_size, factor),
                ..self.flex_style
            }
        };
        Style {
            font_size: self.font_size * factor,
            border: self.border.scale(factor),
            flex_style,
            ..*self
        }
    }
}

impl Style {
    pub fn set_font_size(&mut self, size: f32) -> &mut Self {
        self.font_size = stretch::number::Number::Defined(size);
        self
    }

    pub fn set_fg(&mut self, fg: wgpu::Color) -> &mut Self {
        self.fg = fg;
        self
    }

    pub fn set_bg(&mut self, bg: wgpu::Color) -> &mut Self {
        self.bg = bg;
        self
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) -> &mut Self {
        self.flex_style.aspect_ratio = stretch::number::Number::Defined(aspect_ratio);
        self
    }

    pub fn set_size(&mut self, size: Size) -> &mut Self {
        self.flex_style.size = size.into();
        self
    }

    pub fn set_absolute_position(&mut self, loc: Location, size: Size) -> &mut Self {
        use stretch::{
            geometry::Rect,
            style::{Dimension, PositionType},
        };

        self.flex_style.position_type = PositionType::Absolute;
        self.flex_style.position = Rect {
            start: Dimension::Points(loc.x),
            end: Dimension::Points(loc.x + size.width),
            top: Dimension::Points(loc.y),
            bottom: Dimension::Points(loc.y + size.height),
        };
        self
    }

    pub fn set_relative_position(&mut self, loc: Location, size: Size) -> &mut Self {
        use stretch::{
            geometry::Rect,
            style::{Dimension, PositionType},
        };

        self.flex_style.position_type = PositionType::Relative;
        self.flex_style.position = Rect {
            start: Dimension::Points(loc.x),
            end: Dimension::Points(loc.x + size.width),
            top: Dimension::Points(loc.y),
            bottom: Dimension::Points(loc.y + size.height),
        };
        self
    }
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub struct Border {
    pub style: StyleBorder,
    pub width: stretch::geometry::Rect<stretch::style::Dimension>,
    pub color: wgpu::Color,
    pub radius: stretch::geometry::Rect<stretch::style::Dimension>,
}

impl Default for Border {
    fn default() -> Border {
        use stretch::geometry::Rect;

        Border {
            style: StyleBorder::default(),
            width: Rect::default(),
            color: wgpu::Color::WHITE,
            radius: Rect::default(),
        }
    }
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

impl Style {
    pub fn to_bind_content(&self) -> Vec<u8> {
        let ub = UniformBuffer {
            fg: to_rgba8unorm_color(self.fg),
            bg: to_rgba8unorm_color(self.bg),
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

// TODO: remove this if not required.
//fn translate_rect(
//    rect: stretch::geometry::Rect<stretch::style::Dimension>,
//    offset: Location,
//) -> stretch::geometry::Rect<stretch::style::Dimension> {
//    stretch::geometry::Rect {
//        start: translate_dimension(rect.start, offset.x),
//        end: translate_dimension(rect.end, offset.x),
//        top: translate_dimension(rect.top, offset.y),
//        bottom: translate_dimension(rect.bottom, offset.y),
//    }
//}

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

// TODO: remove this if not required.
//fn translate_dimension(
//    dimen: stretch::style::Dimension,
//    offset: f32,
//) -> stretch::style::Dimension {
//    match dimen {
//        stretch::style::Dimension::Points(val) => {
//            stretch::style::Dimension::Points(val + offset)
//        }
//        val => val,
//    }
//}
