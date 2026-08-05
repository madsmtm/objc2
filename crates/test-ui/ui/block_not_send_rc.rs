use block2::{RcBlock, SendableBlock};

fn foo(_: &SendableBlock<'_, fn()>) {}

fn main() {
    let non_send = std::rc::Rc::new(10);

    foo(&RcBlock::new(|| println!("{non_send}")));
}
