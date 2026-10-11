//! Implementing `NSApplicationDelegate` for a custom class.
#![cfg_attr(not(target_os = "macos"), allow(dead_code, unused))]
#![cfg_attr(feature = "unstable-darwin-objc", feature(darwin_objc))]
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, Ivars, MainThreadMarker, MainThreadOnly};
#[cfg(target_os = "macos")]
use objc2_app_kit::{NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate};
use objc2_foundation::{
    ns_string, NSCopying, NSNotification, NSObject, NSObjectProtocol, NSString,
};

#[cfg(target_os = "macos")]
define_class!(
    // SAFETY:
    // - The superclass NSObject does not have any subclassing requirements.
    // - `AppDelegate` does not implement `Drop`.
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug)]
    #[allow(unused)]
    struct AppDelegate {
        ivar: u8,
        another_ivar: bool,
        box_ivar: Box<i32>,
        maybe_box_ivar: Option<Box<i32>>,
        id_ivar: Retained<NSString>,
        maybe_retained_ivar: Option<Retained<NSString>>,
    }

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn did_finish_launching(&self, notification: &NSNotification) {
            println!("Did finish launching!");
            // Do something with the notification
            dbg!(notification);
            // Print some instance variables
            dbg!(self.ivar());
            dbg!(self.another_ivar());

            NSApplication::main(MainThreadMarker::from(self));
        }

        #[unsafe(method(applicationWillTerminate:))]
        fn will_terminate(&self, _notification: &NSNotification) {
            println!("Will terminate!");
        }
    }
);

#[cfg(target_os = "macos")]
impl AppDelegate {
    fn new(ivar: u8, another_ivar: bool, mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm);
        let this = this.set_ivars(Ivars::<Self> {
            ivar,
            another_ivar,
            box_ivar: Box::new(2),
            maybe_box_ivar: None,
            id_ivar: NSString::from_str("abc"),
            maybe_retained_ivar: Some(ns_string!("def").copy()),
        });
        unsafe { msg_send![super(this), init] }
    }
}

#[cfg(target_os = "macos")]
fn main() {
    let mtm: MainThreadMarker = MainThreadMarker::new().unwrap();

    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Regular);

    // configure the application delegate
    let delegate = AppDelegate::new(42, true, mtm);
    let object = ProtocolObject::from_ref(&*delegate);
    app.setDelegate(Some(object));

    // run the app
    app.run();
}

#[cfg(not(target_os = "macos"))]
fn main() {
    panic!("This example is currently only supported on macOS");
}
