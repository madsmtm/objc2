//! Test that "lending" blocks are not (yet) creatable.
use block2::{Block, RcBlock};

fn takes_lending_block<'b>(block: &Block<'b, fn() -> &'b i32>) {
    assert_eq!(*block.call(), 42);
}

fn main() {
    let x = Box::new(42);
    takes_lending_block(&RcBlock::new(move || &*x));
}
