//! Test that implementing `NSApplicationDelegate` and similar requires
//! a `MainThreadOnly` class.
use icrate::AppKit::{NSApplication, NSApplicationDelegate};
use icrate::Foundation::{MainThreadMarker, NSNotification, NSObject, NSObjectProtocol};
use objc2::rc::Id;
use objc2::runtime::ProtocolObject;
use objc2::{declare_class, extern_methods, mutability, ClassType};

declare_class!(
    struct CustomObject;

    unsafe impl ClassType for CustomObject {
        type Super = NSObject;
        type Mutability = mutability::InteriorMutable; // Not `MainThreadOnly`
        const NAME: &'static str = "CustomObject";
    }

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
        fn new(mtm: MainThreadMarker) -> Id<Self>;
    }
);

fn main() {
    let mtm = MainThreadMarker::new().unwrap();
    let app = NSApplication::sharedApplication(mtm);

    let delegate = CustomObject::new(mtm);
    app.setDelegate(Some(&delegate));
}
