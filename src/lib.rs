#![allow(clippy::type_complexity)]
use std::cell::RefCell;
use std::error::Error;
use std::sync::{Arc, Weak};

pub struct Event<TEvent> {
    message: TEvent,
    was_stopped: bool,
}

impl<TEvent> Event<TEvent> {
    pub fn new(message: TEvent) -> Self {
        Self { message, was_stopped: false }
    }

    pub fn data(&self) -> &TEvent {
        &self.message
    }

    pub fn stop(&mut self) {
        self.was_stopped = true
    }

    pub fn stopped(&self) -> bool {
        self.was_stopped
    }
}

pub struct Listener<TEvent> {
    handler: Option<Box<dyn FnMut(&mut Event<TEvent>) -> Result<(), Box<dyn Error>>>>,
    once: bool,
}

impl<TEvent> Listener<TEvent> {
    pub fn cancel(&mut self) {
        self.handler = None;
    }
}

impl<TEvent> Drop for Listener<TEvent> {
    fn drop(&mut self) {
        self.cancel()
    }
}

pub struct Subscription<TEvent> {
    listener: Option<Arc<RefCell<Listener<TEvent>>>>,
}

impl<TEvent> Subscription<TEvent> {
    fn cancel(&mut self) {
        if let Some(lst) = &self.listener {
            lst.borrow_mut().cancel();
            self.listener = None;
        }
    }
}

impl<TEvent> Drop for Subscription<TEvent> {
    fn drop(&mut self) {
        self.cancel()
    }
}

pub trait Dispatchable<TEvent> {
    fn subscribe(&mut self, listener: Arc<RefCell<Listener<TEvent>>>) -> Subscription<TEvent>;

    fn emit(&mut self, event: TEvent) -> Result<(), Box<dyn Error>>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    fn reset(&mut self);
}

pub struct EventEmitter<TEvent> {
    listeners: Vec<Weak<RefCell<Listener<TEvent>>>>,
}

impl<TEvent> EventEmitter<TEvent> {
    pub fn new() -> EventEmitter<TEvent> {
        Self { listeners: vec![] }
    }

    pub fn on(&mut self, lst: Box<dyn FnMut(&mut Event<TEvent>) -> Result<(), Box<dyn std::error::Error>>>) -> Subscription<TEvent> {
        self.subscribe(Arc::new(RefCell::new(Listener {
            handler: Some(lst),
            once: false,
        })))
    }

    pub fn once(&mut self, lst: Box<dyn FnMut(&mut Event<TEvent>) -> Result<(), Box<dyn std::error::Error>>>) -> Subscription<TEvent> {
        self.subscribe(Arc::new(RefCell::new(Listener {
            handler: Some(lst),
            once: true,
        })))
    }

    pub fn cleanup(&mut self) {
        self.listeners.retain(|ref l| {
            if let Some(l) = l.clone().upgrade() {
                l.borrow().handler.is_some()
            } else {
                false
            }
        });
    }
}

impl<TEvent> Dispatchable<TEvent> for EventEmitter<TEvent> {
    fn subscribe(&mut self, listener: Arc<RefCell<Listener<TEvent>>>) -> Subscription<TEvent> {
        self.listeners.push(Arc::downgrade(&listener));

        Subscription { listener: Some(listener) }
    }

    fn emit(&mut self, message: TEvent) -> Result<(), Box<dyn std::error::Error>> {
        let mut cleanup = false;

        let mut event = Event::new(message);

        for lst in self.listeners.iter() {
            if let Some(lst) = lst.clone().upgrade() {
                let mut lst = lst.borrow_mut();

                if let Some(handler) = &mut lst.handler {
                    (handler)(&mut event)?;
                } else {
                    cleanup = true
                }

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

    fn len(&self) -> usize {
        self.listeners.len()
    }

    fn is_empty(&self) -> bool {
        self.listeners.is_empty()
    }

    fn reset(&mut self) {
        self.listeners.clear();
    }
}

impl<TEvent> Drop for EventEmitter<TEvent> {
    fn drop(&mut self) {
        self.reset()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ee() {
        #[derive(Debug, PartialEq)]
        struct SomeEvent {
            pub ev: u32,
            pub txt: &'static str,
        }

        let mut ee: EventEmitter<Arc<SomeEvent>> = EventEmitter::new();

        let fired_ev: Arc<RefCell<Option<Arc<SomeEvent>>>> = Arc::new(RefCell::new(Option::None));

        let mut subs1 = {
            let fired_ev_clone = fired_ev.clone();

            ee.on(Box::new(move |ev| {
                *fired_ev_clone.borrow_mut() = Option::Some(Arc::clone(ev.data()));
                Ok(())
            }))
        };

        let mut subs2 = {
            let fired_ev_clone = fired_ev.clone();

            ee.on(Box::new(move |ev| {
                *fired_ev_clone.borrow_mut() = Option::Some(Arc::clone(ev.data()));
                Ok(())
            }))
        };

        assert!(fired_ev.borrow().is_none());

        ee.emit(Arc::new(SomeEvent { ev: 123, txt: "hello" }));

        assert!(fired_ev.borrow().is_some());

        assert_eq!(ee.len(), 2);

        subs1.cancel();

        assert_eq!(ee.len(), 2);

        ee.cleanup();

        assert_eq!(ee.len(), 1);

        if let Some(ev) = fired_ev.borrow().clone() {
            assert_eq!(ev.ev, 123);
        }

        ee.emit(Arc::new(SomeEvent { ev: 333, txt: "world" }));

        if let Some(ev) = fired_ev.borrow().clone() {
            assert_eq!(ev.ev, 333);
        }

        let mut subs3 = {
            let fired_ev_clone = fired_ev.clone();
            ee.once(Box::new(move |ev| {
                *fired_ev_clone.borrow_mut() = Option::Some(ev.data().clone());
                Ok(())
            }))
        };

        assert_eq!(ee.len(), 2);

        ee.emit(Arc::new(SomeEvent { ev: 444, txt: "world" }));

        if let Some(ev) = fired_ev.borrow().clone() {
            assert_eq!(ev.ev, 444);
        }

        subs2.cancel();

        ee.emit(Arc::new(SomeEvent { ev: 555, txt: "world" }));

        if let Some(ev) = fired_ev.borrow().clone() {
            assert_eq!(ev.ev, 444);
        }

        assert_eq!(ee.len(), 0);

        ee.reset();

        assert_eq!(ee.len(), 0);

        drop(ee);

        subs3.cancel();
    }
}
