//! Test that AnyObject and NSObject are not Send and Sync, because their
//! subclasses might not be.
//!
//! Also test that `NSValue` is not Send nor Sync, because its contained value
//! might not be.
use icrate::Foundation::NSValue;
use objc2::runtime::{NSObject, AnyObject};

fn needs_sync<T: ?Sized + Sync>() {}
fn needs_send<T: ?Sized + Send>() {}

fn main() {
    needs_sync::<AnyObject>();
    needs_send::<AnyObject>();
    needs_sync::<NSObject>();
    needs_send::<NSObject>();
    needs_sync::<NSValue>();
    needs_send::<NSValue>();
}
