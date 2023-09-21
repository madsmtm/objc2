#![deny(unsafe_op_in_unsafe_fn)]
use std::ptr::NonNull;

use icrate::AppKit::{NSApplication, NSApplicationActivationPolicyRegular, NSApplicationDelegate};
use icrate::Foundation::{
    ns_string, MainThreadMarker, NSCopying, NSNotification, NSObject, NSObjectProtocol, NSString,
};
use objc2::declare::{Ivar, IvarBool, IvarDrop, IvarEncode};
use objc2::rc::Id;
use objc2::{declare_class, msg_send, msg_send_id, mutability, ClassType};

declare_class!(
    #[derive(Debug)]
    struct AppDelegate {
        ivar: IvarEncode<u8, "_ivar">,
        another_ivar: IvarBool<"_another_ivar">,
        box_ivar: IvarDrop<Box<i32>, "_box_ivar">,
        maybe_box_ivar: IvarDrop<Option<Box<i32>>, "_maybe_box_ivar">,
        id_ivar: IvarDrop<Id<NSString>, "_id_ivar">,
        maybe_id_ivar: IvarDrop<Option<Id<NSString>>, "_maybe_id_ivar">,
    }

    mod ivars;

    unsafe impl ClassType for AppDelegate {
        type Super = NSObject;
        type Mutability = mutability::MainThreadOnly;
        const NAME: &'static str = "MyAppDelegate";
    }

    unsafe impl AppDelegate {
        #[method(initWith:another:)]
        unsafe fn init_with(
            this: *mut Self,
            ivar: u8,
            another_ivar: bool,
        ) -> Option<NonNull<Self>> {
            let this: Option<&mut Self> = unsafe { msg_send![super(this), init] };

            this.map(|this| {
                Ivar::write(&mut this.ivar, ivar);
                Ivar::write(&mut this.another_ivar, another_ivar);
                Ivar::write(&mut this.maybe_box_ivar, None);
                Ivar::write(&mut this.maybe_id_ivar, Some(ns_string!("def").copy()));
                Ivar::write(&mut this.box_ivar, Box::new(2));
                Ivar::write(&mut this.id_ivar, NSString::from_str("abc"));
                NonNull::from(this)
            })
        }
    }

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[method(applicationDidFinishLaunching:)]
        fn did_finish_launching(&self, notification: &NSNotification) {
            println!("Did finish launching!");
            // Do something with the notification
            dbg!(notification);
        }

        #[method(applicationWillTerminate:)]
        fn will_terminate(&self, _notification: &NSNotification) {
            println!("Will terminate!");
        }
    }
);

impl AppDelegate {
    pub fn new(ivar: u8, another_ivar: bool, mtm: MainThreadMarker) -> Id<Self> {
        unsafe { msg_send_id![mtm.alloc(), initWith: ivar, another: another_ivar] }
    }
}

fn main() {
    let mtm: MainThreadMarker = MainThreadMarker::new().unwrap();

    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicyRegular);

    // initialize the delegate
    let delegate = AppDelegate::new(42, true, mtm);

    println!("{delegate:?}");

    // configure the application delegate
    app.setDelegate(Some(&delegate));

    // run the app
    unsafe { app.run() };
}
