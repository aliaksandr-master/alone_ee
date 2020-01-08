use crate::listener::Listener;
use std::cell::RefCell;
use std::fmt;
use std::rc::Weak;

pub struct Subscription<TEvent> {
    listener: Option<Weak<RefCell<Listener<TEvent>>>>,
}

impl<TEvent> fmt::Debug for Subscription<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Subscription<{}>", if self.listener.is_some() { "active" } else { "inactive" })
    }
}

impl<TEvent> Subscription<TEvent> {
    pub fn new(listener: Weak<RefCell<Listener<TEvent>>>) -> Self {
        Self { listener: Some(listener) }
    }
}

impl<TEvent> Drop for Subscription<TEvent> {
    fn drop(&mut self) {
        if let Some(x) = self.listener.take() {
            if let Some(x) = x.upgrade() {
                x.borrow_mut().cancel()
            }
        }
    }
}
