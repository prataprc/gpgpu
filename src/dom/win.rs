use crate::{dom, ColorTarget, Context, Location, Result, Size, State, Style};

pub struct Win {
    state: State<()>,
    children: Vec<dom::Node>,
}

impl Win {
    pub fn new(size: Size, children: Vec<dom::Node>) -> Self {
        let mut style = Style::default();
        style.flex_style.size = Size {
            width: size.width,
            height: size.height,
        }
        .into();
        Win {
            state: State {
                style,
                ..State::default()
            },
            children,
        }
    }

    pub fn set_size(&mut self, width: f32, height: f32) -> &mut Self {
        use stretch::{geometry::Size, style::Dimension};

        self.state.style.flex_style.size = Size {
            width: Dimension::Points(width),
            height: Dimension::Points(height),
        };
        self
    }

    pub fn print(&self, prefix: &str) {
        println!("{}node.Win @ {}", prefix, self.state.box_layout);
        let prefix = "".to_string() + prefix + "  ";
        for child in self.children.iter() {
            child.print(&prefix)
        }
    }
}

impl Win {
    pub fn as_state(&self) -> &State<()> {
        &self.state
    }

    pub fn as_mut_state(&mut self) -> &mut State<()> {
        &mut self.state
    }

    pub fn to_mut_children(&mut self) -> Option<&mut Vec<dom::Node>> {
        Some(&mut self.children)
    }

    pub fn to_extent(&self) -> Size {
        self.state.style.flex_style.size.into()
    }

    pub fn transform(&mut self, offset: Location, scale_factor: f32) {
        self.state.transform(offset, scale_factor);
        for child in self.children.iter_mut() {
            child.transform(offset, scale_factor)
        }
    }

    pub fn redraw(
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
