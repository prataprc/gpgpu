use bytemuck::{Pod, Zeroable};

use std::ops::{Deref, DerefMut};

use crate::Style;

/// Two Dimensional transforms, translate, scale.
pub trait Transform2D {
    fn transform(&self, offset: Location, scale_factor: f32) -> Self;
}

// screen coordinate, in pixels
#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

// screen coordinate, in pixels
#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

/// State common to widgets and doms.
pub struct State<T> {
    pub scale_factor: f32,
    pub offset: Location,
    pub style: Style,
    pub computed_style: Style,
    pub flex_node: Option<stretch::node::Node>,
    pub box_layout: BoxLayout,
    pub attrs: T,
    pub computed_attrs: T,
}

impl<T> Default for State<T>
where
    T: Default,
{
    fn default() -> State<T> {
        State {
            scale_factor: 1.0,
            offset: Location { x: 0.0, y: 0.0 },
            style: Style::default(),
            computed_style: Style::default(),
            flex_node: None,
            box_layout: BoxLayout::default(),
            attrs: T::default(),
            computed_attrs: T::default(),
        }
    }
}

impl<T> AsRef<Style> for State<T> {
    fn as_ref(&self) -> &Style {
        &self.style
    }
}

impl<T> AsMut<Style> for State<T> {
    fn as_mut(&mut self) -> &mut Style {
        &mut self.style
    }
}

impl<T> AsRef<BoxLayout> for State<T> {
    fn as_ref(&self) -> &BoxLayout {
        &self.box_layout
    }
}

impl<T> AsMut<BoxLayout> for State<T> {
    fn as_mut(&mut self) -> &mut BoxLayout {
        &mut self.box_layout
    }
}

impl<T> Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.attrs
    }
}

impl<T> DerefMut for State<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.attrs
    }
}

impl<T> State<T> {
    pub fn translate(&mut self, offset: Location) -> &mut Self {
        self.offset = offset;
        self
    }

    pub fn scale(&mut self, factor: f32) -> &mut Self {
        self.scale_factor = factor;
        self
    }

    pub fn transform(&mut self)
    where
        T: Transform2D,
    {
        self.computed_style = self.style.transform(self.offset, self.scale_factor);
        self.computed_attrs = self.attrs.transform(self.offset, self.scale_factor);
    }

    pub fn as_computed_style(&self) -> &Style {
        &self.computed_style
    }

    pub fn as_computed_attrs(&self) -> &T {
        &self.computed_attrs
    }
}

#[derive(Clone, Copy, Default)]
pub struct BoxLayout {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
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
    pub fn to_vertices(&self, size: wgpu::Extent3d) -> Vec<BoxVertex> {
        let tl = [
            ((self.x / (size.width as f32)) * 2.0) - 1.0,
            1.0 - ((self.y / (size.height as f32)) * 2.0),
            0.0,
            1.0,
        ];
        let tr = [
            (((self.x + self.w) / (size.width as f32)) * 2.0) - 1.0,
            1.0 - ((self.y / (size.height as f32)) * 2.0),
            0.0,
            1.0,
        ];
        let br = [
            (((self.x + self.w) / (size.width as f32)) * 2.0) - 1.0,
            1.0 - (((self.y + self.h) / (size.height as f32)) * 2.0),
            0.0,
            1.0,
        ];
        let bl = [
            ((self.x / (size.width as f32)) * 2.0) - 1.0,
            1.0 - (((self.y + self.h) / (size.height as f32)) * 2.0),
            0.0,
            1.0,
        ];
        vec![
            BoxVertex { position: tl },
            BoxVertex { position: bl },
            BoxVertex { position: tr },
            BoxVertex { position: tr },
            BoxVertex { position: bl },
            BoxVertex { position: br },
        ]
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct BoxVertex {
    position: [f32; 4],
}

impl BoxVertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
        0 => Float32x4,
    ];
}

impl BoxVertex {
    pub fn to_vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<BoxVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}
