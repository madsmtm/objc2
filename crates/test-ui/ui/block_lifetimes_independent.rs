//! Test that lifetimes in blocks are not bound to each other.
//!
//! These tests will succeed if there are `'a: 'b`-like bounds on the closure.
use block2::RcBlock;

fn args<'a, 'b>(
    f: impl Fn(&'a i32, &'b i32) + 'static,
) -> RcBlock<dyn Fn(&'b i32, &'a i32) + 'static> {
    RcBlock::new(f)
}

fn args_return<'a, 'b>(
    f: impl Fn(&'a i32) -> &'b i32 + 'static,
) -> RcBlock<dyn Fn(&'b i32) -> &'a i32 + 'static> {
    RcBlock::new(f)
}

fn args_entire<'a, 'b>(f: impl Fn(&'a i32) + 'b) -> RcBlock<dyn Fn(&'b i32) + 'a> {
    RcBlock::new(f)
}

fn return_entire<'a, 'b>(f: impl Fn() -> &'a i32 + 'b) -> RcBlock<dyn Fn() -> &'b i32 + 'a> {
    RcBlock::new(f)
}

fn main() {}
