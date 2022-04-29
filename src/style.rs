use bytemuck::{Pod, Zeroable};

#[derive(Clone, Copy)]
pub enum Length {
    Px(f32),
    None,
}

impl Default for Length {
    fn default() -> Length {
        Length::None
    }
}

impl Length {
    pub fn from_pixel(pixel: f32) -> Length {
        Length::Px(pixel)
    }

    pub fn to_pixel(&self) -> Option<f32> {
        match self {
            Length::Px(val) => Some(*val),
            Length::None => None,
        }
    }
}

#[derive(Clone, Copy)]
pub enum BorderStyle {
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

impl Default for BorderStyle {
    fn default() -> BorderStyle {
        BorderStyle::Solid
    }
}

impl From<BorderStyle> for f32 {
    fn from(bs: BorderStyle) -> f32 {
        match bs {
            BorderStyle::None => 0.0,
            BorderStyle::Solid => 1.0,
            BorderStyle::Dotted => 2.0,
            BorderStyle::Dashed => 3.0,
            BorderStyle::Hidden => 4.0,
            BorderStyle::Inherit => 5.0,
        }
    }
}

#[derive(Clone)]
pub struct Border {
    pub width: Length,
    pub style: BorderStyle,
    pub color: wgpu::Color,
    pub radius: [f32; 4], // top-right, bottom-right, bottom-left, top-left
}

impl Default for Border {
    fn default() -> Border {
        Border {
            width: Length::Px(1.0),
            style: BorderStyle::Solid,
            color: wgpu::Color::WHITE,
            radius: Default::default(),
        }
    }
}

pub struct Style {
    pub fg: wgpu::Color,
    pub bg: wgpu::Color,
    pub min_width: Length,
    pub width: Length,
    pub max_width: Length,
    pub height: Length,
    pub border: Border,
}

impl Default for Style {
    fn default() -> Style {
        Style {
            fg: wgpu::Color::WHITE,
            bg: wgpu::Color::BLACK,
            min_width: Length::None,
            width: Length::None,
            max_width: Length::None,
            height: Length::None,
            border: Border::default(),
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct UniformBuffer {
    fg: [f32; 4],
    bg: [f32; 4],
    border_color: [f32; 4],
    border_radius: [f32; 4],
    min_width: f32,
    width: f32,
    max_width: f32,
    height: f32,
    border_width: f32,
    border_style: f32,
    padding: [f32; 2],
}

impl UniformBuffer {
    const SIZE: usize = (4 * 4) + (4 * 4) + (4 * 4) + (4 * 4) + (4 * 6) + (4 * 2);
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
            border_color: {
                let wgpu::Color { r, g, b, a } = self.border.color;
                [r as f32, g as f32, b as f32, a as f32]
            },
            border_radius: self.border.radius,
            min_width: self.min_width.to_pixel().unwrap_or(-1.0),
            width: self.width.to_pixel().unwrap_or(-1.0),
            max_width: self.max_width.to_pixel().unwrap_or(-1.0),
            height: self.height.to_pixel().unwrap_or(-1.0),
            border_width: self.border.width.to_pixel().unwrap_or(-1.0),
            border_style: self.border.style.into(),
            padding: Default::default(),
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
