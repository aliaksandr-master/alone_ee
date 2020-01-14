use crate::listener::{EventHandler, EventHandlerResult, Listener};
use crate::observer::Observer;
use crate::subscription::Subscription;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct EventEmitter<TEvent> {
    listeners: Vec<Option<Listener<TEvent>>>,
}

impl<TEvent> fmt::Display for EventEmitter<TEvent> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EventEmitter<{}>", self.len())
    }
}

impl<TEvent> EventEmitter<TEvent> {
    pub fn new() -> Self {
        Self { listeners: Vec::new() }
    }

    pub fn on(&mut self, handler: EventHandler<TEvent>) -> Subscription<TEvent> {
        self.subscribe(Listener::new(false, handler))
    }

    pub fn once(&mut self, handler: EventHandler<TEvent>) -> Subscription<TEvent> {
        self.subscribe(Listener::new(true, handler))
    }

    pub fn is_empty(&self) -> bool {
        self.listeners.is_empty()
    }

    pub fn reset(&mut self) {
        self.listeners = Vec::new();
    }

    pub fn len(&self) -> usize {
        self.listeners.iter().filter(|slot| slot.as_ref().map_or(false, |l| l.is_active())).count()
    }

    pub fn emit(&mut self, message: &TEvent) -> EventHandlerResult {
        self.publish(message)
    }
}

impl<TEvent> Observer<TEvent> for EventEmitter<TEvent> {
    fn subscribe(&mut self, listener: Listener<TEvent>) -> Subscription<TEvent> {
        let subsc = Subscription::new(listener.get_activation_flag());
        self.listeners.push(Some(listener));
        subsc
    }

    fn publish(&mut self, message: &TEvent) -> EventHandlerResult {
        let mut res = Ok(());

        for slot in self.listeners.iter_mut() {
            if res.is_err() {
                break;
            }

            if let Some(lst) = slot {
                if !lst.is_active() {
                    slot.take();
                    continue;
                }

                res = lst.call(message);

                if lst.once {
                    slot.take();
                }
            }
        }

        self.listeners.retain(|slot| slot.is_some());

        res
    }
}

impl<TEvent> Drop for EventEmitter<TEvent> {
    fn drop(&mut self) {
        self.reset()
    }
}
