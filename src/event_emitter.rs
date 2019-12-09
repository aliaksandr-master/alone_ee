use crate::dispatchable::Observer;
use crate::event::Event;
use crate::listener::{EventHandler, EventHandlerResult, Listener};
use crate::subscription::Subscription;
use std::fmt;
use std::sync::{Arc, RwLock, Weak};

#[derive(Debug, Default)]
pub struct EventEmitter<TEvent> {
    listeners: Vec<Weak<RwLock<Listener<TEvent>>>>,
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
        self.subscribe(Arc::new(RwLock::new(Listener {
            handler: Some(lst),
            once: false,
        })))
    }

    pub fn once(&mut self, lst: EventHandler<TEvent>) -> Subscription<TEvent> {
        self.subscribe(Arc::new(RwLock::new(Listener {
            handler: Some(lst),
            once: true,
        })))
    }

    pub fn cleanup(&mut self) {
        self.listeners.retain(|ref l| {
            if let Some(l) = l.upgrade() {
                l.read().unwrap().handler.is_some()
            } else {
                false
            }
        });
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
    fn subscribe(&mut self, listener: Arc<RwLock<Listener<TEvent>>>) -> Subscription<TEvent> {
        self.listeners.push(Arc::downgrade(&listener));

        Subscription::new(listener)
    }

    fn publish(&mut self, message: TEvent) -> EventHandlerResult {
        let mut cleanup = false;
        let mut event = Event::new(message);

        for lst in self.listeners.iter() {
            if let Some(lst) = lst.clone().upgrade() {
                let mut lst = lst.write().unwrap();

                if let Some(handler) = &mut lst.handler {
                    (handler)(&mut event)?;

                    if lst.once {
                        cleanup = true;
                        lst.handler = None;
                    }
                } else {
                    cleanup = true;
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
