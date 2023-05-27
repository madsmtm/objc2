#![no_main]
use icrate::Foundation::NSString;
use libfuzzer_sys::fuzz_target;
use objc2::rc::autoreleasepool;

fuzz_target!(|s: &str| {
    autoreleasepool(|pool| {
        let obj = NSString::from_str(s);
        assert_eq!(obj.as_str(pool), s);
    });
});
