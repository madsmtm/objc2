//! Ideally, blocks should have the same variance as `fn(A) -> R`, namely with
//! `A` being contravariant, and `R` being invariant, see:
//! <https://doc.rust-lang.org/nightly/reference/subtyping.html#variance>
//!
//! However, this is currently not the case for `dyn Fn(A) -> R`, and hence
//! not for blocks, see:
//! <https://github.com/rust-lang/rust/issues/100728>
use block2::{Block, GlobalBlock, RcBlock, StackBlock};

fn block<'a, 'r, 'f, 'b>(
    b: &'b Block<dyn Fn(&'a i32) -> &'static i32 + 'static>,
) -> &'b Block<dyn Fn(&'static i32) -> &'r i32 + 'f> {
    b
}

fn global<'a, 'r, 'f>(
    b: GlobalBlock<dyn Fn(&'a i32) -> &'static i32 + 'static>,
) -> GlobalBlock<dyn Fn(&'static i32) -> &'r i32 + 'f> {
    b
}

fn rc<'a, 'r, 'f>(
    b: RcBlock<dyn Fn(&'a i32) -> &'static i32 + 'static>,
) -> RcBlock<dyn Fn(&'static i32) -> &'r i32 + 'f> {
    b
}

fn stack<'a, 'r, 'f, 'b, F>(
    b: StackBlock<'static, (&'a i32,), &'static i32, F>,
) -> StackBlock<'f, (&'static i32,), &'r i32, F> {
    b
}

fn main() {}
