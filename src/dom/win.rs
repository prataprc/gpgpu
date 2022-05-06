use crate::{dom, Size, State};

pub struct Win {
    state: State<()>,
    children: Vec<dom::Node>,
}

impl Win {
    pub fn new(state: State<()>) -> Self {
        Win {
            state,
            children: Vec::default(),
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

    pub fn to_extent(&self) -> Option<Size> {
        None
    }
}
