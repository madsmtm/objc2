#![no_main] // Required, we build this with `-bundle`.

use objc2::{define_class, ClassType, MainThreadOnly};
use objc2_foundation::ns_string;
use objc2_xc_test::XCTestCase;
use objc2_xc_ui_automation::{
    XCUIApplication, XCUIElementTypeQueryProvider, XCUIScreenshotProviding,
};

define_class!(
    #[unsafe(super = XCTestCase)]
    #[thread_kind = MainThreadOnly]
    struct TestCase;

    impl TestCase {
        #[unsafe(method(testScreenshot))]
        fn test_screenshot(&self) {
            let app = XCUIApplication::new(self.mtm());
            app.launch();

            // Save screenshot.
            let screenshot = app.windows().element().screenshot();
            let res = screenshot
                .PNGRepresentation()
                .writeToFile_atomically(ns_string!("screenshot.png"), false);
            assert!(res, "failed writing screenshot");

            // TODO: For some reason, we have to terminate the application
            // manually when running outside Xcode?
            app.terminate();
        }
    }
);

/// Load and initialize the class such that XCTest can see it.
#[ctor::ctor]
unsafe fn setup() {
    let _ = TestCase::class();
}
