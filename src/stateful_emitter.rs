use crate::listener::{EventHandler, EventHandlerResult, Listener};
use crate::observer::Observer;
use crate::subscription::Subscription;
use std::cell::{Ref, RefCell};
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct StatefulEmitter<TEvent> {
    listeners: Rc<RefCell<Vec<Rc<RefCell<Listener<TEvent>>>>>>,
    last_event: Option<TEvent>,
}

impl<TEvent> fmt::Display for StatefulEmitter<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StatefulEmitter<{}>", self.listeners.borrow().len())
    }
}

impl<TEvent> StatefulEmitter<TEvent> {
    pub fn new() -> Self {
        Self {
            listeners: Rc::new(RefCell::new(vec![])),
            last_event: None,
        }
    }

    pub fn last(&self) -> Option<&TEvent> {
        self.last_event.as_ref()
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

    fn cleanup(&mut self) {
        self.listeners.borrow_mut().retain(|ref l| l.borrow().active());
    }

    pub fn len(&self) -> usize {
        self.listeners.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.listeners.borrow().is_empty()
    }

    pub fn reset(&mut self) {
        self.last_event = None;
        self.listeners.borrow_mut().clear();
    }

    pub fn emit(&mut self, message: TEvent) -> EventHandlerResult {
        self.publish(message)
    }
}

impl<TEvent> Observer<TEvent> for StatefulEmitter<TEvent> {
    fn subscribe(&mut self, listener: Rc<RefCell<Listener<TEvent>>>) -> Subscription<TEvent> {
        let subscription = Subscription::new(Rc::downgrade(&listener), Rc::downgrade(&self.listeners));

        self.listeners.borrow_mut().push(listener);

        subscription
    }

    fn publish(&mut self, message: TEvent) -> EventHandlerResult {
        let mut cleanup = false;

        for lst in self.listeners.borrow().iter() {
            let mut lst = lst.borrow_mut();

            if let Some(handler) = &mut lst.handler {
                (handler)(&message)?;

                if lst.once {
                    cleanup = true;
                    lst.cancel();
                }
            } else {
                cleanup = true;
            }
        }

        if cleanup {
            self.cleanup();
        }

        self.last_event = Some(message);

        Ok(())
    }
}

impl<TEvent> Drop for StatefulEmitter<TEvent> {
    fn drop(&mut self) {
        self.reset()
    }
}
