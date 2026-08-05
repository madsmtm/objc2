use block2::{ManualBlockEncoding, RcBlock, StackBlock};
use std::ffi::CStr;

struct VoidToI32;
unsafe impl ManualBlockEncoding for VoidToI32 {
    type Signature = fn() -> i32;
    const ENCODING_CSTR: &'static CStr = c"i8@?0";
}

fn main() {
    let _ = {
        let x = 2;
        RcBlock::<_>::new(|| x + 2)
    };

    let _ = {
        let x = 2;
        RcBlock::<_>::new(|| x + 2).clone()
    };

    let _ = {
        let x = 2;
        RcBlock::<_>::with_encoding::<_, VoidToI32>(|| x + 2)
    };

    let _ = {
        let x = 2;
        RcBlock::<_>::with_encoding::<_, VoidToI32>(|| x + 2).clone()
    };

    let _ = {
        let x = 2;
        StackBlock::<_, _>::new(|| x + 2)
    };

    let _ = {
        let x = 2;
        StackBlock::<_, _>::new(|| x + 2).copy()
    };

    let _ = {
        let x = 2;
        StackBlock::<_, _>::with_encoding::<VoidToI32>(|| x + 2)
    };

    let _ = {
        let x = 2;
        StackBlock::<_, _>::with_encoding::<VoidToI32>(|| x + 2).copy()
    };
}
