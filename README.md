# alone_ee

The simplest event emitter for rust


```rust
fn main () { 
    use alone_ee::event_emitter::EventEmitter;
    let ee = EventEmitte::Stringr::new();

    let subscription = ee.on(Box::new(|ev| {  // listener will be alive till subscription is alive 
        // do something
        println!("hello {}", ev.data());
        Ok(())
    }));

    ee.once(Box::new(|ev| {   // listener will be alive till subscription is alive or next emit will fired
        // do something
        println!("hello {} one more time", ev.data());
        Ok(())
    }));

    ee.emit("world");

    // you will see 
    //     "hello world"
    //     "hello world one more time"

    drop(subscription); // unbind the listener
}
```

## Testing
```bash
$ cargo test --release
```


## Benchmark
```bash
$ cargo bench
```
