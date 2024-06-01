use objc2::rc::{autoreleasepool, Retained};
use objc2::runtime::NSObject;

fn main() {
    let mut obj = NSObject::new();
    let mut_ptr = Retained::as_mut_ptr(&mut obj);
    autoreleasepool(|pool| {
        let mut_ref = Retained::autorelease_mut(obj, pool);
    });
}
