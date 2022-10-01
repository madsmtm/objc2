# dispatch2

Allows interaction with the [Apple Dispatch](https://developer.apple.com/documentation/dispatch) library in a safe and unsafe way.

## Usage

To use `dispatch2`, add this to your `Cargo.toml`:

```toml
[dependencies]
dispatch2 = "0.1.0"
```

## Example

```rust
use dispatch2::{Queue, QueueAttribute};

fn main() {
    let queue = Queue::new("example_queue", QueueAttribute::Serial);
    queue.exec_async(|| println!("Hello"));
    queue.exec_sync(|| println!("World"));
}
```

## License

dispatch2 is distributed under the terms of either the MIT license or the Apache
License (Version 2.0), at the user's choice.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).
