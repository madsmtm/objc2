//! A Rust interface for Objective-C blocks.
//!
//! For more information on the specifics of the block implementation, see
//! Clang's documentation: http://clang.llvm.org/docs/Block-ABI-Apple.html

use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use libc::{c_int, c_ulong};

use runtime::Class;
use {Id, Message};

#[allow(improper_ctypes)]
#[link(name = "Foundation", kind = "framework")]
extern {
    static _NSConcreteStackBlock: Class;
}

/// Types that may be used as the arguments to an Objective-C block.
pub trait BlockArguments {
    /// Calls the given `Block` with self as the arguments.
    fn call_block<R>(self, block: &mut Block<Self, R>) -> R;

    /// Returns an invoke function for a `ConcreteBlock` that takes this type
    /// of arguments.
    fn invoke_for_concrete_block<R, F>() ->
            unsafe extern fn(*mut ConcreteBlock<Self, R, F>, ...) -> R
            where F: Fn<Self, R>;
}

macro_rules! block_args_impl(
    ($f:ident) => (
        block_args_impl!($f,);
    );
    ($f:ident, $($a:ident : $t:ident),*) => (
        impl<$($t),*> BlockArguments for ($($t,)*) {
            fn call_block<R>(self, block: &mut Block<Self, R>) -> R {
                let invoke: unsafe extern fn(*mut Block<Self, R> $(, $t)*) -> R = unsafe {
                    mem::transmute(block.invoke)
                };
                let ($($a,)*) = self;
                unsafe {
                    invoke(block $(, $a)*)
                }
            }

            fn invoke_for_concrete_block<R, X>() ->
                    unsafe extern fn(*mut ConcreteBlock<Self, R, X>, ...) -> R
                    where X: Fn<Self, R> {
                unsafe extern fn $f<R, X $(, $t)*>(
                        block_ptr: *mut ConcreteBlock<Self, R, X>
                        $(, $a: $t)*) -> R
                        where X: Fn<Self, R> {
                    let block = &*block_ptr;
                    (block.closure)($($a),*)
                }

                unsafe {
                    mem::transmute($f::<R, X $(, $t)*>)
                }
            }
        }
    );
);

