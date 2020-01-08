use std::error::Error;
use std::fmt;

pub type EventHandlerResult = Result<(), Box<dyn Error>>;

pub type EventHandler<TEvent> = Box<dyn FnMut(&TEvent) -> EventHandlerResult>;

pub struct Listener<TEvent> {
    pub handler: EventHandler<TEvent>,
    pub once: bool,
    is_active: bool,
}

impl<TEvent> Listener<TEvent> {
    #[inline(always)]
    pub fn new(once: bool, handler: EventHandler<TEvent>) -> Self {
        Self {
            once,
            handler,
            is_active: true,
        }
    }

    #[inline(always)]
    pub fn call(&mut self, message: &TEvent) -> EventHandlerResult {
        (self.handler)(message)
    }

    #[inline(always)]
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    #[inline(always)]
    pub fn cancel(&mut self) {
        if self.is_active {
            self.is_active = false;
            self.handler = Box::new(|_| Ok(()))
        }
    }
}

impl<TEvent> Drop for Listener<TEvent> {
    fn drop(&mut self) {
        self.cancel()
    }
}

impl<TEvent> fmt::Debug for Listener<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Listener<{}, once:{}>", if self.is_active() { "active" } else { "inactive" }, self.once)
    }
}
