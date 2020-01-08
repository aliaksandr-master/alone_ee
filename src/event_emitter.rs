use crate::listener::{EventHandler, EventHandlerResult, Listener};
use crate::observer::Observer;
use crate::subscription::Subscription;
use std::fmt;
use std::sync::{Arc, RwLock};

#[derive(Debug, Default)]
pub struct EventEmitter<TEvent> {
    listeners: Vec<Arc<RwLock<Listener<TEvent>>>>,
    new_listeners_buf: Vec<Arc<RwLock<Listener<TEvent>>>>,
}

impl<TEvent> fmt::Display for EventEmitter<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EventEmitter<{}>", self.len())
    }
}

impl<TEvent> EventEmitter<TEvent> {
    pub fn new() -> Self {
        Self {
            listeners: vec![],
            new_listeners_buf: vec![],
        }
    }

    pub fn on(&mut self, handler: EventHandler<TEvent>) -> Subscription<TEvent> {
        self.subscribe(Listener::new(false, handler))
    }

    pub fn once(&mut self, handler: EventHandler<TEvent>) -> Subscription<TEvent> {
        self.subscribe(Listener::new(true, handler))
    }

    pub fn is_empty(&self) -> bool {
        self.listeners.is_empty() && self.new_listeners_buf.is_empty()
    }

    pub fn reset(&mut self) {
        self.listeners = vec![];
        self.new_listeners_buf = vec![];
    }

    pub fn len(&self) -> usize {
        self.listeners.iter().filter(|l| l.read().unwrap().is_active()).count()
            + self.new_listeners_buf.iter().filter(|l| l.read().unwrap().is_active()).count()
    }

    pub fn emit(&mut self, message: &TEvent) -> EventHandlerResult {
        self.publish(message)
    }
}

impl<TEvent> Observer<TEvent> for EventEmitter<TEvent> {
    fn subscribe(&mut self, listener: Listener<TEvent>) -> Subscription<TEvent> {
        let listener = Arc::new(RwLock::new(listener));
        let subsc = Subscription::new(Arc::downgrade(&listener));
        self.new_listeners_buf.push(listener);
        subsc
    }

    fn publish(&mut self, message: &TEvent) -> EventHandlerResult {
        let mut res = Ok(());

        if !self.new_listeners_buf.is_empty() {
            self.listeners.extend(self.new_listeners_buf.drain(..))
        } else if self.listeners.is_empty() {
            return res;
        }

        self.listeners.retain(|lst| {
            if res.is_err() {
                return true;
            }

            let mut lst = lst.write().unwrap();

            if !lst.is_active() {
                return false;
            }

            res = lst.call(message);

            !lst.once
        });

        res
    }
}

impl<TEvent> Drop for EventEmitter<TEvent> {
    fn drop(&mut self) {
        self.reset()
    }
}
