use objc2::exception::{catch, throw};
use objc2::msg_send;
use objc2::rc::{autoreleasepool, Id};
use objc2::runtime::Object;
use objc2_foundation::{NSArray, NSException, NSString};

#[track_caller]
fn assert_retain_count(obj: &Object, expected: usize) {
    let retain_count: usize = unsafe { msg_send![obj, retainCount] };
    assert_eq!(retain_count, expected);
}

#[test]
#[cfg_attr(
    feature = "catch_all",
    ignore = "Panics inside `catch` when catch_all is enabled"
)]
fn throw_catch_raise_catch() {
    let name = NSString::from_str("abc");
    let reason = NSString::from_str("def");

    let exc = NSException::new(&name, Some(&reason), None).unwrap();
    assert_retain_count(&exc, 1);

    // TODO: Investigate this!
    let extra_retain = if cfg!(all(
        feature = "apple",
        target_os = "macos",
        target_arch = "x86"
    )) {
        1
    } else {
        0
    };

    let exc = autoreleasepool(|_| {
        let res = unsafe {
            catch(|| {
                let exc = exc; // Move into closure
                let exc = Id::cast::<Object>(exc);
                throw(Some(&exc))
                // exc is dropped on unwind here
            })
        };
        let exc = res.unwrap_err().unwrap();
        let exc = unsafe { Id::cast::<NSException>(exc) };

        assert_retain_count(&exc, 2 + extra_retain);
        exc
    });

    assert_retain_count(&exc, 1 + extra_retain);

    assert_eq!(exc.name(), name);
    assert_eq!(exc.reason().unwrap(), reason);
    assert!(exc.user_info().is_none());
}

#[test]
#[cfg(feature = "catch_all")]
// Intentionally truncated panic message
#[should_panic = "uncaught exception <NSException: 0x"]
fn raise_catch_all() {
    let name = NSString::from_str("abc");
    let reason = NSString::from_str("def");

    let exc = NSException::new(&name, Some(&reason), None).unwrap();
    unsafe { exc.raise() };
}

#[test]
#[cfg_attr(
    feature = "catch_all",
    ignore = "Panics inside `catch` when catch_all is enabled"
)]
// Intentionally truncated panic message
#[should_panic = "called `Result::unwrap()` on an `Err` value: Some(<NSException: 0x"]
fn raise_catch_unwrap() {
    let name = NSString::from_str("abc");
    let reason = NSString::from_str("def");

    let exc = NSException::new(&name, Some(&reason), None).unwrap();
    assert_retain_count(&exc, 1);

    let res = autoreleasepool(|_| {
        let res = unsafe {
            catch(|| {
                if exc.name() == name {
                    exc.raise();
                } else {
                    4
                }
            })
        };
        assert_retain_count(&exc, 3);
        res
    });

    assert_retain_count(&exc, 2);

    let _ = res.unwrap();
}

#[test]
#[cfg_attr(
    feature = "catch_all",
    ignore = "Panics inside `catch` when catch_all is enabled"
)]
fn catch_actual() {
    let res = unsafe {
        catch(|| {
            let arr: Id<NSArray<Object, _>, _> = NSArray::new();
            let _obj: *mut Object = msg_send![&arr, objectAtIndex: 0usize];
        })
    };
    let exc = res.unwrap_err().unwrap();
    let exc = unsafe { Id::cast::<NSException>(exc) };

    assert_eq!(exc.name(), NSString::from_str("NSRangeException"));
    let reason = "*** -[__NSArray0 objectAtIndex:]: index 0 beyond bounds for empty NSArray";
    assert_eq!(exc.reason().unwrap(), NSString::from_str(reason));
    assert!(exc.user_info().is_none());
}
