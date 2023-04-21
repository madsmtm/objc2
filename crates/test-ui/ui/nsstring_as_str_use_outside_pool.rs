//! Test that the lifetime of `NSString::as_str` is bound to the pool.
use icrate::Foundation::NSString;
use objc2::rc::autoreleasepool;

fn main() {
    let ns_string = NSString::new();
    let _s = autoreleasepool(|pool| ns_string.as_str(pool));
}
