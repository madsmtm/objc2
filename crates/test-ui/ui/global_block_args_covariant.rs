use core::cell::Cell;
use std::thread_local;

use block2::global_block;

fn main() {
    thread_local! {
        static CONTAINS_STATIC: Cell<&'static i32> = const { Cell::new(&1) };
    }

    global_block! {
        static PUT_STATIC_IN_THREAD_LOCAL = |val: &'static i32| {
            CONTAINS_STATIC.set(val);
        };
    }

    {
        let x = 5 + 2;
        PUT_STATIC_IN_THREAD_LOCAL.call((&x,));
    }

    // `CONTAINS_STATIC` now references `x`, which has gone out of scope.
    //
    // This would be an error!
    assert_eq!(*CONTAINS_STATIC.get(), 7);
}
