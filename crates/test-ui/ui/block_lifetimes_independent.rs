//! Test that lifetimes in blocks are not bound to each other.
//!
//! These tests will succeed if there are `'a: 'b`-like bounds on the closure.
use block2::{ManualBlockEncoding, RcBlock};
use std::ffi::CStr;
use std::marker::PhantomData;

fn args<'a, 'b>(f: impl Fn(&'a i32, &'b i32) + 'static) -> RcBlock<'static, fn(&'b i32, &'a i32)> {
    RcBlock::new(f)
}

fn args_return<'a, 'b>(
    f: impl Fn(&'a i32) -> &'b i32 + 'static,
) -> RcBlock<'static, fn(&'b i32) -> &'a i32> {
    RcBlock::new(f)
}

fn args_entire<'a, 'b>(f: impl Fn(&'a i32) + 'b) -> RcBlock<'a, fn(&'b i32)> {
    RcBlock::new(f)
}

fn return_entire<'a, 'b>(f: impl Fn() -> &'a i32 + 'b) -> RcBlock<'a, fn() -> &'b i32> {
    RcBlock::new(f)
}

fn args_with_encoding<'a, 'b>(
    f: impl Fn(&'a i32, &'b i32) + 'static,
) -> RcBlock<'static, fn(&'b i32, &'a i32)> {
    struct Enc<'a, 'b>(PhantomData<&'a i32>, PhantomData<&'b i32>);
    unsafe impl<'a, 'b> ManualBlockEncoding for Enc<'a, 'b> {
        type Signature = fn(&'a i32, &'b i32);
        const ENCODING_CSTR: &'static CStr = c"v24@?0^i8^i16";
    }
    RcBlock::with_encoding::<_, Enc<'a, 'b>>(f)
}

fn args_return_with_encoding<'a, 'b>(
    f: impl Fn(&'a i32) -> &'b i32 + 'static,
) -> RcBlock<'static, fn(&'b i32) -> &'a i32> {
    struct Enc<'a, 'b>(PhantomData<&'a i32>, PhantomData<&'b i32>);
    unsafe impl<'a, 'b> ManualBlockEncoding for Enc<'a, 'b> {
        type Signature = fn(&'a i32) -> &'b i32;
        const ENCODING_CSTR: &'static CStr = c"^i816@?0^i8";
    }
    RcBlock::with_encoding::<_, Enc<'a, 'b>>(f)
}

fn args_entire_with_encoding<'a, 'b>(f: impl Fn(&'a i32) + 'b) -> RcBlock<'a, fn(&'b i32)> {
    struct Enc<'a>(PhantomData<&'a i32>);
    unsafe impl<'a> ManualBlockEncoding for Enc<'a> {
        type Signature = fn(&'a i32);
        const ENCODING_CSTR: &'static CStr = c"v16@?0^i8";
    }
    RcBlock::with_encoding::<_, Enc<'a>>(f)
}

fn return_entire_with_encoding<'a, 'b>(
    f: impl Fn() -> &'a i32 + 'b,
) -> RcBlock<'a, fn() -> &'b i32> {
    struct Enc<'a>(PhantomData<&'a i32>);
    unsafe impl<'a> ManualBlockEncoding for Enc<'a> {
        type Signature = fn() -> &'a i32;
        const ENCODING_CSTR: &'static CStr = c"^i8@?0";
    }
    RcBlock::with_encoding::<_, Enc<'a>>(f)
}

fn main() {}
