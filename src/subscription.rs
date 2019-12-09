use crate::listener::Listener;
use std::fmt;
use std::fmt::{Error, Formatter};
use std::sync::{RwLock, Weak};

pub struct Subscription<TEvent> {
    listener: Option<Weak<RwLock<Listener<TEvent>>>>,
}

impl<TEvent> fmt::Debug for Subscription<TEvent> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Subscription<{}>", if self.listener.is_some() { "active" } else { "inactive" })
    }
}

impl<TEvent> fmt::Display for Subscription<TEvent> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Subscription<{}>", if self.listener.is_some() { "active" } else { "inactive" })
    }
}

impl<TEvent> Subscription<TEvent> {
    pub fn new(listener: Weak<RwLock<Listener<TEvent>>>) -> Self {
        Self { listener: Some(listener) }
    }
}

impl<TEvent> Drop for Subscription<TEvent> {
    fn drop(&mut self) {
        if let Some(x) = self.listener.take() {
            if let Some(x) = x.upgrade() {
                x.write().unwrap().cancel()
            }
        }
    }
}
