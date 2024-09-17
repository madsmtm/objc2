use alloc::string::ToString;
use core::cell::RefCell;
use std::ffi::CStr;
use std::thread_local;

use block2::{global_block, Block, ManualBlockEncoding, RcBlock, StackBlock};
use objc2::encode::{Encode, Encoding};
use objc2::rc::Retained;
use objc2::runtime::{AnyObject, Bool, NSObject};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
struct LargeStruct {
    x: f32,
    y: [u8; 100],
}

impl LargeStruct {
    fn get() -> Self {
        let mut y = [10; 100];
        y[42] = 123;
        Self { x: 5.0, y }
    }

    fn mutate(&mut self) {
        self.x -= 1.0;
        self.y[12] += 1;
        self.y[99] = 123;
    }
}

unsafe impl Encode for LargeStruct {
    const ENCODING: Encoding =
        Encoding::Struct("LargeStruct", &[f32::ENCODING, <[u8; 100]>::ENCODING]);
}

type Add12 = Block<dyn Fn(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32) -> i32>;

struct VoidToVoid;
unsafe impl ManualBlockEncoding for VoidToVoid {
    type Arguments = ();
    type Return = ();
    #[cfg(target_pointer_width = "64")]
    const ENCODING_CSTR: &'static CStr = c"v8@?0";
    #[cfg(target_pointer_width = "32")]
    const ENCODING_CSTR: &'static CStr = c"v4@?0";
}

struct VoidToInt;
unsafe impl ManualBlockEncoding for VoidToInt {
    type Arguments = ();
    type Return = i32;
    #[cfg(target_pointer_width = "64")]
    const ENCODING_CSTR: &'static CStr = c"i8@?0";
    #[cfg(target_pointer_width = "32")]
    const ENCODING_CSTR: &'static CStr = c"i4@?0";
}

struct IntToInt;
unsafe impl ManualBlockEncoding for IntToInt {
    type Arguments = (i32,);
    type Return = i32;
    #[cfg(target_pointer_width = "64")]
    const ENCODING_CSTR: &'static CStr = c"i12@?0i8";
    #[cfg(target_pointer_width = "32")]
    const ENCODING_CSTR: &'static CStr = c"i8@?0i4";
}

extern "C-unwind" {
    /// Returns a pointer to a global block that returns 7.
    fn get_int_block() -> *mut Block<dyn Fn() -> i32>;
    /// Returns a pointer to a copied block that returns `i`.
    fn get_int_block_with(i: i32) -> *mut Block<dyn Fn() -> i32>;
    /// Invokes a block and returns its result.
    fn invoke_int_block(block: &Block<dyn Fn() -> i32>) -> i32;

    /// Returns a pointer to a global block that returns its argument + 7.
    fn get_add_block() -> *mut Block<dyn Fn(i32) -> i32>;
    /// Returns a pointer to a copied block that returns its argument + `i`.
    fn get_add_block_with(i: i32) -> *mut Block<dyn Fn(i32) -> i32>;
    /// Invokes a block with `a` and returns the result.
    fn invoke_add_block(block: &Block<dyn Fn(i32) -> i32>, a: i32) -> i32;

    fn get_add_12() -> *mut Add12;
    fn get_add_12_with(x: i32) -> *mut Add12;
    fn invoke_add_12(
        block: &Add12,
        a1: i32,
        a2: i32,
        a3: i32,
        a4: i32,
        a5: i32,
        a6: i32,
        a7: i32,
        a8: i32,
        a9: i32,
        a10: i32,
        a11: i32,
        a12: i32,
    ) -> i32;

    fn get_large_struct_block() -> *mut Block<dyn Fn(LargeStruct) -> LargeStruct>;
    fn get_large_struct_block_with(
        i: LargeStruct,
    ) -> *mut Block<dyn Fn(LargeStruct) -> LargeStruct>;
    fn invoke_large_struct_block(
        block: &Block<dyn Fn(LargeStruct) -> LargeStruct>,
        s: LargeStruct,
    ) -> LargeStruct;

    fn try_block_debugging(x: i32);
}

#[test]
fn test_block_debugging() {
    unsafe { try_block_debugging(5) };
    // Uncomment to see debug output
    // panic!();
}

#[test]
fn test_int_block() {
    #[track_caller]
    fn invoke_assert(block: &Block<dyn Fn() -> i32>, expected: i32) {
        assert_eq!(block.call(()), expected);
        assert_eq!(unsafe { invoke_int_block(block) }, expected);
    }

    global_block! {
        static GLOBAL_BLOCK = || -> i32 {
            42
        };
    }

    invoke_assert(unsafe { &*get_int_block() }, 7);
    invoke_assert(
        &unsafe { RcBlock::from_raw(get_int_block_with(3)) }.unwrap(),
        3,
    );
    invoke_assert(&StackBlock::new(|| 10), 10);
    invoke_assert(&RcBlock::new(|| 6), 6);
    invoke_assert(
        unsafe { &StackBlock::with_encoding::<VoidToInt>(|| 10) },
        10,
    );
    invoke_assert(
        unsafe { &RcBlock::with_encoding::<_, _, _, VoidToInt>(|| 6) },
        6,
    );
    invoke_assert(&GLOBAL_BLOCK, 42);
}

