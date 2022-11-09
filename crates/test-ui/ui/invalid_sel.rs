//! Test invalid selector syntax
use objc2::sel;

fn main() {
    sel!();
    sel!(a: b);
    sel!(a: b: c);
}
