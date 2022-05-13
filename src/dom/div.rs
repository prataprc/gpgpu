use crate::{dom, ColorTarget, Context, Result, Size, State, Viewport};

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

    fn resize(&mut self, size: Size) {
        self.state.resize(size);
        for child in self.children.iter_mut() {
            child.resize(size)
        }
    }

    fn scale_factor_changed(&mut self, scale_factor: f32) {
        self.state.scale_factor_changed(scale_factor);
        for child in self.children.iter_mut() {
            child.scale_factor_changed(scale_factor)
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

    pub fn set_size(
        &mut self,
        width: stretch::style::Dimension,
        height: stretch::style::Dimension,
    ) -> &mut Self {
        use stretch::geometry::Size;

        self.state.style.flex_style.size = Size { width, height };
        self
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
        println!("{}dom.Div @ {}", prefix, self.state.box_layout);
        let prefix = "".to_string() + prefix + "  ";
        for child in self.children.iter() {
            child.print(&prefix)
        }
    }
}
