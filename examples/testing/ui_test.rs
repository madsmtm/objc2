#![no_main] // Required, we build this with `-bundle`.

use objc2::{define_class, ClassType, MainThreadOnly};
use objc2_xc_test::XCTestCase;
use objc2_xc_ui_automation::XCUIApplication;

define_class!(
    #[unsafe(super = XCTestCase)]
    #[thread_kind = MainThreadOnly]
    struct TestCase;

    impl TestCase {
        #[unsafe(method(setUp))]
        fn set_up(&self) {
            // Test setup code in here.
        }

        #[unsafe(method(tearDown))]
        fn tear_down(&self) {
            // Test teardown code in here.
        }

        #[unsafe(method(testScreenshot))]
        fn test_screenshot(&self) {
            let app = unsafe { XCUIApplication::new(self.mtm()) };
            unsafe { app.launch() };

            // Run your test code in here.

            // TODO: For some reason, we have to terminate the application
            // manually when running outside Xcode?
            unsafe { app.terminate() };
        }
    }
);

/// Load and initialize the class such that XCTest can see it.
#[ctor::ctor]
unsafe fn setup() {
    let _ = TestCase::class();
}
