use std::ffi::c_void;

use objc2::rc::{autoreleasepool, Id, Shared};
use objc2::runtime::Object;
use objc2::{class, msg_send};

fn retain_count(obj: &Object) -> usize {
    unsafe { msg_send![obj, retainCount] }
}

fn create_data(bytes: &[u8]) -> Id<Object, Shared> {
    let bytes_ptr = bytes.as_ptr() as *const c_void;
    unsafe {
        // All code between the `msg_send!` and the `retain_autoreleased` must
        // be able to be optimized away for this to work.
        let obj: *mut Object = msg_send![
            class!(NSData),
            dataWithBytes: bytes_ptr,
            length: bytes.len(),
        ];
        Id::retain_autoreleased(obj).unwrap()
    }
}

#[test]
fn test_retain_autoreleased() {
    #[cfg(gnustep)]
    unsafe {
        objc2::__gnustep_hack::get_class_to_force_linkage()
    };

    #[cfg(apple)]
    #[link(name = "Foundation", kind = "framework")]
    extern "C" {}

    autoreleasepool(|_| {
        // Run once to allow DYLD to resolve the symbol stubs.
        // Required for making `retain_autoreleased` work on x86_64.
        let _data = create_data(b"12");

        // When compiled in release mode / with optimizations enabled,
        // subsequent usage of `retain_autoreleased` will succeed in retaining
        // the autoreleased value!
        let expected = if cfg!(all(debug_assertions, not(gnustep))) {
            2
        } else {
            1
        };

        let data = create_data(b"34");
        assert_eq!(retain_count(&data), expected);

        let data = create_data(b"56");
        assert_eq!(retain_count(&data), expected);

        // Here we manually clean up the autorelease, so it will always be 1.
        let data = autoreleasepool(|_| create_data(b"78"));
        assert_eq!(retain_count(&data), 1);
    });
}
