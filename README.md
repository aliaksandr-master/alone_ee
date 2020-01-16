# alone_ee

The simplest event emitter for rust


```rust
fn main () { 
    use alone_ee::event_emitter::EventEmitter;
    let ee = EventEmitte::Stringr::new();

    let subscription1 = ee.on(Box::new(|event_data| {  // listener will be alive till subscription is alive 
        // do something
        println!("hello {}", event_data);
        Ok(())
    }));

    let _subscription2 = ee.once(Box::new(|event_data| {   // listener will be alive till subscription is alive or next emit will fired
        // do something
        println!("hello {} one more time", event_data);
        Ok(())
    }));

    ee.emit("world1").unwrap();
    ee.emit("world2").unwrap();

    // you will see 
    //     "hello world1"
    //     "hello world1 one more time"
    //     "hello world2"

    drop(subscription1); // unbind the listener
    // _subscription2 will be removed automatically
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
