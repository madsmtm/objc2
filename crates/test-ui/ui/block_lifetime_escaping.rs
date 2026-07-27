//! Test the error message when creating an escaping block with a
//! non-`'static` closure.
use block2::{Block, RcBlock};

fn takes_escaping_block<'b>(block: &Block<'static, fn(i32, i32) -> i32>) {
    assert_eq!(block.call(1, 2), 45);
}

fn main() {
    let x = 42;
    // Don't `move`, capture `&x`
    takes_escaping_block(&RcBlock::new(|a, b| x + a + b));
}
