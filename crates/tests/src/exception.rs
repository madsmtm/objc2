use alloc::format;

use icrate::Foundation::{NSArray, NSException, NSString};
use objc2::exception::{catch, throw};
use objc2::msg_send;
use objc2::rc::{autoreleasepool, Id};
use objc2::runtime::Object;

#[track_caller]
fn assert_retain_count(obj: &Object, expected: usize) {
    let retain_count: usize = unsafe { msg_send![obj, retainCount] };
    assert_eq!(retain_count, expected);
}

#[test]
#[cfg_attr(
    feature = "catch-all",
    ignore = "Panics inside `catch` when catch-all is enabled"
)]
fn throw_catch_raise_catch() {
    let name = NSString::from_str("abc");
    let reason = NSString::from_str("def");

    let exc = NSException::new(&name, Some(&reason), None).unwrap();

    assert_retain_count(&exc, 1);

    let exc = autoreleasepool(|_| {
        let exc = NSException::into_exception(exc);
        let res = unsafe { catch(|| throw(exc)) };
        let exc = res.unwrap_err().unwrap();
        let exc = NSException::from_exception(exc).unwrap();

        assert_retain_count(&exc, 1);
        exc
    });

    assert_retain_count(&exc, 1);

    let exc = autoreleasepool(|_| {
        let inner = || {
            autoreleasepool(|pool| {
                let exc = Id::autorelease(exc, pool);
                unsafe { exc.raise() }
            })
        };

        let res = unsafe { catch(inner) };
        let exc = NSException::from_exception(res.unwrap_err().unwrap()).unwrap();

        // Undesired: The inner pool _should_ have been drained on unwind, but
        // it isn't, see `rc::Pool::drain`.
        assert_retain_count(&exc, 2);
        exc
    });

    assert_retain_count(&exc, 1);

    assert_eq!(exc.name(), name);
    assert_eq!(exc.reason().unwrap(), reason);
    assert!(exc.userInfo().is_none());
}

#[test]
#[cfg(feature = "catch-all")]
#[should_panic = "uncaught exception <NSException: 0x"]
fn raise_catch_all1() {
    let name = NSString::from_str("abc");
    let reason = NSString::from_str("def");

    let exc = NSException::new(&name, Some(&reason), None).unwrap();
    unsafe { exc.raise() };
}

#[test]
#[cfg(feature = "catch-all")]
#[should_panic = "> 'abc' reason:def"]
fn raise_catch_all2() {
    let name = NSString::from_str("abc");
    let reason = NSString::from_str("def");

    let exc = NSException::new(&name, Some(&reason), None).unwrap();
    unsafe { exc.raise() };
}

#[test]
#[cfg_attr(
    feature = "catch-all",
    ignore = "Panics inside `catch` when catch-all is enabled"
)]
fn raise_catch() {
    let name = NSString::from_str("abc");
    let reason = NSString::from_str("def");

    let exc = NSException::new(&name, Some(&reason), None).unwrap();
    assert_retain_count(&exc, 1);

    let exc = autoreleasepool(|pool| {
        let exc = Id::autorelease(exc, pool);
        let inner = || {
            if exc.name() == name {
                unsafe { exc.raise() };
            } else {
                42
            }
        };
        let res = unsafe { catch(inner) }.unwrap_err().unwrap();
        assert_retain_count(&exc, 2);
        res
    });

    assert_retain_count(&exc, 1);

    assert_eq!(format!("{exc}"), "def");
    assert_eq!(
        format!("{exc:?}"),
        format!("exception <NSException: {:p}> 'abc' reason:def", &*exc)
    );
}

#[test]
#[cfg_attr(
    feature = "catch-all",
    ignore = "Panics inside `catch` when catch-all is enabled"
)]
fn catch_actual() {
    let res = unsafe {
        catch(|| {
            let arr: Id<NSArray<Object>> = NSArray::new();
            let _obj: *mut Object = msg_send![&arr, objectAtIndex: 0usize];
        })
    };
    let exc = res.unwrap_err().unwrap();

    let name = "NSRangeException";
    let reason = if cfg!(feature = "gnustep-1-7") {
        "Index 0 is out of range 0 (in 'objectAtIndex:')"
    } else {
        "*** -[__NSArray0 objectAtIndex:]: index 0 beyond bounds for empty NSArray"
    };

    assert_eq!(format!("{}", exc), reason);
    assert_eq!(
        format!("{:?}", exc),
        format!(
            "exception <NSException: {:p}> '{}' reason:{}",
            &*exc, name, reason
        )
    );

    let exc = NSException::from_exception(exc).unwrap();
    assert_eq!(exc.name(), NSString::from_str(name));
    assert_eq!(exc.reason().unwrap(), NSString::from_str(reason));
    let user_info = exc.userInfo();
    if cfg!(feature = "gnustep-1-7") {
        let user_info = user_info.unwrap();
        assert_eq!(user_info.len(), 3);
        // TODO: Verify contents of userInfo
    } else {
        assert!(user_info.is_none());
    }
}
