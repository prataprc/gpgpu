use crate::{dom, State};

pub struct Win {
    state: State<()>,
}
dom::derive_dom_attributes!(Win, state);

impl Win {
    pub fn new(state: State<()>) -> Self {
        Win { state }
    }
}
