//! Test that AutoreleasePool is not Send and Sync, because it internally
//! works with thread locals.

use objc2::rc::AutoreleasePool;

fn needs_sync<T: ?Sized + Sync>() {}
fn needs_send<T: ?Sized + Send>() {}

fn main() {
    needs_sync::<AutoreleasePool>();
    needs_send::<AutoreleasePool>();
}
