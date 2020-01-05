use crate::listener::{EventHandlerResult, Listener};
use crate::subscription::Subscription;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Observer<TEvent> {
    fn subscribe(&mut self, listener: Rc<RefCell<Listener<TEvent>>>) -> Subscription<TEvent>;

    fn publish(&mut self, event: &TEvent) -> EventHandlerResult;
}
