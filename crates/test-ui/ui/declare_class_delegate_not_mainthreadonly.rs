//! Test that implementing `NSApplicationDelegate` and similar requires
//! a `MainThreadOnly` class.
use objc2::rc::Retained;
use objc2::{
    declare_class, extern_methods, extern_protocol, AllocAnyThread, ClassType, DeclaredClass,
    MainThreadOnly, ProtocolType,
};
use objc2_foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol};

// Use fake `NSApplicationDelegate` so that this works on iOS too.
extern_protocol!(
    pub unsafe trait NSApplicationDelegate: NSObjectProtocol + MainThreadOnly {
        #[optional]
        #[method(applicationDidFinishLaunching:)]
        unsafe fn applicationDidFinishLaunching(&self, notification: &NSNotification);

        // snip
    }

    unsafe impl ProtocolType for dyn NSApplicationDelegate {}
);

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        type ThreadKind = dyn AllocAnyThread; // Not `MainThreadOnly`
        const NAME: &'static str = "CustomObject";
    }

    impl DeclaredClass for CustomObject {}

    unsafe impl NSObjectProtocol for CustomObject {}

    unsafe impl NSApplicationDelegate for CustomObject {
        #[method(applicationDidFinishLaunching:)]
        unsafe fn application_did_finish_launching(&self, _notification: &NSNotification) {
            // Unclear for the user how to get a main thread marker if `self` is not `MainThreadOnly`
            let _mtm = MainThreadMarker::new().unwrap();
        }
    }
);

extern_methods!(
    unsafe impl CustomObject {
        #[method_id(new)]
        fn new(mtm: MainThreadMarker) -> Retained<Self>;
    }
);

fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    let app = NSApplication::sharedApplication(mtm);

    let delegate = CustomObject::new(mtm);
    app.setDelegate(Some(&delegate));
}
