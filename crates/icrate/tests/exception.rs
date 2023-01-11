#![cfg(feature = "Foundation_NSException")]
#![cfg(feature = "Foundation_NSString")]
#![cfg(feature = "Foundation_NSDictionary")]
use icrate::Foundation::{NSException, NSObject, NSString};

#[test]
fn create_and_query() {
    let exc = NSException::new(
        &NSString::from_str("abc"),
        Some(&NSString::from_str("def")),
        None,
    )
    .unwrap();

    assert_eq!(exc.name(), NSString::from_str("abc"));
    assert_eq!(exc.reason().unwrap(), NSString::from_str("def"));
    assert!(exc.userInfo().is_none());

    let debug = format!("<NSException: {exc:p}> 'abc' reason:def");
    assert_eq!(format!("{exc:?}"), debug);

    let description = if cfg!(feature = "gnustep-1-7") {
        format!("<NSException: {exc:p}> NAME:abc REASON:def")
    } else {
        "def".into()
    };

    let obj: &NSObject = &exc;
    assert_eq!(format!("{obj:?}"), description);

    let exc = NSException::into_exception(exc);

    // Test `Debug` impl of Exception
    assert_eq!(format!("{exc:?}"), format!("exception {debug}"));
    // Test `Display` impl of Exception
    assert_eq!(format!("{exc}"), "def");
}

#[test]
#[should_panic = "'abc' reason:def"]
fn unwrap() {
    let exc = NSException::new(
        &NSString::from_str("abc"),
        Some(&NSString::from_str("def")),
        None,
    )
    .unwrap();

    let _: () = Err(exc).unwrap();
}

// Further tests in `tests::exception`
