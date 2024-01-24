use std::cell::OnceCell;
use std::thread_local;

use block2::{Block, RcBlock};

fn tries_to_retain_past_given_lifetime(block: &Block<dyn Fn() + '_>) {
    thread_local! {
        pub static BLOCK: OnceCell<RcBlock<dyn Fn()>> = const { OnceCell::new() };
    }
    BLOCK.with(|thread_local| {
        thread_local.set(block.copy()).unwrap();
    });
}

fn main() {}
