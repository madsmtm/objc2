use objc2::extern_class;
use objc2::foundation::NSObject;

extern_class! {
    unsafe pub struct NSNumber: NSObject {
        var: u32,
    }
}

fn main() {}
