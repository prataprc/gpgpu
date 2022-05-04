mod win;

use crate::{Error, Result, State, Style};

macro_rules! derive_dom_attributes {
    ($ty:ty, $state:ident) => {
        impl AsRef<crate::Style> for $ty {
            fn as_ref(&self) -> &crate::Style {
                self.$state.as_ref()
            }
        }

        impl AsMut<crate::Style> for $ty {
            fn as_mut(&mut self) -> &mut crate::Style {
                self.$state.as_mut()
            }
        }

        impl AsRef<crate::BoxLayout> for $ty {
            fn as_ref(&self) -> &crate::BoxLayout {
                self.$state.as_ref()
            }
        }

        impl AsMut<crate::BoxLayout> for $ty {
            fn as_mut(&mut self) -> &mut crate::BoxLayout {
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
        use stretch::node::Node;

        let node = {
            let cs: Vec<Node> = children.iter().map(|n| n.to_stretch_node()).collect();
            err_at!(Invalid, self.stretch.new_node(style.flex.clone(), cs))?
        };

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
