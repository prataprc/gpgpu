use crate::{dom, primv, ColorTarget, Context, Extent, Result, State, Viewport};

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

    fn resize(&mut self, extent: Extent, scale_factor: Option<f32>) {
        self.state.resize(extent, scale_factor);
        match &mut self.inner {
            Inner::Circle(val) => {
                val.resize(extent, scale_factor);
            }
        }
    }

    fn to_viewport(&self) -> Viewport {
        self.state.rect.into()
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

    pub fn print(&self, prefix: &str) {
        println!("{}dom.Shape @ {}", prefix, self.state.rect);
        let prefix = "".to_string() + prefix + "  ";
        match &self.inner {
            Inner::Circle(val) => val.print(&prefix),
        }
    }
}
