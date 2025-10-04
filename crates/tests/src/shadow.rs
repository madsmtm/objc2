#![cfg(all(target_os = "macos", feature = "exception"))]
use core::panic::AssertUnwindSafe;

use alloc::string::ToString;
use objc2::exception::catch;
use objc2::rc::Retained;
use objc2_app_kit::{NSColor, NSShadow};
use objc2_foundation::NSString;

/// Test what happens when setting a non-color.
///
/// We conservatively don't mark `-[UIColor setShadowColor:]` as safe because
/// of this, see also <https://github.com/madsmtm/objc2/issues/562>.
#[test]
#[cfg_attr(
    feature = "catch-all",
    ignore = "messes with our use of `exception::catch`"
)]
fn test_invalid_color() {
    let shadow = NSShadow::new();
    // This cast is unsafe in AppKit, but `setShadowColor` in UIKit takes
    // AnyObject, so there it would be safe.
    let color = unsafe { Retained::cast_unchecked::<NSColor>(NSString::new()) };
    shadow.setShadowColor(Some(&color));
    let shadow = AssertUnwindSafe(shadow);

    // AppKit ends up calling the `CGColor` selector.
    let err = catch(|| shadow.set()).unwrap_err().unwrap().to_string();

    assert!(
        err.contains("CGColor") || err.contains("colorUsingColorSpaceName:"),
        "{err:?} did not have expected message"
    );
    assert!(
        err.contains("unrecognized selector sent to instance"),
        "{err:?} did not have expected message"
    );
}
