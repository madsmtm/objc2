use objc2::rc::{autoreleasepool, Id};
use objc2::runtime::NSObject;

fn main() {
    let mut obj = NSObject::new();
    let mut_ptr = Id::as_mut_ptr(&mut obj);
    autoreleasepool(|pool| {
        let mut_ref = Id::autorelease_mut(obj, pool);
    });
}
