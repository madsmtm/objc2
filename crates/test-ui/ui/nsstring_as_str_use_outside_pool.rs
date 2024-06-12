//! Test that the lifetime of `NSString::as_str` is bound to the pool.
use objc2::rc::autoreleasepool;
use objc2_foundation::NSString;

fn main() {
    let ns_string = NSString::new();
    let _s = autoreleasepool(|pool| unsafe { ns_string.to_str(pool) });
}
