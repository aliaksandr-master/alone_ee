use alone_ee::event_emitter::EventEmitter;
use criterion::{black_box, criterion_group, criterion_main, Criterion, ParameterizedBenchmark};
use std::cell::RefCell;
use std::rc::Rc;

fn measure(count_ee_listeners: u32, i_max: u128) -> u128 {
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

    for i in 0..i_max {
        ee.emit(&i).unwrap();
    }

    let res = results.borrow();
    res.clone()
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let listeners_params: Vec<u32> = vec![1, 10, 100, 1000];
    c.bench(
        "event_emitter",
        ParameterizedBenchmark::new(
            "listeners",
            |b, param| b.iter(|| measure(black_box(*param), black_box(1_000))),
            listeners_params,
        ),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
