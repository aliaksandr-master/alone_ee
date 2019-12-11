use crate::listener::Listener;
use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

pub struct Subscription<TEvent> {
    listener: Option<Weak<RefCell<Listener<TEvent>>>>,
    listeners: Option<Weak<RefCell<Vec<Rc<RefCell<Listener<TEvent>>>>>>>,
}

impl<TEvent> fmt::Debug for Subscription<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Subscription<{}>", if self.listener.is_some() { "active" } else { "inactive" })
    }
}

impl<TEvent> Subscription<TEvent> {
    pub fn new(listener: Weak<RefCell<Listener<TEvent>>>, listeners: Weak<RefCell<Vec<Rc<RefCell<Listener<TEvent>>>>>>) -> Self {
        Self {
            listener: Some(listener),
            listeners: Some(listeners),
        }
    }
}

impl<TEvent> Drop for Subscription<TEvent> {
    fn drop(&mut self) {
        if let Some(x) = self.listener.take() {
            if let Some(x) = x.upgrade() {
                x.borrow_mut().cancel()
            }
        }

        if let Some(x) = self.listeners.take() {
            if let Some(x) = x.upgrade() {
                x.borrow_mut().retain(|l| l.borrow().active())
            }
        }
    }
}