#[test]
fn test_add_block() {
    #[track_caller]
    fn invoke_assert(block: &Block<dyn Fn(i32) -> i32>, expected: i32) {
        assert_eq!(block.call((5,)), expected);
        assert_eq!(unsafe { invoke_add_block(block, 5) }, expected);
    }

    global_block! {
        static GLOBAL_BLOCK = |x: i32| -> i32 {
            x + 42
        };
    }

    invoke_assert(unsafe { &*get_add_block() }, 12);
    invoke_assert(
        &unsafe { RcBlock::from_raw(get_add_block_with(3)) }.unwrap(),
        8,
    );
    invoke_assert(&StackBlock::new(|a: i32| a + 6), 11);
    invoke_assert(&RcBlock::new(|a: i32| a + 6), 11);
    invoke_assert(
        unsafe { &StackBlock::with_encoding::<IntToInt>(|a: i32| a + 6) },
        11,
    );
    invoke_assert(
        unsafe { &RcBlock::with_encoding::<_, _, _, IntToInt>(|a: i32| a + 6) },
        11,
    );
    invoke_assert(&GLOBAL_BLOCK, 47);
}

#[test]
fn test_add_12() {
    #[track_caller]
    fn invoke_assert(block: &Add12, expected: i32) {
        assert_eq!(
            block.call((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12)),
            expected
        );
        assert_eq!(
            unsafe { invoke_add_12(block, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12) },
            expected
        );
    }

    struct Enc;
    unsafe impl ManualBlockEncoding for Enc {
        type Arguments = (i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32);
        type Return = i32;
        #[cfg(target_pointer_width = "64")]
        const ENCODING_CSTR: &'static CStr = c"i56@?0i8i12i16i20i24i28i32i36i40i44i48i52";
        #[cfg(target_pointer_width = "32")]
        const ENCODING_CSTR: &'static CStr = c"i52@?0i4i8i12i16i20i24i28i32i36i40i44i48";
    }

    global_block! {
        static GLOBAL_BLOCK = |
            a1: i32, a2: i32, a3: i32, a4: i32,
            a5: i32, a6: i32, a7: i32, a8: i32,
            a9: i32, a10: i32, a11: i32, a12: i32,
        | -> i32 {
            a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9 + a10 + a11 + a12 + 42
        };
    }

    invoke_assert(unsafe { &*get_add_12() }, 78);
    invoke_assert(unsafe { &*get_add_12_with(13) }, 91);
    let closure = |a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12| {
        a1 + a2 + a3 + a4 + a5 + a6 + a7 + a8 + a9 + a10 + a11 + a12
    };
    invoke_assert(&StackBlock::new(closure), 78);
    invoke_assert(&RcBlock::new(closure), 78);
    invoke_assert(unsafe { &StackBlock::with_encoding::<Enc>(closure) }, 78);
    invoke_assert(
        unsafe { &RcBlock::with_encoding::<_, _, _, Enc>(closure) },
        78,
    );
    invoke_assert(&GLOBAL_BLOCK, 120);
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

    struct Enc;
    unsafe impl ManualBlockEncoding for Enc {
        type Arguments = (LargeStruct,);
        type Return = LargeStruct;
        #[cfg(target_pointer_width = "64")]
        const ENCODING_CSTR: &'static CStr = c"{LargeStruct=f[100C]}112@?0{LargeStruct=f[100C]}8";
        #[cfg(target_pointer_width = "32")]
        const ENCODING_CSTR: &'static CStr = c"{LargeStruct=f[100C]}108@?0{LargeStruct=f[100C]}4";
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

    let block = unsafe {
        StackBlock::with_encoding::<Enc>(|mut x: LargeStruct| {
            x.mutate();
            x
        })
    };
    assert_eq!(unsafe { invoke_large_struct_block(&block, data) }, new_data);
    let block = block.copy();
    assert_eq!(unsafe { invoke_large_struct_block(&block, data) }, new_data);
}

#[test]
fn test_block_copy() {
    let s = "Hello!".to_string();
    let expected_len = s.len() as i32;
    let closure = move || s.len() as i32;

    for block in [StackBlock::new(closure.clone()), unsafe {
        StackBlock::with_encoding::<VoidToInt>(closure)
    }] {
        assert_eq!(unsafe { invoke_int_block(&block) }, expected_len);
        let copied = block.copy();
        assert_eq!(unsafe { invoke_int_block(&copied) }, expected_len);
    }
}

#[test]
fn test_block_stack_move() {
    fn make_block() -> StackBlock<'static, (), i32, impl Fn() -> i32> {
        let x = 7;
        StackBlock::new(move || x)
    }
    fn make_block_with_encoding() -> StackBlock<'static, (), i32, impl Fn() -> i32> {
        let x = 7;
        unsafe { StackBlock::with_encoding::<VoidToInt>(move || x) }
    }

    for block in [
        &make_block() as &Block<dyn Fn() -> i32>,
        &make_block_with_encoding() as &Block<dyn Fn() -> i32>,
    ] {
        assert_eq!(unsafe { invoke_int_block(block) }, 7);
    }
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
    macro_rules! test_with {
        ($ctor:path) => {{
            let mut expected = Count::current();
            let counter = CloneDropTracker::new();
            expected.new += 1;
            expected.assert_current();

            #[allow(unused_unsafe)]
            let block = unsafe {
                $ctor(move || {
                    let _ = &counter;
                })
            };
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
        }};
    }

    test_with!(StackBlock::new);
    test_with!(StackBlock::with_encoding::<VoidToVoid>);
}

