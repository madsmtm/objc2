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
    pub fn call(&self, args: A) -> R {
        args.invoke_block(self)
    }
}

#[repr(C)]
struct ConcreteBlock<C: Clone> {
    isa: *const Class,
    flags: c_int,
    _reserved: c_int,
    pub invoke: unsafe extern fn(&mut ConcreteBlock<C>, ...),
    descriptor: *const BlockDescriptor<C>,
    pub context: C,
}

impl<C: Clone> ConcreteBlock<C> {
    pub fn new(invoke: unsafe extern fn(&mut ConcreteBlock<C>, ...), context: C) -> ConcreteBlock<C> {
        ConcreteBlock {
            isa: &_NSConcreteStackBlock,
            // 1 << 25 = BLOCK_HAS_COPY_DISPOSE
            flags: 1 << 25,
            _reserved: 0,
            invoke: invoke,
            // TODO: don't leak memory here
            descriptor: unsafe {
                mem::transmute(box BlockDescriptor::<C>::new())
            },
            context: context,
        }
    }

    pub fn copy(&self) -> Id<ConcreteBlock<C>> {
        unsafe {
            let block = msg_send![self copy] as *mut ConcreteBlock<C>;
            Id::from_retained_ptr(block)
        }
    }
}

impl<C: Clone> Message for ConcreteBlock<C> { }

impl<C: Clone> Clone for ConcreteBlock<C> {
    fn clone(&self) -> ConcreteBlock<C> {
        ConcreteBlock::new(self.invoke, self.context.clone())
    }
}

unsafe extern fn block_context_dispose<C: Clone>(block: &mut ConcreteBlock<C>) {
    let mut context = mem::uninitialized();
    ptr::copy_nonoverlapping_memory(&mut context, &block.context, 1);
    drop(context);
}

unsafe extern fn block_context_copy<C: Clone>(dst: &mut ConcreteBlock<C>, src: &ConcreteBlock<C>) {
    dst.context = src.context.clone();
}

#[repr(C)]
struct BlockDescriptor<C: Clone> {
    _reserved: c_ulong,
    block_size: c_ulong,
    copy_helper: unsafe extern fn(&mut ConcreteBlock<C>, &ConcreteBlock<C>),
    dispose_helper: unsafe extern fn(&mut ConcreteBlock<C>),
}

impl<C: Clone> BlockDescriptor<C> {
    pub fn new() -> BlockDescriptor<C> {
        BlockDescriptor {
            _reserved: 0,
            block_size: mem::size_of::<ConcreteBlock<C>>() as c_ulong,
            copy_helper: block_context_copy::<C>,
            dispose_helper: block_context_dispose::<C>,
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
        extern fn block_get_int(block: &ConcreteBlock<int>) -> int {
            block.context
        }

        let result = unsafe {
            let block = ConcreteBlock::new(mem::transmute(block_get_int), 13i);
            invoke_int_block(mem::transmute(&block))
        };
        assert!(result == 13);
    }
}
