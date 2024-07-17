use block2::{ManualBlockEncoding, RcBlock, StackBlock};
use std::ffi::CStr;

struct VoidToI32;
unsafe impl ManualBlockEncoding for VoidToI32 {
    type Arguments = ();
    type Return = i32;
    const ENCODING_CSTR: &'static CStr = c"i8@?0";
}

fn main() {
    let _ = {
        let x = 2;
        RcBlock::new(|| x + 2)
    };

    let _ = {
        let x = 2;
        RcBlock::new(|| x + 2).clone()
    };

    let _ = {
        let x = 2;
        unsafe { RcBlock::with_encoding::<_, _, _, VoidToI32>(|| x + 2) }
    };

    let _ = {
        let x = 2;
        unsafe { RcBlock::with_encoding::<_, _, _, VoidToI32>(|| x + 2).clone() }
    };

    let _ = {
        let x = 2;
        StackBlock::new(|| x + 2)
    };

    let _ = {
        let x = 2;
        StackBlock::new(|| x + 2).copy()
    };

    let _ = {
        let x = 2;
        unsafe { StackBlock::with_encoding::<VoidToI32>(|| x + 2) }
    };

    let _ = {
        let x = 2;
        unsafe { StackBlock::with_encoding::<VoidToI32>(|| x + 2).copy() }
    };
}
