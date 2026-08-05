use block2::{Block, RcBlock, SendableBlock};

fn main() {
    let block: RcBlock<'_, _> = RcBlock::new(|| {});
    let block_ref: &Block<'_, _> = &*block;

    let _: &RcBlock<'_, _, dyn Send + Sync> = block.as_ref();
    let _: &SendableBlock<'_, _> = block.as_ref();

    let _: &SendableBlock<'_, _> = block_ref.as_ref();
}
