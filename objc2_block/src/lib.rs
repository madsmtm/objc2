/*!
A Rust interface for Objective-C blocks.

For more information on the specifics of the block implementation, see
Clang's documentation: <http://clang.llvm.org/docs/Block-ABI-Apple.html>

# Invoking blocks

The `Block` struct is used for invoking blocks from Objective-C. For example,
consider this Objective-C function:

``` objc
int32_t sum(int32_t (^block)(int32_t, int32_t)) {
    return block(5, 8);
}
```

We could write it in Rust as the following:

```
# use objc2_block::Block;
unsafe fn sum(block: &Block<(i32, i32), i32>) -> i32 {
    block.call((5, 8))
}
```

Note the extra parentheses in the `call` method, since the arguments must be
passed as a tuple.

# Creating blocks

Creating a block to pass to Objective-C can be done with the `ConcreteBlock`
struct. For example, to create a block that adds two `i32`s, we could write:

```
# use objc2_block::ConcreteBlock;
let block = ConcreteBlock::new(|a: i32, b: i32| a + b);
let block = block.copy();
assert!(unsafe { block.call((5, 8)) } == 13);
```

It is important to copy your block to the heap (with the `copy` method) before
passing it to Objective-C; this is because our `ConcreteBlock` is only meant
to be copied once, and we can enforce this in Rust, but if Objective-C code
were to copy it twice we could have a double free.
*/

#![no_std]
// Update in Cargo.toml as well.
#![doc(html_root_url = "https://docs.rs/objc2_block/0.1.6")]

extern crate alloc;
extern crate std;

#[cfg(test)]
mod test_utils;

use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem;
use core::ops::{Deref, DerefMut};
use core::ptr;
use std::os::raw::{c_int, c_ulong};

pub use objc2_block_sys as ffi;
use objc2_encode::{Encode, EncodeArguments, Encoding, RefEncode};

/// Types that may be used as the arguments to an Objective-C block.
pub trait BlockArguments: Sized {
    /// Calls the given `Block` with self as the arguments.
    ///
    /// # Safety
    ///
    /// The given block must point to a valid `Block`.
    ///
    /// This invokes foreign code whose safety the user must guarantee.
    unsafe fn call_block<R>(self, block: *mut Block<Self, R>) -> R;
}

macro_rules! block_args_impl {
    ($($a:ident : $t:ident),*) => (
        impl<$($t),*> BlockArguments for ($($t,)*) {
            unsafe fn call_block<R>(self, block: *mut Block<Self, R>) -> R {
                let invoke: unsafe extern fn(*mut Block<Self, R> $(, $t)*) -> R = {
                    let base = block as *mut BlockBase<Self, R>;
                    mem::transmute((*base).invoke)
                };
                let ($($a,)*) = self;
                invoke(block $(, $a)*)
            }
        }
    );
}

block_args_impl!();
block_args_impl!(a: A);
block_args_impl!(a: A, b: B);
block_args_impl!(a: A, b: B, c: C);
block_args_impl!(a: A, b: B, c: C, d: D);
block_args_impl!(a: A, b: B, c: C, d: D, e: E);
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F);
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G);
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
block_args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K
);
block_args_impl!(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L
);

#[repr(C)]
struct BlockBase<A, R> {
    isa: *const ffi::Class,
    flags: c_int,
    _reserved: c_int,
    invoke: unsafe extern "C" fn(*mut Block<A, R>, ...) -> R,
}

/// An Objective-C block that takes arguments of `A` when called and
/// returns a value of `R`.
#[repr(C)]
pub struct Block<A, R> {
    _base: PhantomData<BlockBase<A, R>>,
}

unsafe impl<A: BlockArguments + EncodeArguments, R: Encode> RefEncode for Block<A, R> {
    const ENCODING_REF: Encoding<'static> = Encoding::Block;
}

impl<A: BlockArguments + EncodeArguments, R: Encode> Block<A, R> {
    /// Call self with the given arguments.
    ///
    /// # Safety
    ///
    /// This invokes foreign code that the caller must verify doesn't violate
    /// any of Rust's safety rules.
    ///
    /// For example, if this block is shared with multiple references, the
    /// caller must ensure that calling it will not cause a data race.
    pub unsafe fn call(&self, args: A) -> R {
        args.call_block(self as *const _ as *mut _)
    }
}

/// A reference-counted Objective-C block.
pub struct RcBlock<A, R> {
    ptr: *mut Block<A, R>,
}

