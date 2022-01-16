//! Test that NSValue has proper bounds on its Eq implementation.

use objc2_foundation::NSValue;

fn needs_eq<T: Eq>() {}

fn main() {
    // Compiles fine
    needs_eq::<NSValue<u32>>();
    // Fails
    needs_eq::<NSValue<f32>>();
}
