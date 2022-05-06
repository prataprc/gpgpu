pub mod circle;
pub mod win;

use crate::{BoxLayout, Error, Result, Size, Style};

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

impl Node {
    fn to_flex_node(&self) -> stretch::node::Node {
        dispatch!(get_state, self, as_state().flex_node).unwrap()
    }

    fn to_style(&self) -> Style {
        dispatch!(get_state, self, as_state().style).clone()
    }

    fn to_mut_children(&mut self) -> Option<&mut Vec<Node>> {
        dispatch!(call, self, to_mut_children())
    }

    fn to_extent(&self) -> Option<Size> {
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

    pub fn build_layout(&mut self) -> Result<()> {
        build_layout(&mut self.flex, &mut self.root)
    }

    pub fn compute_layout(
        &mut self,
        width: Option<f32>,
        height: Option<f32>,
    ) -> Result<()> {
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
}

pub fn build_layout(flex: &mut stretch::node::Stretch, node: &mut Node) -> Result<()> {
    use stretch::geometry;

    let flex_style = node.to_style().flex_style;
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
            let size = match node.to_extent() {
                Some(Size { width, height }) => geometry::Size { width, height },
                None => geometry::Size {
                    width: 0.0,
                    height: 0.0,
                },
            };
            node.set_flex_node(err_at!(
                Invalid,
                flex.new_leaf(flex_style, Box::new(move |_| Ok(size)))
            )?);
        }
    }

    Ok(())
}

pub fn gather_layout(flex: &stretch::node::Stretch, node: &mut Node) -> Result<()> {
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
