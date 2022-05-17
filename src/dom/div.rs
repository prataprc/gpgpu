use crate::{dom, ColorTarget, Context, Extent, Result, State, Viewport};

pub struct Div {
    state: State<()>,
    children: Vec<dom::Node>,
}

impl AsRef<State<()>> for Div {
    fn as_ref(&self) -> &State<()> {
        &self.state
    }
}

impl AsMut<State<()>> for Div {
    fn as_mut(&mut self) -> &mut State<()> {
        &mut self.state
    }
}

impl dom::Domesticate for Div {
    fn to_mut_children(&mut self) -> Option<&mut Vec<dom::Node>> {
        Some(&mut self.children)
    }

    fn resize(&mut self, extent: Extent, scale_factor: Option<f32>) {
        self.state.resize(extent, scale_factor);
        for child in self.children.iter_mut() {
            child.resize(extent, scale_factor)
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
        for child in self.children.iter_mut() {
            child.redraw(context, encoder, target)?
        }

        Ok(())
    }
}

impl Div {
    pub fn new(children: Vec<dom::Node>) -> Self {
        Div {
            state: State::default(),
            children,
        }
    }

    pub fn print(&self, prefix: &str) {
        println!("{}dom.Div @ {}", prefix, self.state.rect);
        let prefix = "".to_string() + prefix + "  ";
        for child in self.children.iter() {
            child.print(&prefix)
        }
    }
}
