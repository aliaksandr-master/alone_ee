use crate::listener::{EventHandlerResult, Listener};
use crate::subscription::Subscription;

pub trait Observer<TEvent> {
    fn subscribe(&mut self, listener: Listener<TEvent>) -> Subscription;

    fn publish(&mut self, event: &TEvent) -> EventHandlerResult;
}
