use std::sync::Arc;

use crate::{spinlock::Spinlock, Screen};

/// Rendering state
pub struct Render<S> {
    pub screen: Arc<Screen>,
    pub state: Spinlock<Arc<S>>,
}

impl<S> Render<S> {
    pub fn new(screen: Screen, state: S) -> Render<S> {
        Render {
            screen: Arc::new(screen),
            state: Spinlock::new(Arc::new(state)),
        }
    }

    pub fn as_state(&self) -> Arc<S> {
        Arc::clone(&self.state.read())
    }

    pub fn to_state(&self) -> S
    where
        S: Clone,
    {
        self.state.read().as_ref().clone()
    }

    pub fn set_state(&self, state: S) {
        *self.state.write() = Arc::new(state)
    }
}
