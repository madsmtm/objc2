//! Test that the lifetime of `NSString::as_str` is bound to the string.
use objc2::rc::autoreleasepool;
use objc2_foundation::NSString;

fn main() {
    autoreleasepool(|pool| {
        let ns_string = NSString::new();
        let s = ns_string.as_str(pool);
        drop(ns_string);
        println!("{}", s);
    });
}