#[test]
fn rc_new_clone_drop() {
    macro_rules! test_with {
        ($ctor:path) => {{
            let mut expected = Count::current();
            let counter = CloneDropTracker::new();
            expected.new += 1;
            expected.assert_current();

            #[allow(unused_unsafe)]
            let block = unsafe {
                $ctor(move || {
                    let _ = &counter;
                })
            };
            expected.assert_current();

            let clone = block.clone();
            expected.assert_current();

            drop(clone);
            expected.assert_current();

            drop(block);
            expected.drop += 1;
            expected.assert_current();
        }};
    }

    test_with!(RcBlock::new);
    test_with!(RcBlock::with_encoding::<_, _, _, VoidToVoid>);
}

#[test]
fn stack_to_rc() {
    macro_rules! test_with {
        ($ctor:path) => {{
            let mut expected = Count::current();
            let counter = CloneDropTracker::new();
            expected.new += 1;
            expected.assert_current();

            #[allow(unused_unsafe)]
            let stack = unsafe {
                $ctor(move || {
                    let _ = &counter;
                })
            };
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
        }};
    }

    test_with!(StackBlock::new);
    test_with!(StackBlock::with_encoding::<VoidToVoid>);
}

#[test]
fn retain_release_rc_block() {
    macro_rules! test_with {
        ($ctor:path) => {{
            let mut expected = Count::current();
            let counter = CloneDropTracker::new();
            expected.new += 1;
            expected.assert_current();

            #[allow(unused_unsafe)]
            let block = unsafe {
                $ctor(move || {
                    let _ = &counter;
                })
            };
            expected.assert_current();

            let ptr = &*block as *const Block<_> as *mut AnyObject;
            let obj = unsafe { Retained::retain(ptr) }.unwrap();
            expected.assert_current();

            drop(block);
            expected.assert_current();

            drop(obj);
            expected.drop += 1;
            expected.assert_current();
        }};
    }

    test_with!(RcBlock::new);
    test_with!(RcBlock::with_encoding::<_, _, _, VoidToVoid>);
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
    // Don't use `Retained::retain`, as that has debug assertions against the kind
    // of things GNUStep is doing.
    let obj = if cfg!(feature = "gnustep-1-7") {
        let ptr = unsafe { objc2::ffi::objc_retain(ptr.cast()).cast() };
        unsafe { Retained::from_raw(ptr) }.unwrap()
    } else {
        unsafe { Retained::retain(ptr) }.unwrap()
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
    let obj1 = NSObject::new();
    let obj2 = NSObject::new();
    let closure = move || Bool::new(obj1 == obj2);

    struct Enc;
    unsafe impl ManualBlockEncoding for Enc {
        type Arguments = ();
        type Return = Bool;

        const ENCODING_CSTR: &'static CStr = {
            match (Bool::ENCODING, cfg!(target_pointer_width = "64")) {
                (Encoding::Char, true) => c"c8@?0",
                (Encoding::UChar, true) => c"C8@?0",
                (Encoding::Bool, true) => c"b8@?0",
                (Encoding::Char, false) => c"c4@?0",
                (Encoding::UChar, false) => c"C4@?0",
                (Encoding::Bool, false) => c"b4@?0",
                _ => panic!("invalid Bool encoding"),
            }
        };
    }

    for stack_block in [StackBlock::new(closure.clone()), unsafe {
        StackBlock::with_encoding::<Enc>(closure)
    }] {
        let rc_block = stack_block.copy();
        assert!(rc_block.call(()).is_false());
    }
}
