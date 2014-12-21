#![allow(dead_code)]

use std::mem;
use std::ptr;
use libc::{c_int, c_ulong};

use runtime::Class;
use {Id, Message};

#[allow(improper_ctypes)]
#[link(name = "Foundation", kind = "framework")]
extern {
    static _NSConcreteStackBlock: Class;
}

pub trait BlockArguments {
    fn invoke_block<R>(self, block: &Block<Self, R>) -> R;
}

impl BlockArguments for () {
    fn invoke_block<R>(self, block: &Block<(), R>) -> R {
        let invoke: unsafe extern fn(*mut Block<(), R>) -> R = unsafe {
            mem::transmute(block.invoke)
        };
        let block_ptr = block as *const _ as *mut _;
        unsafe {
            invoke(block_ptr)
        }
    }
}

#[repr(C)]
pub struct Block<A: BlockArguments, R> {
    isa: *const Class,
    flags: c_int,
    _reserved: c_int,
    invoke: unsafe extern fn(*mut Block<A, R>, ...) -> R,
}

impl<A: BlockArguments, R> Block<A, R> {
    pub fn copy(&self) -> Id<Block<A, R>> {
        unsafe {
            let block = msg_send![self copy] as *mut Block<A, R>;
            Id::from_retained_ptr(block)
        }
    }

    pub fn call(&self, args: A) -> R {
        args.invoke_block(self)
    }
}

impl<A: BlockArguments, R> Message for Block<A, R> { }

#[repr(C)]
struct ConcreteBlock<A: BlockArguments, R, C: Clone> {
    base: Block<A, R>,
    descriptor: *const BlockDescriptor<ConcreteBlock<A, R, C>>,
    pub context: C,
}

impl<A: BlockArguments, R, C: Clone> ConcreteBlock<A, R, C> {
    pub fn new(invoke: unsafe extern fn(&ConcreteBlock<A, R, C>, ...) -> R, context: C) -> ConcreteBlock<A, R, C> {
        ConcreteBlock {
            base: Block {
                isa: &_NSConcreteStackBlock,
                // 1 << 25 = BLOCK_HAS_COPY_DISPOSE
                flags: 1 << 25,
                _reserved: 0,
                invoke: unsafe { mem::transmute(invoke) },
            },
            // TODO: don't leak memory here
            descriptor: unsafe {
                mem::transmute(box BlockDescriptor::<A, R, C>::new())
            },
            context: context,
        }
    }
}

impl<A: BlockArguments, R, C: Clone> Clone for ConcreteBlock<A, R, C> {
    fn clone(&self) -> ConcreteBlock<A, R, C> {
        let invoke = unsafe { mem::transmute(self.base.invoke) };
        ConcreteBlock::new(invoke, self.context.clone())
    }
}

unsafe extern fn block_context_dispose<A: BlockArguments, R, C: Clone>(
        block: &mut ConcreteBlock<A, R, C>) {
    let mut context = mem::uninitialized();
    ptr::copy_nonoverlapping_memory(&mut context, &block.context, 1);
    drop(context);
}

unsafe extern fn block_context_copy<A: BlockArguments, R, C: Clone>(
        dst: &mut ConcreteBlock<A, R, C>, src: &ConcreteBlock<A, R, C>) {
    dst.context = src.context.clone();
}

#[repr(C)]
struct BlockDescriptor<B> {
    _reserved: c_ulong,
    block_size: c_ulong,
    copy_helper: unsafe extern fn(&mut B, &B),
    dispose_helper: unsafe extern fn(&mut B),
}

impl<A: BlockArguments, R, C: Clone> BlockDescriptor<ConcreteBlock<A, R, C>> {
    pub fn new() -> BlockDescriptor<ConcreteBlock<A, R, C>> {
        BlockDescriptor {
            _reserved: 0,
            block_size: mem::size_of::<ConcreteBlock<A, R, C>>() as c_ulong,
            copy_helper: block_context_copy::<A, R, C>,
            dispose_helper: block_context_dispose::<A, R, C>,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use objc_test_utils::invoke_int_block;
    use super::ConcreteBlock;

    #[test]
    fn test_create_block() {
        extern fn block_get_int(block: &ConcreteBlock<(), int, int>) -> int {
            block.context
        }

        let result = unsafe {
            let block: ConcreteBlock<(), int, int> =
                ConcreteBlock::new(mem::transmute(block_get_int), 13i);
            invoke_int_block(mem::transmute(&block))
        };
        assert!(result == 13);
    }
}
