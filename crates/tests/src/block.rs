use core::cell::RefCell;
use std::thread_local;

use alloc::string::ToString;
use block2::{global_block, Block, RcBlock, StackBlock};
use objc2::encode::{Encode, Encoding};
use objc2::rc::Id;
use objc2::runtime::{AnyObject, Bool, NSObject};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LargeStruct {
    pub x: f32,
    pub y: [u8; 100],
}

impl LargeStruct {
    pub fn get() -> Self {
        let mut y = [10; 100];
        y[42] = 123;
        Self { x: 5.0, y }
    }

    pub fn mutate(&mut self) {
        self.x -= 1.0;
        self.y[12] += 1;
        self.y[99] = 123;
    }
}

unsafe impl Encode for LargeStruct {
    const ENCODING: Encoding =
        Encoding::Struct("LargeStruct", &[f32::ENCODING, <[u8; 100]>::ENCODING]);
}

extern "C" {
    /// Returns a pointer to a global block that returns 7.
    pub fn get_int_block() -> *mut Block<dyn Fn() -> i32>;
    /// Returns a pointer to a copied block that returns `i`.
    pub fn get_int_block_with(i: i32) -> *mut Block<dyn Fn() -> i32>;
    /// Invokes a block and returns its result.
    pub fn invoke_int_block(block: &Block<dyn Fn() -> i32>) -> i32;

    /// Returns a pointer to a global block that returns its argument + 7.
    pub fn get_add_block() -> *mut Block<dyn Fn(i32) -> i32>;
    /// Returns a pointer to a copied block that returns its argument + `i`.
    pub fn get_add_block_with(i: i32) -> *mut Block<dyn Fn(i32) -> i32>;
    /// Invokes a block with `a` and returns the result.
    pub fn invoke_add_block(block: &Block<dyn Fn(i32) -> i32>, a: i32) -> i32;

    pub fn get_large_struct_block() -> *mut Block<dyn Fn(LargeStruct) -> LargeStruct>;
    pub fn get_large_struct_block_with(
        i: LargeStruct,
    ) -> *mut Block<dyn Fn(LargeStruct) -> LargeStruct>;
    pub fn invoke_large_struct_block(
        block: &Block<dyn Fn(LargeStruct) -> LargeStruct>,
        s: LargeStruct,
    ) -> LargeStruct;

    pub fn try_block_debugging(x: i32);
}

#[test]
fn test_block_debugging() {
    unsafe { try_block_debugging(5) };
    // Uncomment to see debug output
    // panic!();
}

#[test]
fn test_int_block() {
    unsafe {
        assert_eq!(invoke_int_block(&*get_int_block()), 7);
        assert_eq!(invoke_int_block(&*get_int_block_with(13)), 13);
    }
}

#[test]
fn test_add_block() {
    unsafe {
        assert_eq!(invoke_add_block(&*get_add_block(), 5), 12);
        assert_eq!(invoke_add_block(&*get_add_block_with(3), 5), 8);
    }
}

#[test]
fn test_large_struct_block() {
    let data = LargeStruct::get();
    let mut expected = data;
    expected.mutate();

    assert_eq!(
        unsafe { invoke_large_struct_block(&*get_large_struct_block(), data) },
        expected
    );
    assert_eq!(
        unsafe { invoke_large_struct_block(&*get_large_struct_block_with(expected), data) },
        expected
    );

    global_block! {
        static BLOCK = |data: LargeStruct| -> LargeStruct {
            let mut data = data;
            data.mutate();
            data
        };
    }

    let data = LargeStruct::get();
    let mut new_data = data;
    new_data.mutate();

    assert_eq!(BLOCK.call((data,)), new_data);
    assert_eq!(unsafe { invoke_large_struct_block(&BLOCK, data) }, new_data);

    let block = StackBlock::new(|mut x: LargeStruct| {
        x.mutate();
        x
    });
    assert_eq!(unsafe { invoke_large_struct_block(&block, data) }, new_data);
    let block = block.copy();
    assert_eq!(unsafe { invoke_large_struct_block(&block, data) }, new_data);
}

global_block! {
    /// Test `global_block` in an external crate
    static MY_BLOCK = || -> i32 {
        42
    };
}

#[test]
fn test_global_block() {
    assert_eq!(unsafe { invoke_int_block(&MY_BLOCK) }, 42);
}

#[test]
fn test_call_block() {
    let block = unsafe { RcBlock::from_raw(get_int_block_with(13)) }.unwrap();
    assert_eq!(block.call(()), 13);
}

#[test]
fn test_call_block_args() {
    let block = unsafe { RcBlock::from_raw(get_add_block_with(13)) }.unwrap();
    assert_eq!(block.call((2,)), 15);
}

#[test]
fn test_create_block() {
    let block = StackBlock::new(|| 13);
    let result = unsafe { invoke_int_block(&block) };
    assert_eq!(result, 13);
}

#[test]
fn test_create_block_args() {
    let block = StackBlock::new(|a: i32| a + 5);
    let result = unsafe { invoke_add_block(&block, 6) };
    assert_eq!(result, 11);
}

#[test]
fn test_block_copy() {
    let s = "Hello!".to_string();
    let expected_len = s.len() as i32;
    let block = StackBlock::new(move || s.len() as i32);
    assert_eq!(unsafe { invoke_int_block(&block) }, expected_len);

    let copied = block.copy();
    assert_eq!(unsafe { invoke_int_block(&copied) }, expected_len);
}

