/*!
A Rust interface for Objective-C blocks.

For more information on the specifics of the block implementation, see
Clang's documentation: http://clang.llvm.org/docs/Block-ABI-Apple.html

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
# use objc::block::Block;
fn sum(block: &mut Block<(i32, i32), i32>) -> i32 {
    block.call((5, 8))
}
```

Note the extra parentheses in the `call` method, since the arguments must be
passed as a tuple.

# Creating blocks

Creating a block to pass to Objective-C can be done with the `ConcreteBlock`
struct. For example, to create a block that adds two `i32`s, we could write:

```
# use objc::block::ConcreteBlock;
let block = ConcreteBlock::new(|a: i32, b: i32| a + b);
let mut block = block.copy();
assert!(block.call((5, 8)) == 13);
```

It is important to copy your block to the heap (with the `copy` method) before
passing it to Objective-C; this is because our `ConcreteBlock` is only meant
to be copied once, and we can enforce this in Rust, but if Objective-C code
were to copy it twice we could have a double free.
*/

use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr;
use libc::{c_int, c_ulong};

use runtime::{Class, Object};
use Id;

#[link(name = "Foundation", kind = "framework")]
extern {
    static _NSConcreteStackBlock: Class;
}

/// Types that may be used as the arguments to an Objective-C block.
pub trait BlockArguments {
    /// Calls the given `Block` with self as the arguments.
    fn call_block<R>(self, block: &mut Block<Self, R>) -> R;
}

