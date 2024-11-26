//! Test that implementing `NSApplicationDelegate` and similar requires
//! a `MainThreadOnly` class.
use objc2::rc::Retained;
use objc2::{
    define_class, extern_methods, extern_protocol, AllocAnyThread, MainThreadOnly, ProtocolType,
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

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = AllocAnyThread] // Not `MainThreadOnly`
    #[name = "CustomObject"]
    struct CustomObject;

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

fn main() {}
