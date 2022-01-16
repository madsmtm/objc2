//! Test that Object and NSObject are not Send and Sync, because their
//! subclasses might not be.

use objc2::runtime::Object;
use objc2_foundation::NSObject;

fn needs_sync<T: ?Sized + Sync>() {}
fn needs_send<T: ?Sized + Send>() {}

fn main() {
    needs_sync::<Object>();
    needs_send::<Object>();
    needs_sync::<NSObject>();
    needs_send::<NSObject>();
}
