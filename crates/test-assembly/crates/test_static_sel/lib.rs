//! Test the output of the `sel!` macro.
#![cfg(target_vendor = "apple")]
use objc2::runtime::Sel;
use objc2::sel;

#[no_mangle]
fn get_sel() -> Sel {
    sel!(simple)
}

#[no_mangle]
fn get_same_sel() -> Sel {
    sel!(simple)
}

#[no_mangle]
fn get_common_twice() -> (Sel, Sel) {
    (sel!(alloc), sel!(alloc))
}

#[no_mangle]
fn get_different_sel() -> Sel {
    sel!(i: am: different:)
}

#[no_mangle]
fn unused_sel() {
    let _ = sel!(unused);
}

#[no_mangle]
fn use_fns() -> [Sel; 4] {
    let s1 = get_sel();
    let s2 = get_same_sel();
    let s3 = get_different_sel();
    let s4 = sel!(fourthSel);
    [s1, s2, s3, s4]
}

#[no_mangle]
fn use_same_twice() -> [Sel; 2] {
    // Should not need to load twice
    [get_sel(), get_sel()]
}

#[no_mangle]
fn use_in_loop(n: usize) {
    for _i in 0..n {
        // Should be a noop
        let _ = sel!(loopedSelector);
    }
}
