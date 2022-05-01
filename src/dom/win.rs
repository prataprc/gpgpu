use crate::dom;

pub struct Win {
    state: dom::State<()>,
}
dom::derive_dom_attributes!(Win, state);

impl Win {
    pub fn new(state: dom::State<()>) -> Self {
        Win { state }
    }
}
