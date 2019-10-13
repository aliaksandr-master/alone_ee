use std::sync::{Arc, RwLock};

pub struct Listener<T> {
    _id: usize,
    pub trigger: Box<dyn FnMut(Arc<T>)>,
}

pub struct EventEmitter<T> {
    _next_lstnr_id: usize,
    _fired_times: usize,
    pub listeners: Arc<RwLock<Vec<Listener<T>>>>,
}

impl<T> EventEmitter<T> {
    pub fn new() -> Self {
        EventEmitter {
            _next_lstnr_id: 0,
            _fired_times: 0,
            listeners: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn fired_times(&self) -> usize {
        self._fired_times
    }

    pub fn reset(&mut self) {
        self._fired_times = 0;
        self._next_lstnr_id = 0;
        *(self.listeners.write().expect("not poisoned")) = Vec::new();
    }

    pub fn on(&mut self, listener: Box<dyn FnMut(Arc<T>)>) -> impl FnOnce() {
        let id = self._next_lstnr_id;
        self._next_lstnr_id += 1;

        self.listeners
            .write()
            .expect("not poisoned")
            .push(Listener {
                _id: id,
                trigger: listener,
            });

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
                .retain(|listener| listener._id != id);
        }
    }

    pub fn emit (&mut self, value: Arc<T>) -> Arc<T> {
        self._fired_times += 1;
        for lst in self.listeners.write().expect("not poisoned").iter_mut() {
            (lst.trigger)(value.clone());
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ee() {
        #[derive(Debug)]
        #[derive(PartialEq)]
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

        ee.emit(ev1.clone());

        rm();

        let ev2 = Arc::new(SomeEvent {
            ev: 333,
            txt: "world",
        });

        ee.emit(ev2.clone());

//        let tmp_ev1 = ev1.clone();
//        let tmp_evF = fired_ev.into_inner().unwrap().unwrap().clone();

        assert_eq!(ee.fired_times(), 2);
//        assert!(Arc::ptr_eq(&tmp_ev1, &tmp_evF));
    }
}
