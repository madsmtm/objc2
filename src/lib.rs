//! A Rust interface for Objective-C blocks.
//!
//! For more information on the specifics of the block implementation, see
//! Clang's documentation: http://clang.llvm.org/docs/Block-ABI-Apple.html

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

/// An invoke function for a `Block`; this is the raw C function called by the
/// Objective-C runtime.
type BlockInvoke<A, R> = unsafe extern fn(*mut Block<A, R>, ...) -> R;

/// An invoke function for a `ConcreteBlock`; this is the raw C function called
/// by the Objective-C runtime.
pub type ConcreteBlockInvoke<A, R, C> =
    unsafe extern fn(*mut ConcreteBlock<A, R, C>, ...) -> R;

/// Types that may be used as the arguments to an Objective-C block.
pub trait BlockArguments {
    /// Calls the given `Block` with self as the arguments.
    fn call_block<R>(self, block: &Block<Self, R>) -> R;

    /// Returns an invoke function for a `ConcreteBlock` that takes this type
    /// of arguments.
    fn invoke_for_concrete_block<R, C: Clone>() -> ConcreteBlockInvoke<Self, R, C>;
}

macro_rules! block_args_impl(
    ($f:ident $(, $a:ident : $t:ident)*) => (
        impl<$($t),*> BlockArguments for ($($t,)*) {
            fn call_block<R>(self, block: &Block<($($t,)*), R>) -> R {
                let invoke: unsafe extern fn(*mut Block<($($t,)*), R> $(, $t)*) -> R = unsafe {
                    mem::transmute(block.invoke)
                };
                let ($($a,)*) = self;
                let block_ptr = block as *const _ as *mut _;
                unsafe {
                    invoke(block_ptr $(, $a)*)
                }
            }

            fn invoke_for_concrete_block<R, X: Clone>() ->
                    ConcreteBlockInvoke<($($t,)*), R, X> {
                unsafe extern fn $f<R, X: Clone $(, $t)*>(
                        block_ptr: *mut ConcreteBlock<($($t,)*), R, X>
                        $(, $a: $t)*) -> R {
                    let args = ($($a,)*);
                    let block = &*block_ptr;
                    (block.rust_invoke)(&block.context, args)
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
pub struct Block<A: BlockArguments, R> {
    isa: *const Class,
    flags: c_int,
    _reserved: c_int,
    invoke: BlockInvoke<A, R>,
}

impl<A: BlockArguments, R> Block<A, R> {
    /// Copy self onto the heap.
    pub fn copy(&self) -> Id<Block<A, R>> {
        unsafe {
            let block = msg_send![self copy] as *mut Block<A, R>;
            Id::from_retained_ptr(block)
        }
    }

    /// Call self with the given arguments.
    pub fn call(&self, args: A) -> R {
        args.call_block(self)
    }
}

impl<A: BlockArguments, R> Message for Block<A, R> { }

/// An Objective-C block whose size is known at compile time and may be
/// constructed on the stack.
#[repr(C)]
pub struct ConcreteBlock<A: BlockArguments, R, C: Clone> {
    base: Block<A, R>,
    descriptor: *const BlockDescriptor<ConcreteBlock<A, R, C>>,
    rust_invoke: fn (&C, A) -> R,
    context: C,
}

impl<A: BlockArguments, R, C: Clone> ConcreteBlock<A, R, C> {
    /// Constructs a `ConcreteBlock` with the given invoke function and context.
    /// When the block is called, it will return the value that results from
    /// calling the invoke function with a reference to its context.
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

impl<A: BlockArguments, R, C: Clone> Deref<Block<A, R>> for ConcreteBlock<A, R, C> {
    fn deref(&self) -> &Block<A, R> {
        &self.base
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
    // The src block actually gets memmoved to the destination beforehand,
    // but we'll set the function pointer, too, to be safe.
    dst.rust_invoke = src.rust_invoke;
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
    fn new() -> BlockDescriptor<ConcreteBlock<A, R, C>> {
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
    use Id;
    use objc_test_utils;
    use super::{Block, ConcreteBlock};

    fn get_int_block() -> &'static Block<(), int> {
        unsafe {
            &*(objc_test_utils::get_int_block() as *const _)
        }
    }

    fn get_int_block_with(i: int) -> Id<Block<(), int>> {
        unsafe {
            let ptr = objc_test_utils::get_int_block_with(i);
            Id::from_retained_ptr(ptr as *mut _)
        }
    }

    fn get_add_block() -> &'static Block<(int,), int> {
        unsafe {
            &*(objc_test_utils::get_add_block() as *const _)
        }
    }

    fn get_add_block_with(i: int) -> Id<Block<(int,), int>> {
        unsafe {
            let ptr = objc_test_utils::get_add_block_with(i);
            Id::from_retained_ptr(ptr as *mut _)
        }
    }

    fn invoke_int_block(block: &Block<(), int>) -> int {
        let ptr = block as *const _ as *const _;
        unsafe {
            objc_test_utils::invoke_int_block(ptr)
        }
    }

    fn invoke_add_block(block: &Block<(int,), int>, a: int) -> int {
        let ptr = block as *const _ as *const _;
        unsafe {
            objc_test_utils::invoke_add_block(ptr, a)
        }
    }

    #[test]
    fn test_call_block() {
        let block = get_int_block();
        assert!(block.call(()) == 7);

        let block = get_int_block_with(13);
        assert!(block.call(()) == 13);
    }

    #[test]
    fn test_call_block_args() {
        let block = get_add_block();
        assert!(block.call((2,)) == 9);

        let block = get_add_block_with(13);
        assert!(block.call((2,)) == 15);
    }

    #[test]
    fn test_create_block() {
        fn block_get_int(context: &int, _args: ()) -> int {
            *context
        }

        let block = ConcreteBlock::new(block_get_int, 13);
        let result = invoke_int_block(&*block);
        assert!(result == 13);
    }

    #[test]
    fn test_create_block_args() {
        fn block_add_int(context: &int, (a,): (int,)) -> int {
            a + *context
        }

        let block = ConcreteBlock::new(block_add_int, 5);
        let result = invoke_add_block(&*block, 6);
        assert!(result == 11);
    }

    #[test]
    fn test_concrete_block_copy() {
        fn block_get_string_len(context: &String, _args: ()) -> uint {
            context.len()
        }

        let s = "Hello!".into_string();
        let expected_len = s.len();
        let block = ConcreteBlock::new(block_get_string_len, s);
        assert!(block.call(()) == expected_len);

        let copied = block.copy();
        assert!(copied.call(()) == expected_len);

        drop(block);
        assert!(copied.call(()) == expected_len);
    }
}
