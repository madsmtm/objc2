//! Test the output of the `class!` macro.
use objc2::class;
use objc2::runtime::AnyClass;

#[export_name = "fn1_get_class"]
fn get_class() -> &'static AnyClass {
    class!(NSObject)
}

#[export_name = "fn1_get_same_class"]
fn get_same_class() -> &'static AnyClass {
    class!(NSObject)
}

#[export_name = "fn3_get_different_class"]
fn get_different_class() -> &'static AnyClass {
    class!(NSString)
}

#[export_name = "fn4_unused_class"]
fn unused_class() {
    let _ = class!(NSData);
}

#[export_name = "fn5_use_fns"]
fn use_fns() -> [&'static AnyClass; 4] {
    let s1 = get_class();
    let s2 = get_same_class();
    let s3 = get_different_class();
    let s4 = class!(NSException);
    [s1, s2, s3, s4]
}

#[export_name = "fn6_use_same_twice"]
fn use_same_twice() -> [&'static AnyClass; 2] {
    // Should not need to load twice
    [get_class(), get_class()]
}

#[export_name = "fn7_use_in_loop"]
fn use_in_loop(n: usize) {
    for _i in 0..n {
        // Should be a noop
        let _ = class!(NSLock);
    }
}
