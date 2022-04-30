use bytemuck::{Pod, Zeroable};

const DEFAULT_FONT_SIZE: u32 = 15; // in pixels.

macro_rules! get_style {
    ($this:ident, $parent:ident, $field:ident, $default:expr) => {{
        match $this.$field {
            Some(val) => val,
            None => match $parent.$field {
                Some(val) => val,
                None => $default,
            },
        }
    }};
}

pub struct ComputedStyle {
    font_size: u32, // in pixels
    fg: wgpu::Color,
    bg: wgpu::Color,
    min_width: u32, // in pixels
    max_width: u32, // in pixels
    width: u32,     // in pixels
    height: u32,    // in pixels
    border_style: StyleBorder,
    border_width: u32, // in pixels
    border_color: wgpu::Color,
    border_radius: [u32; 4], // tr, bottom-right, bottom-left and top-left in pixels
    padding: [u32; 4],       // top, right, bottom and left in pixels
    margin: [u32; 4],        // top, right, bottom and left in pixels
    _padding: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct UniformBuffer {
    fg: [f32; 4],
    bg: [f32; 4],
    border_color: [f32; 4],
    border_radius: [u32; 4],
    min_width: u32,
    width: u32,
    max_width: u32,
    height: u32,
    border_width: u32,
    border_style: u32,
    padding: [u32; 4],
    margin: [u32; 4],
}

impl UniformBuffer {
    const SIZE: usize = (16 * 4) + (6 * 4) + (8 * 4) + 8;
}

impl ComputedStyle {
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
                let wgpu::Color { r, g, b, a } = self.border_color;
                [r as f32, g as f32, b as f32, a as f32]
            },
            border_radius: self.border_radius,
            min_width: self.min_width,
            width: self.width,
            max_width: self.max_width,
            height: self.height,
            border_width: self.border_width,
            border_style: self.border_style.into(),
            padding: self.padding,
            margin: self.margin,
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
pub enum Display {
    Block,
    Inline,
    Hidden,
}

#[derive(Clone, Copy)]
pub enum FlowAlign {
    Left,
    Right,
    Center,
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

impl From<StyleBorder> for u32 {
    fn from(bs: StyleBorder) -> u32 {
        match bs {
            StyleBorder::None => 0,
            StyleBorder::Solid => 1,
            StyleBorder::Dotted => 2,
            StyleBorder::Dashed => 3,
            StyleBorder::Hidden => 4,
            StyleBorder::Inherit => 5,
        }
    }
}

pub enum StyleValue<T> {
    Initial(T),
    Inherit(T),
    Default(T),
    Dynamic,
}

macro_rules! create_style_value_type {
    ($(
        ($type_name:ident, $style_field:ident, $ty_param:ty, $default:expr),
    )*) => {
        $(
            struct $type_name($ty_param);
            impl Default for $type_name {
                fn default() -> $type_name {
                    $type_name($default)
                }
            }
        )*

        pub struct Style {
            $(
                $style_field: $type_name,
            )*
        }

        impl Default for Style {
            fn default() -> Style {
                Style {
                    $(
                        $style_field: $type_name::default(),
                    )*
                }
            }
        }
    };
}

create_style_value_type![
    (
        font_size,
        FontSize,
        StyleValue<u32>,
        StyleValue::Default(DEFAULT_FONT_SIZE)
    ),
    (
        fg,
        Fg,
        StyleValue<wgpu::Color>,
        StyleValue::Default(wgpu::Color::WHITE)
    ),
    (
        bg,
        Bg,
        StyleValue<wgpu::Color>,
        StyleValue::Default(wgpu::Color::BLACK)
    ),
    (min_width, MinWidth, StyleValue<u32>, StyleValue::Default(u32::MIN)),
    (max_width, MaxWidth, StyleValue<u32>, StyleValue::Default(u32::MAX)),
    (width, Width, StyleValue<u32>, StyleValue::Dynamic),
    (height, Height, StyleValue<u32>, StyleValue::Dynamic),
    (
        border_style,
        StyleBorder,
        StyleValue<StyleBorder>,
        StyleValue::Default(StyleBorder::None)
    ),
    (
        border_width,
        BorderWidth,
        StyleValue<u32>,
        StyleValue::Default(1)
    ),
    (
        border_color,
        BorderColor,
        StyleValue<wgpu::Color>,
        StyleValue::Default(wgpu::Color::WHITE)
    ),
    (
        border_radius,
        BorderRadius,
        StyleValue<[u32; 4]>,
        StyleValue::Default([0_u32; 4])
    ),
    (
        padding,
        Padding,
        StyleValue<[u32; 4]>,
        StyleValue::Default([0_u32; 4])
    ),
    (
        margin,
        Margin,
        StyleValue<[u32; 4]>,
        StyleValue::Default([0_u32; 4])
    ),
];
