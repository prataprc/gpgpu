mod style;
mod win;

pub use style::Style;

use std::ops::{Deref, DerefMut};

use crate::{Error, Result};

macro_rules! derive_dom_attributes {
    ($ty:ty, $state:ident) => {
        impl AsRef<crate::dom::Style> for $ty {
            fn as_ref(&self) -> &crate::dom::Style {
                self.$state.as_ref()
            }
        }

        impl AsMut<crate::dom::Style> for $ty {
            fn as_mut(&mut self) -> &mut crate::dom::Style {
                self.$state.as_mut()
            }
        }

        impl AsRef<crate::dom::BoxLayout> for $ty {
            fn as_ref(&self) -> &crate::dom::BoxLayout {
                self.$state.as_ref()
            }
        }

        impl AsMut<crate::dom::BoxLayout> for $ty {
            fn as_mut(&mut self) -> &mut crate::dom::BoxLayout {
                self.$state.as_mut()
            }
        }

        impl AsRef<stretch::node::Node> for $ty {
            fn as_ref(&self) -> &stretch::node::Node {
                self.$state.as_ref()
            }
        }

        impl AsMut<stretch::node::Node> for $ty {
            fn as_mut(&mut self) -> &mut stretch::node::Node {
                self.$state.as_mut()
            }
        }
    };
}
pub(crate) use derive_dom_attributes;

#[derive(Default)]
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

pub struct State<T> {
    style: Style,
    layout: BoxLayout,
    node: stretch::node::Node,
    state: T,
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
        &self.layout
    }
}

impl<T> AsMut<BoxLayout> for State<T> {
    fn as_mut(&mut self) -> &mut BoxLayout {
        &mut self.layout
    }
}

impl<T> AsRef<stretch::node::Node> for State<T> {
    fn as_ref(&self) -> &stretch::node::Node {
        &self.node
    }
}

impl<T> AsMut<stretch::node::Node> for State<T> {
    fn as_mut(&mut self) -> &mut stretch::node::Node {
        &mut self.node
    }
}

impl<T> Deref for State<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.state
    }
}

impl<T> DerefMut for State<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.state
    }
}

pub enum Node {
    Win(win::Win),
    Root,
}

impl From<win::Win> for Node {
    fn from(val: win::Win) -> Node {
        Node::Win(val)
    }
}

impl Node {
    fn to_stretch_node(&self) -> stretch::node::Node {
        let node: &stretch::node::Node = match self {
            Node::Win(val) => val.as_ref(),
            Node::Root => unreachable!(),
        };
        node.clone()
    }
}

pub struct Dom {
    root: Node,
    stretch: stretch::node::Stretch,
}

impl Dom {
    pub fn new() -> Dom {
        Dom {
            root: Node::Root,
            stretch: stretch::node::Stretch::new(),
        }
    }

    pub fn new_win(&mut self, style: Style, children: &[Node]) -> Result<Node> {
        let children: Vec<stretch::node::Node> =
            children.iter().map(|n| n.to_stretch_node()).collect();
        let node = err_at!(Invalid, self.stretch.new_node(style.flex.clone(), children))?;

        let state: State<()> = State {
            style,
            layout: Default::default(),
            node,
            state: Default::default(),
        };

        Ok(win::Win::new(state).into())
    }

    pub fn set_root(&mut self, node: Node) -> Node {
        std::mem::replace(&mut self.root, node)
    }
}
