use crate::{dom, primv, ColorTarget, Context, Result, Size, State, Viewport};

pub struct Shape {
    state: State<()>,
    inner: Inner,
}

enum Inner {
    Circle(primv::circle::Circle),
}

impl AsRef<State<()>> for Shape {
    fn as_ref(&self) -> &State<()> {
        &self.state
    }
}

impl AsMut<State<()>> for Shape {
    fn as_mut(&mut self) -> &mut State<()> {
        &mut self.state
    }
}

impl dom::Domesticate for Shape {
    fn to_mut_children(&mut self) -> Option<&mut Vec<dom::Node>> {
        None
    }

    fn resize(&mut self, size: Size) {
        self.state.resize(size);
        match &mut self.inner {
            Inner::Circle(val) => {
                val.resize(size);
            }
        }
    }

    fn scale_factor_changed(&mut self, scale_factor: f32) {
        self.state.scale_factor_changed(scale_factor);
        match &mut self.inner {
            Inner::Circle(val) => {
                val.scale_factor_changed(scale_factor);
            }
        }
    }

    fn to_viewport(&self) -> Viewport {
        self.state.box_layout.to_viewport()
    }

    fn redraw(
        &mut self,
        context: &Context,
        encoder: &mut wgpu::CommandEncoder,
        target: &mut ColorTarget,
    ) -> Result<()> {
        match &mut self.inner {
            Inner::Circle(val) => val.redraw(context, encoder, target),
        }
    }
}
impl Shape {
    pub fn new_circle(val: primv::circle::Circle) -> Self {
        let mut state = State::<()>::default();
        state.style.flex_style.size = val.to_extent().into();
        Shape {
            state,
            inner: Inner::Circle(val),
        }
    }

    pub fn set_position(
        &mut self,
        typ: stretch::style::PositionType,
        position: stretch::geometry::Rect<stretch::style::Dimension>,
    ) -> &mut Self {
        self.state.style.flex_style.position_type = typ;
        self.state.style.flex_style.position = position;
        self
    }

    pub fn print(&self, prefix: &str) {
        println!("{}dom.Shape @ {}", prefix, self.state.box_layout);
        let prefix = "".to_string() + prefix + "  ";
        match &self.inner {
            Inner::Circle(val) => val.print(&prefix),
        }
    }
}
