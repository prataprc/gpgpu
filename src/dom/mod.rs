use log::trace;

pub mod circle;
pub mod win;

use crate::{BoxLayout, ColorTarget, Context, Error, Location, Result, Size, Style};

macro_rules! dispatch {
    (call, $this:ident, $($toks:tt)*) => {
        match $this {
            Node::Win(val) => val.$($toks)*,
            Node::Circle(val) => val.$($toks)*,
        }
    };
    (get_state, $this:ident, $($toks:tt)*) => {
        match $this {
            Node::Win(val) => val.$($toks)*,
            Node::Circle(val) => val.$($toks)*,
        }
    };
    (set_state, $this:ident, $($toks:tt)*) => {
        match $this {
            Node::Win(val) => &mut val.$($toks)*,
            Node::Circle(val) => &mut val.$($toks)*,
        }
    };
}

pub enum Node {
    Win(win::Win),
    Circle(circle::Circle),
}

impl From<win::Win> for Node {
    fn from(val: win::Win) -> Node {
        Node::Win(val)
    }
}

impl From<circle::Circle> for Node {
    fn from(val: circle::Circle) -> Node {
        Node::Circle(val)
    }
}

impl Node {
    fn to_flex_node(&self) -> stretch::node::Node {
        dispatch!(get_state, self, as_state().flex_node).unwrap()
    }

    fn to_computed_style(&self) -> Style {
        dispatch!(get_state, self, as_state().computed_style).clone()
    }

    fn to_mut_children(&mut self) -> Option<&mut Vec<Node>> {
        dispatch!(call, self, to_mut_children())
    }

    fn to_extent(&self) -> Size {
        dispatch!(call, self, to_extent())
    }

    fn set_flex_node(&mut self, flex_node: stretch::node::Node) {
        let p = dispatch!(set_state, self, as_mut_state().flex_node);
        *p = Some(flex_node);
    }

    fn set_box_layout(&mut self, box_layout: BoxLayout) {
        let p = dispatch!(set_state, self, as_mut_state().box_layout);
        *p = box_layout;
    }

    fn print(&self, prefix: &str) {
        dispatch!(call, self, print(prefix))
    }
}

impl Node {
    fn transform(&mut self, offset: Location, scale_factor: f32) {
        dispatch!(call, self, transform(offset, scale_factor))
    }

    fn redraw(
        &mut self,
        context: &Context,
        encoder: &mut wgpu::CommandEncoder,
        target: &mut ColorTarget,
    ) -> Result<()> {
        use std::mem;

        let view_port = {
            let view_port = dispatch!(get_state, self, as_state().to_viewport());
            mem::replace(&mut target.view_port, view_port)
        };
        dispatch!(call, self, redraw(context, encoder, target))?;
        let _view_port = mem::replace(&mut target.view_port, view_port);
        Ok(())
    }
}

pub struct Dom {
    root: Node,
    flex: stretch::node::Stretch,
}

impl Dom {
    pub fn new(win: win::Win) -> Dom {
        let root = Node::Win(win);
        Dom {
            root: root,
            flex: stretch::node::Stretch::new(),
        }
    }

    pub fn compute_layout(
        &mut self,
        width: Option<f32>,
        height: Option<f32>,
    ) -> Result<()> {
        build_layout(&mut self.flex, &mut self.root)?;

        let size = stretch::geometry::Size {
            width: match width {
                Some(w) => stretch::number::Number::Defined(w),
                None => stretch::number::Number::Undefined,
            },
            height: match height {
                Some(h) => stretch::number::Number::Defined(h),
                None => stretch::number::Number::Undefined,
            },
        };
        let flex_node = self.root.to_flex_node();
        err_at!(Invalid, self.flex.compute_layout(flex_node, size))?;
        gather_layout(&self.flex, &mut self.root)
    }

    pub fn to_extent(&self) -> Size {
        self.root.to_extent()
    }

    pub fn print(&self) {
        self.root.print("")
    }
}

impl Dom {
    pub fn transform(&mut self, offset: Location, scale_factor: f32) {
        self.root.transform(offset, scale_factor)
    }

    pub fn redraw(
        &mut self,
        context: &Context,
        encoder: &mut wgpu::CommandEncoder,
        target: &mut ColorTarget,
    ) -> Result<()> {
        self.root.redraw(context, encoder, target)
    }
}

fn build_layout(flex: &mut stretch::node::Stretch, node: &mut Node) -> Result<()> {
    use stretch::geometry;

    let flex_style = node.to_computed_style().flex_style;
    match node.to_mut_children() {
        Some(nodes) => {
            let mut children = vec![];
            for node in nodes.iter_mut() {
                build_layout(flex, node)?;
                children.push(node.to_flex_node());
            }
            let flex_node = err_at!(Invalid, flex.new_node(flex_style, children))?;
            node.set_flex_node(flex_node);
        }
        None => {
            let size: geometry::Size<f32> = node.to_extent().into();
            node.set_flex_node(err_at!(
                Invalid,
                flex.new_leaf(
                    flex_style,
                    Box::new(move |x| {
                        trace!("{:?}, {:?}", x, size);
                        Ok(size)
                    })
                )
            )?);
        }
    }

    Ok(())
}

fn gather_layout(flex: &stretch::node::Stretch, node: &mut Node) -> Result<()> {
    node.set_box_layout(
        err_at!(Invalid, flex.layout(node.to_flex_node()))?
            .clone()
            .into(),
    );
    if let Some(nodes) = node.to_mut_children() {
        for node in nodes.iter_mut() {
            gather_layout(flex, node)?
        }
    }

    Ok(())
}
