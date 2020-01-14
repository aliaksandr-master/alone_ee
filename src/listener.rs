use std::cell::Cell;
use std::error::Error;
use std::fmt;
use std::rc::{Rc, Weak};

pub type EventHandlerResult = Result<(), Box<dyn Error>>;

pub type EventHandler<TEvent> = Box<dyn FnMut(&TEvent) -> EventHandlerResult>;

pub struct Listener<TEvent> {
    pub handler: EventHandler<TEvent>,
    pub once: bool,
    is_active: Rc<Cell<bool>>,
}

impl<TEvent> Listener<TEvent> {
    #[inline(always)]
    pub fn new(once: bool, handler: EventHandler<TEvent>) -> Self {
        Self {
            once,
            handler,
            is_active: Rc::new(Cell::new(true)),
        }
    }

    #[inline(always)]
    pub fn call(&mut self, message: &TEvent) -> EventHandlerResult {
        (self.handler)(message)
    }

    #[inline(always)]
    pub fn is_active(&self) -> bool {
        self.is_active.get()
    }

    #[inline(always)]
    pub fn get_activation_flag(&self) -> Weak<Cell<bool>> {
        return Rc::downgrade(&self.is_active);
    }
}

impl<TEvent> fmt::Debug for Listener<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Listener<{}, once:{}>", if self.is_active() { "active" } else { "inactive" }, self.once)
    }
}
