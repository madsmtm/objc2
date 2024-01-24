use block2::{RcBlock, StackBlock};

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
        StackBlock::new(|| x + 2)
    };

    let _ = {
        let x = 2;
        StackBlock::new(|| x + 2).copy()
    };
}
