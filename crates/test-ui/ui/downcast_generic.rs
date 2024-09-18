use objc2_foundation::{NSArray, NSObject};

fn main() {
    let arr = NSArray::new();

    // Possible
    let _ = arr.downcast::<NSArray>();

    // Not possible
    let _ = arr.downcast::<NSArray<NSObject>>();
}
