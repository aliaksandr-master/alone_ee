use crate::listener::{EventHandler, EventHandlerResult, Listener};
use crate::observer::Observer;
use crate::subscription::Subscription;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct EventEmitter<TEvent> {
    listeners: Vec<Rc<RefCell<Listener<TEvent>>>>,
    new_listeners: Vec<Rc<RefCell<Listener<TEvent>>>>,
}

impl<TEvent> fmt::Display for EventEmitter<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EventEmitter<{}>", self.listeners.len())
    }
}

impl<TEvent> EventEmitter<TEvent> {
    pub fn new() -> Self {
        Self {
            listeners: vec![],
            new_listeners: vec![],
        }
    }

    pub fn on(&mut self, handler: EventHandler<TEvent>) -> Subscription<TEvent> {
        self.subscribe(Rc::new(RefCell::new(Listener::new(false, handler))))
    }

    pub fn once(&mut self, handler: EventHandler<TEvent>) -> Subscription<TEvent> {
        self.subscribe(Rc::new(RefCell::new(Listener::new(true, handler))))
    }

    pub fn is_empty(&self) -> bool {
        self.listeners.is_empty() && self.new_listeners.is_empty()
    }

    pub fn reset(&mut self) {
        self.listeners = vec![];
    }

    pub fn emit(&mut self, message: &TEvent) -> EventHandlerResult {
        self.publish(message)
    }
}

impl<TEvent> Observer<TEvent> for EventEmitter<TEvent> {
    fn subscribe(&mut self, listener: Rc<RefCell<Listener<TEvent>>>) -> Subscription<TEvent> {
        let subsc = Subscription::new(Rc::downgrade(&listener));
        self.new_listeners.push(listener);
        subsc
    }

    fn publish(&mut self, message: &TEvent) -> EventHandlerResult {
        let mut res = Ok(());

        if !self.new_listeners.is_empty() {
            self.listeners.extend(self.new_listeners.drain(..))
        } else if self.listeners.is_empty() {
            return res;
        }

        self.listeners.retain(|lst| {
            if res.is_err() {
                return true;
            }

            let mut lst = lst.borrow_mut();

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
