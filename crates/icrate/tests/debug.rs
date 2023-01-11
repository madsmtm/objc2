#![allow(unused_imports)]
#![cfg(feature = "Foundation")]
use icrate::Foundation;

#[test]
#[cfg(feature = "Foundation_NSProcessInfo")]
#[cfg(feature = "Foundation_NSString")]
fn test_process_info() {
    let info = Foundation::NSProcessInfo::processInfo();
    let expected = format!(
        "NSProcessInfo {{ processName: {:?}, .. }}",
        info.processName()
    );
    assert_eq!(format!("{info:?}"), expected);
}