macro_rules! block_args_impl {
    ($($a:ident : $t:ident),*) => (
        impl<$($t),*> BlockArguments for ($($t,)*) {
            fn call_block<R>(self, block: &mut Block<Self, R>) -> R {
                let invoke: unsafe extern fn(*mut Block<Self, R> $(, $t)*) -> R = unsafe {
                    let base = &*(block as *mut _ as *mut BlockBase<Self, R>);
                    mem::transmute(base.invoke)
                };
                let ($($a,)*) = self;
                unsafe {
                    invoke(block $(, $a)*)
                }
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
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K);
block_args_impl!(a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L);

#[repr(C)]
struct BlockBase<A, R> {
    isa: *const Class,
    flags: c_int,
    _reserved: c_int,
    invoke: unsafe extern fn(*mut Block<A, R>, ...) -> R,
}

/// An Objective-C block that takes arguments of `A` when called and
/// returns a value of `R`.
#[repr(C)]
pub struct Block<A, R> {
    _base: PhantomData<BlockBase<A, R>>,
}

// TODO: impl FnMut when it's possible
impl<A: BlockArguments, R> Block<A, R> where A: BlockArguments {
    /// Call self with the given arguments.
    pub fn call(&mut self, args: A) -> R {
        args.call_block(self)
    }
}

/// Types that may be converted into a `ConcreteBlock`.
pub trait IntoConcreteBlock<A> where A: BlockArguments {
    /// The return type of the resulting `ConcreteBlock`.
    type Ret;

    /// Consumes self to create a `ConcreteBlock`.
    fn into_concrete_block(self) -> ConcreteBlock<A, Self::Ret, Self>;
}

macro_rules! concrete_block_impl {
    ($f:ident) => (
        concrete_block_impl!($f,);
    );
    ($f:ident, $($a:ident : $t:ident),*) => (
        impl<$($t,)* R, X> IntoConcreteBlock<($($t,)*)> for X
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

                unsafe {
                    ConcreteBlock::with_invoke(
                        mem::transmute($f::<$($t,)* R, X>), self)
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
concrete_block_impl!(concrete_block_invoke_args6, a: A, b: B, c: C, d: D, e: E, f: F);
concrete_block_impl!(concrete_block_invoke_args7, a: A, b: B, c: C, d: D, e: E, f: F, g: G);
concrete_block_impl!(concrete_block_invoke_args8, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H);
concrete_block_impl!(concrete_block_invoke_args9, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I);
concrete_block_impl!(concrete_block_invoke_args10, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J);
concrete_block_impl!(concrete_block_invoke_args11, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K);
concrete_block_impl!(concrete_block_invoke_args12, a: A, b: B, c: C, d: D, e: E, f: F, g: G, h: H, i: I, j: J, k: K, l: L);

/// An Objective-C block whose size is known at compile time and may be
/// constructed on the stack.
#[repr(C)]
pub struct ConcreteBlock<A, R, F> {
    base: BlockBase<A, R>,
    descriptor: Box<BlockDescriptor<ConcreteBlock<A, R, F>>>,
    closure: F,
}

impl<A, R, F> ConcreteBlock<A, R, F>
        where A: BlockArguments, F: IntoConcreteBlock<A, Ret=R> {
    /// Constructs a `ConcreteBlock` with the given closure.
    /// When the block is called, it will return the value that results from
    /// calling the closure.
    pub fn new(closure: F) -> Self {
        closure.into_concrete_block()
    }
}

impl<A, R, F> ConcreteBlock<A, R, F> {
    /// Constructs a `ConcreteBlock` with the given invoke function and closure.
    /// Unsafe because the caller must ensure the invoke function takes the
    /// correct arguments.
    unsafe fn with_invoke(invoke: unsafe extern fn(*mut Self, ...) -> R,
            closure: F) -> Self {
        ConcreteBlock {
            base: BlockBase {
                isa: &_NSConcreteStackBlock,
                // 1 << 25 = BLOCK_HAS_COPY_DISPOSE
                flags: 1 << 25,
                _reserved: 0,
                invoke: mem::transmute(invoke),
            },
            descriptor: Box::new(BlockDescriptor::new()),
            closure: closure,
        }
    }
}

impl<A, R, F> ConcreteBlock<A, R, F> where F: 'static {
    /// Copy self onto the heap.
    pub fn copy(self) -> Id<Block<A, R>> {
        unsafe {
            // The copy method is declared as returning an object pointer.
            let block: *mut Object = msg_send![&*self, copy];
            let block = block as *mut Block<A, R>;
            // At this point, our copy helper has been run so the block will
            // be moved to the heap and we can forget the original block
            // because the heap block will drop in our dispose helper.
            mem::forget(self);
            Id::from_retained_ptr(block)
        }
    }
}

impl<A, R, F> Clone for ConcreteBlock<A, R, F> where F: Clone {
    fn clone(&self) -> Self {
        unsafe {
            ConcreteBlock::with_invoke(mem::transmute(self.base.invoke),
                self.closure.clone())
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
    use test_utils;
    use Id;
    use super::{Block, ConcreteBlock};

    #[test]
    fn test_call_block() {
        let mut block = test_utils::get_int_block_with(13);
        assert!(block.call(()) == 13);
    }

    #[test]
    fn test_call_block_args() {
        let mut block = test_utils::get_add_block_with(13);
        assert!(block.call((2,)) == 15);
    }

    #[test]
    fn test_create_block() {
        let mut block = ConcreteBlock::new(|| 13);
        let result = test_utils::invoke_int_block(&mut block);
        assert!(result == 13);
    }

    #[test]
    fn test_create_block_args() {
        let mut block = ConcreteBlock::new(|a: i32| a + 5);
        let result = test_utils::invoke_add_block(&mut block, 6);
        assert!(result == 11);
    }

    #[test]
    fn test_concrete_block_copy() {
        let s = "Hello!".to_string();
        let expected_len = s.len() as i32;
        let mut block = ConcreteBlock::new(move || s.len() as i32);
        assert!(test_utils::invoke_int_block(&mut block) == expected_len);

        let mut copied = block.copy();
        assert!(test_utils::invoke_int_block(&mut copied) == expected_len);
    }

    #[test]
    fn test_concrete_block_stack_copy() {
        fn make_block() -> Id<Block<(), i32>> {
            let x = 7;
            let block = ConcreteBlock::new(move || x);
            block.copy()
        }

        let mut block = make_block();
        assert!(test_utils::invoke_int_block(&mut block) == 7);
    }
}
