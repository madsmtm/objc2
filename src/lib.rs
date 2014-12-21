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

pub type BlockInvoke<A, R> =
    unsafe extern fn(*mut Block<A, R>, ...) -> R;

pub type ConcreteBlockInvoke<A, R, C> =
    unsafe extern fn(*mut ConcreteBlock<A, R, C>, ...) -> R;

pub trait BlockArguments {
    fn call_block<R>(self, block: &Block<Self, R>) -> R;

    fn invoke_for_concrete_block<R, C: Clone>() -> ConcreteBlockInvoke<Self, R, C>;
}

impl BlockArguments for () {
    fn call_block<R>(self, block: &Block<(), R>) -> R {
        let invoke: unsafe extern fn(*mut Block<(), R>) -> R = unsafe {
            mem::transmute(block.invoke)
        };
        let block_ptr = block as *const _ as *mut _;
        unsafe {
            invoke(block_ptr)
        }
    }

    fn invoke_for_concrete_block<R, C: Clone>() -> ConcreteBlockInvoke<(), R, C> {
        unsafe extern fn concrete_block_invoke_args0<R, C: Clone>(
                block_ptr: *mut ConcreteBlock<(), R, C>) -> R {
            let block = &*block_ptr;
            (block.rust_invoke)(&block.context, ())
        }

        unsafe {
            mem::transmute(concrete_block_invoke_args0::<R, C>)
        }
    }
}

#[repr(C)]
pub struct Block<A: BlockArguments, R> {
    isa: *const Class,
    flags: c_int,
    _reserved: c_int,
    invoke: BlockInvoke<A, R>,
}

impl<A: BlockArguments, R> Block<A, R> {
    pub fn copy(&self) -> Id<Block<A, R>> {
        unsafe {
            let block = msg_send![self copy] as *mut Block<A, R>;
            Id::from_retained_ptr(block)
        }
    }

    pub fn call(&self, args: A) -> R {
        args.call_block(self)
    }
}

impl<A: BlockArguments, R> Message for Block<A, R> { }

#[repr(C)]
struct ConcreteBlock<A: BlockArguments, R, C: Clone> {
    base: Block<A, R>,
    descriptor: *const BlockDescriptor<ConcreteBlock<A, R, C>>,
    rust_invoke: fn (&C, A) -> R,
    context: C,
}

impl<A: BlockArguments, R, C: Clone> ConcreteBlock<A, R, C> {
    pub fn new(invoke: fn (&C, A) -> R, context: C) -> ConcreteBlock<A, R, C> {
        let extern_invoke: ConcreteBlockInvoke<A, R, C> =
            BlockArguments::invoke_for_concrete_block();
        ConcreteBlock {
            base: Block {
                isa: &_NSConcreteStackBlock,
                // 1 << 25 = BLOCK_HAS_COPY_DISPOSE
                flags: 1 << 25,
                _reserved: 0,
                invoke: unsafe { mem::transmute(extern_invoke) },
            },
            // TODO: don't leak memory here
            descriptor: unsafe {
                mem::transmute(box BlockDescriptor::<A, R, C>::new())
            },
            rust_invoke: invoke,
            context: context,
        }
    }
}

impl<A: BlockArguments, R, C: Clone> Clone for ConcreteBlock<A, R, C> {
    fn clone(&self) -> ConcreteBlock<A, R, C> {
        ConcreteBlock::new(self.rust_invoke, self.context.clone())
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
        fn block_get_int(context: &int, _args: ()) -> int {
            *context
        }

        let result = unsafe {
            let block = ConcreteBlock::new(block_get_int, 13i);
            invoke_int_block(mem::transmute(&block))
        };
        assert!(result == 13);
    }
}
