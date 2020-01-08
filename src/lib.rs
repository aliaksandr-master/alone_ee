#![allow(clippy::type_complexity)]
#![allow(dead_code)]

pub mod event_emitter;
pub mod listener;
pub mod observer;
pub mod subscription;

#[cfg(test)]
mod tests {
    use crate::event_emitter::EventEmitter;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::time::Instant;

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

    #[test]
    fn test_speed() {
        fn measure(count_ee_listeners: u32, i_max: u128, mark_every: u128) {
            let mut ee = EventEmitter::<u128>::new();
            let mut subs = vec![];
            let results = Rc::new(RefCell::new(0_u128));

            for _i in 0..count_ee_listeners {
                let results = results.clone();
                let sbs = ee.on(Box::new(move |s| {
                    //
                    let mut r = results.borrow_mut();
                    *r += s * 2;
                    Ok(())
                }));
                subs.push(sbs);
            }

            let mut it_now = Instant::now();

            let now = Instant::now();
            if mark_every > 0 {
                for i in 0..i_max {
                    if i % mark_every == 0 {
                        println!("done: {} in {:?}", i, it_now.elapsed());
                        it_now = Instant::now();
                    }
                    ee.emit(&i).unwrap();
                }
            } else {
                for i in 0..i_max {
                    ee.emit(&i).unwrap();
                }
            }

            println!(
                "listeners_count: {:?} result: {:?}, duration {:?}, rate: {:?}",
                count_ee_listeners,
                results.borrow(),
                now.elapsed(),
                now.elapsed().as_nanos() / count_ee_listeners as u128
            );
        }

        measure(1, 1_000_000, 0);
        measure(10, 1_000_000, 0);
        measure(100, 1_000_000, 0);
    }
}
