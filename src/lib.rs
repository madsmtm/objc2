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

#[repr(C)]
struct Block<C: Clone> {
    isa: *const Class,
    flags: c_int,
    _reserved: c_int,
    pub invoke: unsafe extern fn(&mut Block<C>, ...),
    descriptor: *const BlockDescriptor<C>,
    pub context: C,
}

impl<C: Clone> Block<C> {
    pub fn new(invoke: unsafe extern fn(&mut Block<C>, ...), context: C) -> Block<C> {
        Block {
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

    pub fn copy(&self) -> Id<Block<C>> {
        unsafe {
            let block = msg_send![self copy] as *mut Block<C>;
            Id::from_retained_ptr(block)
        }
    }
}

impl<C: Clone> Message for Block<C> { }

impl<C: Clone> Clone for Block<C> {
    fn clone(&self) -> Block<C> {
        Block::new(self.invoke, self.context.clone())
    }
}

unsafe extern fn block_context_dispose<C: Clone>(block: &mut Block<C>) {
    let mut context = mem::uninitialized();
    ptr::copy_nonoverlapping_memory(&mut context, &block.context, 1);
    drop(context);
}

unsafe extern fn block_context_copy<C: Clone>(dst: &mut Block<C>, src: &Block<C>) {
    dst.context = src.context.clone();
}

#[repr(C)]
struct BlockDescriptor<C: Clone> {
    _reserved: c_ulong,
    block_size: c_ulong,
    copy_helper: unsafe extern fn(&mut Block<C>, &Block<C>),
    dispose_helper: unsafe extern fn(&mut Block<C>),
}

impl<C: Clone> BlockDescriptor<C> {
    pub fn new() -> BlockDescriptor<C> {
        BlockDescriptor {
            _reserved: 0,
            block_size: mem::size_of::<Block<C>>() as c_ulong,
            copy_helper: block_context_copy::<C>,
            dispose_helper: block_context_dispose::<C>,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::mem;
    use objc_test_utils::invoke_int_block;
    use super::Block;

    #[test]
    fn test_create_block() {
        extern fn block_get_int(block: &Block<int>) -> int {
            block.context
        }

        let result = unsafe {
            let block = Block::new(mem::transmute(block_get_int), 13i);
            invoke_int_block(mem::transmute(&block))
        };
        assert!(result == 13);
    }
}
