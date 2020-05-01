#![allow(clippy::type_complexity)]
#![allow(dead_code)]

mod event_emitter;
mod listener;
mod observer;
mod subscription;

pub use event_emitter::EventEmitter;
pub use listener::{EventHandler, EventHandlerResult};
pub use observer::Observer;
pub use subscription::Subscription;

#[cfg(test)]
mod tests {
    use crate::event_emitter::EventEmitter;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_ee() {
        #[derive(Debug, PartialEq)]
        struct SomeEvent {
            pub ev: u32,
            pub txt: &'static str,
        }

        let mut ee = EventEmitter::<Rc<SomeEvent>>::new();

        let fired_ev: Rc<RefCell<Option<Rc<SomeEvent>>>> = Rc::new(RefCell::new(Option::None));

        let subs1 = {
            let fired_ev_clone = fired_ev.clone();

            ee.on(Box::new(move |ev| {
                *fired_ev_clone.borrow_mut() = Option::Some(Rc::clone(ev));
                Ok(())
            }))
        };

        let subs2 = {
            let fired_ev_clone = fired_ev.clone();

            ee.on(Box::new(move |ev| {
                *fired_ev_clone.borrow_mut() = Option::Some(Rc::clone(ev));
                Ok(())
            }))
        };

        assert!(fired_ev.borrow().is_none());

        ee.emit(&Rc::new(SomeEvent { ev: 123, txt: "hello" })).unwrap();

        assert!(fired_ev.borrow().is_some());

        assert_eq!(ee.is_empty(), false);

        drop(subs1);

        assert_eq!(ee.is_empty(), false);

        if let Some(ev) = fired_ev.borrow().clone() {
            assert_eq!(ev.ev, 123);
        }

        ee.emit(&Rc::new(SomeEvent { ev: 333, txt: "world" })).unwrap();

        if let Some(ev) = fired_ev.borrow().clone() {
            assert_eq!(ev.ev, 333);
        }

        let subs3 = {
            let fired_ev_clone = fired_ev.clone();
            ee.once(Box::new(move |ev| {
                *fired_ev_clone.borrow_mut() = Option::Some(ev.clone());
                Ok(())
            }))
        };

        assert_eq!(ee.is_empty(), false);

        ee.emit(&Rc::new(SomeEvent { ev: 444, txt: "world" })).unwrap();

        if let Some(ev) = fired_ev.borrow().clone() {
            assert_eq!(ev.ev, 444);
        }

        drop(subs2);

        ee.emit(&Rc::new(SomeEvent { ev: 555, txt: "world" })).unwrap();

        if let Some(ev) = fired_ev.borrow().clone() {
            assert_eq!(ev.ev, 444);
        }

        assert_eq!(ee.is_empty(), true);

        ee.reset();

        assert_eq!(ee.is_empty(), true);

        drop(ee);

        drop(subs3);
    }
}
