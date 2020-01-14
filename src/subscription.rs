use crate::listener::Listener;
use std::cell::Cell;
use std::fmt;
use std::marker::PhantomData;
use std::rc::{Rc, Weak};

pub struct Subscription<TEvent> {
    shared_active_state: Option<Weak<Cell<bool>>>,
    _p: PhantomData<TEvent>,
}

impl<TEvent> fmt::Debug for Subscription<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Subscription<{}>", if self.shared_active_state.is_some() { "active" } else { "inactive" })
    }
}

impl<TEvent> Subscription<TEvent> {
    pub fn new(shared_state: Weak<Cell<bool>>) -> Self {
        Self {
            shared_active_state: Some(shared_state),
            _p: PhantomData,
        }
    }
}

impl<TEvent> Drop for Subscription<TEvent> {
    fn drop(&mut self) {
        if let Some(x) = self.shared_active_state.take() {
            if let Some(x) = x.upgrade() {
                x.set(false)
            }
        }
    }
}
