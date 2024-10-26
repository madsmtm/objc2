//! Unknown OS name in `available!` macro.
use objc2::available;

fn main() {
    available!(unknown = 1.2);
}
