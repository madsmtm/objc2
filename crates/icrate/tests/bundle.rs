#![cfg(feature = "Foundation_NSBundle")]
use icrate::Foundation::NSBundle;

#[test]
#[cfg(feature = "Foundation_NSString")]
#[cfg(feature = "Foundation_NSDictionary")]
#[cfg_attr(not(target_os = "macos"), ignore = "varies between platforms")]
fn try_running_functions() {
    // This is mostly empty since cargo doesn't bundle the application
    // before executing.
    let bundle = NSBundle::mainBundle();
    println!("{bundle:?}");
    assert_eq!(format!("{:?}", bundle.infoDictionary().unwrap()), "{}");
    assert_eq!(bundle.name(), None);
}