block_args_impl!(concrete_block_invoke_args0);
block_args_impl!(concrete_block_invoke_args1, a: A);
block_args_impl!(concrete_block_invoke_args2, a: A, b: B);
block_args_impl!(concrete_block_invoke_args3, a: A, b: B, c: C);
block_args_impl!(concrete_block_invoke_args4, a: A, b: B, c: C, d: D);
block_args_impl!(concrete_block_invoke_args5, a: A, b: B, c: C, d: D, e: E);
block_args_impl!(concrete_block_invoke_args6, a: A, b: B, c: C, d: D, e: E, f: F);
block_args_impl!(concrete_block_invoke_args7, a: A, b: B, c: C, d: D, e: E, f: F, g: G);
block_args_impl!(concrete_block_invoke_args8, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
block_args_impl!(concrete_block_invoke_args9, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
block_args_impl!(concrete_block_invoke_args10, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
block_args_impl!(concrete_block_invoke_args11, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K);
block_args_impl!(concrete_block_invoke_args12, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L);

/// An Objective-C block that takes arguments of `A` when called and
/// returns a value of `R`.
#[repr(C)]
pub struct Block<A, R> where A: BlockArguments {
    isa: *const Class,
    flags: c_int,
    _reserved: c_int,
    invoke: unsafe extern fn(*mut Block<A, R>, ...) -> R,
}

// TODO: impl FnMut when it's possible
impl<A: BlockArguments, R> Block<A, R> where A: BlockArguments {
    /// Call self with the given arguments.
    pub fn call(&mut self, args: A) -> R {
        args.call_block(self)
    }
}

unsafe impl<A, R> Message for Block<A, R> where A: BlockArguments { }

/// An Objective-C block whose size is known at compile time and may be
/// constructed on the stack.
#[repr(C)]
pub struct ConcreteBlock<A, R, F> where A: BlockArguments {
    base: Block<A, R>,
    descriptor: Box<BlockDescriptor<ConcreteBlock<A, R, F>>>,
    closure: F,
}

impl<A, R, F> ConcreteBlock<A, R, F> where A: BlockArguments, F: Fn<A, R> {
    /// Constructs a `ConcreteBlock` with the given closure.
    /// When the block is called, it will return the value that results from
    /// calling the closure.
    pub fn new(closure: F) -> Self {
        let extern_invoke =
            BlockArguments::invoke_for_concrete_block::<R, F>();
        ConcreteBlock {
            base: Block {
                isa: &_NSConcreteStackBlock,
                // 1 << 25 = BLOCK_HAS_COPY_DISPOSE
                flags: 1 << 25,
                _reserved: 0,
                invoke: unsafe { mem::transmute(extern_invoke) },
            },
            descriptor: Box::new(BlockDescriptor::<Self>::new()),
            closure: closure,
        }
    }

    /// Copy self onto the heap.
    pub fn copy(self) -> Id<Block<A, R>> {
        unsafe {
            let block: *mut Block<A, R> = msg_send![&self.base, copy];
            // At this point, our copy helper has been run so the block will
            // be moved to the heap and we can forget the original block
            // because the heap block will drop in our dispose helper.
            mem::forget(self);
            Id::from_retained_ptr(block)
        }
    }
}

impl<A, R, F> Clone for ConcreteBlock<A, R, F>
        where A: BlockArguments, F: Fn<A, R> + Clone {
    fn clone(&self) -> Self {
        ConcreteBlock::new(self.closure.clone())
    }
}

impl<A, R, F> Deref for ConcreteBlock<A, R, F>
        where A: BlockArguments, F: Fn<A, R> {
    type Target = Block<A, R>;

    fn deref(&self) -> &Block<A, R> {
        &self.base
    }
}

impl<A, R, F> DerefMut for ConcreteBlock<A, R, F>
        where A: BlockArguments, F: Fn<A, R> {
    fn deref_mut(&mut self) -> &mut Block<A, R> {
        &mut self.base
    }
}

unsafe extern fn block_context_dispose<B>(block: &mut B) {
    // Read the block onto the stack and let it drop
    ptr::read(block);
}

unsafe extern fn block_context_copy<B>(_dst: &mut B, _src: &B) {
    // The runtime memmoves the src block into the dst block, nothing to do
}

#[repr(C)]
struct BlockDescriptor<B> {
    _reserved: c_ulong,
    block_size: c_ulong,
    copy_helper: unsafe extern fn(&mut B, &B),
    dispose_helper: unsafe extern fn(&mut B),
}

impl<B> BlockDescriptor<B> {
    fn new() -> BlockDescriptor<B> {
        BlockDescriptor {
            _reserved: 0,
            block_size: mem::size_of::<B>() as c_ulong,
            copy_helper: block_context_copy::<B>,
            dispose_helper: block_context_dispose::<B>,
        }
    }
}

#[cfg(test)]
mod tests {
    use Id;
    use objc_test_utils;
    use super::{Block, ConcreteBlock};

    fn get_int_block_with(i: i32) -> Id<Block<(), i32>> {
        unsafe {
            let ptr = objc_test_utils::get_int_block_with(i);
            Id::from_retained_ptr(ptr as *mut _)
        }
    }

    fn get_add_block_with(i: i32) -> Id<Block<(i32,), i32>> {
        unsafe {
            let ptr = objc_test_utils::get_add_block_with(i);
            Id::from_retained_ptr(ptr as *mut _)
        }
    }

    fn invoke_int_block(block: &mut Block<(), i32>) -> i32 {
        let ptr = block as *mut _;
        unsafe {
            objc_test_utils::invoke_int_block(ptr as *mut _)
        }
    }

    fn invoke_add_block(block: &mut Block<(i32,), i32>, a: i32) -> i32 {
        let ptr = block as *mut _;
        unsafe {
            objc_test_utils::invoke_add_block(ptr as *mut _, a)
        }
    }

    #[test]
    fn test_call_block() {
        let mut block = get_int_block_with(13);
        assert!(block.call(()) == 13);
    }

    #[test]
    fn test_call_block_args() {
        let mut block = get_add_block_with(13);
        assert!(block.call((2,)) == 15);
    }

    #[test]
    fn test_create_block() {
        let mut block = ConcreteBlock::new(|&:| 13);
        let result = invoke_int_block(&mut *block);
        assert!(result == 13);
    }

    #[test]
    fn test_create_block_args() {
        let mut block = ConcreteBlock::new(|&: a: i32| a + 5);
        let result = invoke_add_block(&mut *block, 6);
        assert!(result == 11);
    }

    #[test]
    fn test_concrete_block_copy() {
        let s = String::from_str("Hello!");
        let expected_len = s.len() as i32;
        let mut block = ConcreteBlock::new(move |&:| s.len() as i32);
        assert!(invoke_int_block(&mut *block) == expected_len);

        let mut copied = block.copy();
        assert!(invoke_int_block(&mut *copied) == expected_len);
    }
}
