use block2::{ManualBlockEncoding, StackBlock};
use std::ffi::CStr;

struct Foo;

fn main() {
    struct FooBlockEncoding;
    unsafe impl ManualBlockEncoding for FooBlockEncoding {
        type Arguments = ();
        type Return = ();
        const ENCODING_CSTR: &'static CStr = c"v8@?0";
    }

    let foo = Foo;
    let _ = StackBlock::with_encoding::<FooBlockEncoding>(move || {
        let _ = &foo;
    });
}
