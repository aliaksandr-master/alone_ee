use std::cell::Cell;
use std::fmt;
use std::rc::Weak;

pub struct Subscription {
    shared_active_state: Weak<Cell<bool>>,
}

impl fmt::Debug for Subscription {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Subscription<{}>",
            if self.shared_active_state.upgrade().is_some() { "active" } else { "inactive" }
        )
    }
}

impl Subscription {
    pub fn new(shared_state: Weak<Cell<bool>>) -> Self {
        Self {
            shared_active_state: shared_state,
        }
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(x) = self.shared_active_state.upgrade() {
            x.set(false)
        }
    }
}
