use std::sync::{Arc, RwLock};

pub struct Listener<T> {
    id: usize,
    pub handler: Box<dyn FnMut(Arc<T>)>,
}

pub struct EventEmitter<T> {
    next_listener_id: usize,
    pub listeners: Arc<RwLock<Vec<Listener<T>>>>,
}

impl<T> EventEmitter<T> {
    pub fn new() -> Self {
        EventEmitter {
            next_listener_id: 0,
            listeners: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn len(&self) -> usize {
        self.listeners.read().expect("not poisoned").len()
    }

    pub fn reset(&mut self) {
        self.listeners.write().expect("not poisoned").clear();
    }

    pub fn on(&mut self, handler: Box<dyn FnMut(Arc<T>)>) -> impl FnOnce() {
        let id = self.next_listener_id;
        self.next_listener_id += 1;

        self.listeners
            .write()
            .expect("not poisoned")
            .push(Listener { id, handler });

        let listeners = self.listeners.clone();

        let mut removed = false;

        move || {
            if removed {
                return;
            }
            removed = true;
            listeners
                .write()
                .expect("not poisoned")
                .retain(|listener| listener.id != id);
        }
    }

    pub fn emit(&self, value: Arc<T>) -> Arc<T> {
        for lst in self.listeners.write().expect("not poisoned").iter_mut() {
            (lst.handler)(value.clone());
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ee() {
        #[derive(Debug, PartialEq)]
        struct SomeEvent {
            ev: u32,
            txt: &'static str,
        }

        let mut ee: EventEmitter<SomeEvent> = EventEmitter::new();

        let fired_ev: Arc<RwLock<Option<Arc<SomeEvent>>>> = Arc::new(RwLock::new(Option::None));

        let fired_ev_clone = fired_ev.clone();

        let rm = ee.on(Box::new(move |ev| {
            let mut mut_fired_ev = fired_ev_clone.write().unwrap();
            *mut_fired_ev = Option::Some(ev.clone())
        }));

        let ev1 = Arc::new(SomeEvent {
            ev: 123,
            txt: "hello",
        });

        assert!(fired_ev.read().expect("not poisoned").is_none());

        ee.emit(ev1.clone());

        assert!(fired_ev.read().expect("not poisoned").is_some());

        assert_eq!(ee.len(), 1);

        rm();

        assert_eq!(ee.len(), 0);

        let ev2 = Arc::new(SomeEvent {
            ev: 333,
            txt: "world",
        });

        ee.emit(ev2.clone());

        //        let tmp_ev1 = ev1.clone();
        //        let tmp_evF = fired_ev.into_inner().unwrap().unwrap().clone();

        //        assert!(Arc::ptr_eq(&tmp_ev1, &tmp_evF));

        let fired_ev_clone = fired_ev.clone();

        let rm = ee.on(Box::new(move |ev| {
            let mut mut_fired_ev = fired_ev_clone.write().unwrap();
            *mut_fired_ev = Option::Some(ev.clone())
        }));

        assert_eq!(ee.len(), 1);

        ee.reset();

        rm();

        assert_eq!(ee.len(), 0);
    }
}
