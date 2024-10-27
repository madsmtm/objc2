//! Same OS name repeated in the `available!` macro.
use objc2::available;

fn main() {
    available!(macos = 1.2, macos = 1.2);
}
