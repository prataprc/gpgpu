use crate::{dom, ColorTarget, Context, Result, Size, State, Style, Viewport};

pub struct Win {
    state: State<()>,
    children: Vec<dom::Node>,
}

impl AsRef<State<()>> for Win {
    fn as_ref(&self) -> &State<()> {
        &self.state
    }
}

impl AsMut<State<()>> for Win {
    fn as_mut(&mut self) -> &mut State<()> {
        &mut self.state
    }
}

impl dom::Domesticate for Win {
    fn to_mut_children(&mut self) -> Option<&mut Vec<dom::Node>> {
        Some(&mut self.children)
    }

    fn resize(&mut self, size: Size, scale_factor: Option<f32>) {
        self.state.resize(size, scale_factor);
        for child in self.children.iter_mut() {
            child.resize(size, scale_factor)
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

impl Win {
    pub fn new(children: Vec<dom::Node>) -> Self {
        use stretch::{geometry::Size, style::Dimension};

        let mut style = Style::default();
        style.flex_style.size = Size {
            width: Dimension::Percent(1.0),
            height: Dimension::Percent(1.0),
        };
        Win {
            state: State {
                style,
                ..State::default()
            },
            children,
        }
    }

    pub fn print(&self, prefix: &str) {
        println!("{}node.Win @ {}", prefix, self.state.box_layout);
        let prefix = "".to_string() + prefix + "  ";
        for child in self.children.iter() {
            child.print(&prefix)
        }
    }
}
