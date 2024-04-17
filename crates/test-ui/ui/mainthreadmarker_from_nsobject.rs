use objc2_foundation::{MainThreadMarker, NSObject};

fn main() {
    let obj = NSObject::new();
    let mtm = MainThreadMarker::from(&*obj);
}
