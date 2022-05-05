pub mod circle;
pub mod win;

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
    };
}
pub(crate) use derive_dom_attributes;

pub enum Node {
    Win(win::Win),
}

pub struct Dom {
    root: Node,
    stretch: stretch::node::Stretch,
}

impl Dom {
    pub fn new_win(win: win::Win) -> Dom {
        let root = Node::Win(win);
        Dom {
            root,
            stretch: stretch::node::Stretch::new(),
        }
    }
}
