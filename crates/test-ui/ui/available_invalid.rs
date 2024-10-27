//! Various invalid usage of the `available!` macro.
use objc2::available;

fn main() {
    // Space between version
    available!(macos = 1 1);

    // Various invalid syntax
    available!(macos = ABCD);
    available!(macos = );
    available!(macos: 1.2);
    available!(macos);
}
