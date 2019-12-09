use crate::event::Event;
use std::error::Error;
use std::fmt;

pub type EventHandlerResult = Result<(), Box<dyn Error>>;

pub type EventHandler<TEvent> = Box<dyn FnMut(&mut Event<TEvent>) -> EventHandlerResult>;

pub struct Listener<TEvent> {
    pub handler: Option<EventHandler<TEvent>>,
    pub once: bool,
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

impl<TEvent> fmt::Display for Listener<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Listener<handler: {}, once:{}>",
            if self.handler.is_some() { "active" } else { "inactive" },
            self.once
        )
    }
}

impl<TEvent> fmt::Debug for Listener<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Listener<handler: {}, once:{}>",
            if self.handler.is_some() { "active" } else { "inactive" },
            self.once
        )
    }
}
