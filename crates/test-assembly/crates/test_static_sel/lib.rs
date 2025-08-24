//! Test the output of the `sel!` macro.
#![cfg(target_vendor = "apple")]
use objc2::runtime::Sel;
use objc2::sel;

#[export_name = "fn1_get_sel"]
fn get_sel() -> Sel {
    sel!(simple)
}

#[export_name = "fn2_get_same_sel"]
fn get_same_sel() -> Sel {
    sel!(simple)
}

#[export_name = "fn3_get_common_twice"]
fn get_common_twice() -> (Sel, Sel) {
    (sel!(alloc), sel!(alloc))
}

#[export_name = "fn4_get_different_sel"]
fn get_different_sel() -> Sel {
    sel!(i: am: different:)
}

#[export_name = "fn5_unused_sel"]
fn unused_sel() {
    let _ = sel!(unused);
}

#[export_name = "fn6_use_fns"]
fn use_fns() -> [Sel; 4] {
    let s1 = get_sel();
    let s2 = get_same_sel();
    let s3 = get_different_sel();
    let s4 = sel!(fourthSel);
    [s1, s2, s3, s4]
}

#[export_name = "fn7_use_same_twice"]
fn use_same_twice() -> [Sel; 2] {
    // Should not need to load twice
    [get_sel(), get_sel()]
}

#[export_name = "fn8_use_in_loop"]
fn use_in_loop(n: usize) {
    for _i in 0..n {
        // Should be a noop
        let _ = sel!(loopedSelector);
    }
}