#[test]
fn test_block_stack_move() {
    fn make_block() -> StackBlock<'static, (), i32, impl Fn() -> i32> {
        let x = 7;
        StackBlock::new(move || x)
    }

    let block = make_block();
    assert_eq!(unsafe { invoke_int_block(&block) }, 7);
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
struct Count {
    new: usize,
    clone: usize,
    drop: usize,
}

thread_local! {
    static COUNT: RefCell<Count> = RefCell::default();
}

impl Count {
    fn current() -> Self {
        COUNT.with_borrow(|data| data.clone())
    }

    #[track_caller]
    fn assert_current(&self) {
        let current = Self::current();
        if current != *self {
            panic!(
                "got differing amounts of calls:
       current: `{current:?}`,
      expected: `{self:?}`"
            )
        }
    }
}

struct CloneDropTracker(());

impl CloneDropTracker {
    fn new() -> Self {
        COUNT.with_borrow_mut(|count| {
            count.new += 1;
        });
        Self(())
    }
}

impl Clone for CloneDropTracker {
    fn clone(&self) -> Self {
        COUNT.with_borrow_mut(|count| {
            count.clone += 1;
        });
        Self(())
    }
}

impl Drop for CloneDropTracker {
    fn drop(&mut self) {
        COUNT.with_borrow_mut(|count| {
            count.drop += 1;
        });
    }
}

#[test]
fn stack_new_clone_drop() {
    let mut expected = Count::current();

    let counter = CloneDropTracker::new();
    expected.new += 1;
    expected.assert_current();

    let block = StackBlock::new(move || {
        let _ = &counter;
    });
    expected.assert_current();

    let clone = block.clone();
    expected.clone += 1;
    expected.assert_current();

    drop(clone);
    expected.drop += 1;
    expected.assert_current();

    drop(block);
    expected.drop += 1;
    expected.assert_current();
}

#[test]
fn rc_new_clone_drop() {
    let mut expected = Count::current();

    let counter = CloneDropTracker::new();
    expected.new += 1;
    expected.assert_current();

    let block = RcBlock::new(move || {
        let _ = &counter;
    });
    expected.assert_current();

    let clone = block.clone();
    expected.assert_current();

    drop(clone);
    expected.assert_current();

    drop(block);
    expected.drop += 1;
    expected.assert_current();
}

#[test]
fn stack_to_rc() {
    let mut expected = Count::current();

    let counter = CloneDropTracker::new();
    expected.new += 1;
    expected.assert_current();

    let stack = StackBlock::new(move || {
        let _ = &counter;
    });
    expected.assert_current();

    let rc1 = stack.copy();
    expected.clone += 1;
    expected.assert_current();

    let rc2 = stack.copy();
    expected.clone += 1;
    expected.assert_current();

    let clone2 = rc2.clone();
    expected.assert_current();

    drop(rc2);
    expected.assert_current();

    drop(stack);
    expected.drop += 1;
    expected.assert_current();

    drop(rc1);
    expected.drop += 1;
    expected.assert_current();

    drop(clone2);
    expected.drop += 1;
    expected.assert_current();
}

#[test]
fn retain_release_rc_block() {
    let mut expected = Count::current();

    let counter = CloneDropTracker::new();
    expected.new += 1;
    expected.assert_current();

    let block = RcBlock::new(move || {
        let _ = &counter;
    });
    expected.assert_current();

    let ptr = &*block as *const Block<_> as *mut AnyObject;
    let obj = unsafe { Id::retain(ptr) }.unwrap();
    expected.assert_current();

    drop(block);
    expected.assert_current();

    drop(obj);
    expected.drop += 1;
    expected.assert_current();
}

/// Retaining/releasing stack blocks is kinda weird and unsupported.
///
/// As an example, the reference count is not increased for stack blocks on
/// Apple's runtime, while on GNUStep, the `-retain` returns the new block,
/// which is generally very unexpected behaviour.
#[test]
fn retain_release_stack_block() {
    let mut expected = Count::current();

    let counter = CloneDropTracker::new();
    expected.new += 1;
    expected.assert_current();

    let block = StackBlock::new(move || {
        let _ = &counter;
    });
    expected.assert_current();

    let ptr = &*block as *const Block<_> as *mut AnyObject;
    // Don't use `Id::retain`, as that has debug assertions against the kind
    // of things GNUStep is doing.
    let obj = if cfg!(feature = "gnustep-1-7") {
        let ptr = unsafe { objc2::ffi::objc_retain(ptr.cast()).cast() };
        unsafe { Id::from_raw(ptr) }.unwrap()
    } else {
        unsafe { Id::retain(ptr) }.unwrap()
    };
    if cfg!(feature = "gnustep-1-7") {
        expected.clone += 1;
    }
    expected.assert_current();

    drop(obj);
    if cfg!(feature = "gnustep-1-7") {
        expected.drop += 1;
    }
    expected.assert_current();

    drop(block);
    expected.drop += 1;
    expected.assert_current();
}

#[test]
fn capture_id() {
    let stack_block = {
        let obj1 = NSObject::new();
        let obj2 = NSObject::new();
        StackBlock::new(move || Bool::new(obj1 == obj2))
    };

    let rc_block = stack_block.copy();

    assert!(rc_block.call(()).is_false());
}
