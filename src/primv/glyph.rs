use crate::{dom, fonts, ColorTarget, Context, Result, Size, State, Viewport};

pub struct GlyphBox {
    state: State<()>,
    name: String,
    code_point: u32,
    ch: char,
}

impl AsRef<State<()>> for GlyphBox {
    fn as_ref(&self) -> &State<()> {
        &self.state
    }
}

impl AsMut<State<()>> for GlyphBox {
    fn as_mut(&mut self) -> &mut State<()> {
        &mut self.state
    }
}

impl dom::Domesticate for GlyphBox {
    fn to_mut_children(&mut self) -> Option<&mut Vec<dom::Node>> {
        None
    }

    fn resize(&mut self, size: Size, scale_factor: Option<f32>) {
        self.state.resize(size, scale_factor);
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
        Ok(())
    }
}

impl GlyphBox {
    pub fn new(g: fonts::Glyph) -> GlyphBox {
        GlyphBox {
            state: State::default(),
            name: g.to_name(),
            code_point: g.to_code_point(),
            ch: g.to_char(),
        }
    }

    pub fn print(&self, prefix: &str) {
        println!("{}dom.GlyphBox @ {}", prefix, self.state.box_layout);
    }
}
