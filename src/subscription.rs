use crate::listener::Listener;
use std::sync::{Arc, RwLock};

pub struct Subscription<TEvent> {
    listener: Option<Arc<RwLock<Listener<TEvent>>>>,
}

impl<TEvent> Subscription<TEvent> {
    pub fn new(listener: Arc<RwLock<Listener<TEvent>>>) -> Self {
        Self { listener: Some(listener) }
    }
}

impl<TEvent> Drop for Subscription<TEvent> {
    fn drop(&mut self) {
        if let Some(x) = self.listener.take() {
            x.write().unwrap().cancel()
        }
    }
}
