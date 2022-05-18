use bytemuck::{Pod, Zeroable};
use cgmath::Point2;

use std::{
    fmt,
    ops::{Deref, DerefMut},
    result,
};

use crate::Style;

// Resize
// Dimension, Extent, Origin, Rect, State, AspectRatio, BoxVertex, Viewport, GlyphRect

pub trait Resize {
    fn resize(&mut self, extent: Extent, scale_factor: Option<f32>);

    fn computed(&self) -> Self;
}

impl Resize for () {
    fn resize(&mut self, _: Extent, _scale_factor: Option<f32>) {
        ()
    }

    fn computed(&self) -> Self {
        ()
    }
}

pub enum Dimension {
    Undefined,
    Auto,
    Points(f32),
    Percent(f32),
}

impl From<stretch::style::Dimension> for Dimension {
    fn from(val: stretch::style::Dimension) -> Dimension {
        match val {
            stretch::style::Dimension::Undefined => Dimension::Undefined,
            stretch::style::Dimension::Auto => Dimension::Auto,
            stretch::style::Dimension::Points(val) => Dimension::Points(val),
            stretch::style::Dimension::Percent(val) => Dimension::Percent(val),
        }
    }
}

impl From<Dimension> for stretch::style::Dimension {
    fn from(val: Dimension) -> stretch::style::Dimension {
        match val {
            Dimension::Undefined => stretch::style::Dimension::Undefined,
            Dimension::Auto => stretch::style::Dimension::Auto,
            Dimension::Points(val) => stretch::style::Dimension::Points(val),
            Dimension::Percent(val) => stretch::style::Dimension::Percent(val),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Extent {
    pub width: f32,
    pub height: f32,
}

impl From<wgpu::Extent3d> for Extent {
    fn from(val: wgpu::Extent3d) -> Self {
        Extent {
            width: val.width as f32,
            height: val.height as f32,
        }
    }
}

impl From<winit::dpi::PhysicalSize<u32>> for Extent {
    fn from(val: winit::dpi::PhysicalSize<u32>) -> Extent {
        Extent {
            width: val.width as f32,
            height: val.height as f32,
        }
    }
}

impl From<Extent> for stretch::geometry::Size<stretch::style::Dimension> {
    fn from(val: Extent) -> Self {
        use stretch::style::Dimension;

        stretch::geometry::Size {
            width: Dimension::Points(val.width),
            height: Dimension::Points(val.height),
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Origin {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Default)]
pub struct Rect {
    pub origin: Origin,
    pub extent: Extent,
}

impl From<Rect> for stretch::geometry::Rect<stretch::style::Dimension> {
    fn from(val: Rect) -> Self {
        use stretch::style::Dimension;

        stretch::geometry::Rect {
            start: Dimension::Points(val.origin.x),
            end: Dimension::Points(val.origin.x + val.extent.width),
            top: Dimension::Points(val.origin.y),
            bottom: Dimension::Points(val.origin.y + val.extent.height),
        }
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "Box<{},{}..{},{}>",
            self.origin.x, self.origin.y, self.extent.width, self.extent.height
        )
    }
}

impl From<stretch::result::Layout> for Rect {
    fn from(val: stretch::result::Layout) -> Rect {
        let stretch::result::Layout {
            size: stretch::geometry::Size { width, height },
            location: stretch::geometry::Point { x, y },
            ..
        } = val;

        let origin = Origin { x, y };
        let extent = Extent { width, height };
        Rect { origin, extent }
    }
}

impl Rect {
    pub fn to_aspect_ratio(&self) -> AspectRatio {
        if self.extent.width > self.extent.height {
            let x = 1.0;
            let y = self.extent.height / self.extent.width;
            AspectRatio((x, y).into())
        } else {
            let x = self.extent.width / self.extent.height;
            let y = 1.0;
            AspectRatio((x, y).into())
        }
    }

    // normalized-compute-coordinate
    pub fn to_ncc(&self, point: Point2<f32>) -> Point2<f32> {
        let ar = self.to_aspect_ratio();
        let x = (point.x / self.extent.width) * ar.x;
        let y = (point.y / self.extent.height) * ar.y;
        (x, y).into()
    }

    // normalized-device-coordinate
    pub fn to_ndc(&self, point: Point2<f32>) -> Point2<f32> {
        let ar = self.to_aspect_ratio();
        let x = (point.x / self.extent.width) / ar.x;
        let y = (point.y / self.extent.height) / ar.y;
        (x, y).into()
    }

    pub fn to_origin(&self) -> Point2<f32> {
        (self.origin.x, self.origin.y).into()
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

/// State common to widgets and doms.
pub struct State<A> {
    pub style: Style,
    pub computed_style: Style,
    pub flex_node: Option<stretch::node::Node>,
    pub rect: Rect,
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
            rect: Rect::default(),
            attrs: A::default(),
            computed_attrs: A::default(),
        }
    }
}

impl<A> AsRef<Style> for State<A> {
    fn as_ref(&self) -> &Style {
        &self.style
    }
}

impl<A> AsMut<Style> for State<A> {
    fn as_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl<A> AsRef<Rect> for State<A> {
    fn as_ref(&self) -> &Rect {
        &self.rect
    }
}

impl<A> AsMut<Rect> for State<A> {
    fn as_mut(&mut self) -> &mut Rect {
        &mut self.rect
    }
}

impl<A> State<A> {
    pub fn resize(&mut self, extent: Extent, scale_factor: Option<f32>)
    where
        A: Resize + fmt::Debug,
    {
        self.style.resize(extent, scale_factor);
        self.computed_style = self.style.computed();

        self.attrs.resize(extent, scale_factor);
        self.computed_attrs = self.attrs.computed();
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
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

impl From<Rect> for Viewport {
    fn from(val: Rect) -> Viewport {
        Viewport {
            x: val.origin.x,
            y: val.origin.y,
            width: val.extent.width,
            height: val.extent.height,
            min_depth: 1.0,
            max_depth: 1.0,
        }
    }
}

impl Viewport {
    pub fn root_viewport(extent: wgpu::Extent3d) -> Viewport {
        Viewport {
            x: 0.0,
            y: 0.0,
            width: extent.width as f32,
            height: extent.height as f32,
            min_depth: 1.0,
            max_depth: 1.0,
        }
    }

    pub fn set_viewport(&self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_viewport(
            self.x,
            self.y,
            self.width,
            self.height,
            self.min_depth,
            self.max_depth,
        );
    }
}

#[derive(Copy, Clone)]
pub struct GlyphRect {
    pub x_min: f32,
    pub y_min: f32,
    pub x_max: f32,
    pub y_max: f32,
}

impl From<ttf_parser::Rect> for GlyphRect {
    fn from(val: ttf_parser::Rect) -> GlyphRect {
        GlyphRect {
            x_min: val.x_min as f32,
            y_min: val.y_min as f32,
            x_max: val.x_max as f32,
            y_max: val.y_max as f32,
        }
    }
}

impl GlyphRect {
    pub fn scale(&self, factor: f32) -> GlyphRect {
        GlyphRect {
            x_min: self.x_min * factor,
            y_min: self.y_min * factor,
            x_max: self.x_max * factor,
            y_max: self.y_max * factor,
        }
    }
}

impl fmt::Debug for GlyphRect {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "({},{})->({},{})",
            self.x_min, self.y_min, self.x_max, self.y_max
        )
    }
}
