# dispatch2

Rust bindings and wrappers for the Grand Central Dispatch (GCD)

## Usage

To use `dispatch2`, add this to your `Cargo.toml`:

```toml
[dependencies]
dispatch2 = { git = "https://github.com/Thog/dispatch2" }
```

## Example

```rust
use dispatch2::{Queue, QueueAttribute};

let queue = Queue::new("example_queue", QueueAttribute::Serial);

queue.exec_async(|| println!("Hello"));
queue.exec_sync(|| println!("World"));
```

## License

dispatch2 is distributed under the terms of either the MIT license or the Apache
License (Version 2.0), at the user's choice.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
