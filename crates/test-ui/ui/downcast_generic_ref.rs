use objc2_foundation::{NSArray, NSObject, NSString};

fn main() {
    let arr = NSArray::from_retained_slice(&[NSObject::new()]);

    // This is invalid and doesn't type check.
    let _arr = arr.downcast_ref::<NSArray<NSString>>();
}
