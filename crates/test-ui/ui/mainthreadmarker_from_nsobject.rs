use icrate::Foundation::{MainThreadMarker, NSObject};

fn main() {
    let obj = NSObject::new();
    let mtm = MainThreadMarker::from(&*obj);
}
