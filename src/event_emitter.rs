use crate::event::Event;
use crate::listener::{EventHandler, EventHandlerResult, Listener};
use crate::observer::Observer;
use crate::subscription::Subscription;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct EventEmitter<TEvent> {
    listeners: Vec<Rc<RefCell<Listener<TEvent>>>>,
}

impl<TEvent> fmt::Display for EventEmitter<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EventEmitter<{}>", self.listeners.len())
    }
}

impl<TEvent> EventEmitter<TEvent> {
    pub fn new() -> Self {
        Self { listeners: vec![] }
    }

    pub fn on(&mut self, lst: EventHandler<TEvent>) -> Subscription<TEvent> {
        self.subscribe(Rc::new(RefCell::new(Listener {
            handler: Some(lst),
            once: false,
        })))
    }

    pub fn once(&mut self, lst: EventHandler<TEvent>) -> Subscription<TEvent> {
        self.subscribe(Rc::new(RefCell::new(Listener {
            handler: Some(lst),
            once: true,
        })))
    }

    pub fn cleanup(&mut self) {
        self.listeners.retain(|ref l| l.borrow().handler.is_some());
    }

    pub fn len(&self) -> usize {
        self.listeners.len()
    }

    pub fn is_empty(&self) -> bool {
        self.listeners.is_empty()
    }

    pub fn reset(&mut self) {
        self.listeners.clear();
    }

    pub fn emit(&mut self, message: TEvent) -> EventHandlerResult {
        self.publish(message)
    }
}

impl<TEvent> Observer<TEvent> for EventEmitter<TEvent> {
    fn subscribe(&mut self, listener: Rc<RefCell<Listener<TEvent>>>) -> Subscription<TEvent> {
        let subscription = Subscription::new(Rc::downgrade(&listener));

        self.listeners.push(listener);

        subscription
    }

    fn publish(&mut self, message: TEvent) -> EventHandlerResult {
        let mut cleanup = false;
        let mut event = Event::new(message);

        for lst in self.listeners.iter() {
            let mut lst = lst.borrow_mut();

            if let Some(handler) = &mut lst.handler {
                (handler)(&mut event)?;

                if lst.once {
                    cleanup = true;
                    lst.handler = None;
                }
            } else {
                cleanup = true;
            }
        }

        drop(event);

        if cleanup {
            self.cleanup();
        }

        Ok(())
    }
}

impl<TEvent> Drop for EventEmitter<TEvent> {
    fn drop(&mut self) {
        self.reset()
    }
}
