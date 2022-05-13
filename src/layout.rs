use bytemuck::{Pod, Zeroable};
use cgmath::Point2;

use std::{
    fmt,
    ops::{Deref, DerefMut},
    result,
};

use crate::Style;

pub trait Resize {
    fn resize(&mut self, size: Size);

    fn scale_factor_changed(&mut self, scale_factor: f32);

    fn computed(&self) -> Self;
}

impl Resize for () {
    fn resize(&mut self, _: Size) {
        ()
    }

    fn scale_factor_changed(&mut self, _: f32) {
        ()
    }

    fn computed(&self) -> Self {
        ()
    }
}

// screen coordinate, in pixels
#[derive(Clone, Copy, Debug, Default)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

// screen coordinate, in pixels
#[derive(Clone, Copy, Debug, Default)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl From<winit::dpi::PhysicalSize<u32>> for Size {
    fn from(val: winit::dpi::PhysicalSize<u32>) -> Size {
        Size {
            width: val.width as f32,
            height: val.height as f32,
        }
    }
}

impl From<Size> for stretch::geometry::Size<stretch::style::Dimension> {
    fn from(val: Size) -> stretch::geometry::Size<stretch::style::Dimension> {
        stretch::geometry::Size {
            width: stretch::style::Dimension::Points(val.width),
            height: stretch::style::Dimension::Points(val.height),
        }
    }
}

impl From<Size> for stretch::geometry::Size<f32> {
    fn from(val: Size) -> stretch::geometry::Size<f32> {
        stretch::geometry::Size {
            width: val.width,
            height: val.height,
        }
    }
}

impl From<stretch::geometry::Size<stretch::style::Dimension>> for Size {
    fn from(val: stretch::geometry::Size<stretch::style::Dimension>) -> Size {
        let width = match val.width {
            stretch::style::Dimension::Points(w) => w,
            _ => 0.0,
        };
        let height = match val.width {
            stretch::style::Dimension::Points(h) => h,
            _ => 0.0,
        };
        Size { width, height }
    }
}

impl From<stretch::geometry::Size<stretch::number::Number>> for Size {
    fn from(val: stretch::geometry::Size<stretch::number::Number>) -> Size {
        let width = match val.width {
            stretch::number::Number::Defined(w) => w,
            _ => 0.0,
        };
        let height = match val.width {
            stretch::number::Number::Defined(h) => h,
            _ => 0.0,
        };
        Size { width, height }
    }
}

/// State common to widgets and doms.
pub struct State<A> {
    pub style: Style,
    pub computed_style: Style,
    pub flex_node: Option<stretch::node::Node>,
    pub box_layout: BoxLayout,
    pub attrs: A,
    pub computed_attrs: A,
}

impl<A> Default for State<A>
where
    A: Default,
{
    fn default() -> State<A> {
        State {
            style: Style::default(),
            computed_style: Style::default(),
            flex_node: None,
            box_layout: BoxLayout::default(),
            attrs: A::default(),
            computed_attrs: A::default(),
        }
    }
}

impl<A> AsRef<BoxLayout> for State<A> {
    fn as_ref(&self) -> &BoxLayout {
        &self.box_layout
    }
}

impl<A> AsMut<BoxLayout> for State<A> {
    fn as_mut(&mut self) -> &mut BoxLayout {
        &mut self.box_layout
    }
}

impl<A> State<A> {
    pub fn resize(&mut self, size: Size)
    where
        A: Resize + fmt::Debug,
    {
        self.style.resize(size);
        self.computed_style = self.style.computed();

        self.attrs.resize(size);
        self.computed_attrs = self.attrs.computed();
    }

    pub fn scale_factor_changed(&mut self, scale_factor: f32)
    where
        A: Resize + fmt::Debug,
    {
        self.style.scale_factor_changed(scale_factor);
        self.computed_style = self.style.computed();

        self.attrs.scale_factor_changed(scale_factor);
        self.computed_attrs = self.attrs.computed();
    }
}

#[derive(Clone, Copy, Default)]
pub struct BoxLayout {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl fmt::Display for BoxLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "Box<{},{}..{},{}>", self.x, self.y, self.w, self.h)
    }
}

impl From<stretch::result::Layout> for BoxLayout {
    fn from(val: stretch::result::Layout) -> BoxLayout {
        let stretch::result::Layout {
            size: stretch::geometry::Size { width, height },
            location: stretch::geometry::Point { x, y },
            ..
        } = val;

        BoxLayout {
            x,
            y,
            w: width,
            h: height,
        }
    }
}

impl BoxLayout {
    pub fn to_aspect_ratio(&self) -> AspectRatio {
        if self.w > self.h {
            let x = 1.0;
            let y = self.h / self.w;
            AspectRatio((x, y).into())
        } else {
            let x = self.w / self.h;
            let y = 1.0;
            AspectRatio((x, y).into())
        }
    }

    pub fn to_ncc(&self, point: Point2<f32>) -> Point2<f32> {
        let ar = self.to_aspect_ratio();
        let x = (point.x / self.w) * ar.x;
        let y = (point.y / self.h) * ar.y;
        (x, y).into()
    }

    pub fn to_ndc(&self, point: Point2<f32>) -> Point2<f32> {
        let ar = self.to_aspect_ratio();
        let x = (point.x / self.w) / ar.x;
        let y = (point.y / self.h) / ar.y;
        (x, y).into()
    }

    pub fn to_viewport(&self) -> Viewport {
        Viewport {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            min_depth: 1.0,
            max_depth: 1.0,
        }
    }

    pub fn to_origin(&self) -> Point2<f32> {
        (self.x, self.y).into()
    }
}

pub struct AspectRatio(Point2<f32>);

impl Deref for AspectRatio {
    type Target = Point2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AspectRatio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct BoxVertex {
    pub position: [f32; 4],
}

impl BoxVertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
        0 => Float32x4,
    ];

    pub fn to_vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<BoxVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl Viewport {
    pub fn root_viewport(size: Size) -> Viewport {
        Viewport {
            x: 0.0,
            y: 0.0,
            w: size.width,
            h: size.height,
            min_depth: 1.0,
            max_depth: 1.0,
        }
    }

    pub fn set_viewport(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_viewport(
            self.x,
            self.y,
            self.w,
            self.h,
            self.min_depth,
            self.max_depth,
        );
    }
}