impl<A, R> RcBlock<A, R> {
    /// Construct an `RcBlock` for the given block without copying it.
    /// The caller must ensure the block has a +1 reference count.
    ///
    /// # Safety
    ///
    /// The given pointer must point to a valid `Block` and must have a +1
    /// reference count or it will be overreleased when the `RcBlock` is
    /// dropped.
    pub unsafe fn new(ptr: *mut Block<A, R>) -> Self {
        RcBlock { ptr }
    }

    /// Constructs an `RcBlock` by copying the given block.
    ///
    /// # Safety
    ///
    /// The given pointer must point to a valid `Block`.
    pub unsafe fn copy(ptr: *mut Block<A, R>) -> Self {
        let ptr = ffi::_Block_copy(ptr as *const c_void) as *mut Block<A, R>;
        RcBlock { ptr }
    }
}

impl<A, R> Clone for RcBlock<A, R> {
    fn clone(&self) -> RcBlock<A, R> {
        unsafe { RcBlock::copy(self.ptr) }
    }
}

impl<A, R> Deref for RcBlock<A, R> {
    type Target = Block<A, R>;

    fn deref(&self) -> &Block<A, R> {
        unsafe { &*self.ptr }
    }
}

impl<A, R> Drop for RcBlock<A, R> {
    fn drop(&mut self) {
        unsafe {
            ffi::_Block_release(self.ptr as *const c_void);
        }
    }
}

/// Types that may be converted into a `ConcreteBlock`.
pub trait IntoConcreteBlock<A: BlockArguments + EncodeArguments>: Sized {
    /// The return type of the resulting `ConcreteBlock`.
    type Ret: Encode;

    /// Consumes self to create a `ConcreteBlock`.
    fn into_concrete_block(self) -> ConcreteBlock<A, Self::Ret, Self>;
}

macro_rules! concrete_block_impl {
    ($f:ident) => (
        concrete_block_impl!($f,);
    );
    ($f:ident, $($a:ident : $t:ident),*) => (
        impl<$($t: Encode,)* R: Encode, X> IntoConcreteBlock<($($t,)*)> for X
                where X: Fn($($t,)*) -> R {
            type Ret = R;

            fn into_concrete_block(self) -> ConcreteBlock<($($t,)*), R, X> {
                unsafe extern fn $f<$($t,)* R, X>(
                        block_ptr: *mut ConcreteBlock<($($t,)*), R, X>
                        $(, $a: $t)*) -> R
                        where X: Fn($($t,)*) -> R {
                    let block = &*block_ptr;
                    (block.closure)($($a),*)
                }

                let f: unsafe extern fn(*mut ConcreteBlock<($($t,)*), R, X> $(, $a: $t)*) -> R = $f;
                unsafe {
                    ConcreteBlock::with_invoke(mem::transmute(f), self)
                }
            }
        }
    );
}

concrete_block_impl!(concrete_block_invoke_args0);
concrete_block_impl!(concrete_block_invoke_args1, a: A);
concrete_block_impl!(concrete_block_invoke_args2, a: A, b: B);
concrete_block_impl!(concrete_block_invoke_args3, a: A, b: B, c: C);
concrete_block_impl!(concrete_block_invoke_args4, a: A, b: B, c: C, d: D);
concrete_block_impl!(concrete_block_invoke_args5, a: A, b: B, c: C, d: D, e: E);
concrete_block_impl!(
    concrete_block_invoke_args6,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F
);
concrete_block_impl!(
    concrete_block_invoke_args7,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G
);
concrete_block_impl!(
    concrete_block_invoke_args8,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H
);
concrete_block_impl!(
    concrete_block_invoke_args9,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I
);
concrete_block_impl!(
    concrete_block_invoke_args10,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J
);
concrete_block_impl!(
    concrete_block_invoke_args11,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K
);
concrete_block_impl!(
    concrete_block_invoke_args12,
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G,
    h: H,
    i: I,
    j: J,
    k: K,
    l: L
);

/// An Objective-C block whose size is known at compile time and may be
/// constructed on the stack.
#[repr(C)]
pub struct ConcreteBlock<A, R, F> {
    base: BlockBase<A, R>,
    descriptor: *const BlockDescriptor<ConcreteBlock<A, R, F>>,
    closure: F,
}

unsafe impl<A: BlockArguments + EncodeArguments, R: Encode, F> RefEncode
    for ConcreteBlock<A, R, F>
{
    const ENCODING_REF: Encoding<'static> = Encoding::Block;
}

