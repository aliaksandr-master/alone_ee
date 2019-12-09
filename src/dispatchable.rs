use crate::listener::{EventHandlerResult, Listener};
use crate::subscription::Subscription;
use std::sync::{Arc, RwLock};

pub trait Observer<TEvent> {
    fn subscribe(&mut self, listener: Arc<RwLock<Listener<TEvent>>>) -> Subscription<TEvent>;

    fn publish(&mut self, event: TEvent) -> EventHandlerResult;
}
