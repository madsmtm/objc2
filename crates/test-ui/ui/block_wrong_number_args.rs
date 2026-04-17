use block2::{Block, RcBlock, StackBlock};

fn takes_block0(block: &Block<dyn Fn()>) {
    block.call(1);
}

fn takes_block1(block: &Block<dyn Fn(i32)>) {
    block.call();
    block.call(1, 2);
}

fn main() {
    takes_block0(&StackBlock::new(|_x| {}));
    takes_block0(&RcBlock::new(|_x| {}));

    takes_block1(&StackBlock::new(|| {}));
    takes_block1(&RcBlock::new(|| {}));

    takes_block1(&StackBlock::new(|_x, _y| {}));
    takes_block1(&RcBlock::new(|_x, _y| {}));
}