impl<A, R, F> ConcreteBlock<A, R, F>
where
    A: BlockArguments + EncodeArguments,
    R: Encode,
    F: IntoConcreteBlock<A, Ret = R>,
{
    /// Constructs a `ConcreteBlock` with the given closure.
    /// When the block is called, it will return the value that results from
    /// calling the closure.
    pub fn new(closure: F) -> Self {
        closure.into_concrete_block()
    }
}

impl<A, R, F> ConcreteBlock<A, R, F> {
    const DESCRIPTOR: BlockDescriptor<Self> = BlockDescriptor {
        _reserved: 0,
        block_size: mem::size_of::<Self>() as c_ulong,
        copy_helper: block_context_copy::<Self>,
        dispose_helper: block_context_dispose::<Self>,
    };

    /// Constructs a `ConcreteBlock` with the given invoke function and closure.
    /// Unsafe because the caller must ensure the invoke function takes the
    /// correct arguments.
    unsafe fn with_invoke(invoke: unsafe extern "C" fn(*mut Self, ...) -> R, closure: F) -> Self {
        ConcreteBlock {
            base: BlockBase {
                isa: &ffi::_NSConcreteStackBlock,
                flags: ffi::BLOCK_HAS_COPY_DISPOSE,
                _reserved: 0,
                invoke: mem::transmute(invoke),
            },
            descriptor: &Self::DESCRIPTOR,
            closure,
        }
    }
}

impl<A, R, F> ConcreteBlock<A, R, F>
where
    F: 'static,
{
    /// Copy self onto the heap as an `RcBlock`.
    pub fn copy(self) -> RcBlock<A, R> {
        unsafe {
            let mut block = self;
            let copied = RcBlock::copy(&mut *block);
            // At this point, our copy helper has been run so the block will
            // be moved to the heap and we can forget the original block
            // because the heap block will drop in our dispose helper.
            mem::forget(block);
            copied
        }
    }
}

impl<A, R, F> Clone for ConcreteBlock<A, R, F>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        unsafe {
            ConcreteBlock::with_invoke(mem::transmute(self.base.invoke), self.closure.clone())
        }
    }
}

impl<A, R, F> Deref for ConcreteBlock<A, R, F> {
    type Target = Block<A, R>;

    fn deref(&self) -> &Block<A, R> {
        unsafe { &*(&self.base as *const _ as *const Block<A, R>) }
    }
}

impl<A, R, F> DerefMut for ConcreteBlock<A, R, F> {
    fn deref_mut(&mut self) -> &mut Block<A, R> {
        unsafe { &mut *(&mut self.base as *mut _ as *mut Block<A, R>) }
    }
}

unsafe extern "C" fn block_context_dispose<B>(block: &mut B) {
    // Read the block onto the stack and let it drop
    ptr::read(block);
}

unsafe extern "C" fn block_context_copy<B>(_dst: &mut B, _src: &B) {
    // The runtime memmoves the src block into the dst block, nothing to do
}

#[repr(C)]
struct BlockDescriptor<B> {
    _reserved: c_ulong,
    block_size: c_ulong,
    copy_helper: unsafe extern "C" fn(&mut B, &B),
    dispose_helper: unsafe extern "C" fn(&mut B),
}

#[cfg(test)]
mod tests {
    use super::{ConcreteBlock, RcBlock};
    use crate::test_utils::*;
    use alloc::string::ToString;

    #[test]
    fn test_call_block() {
        let block = get_int_block_with(13);
        unsafe {
            assert!(block.call(()) == 13);
        }
    }

    #[test]
    fn test_call_block_args() {
        let block = get_add_block_with(13);
        unsafe {
            assert!(block.call((2,)) == 15);
        }
    }

    #[test]
    fn test_create_block() {
        let block = ConcreteBlock::new(|| 13);
        let result = invoke_int_block(&block);
        assert!(result == 13);
    }

    #[test]
    fn test_create_block_args() {
        let block = ConcreteBlock::new(|a: i32| a + 5);
        let result = invoke_add_block(&block, 6);
        assert!(result == 11);
    }

    #[test]
    fn test_concrete_block_copy() {
        let s = "Hello!".to_string();
        let expected_len = s.len() as i32;
        let block = ConcreteBlock::new(move || s.len() as i32);
        assert!(invoke_int_block(&block) == expected_len);

        let copied = block.copy();
        assert!(invoke_int_block(&copied) == expected_len);
    }

    #[test]
    fn test_concrete_block_stack_copy() {
        fn make_block() -> RcBlock<(), i32> {
            let x = 7;
            let block = ConcreteBlock::new(move || x);
            block.copy()
        }

        let block = make_block();
        assert!(invoke_int_block(&block) == 7);
    }
}
