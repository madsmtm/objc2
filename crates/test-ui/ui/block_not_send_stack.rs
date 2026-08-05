use block2::{SendableBlock, StackBlock};

fn foo(_: &SendableBlock<'_, fn()>) {}

fn main() {
    let non_send = std::rc::Rc::new(10);

    foo(&StackBlock::new(|| println!("{non_send}")));
}
