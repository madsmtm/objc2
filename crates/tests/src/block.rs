use core::cell::RefCell;
use std::thread_local;

use block2::{Block, RcBlock, StackBlock};
use objc2::{rc::Id, runtime::AnyObject};

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

    let rc1 = stack.to_rc();
    expected.clone += 1;
    expected.assert_current();

    let rc2 = stack.to_rc();
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

    let ptr = &*block as *const Block<_, _> as *mut AnyObject;
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

    let ptr = &*block as *const Block<_, _> as *mut AnyObject;
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
