#![deny(unsafe_op_in_unsafe_fn)]
#![cfg_attr(not(target_os = "macos"), allow(unused))]
use std::ptr::NonNull;

use icrate::ns_string;
use icrate::Foundation::{NSCopying, NSObject, NSString};
use objc2::declare::{Ivar, IvarBool, IvarDrop, IvarEncode};
use objc2::rc::Id;
use objc2::runtime::AnyObject;
use objc2::{declare_class, msg_send, msg_send_id, mutability, ClassType};

#[cfg(target_os = "macos")]
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[cfg(target_os = "macos")]
declare_class!(
    #[derive(Debug)]
    struct CustomAppDelegate {
        pub ivar: IvarEncode<u8, "_ivar">,
        another_ivar: IvarBool<"_another_ivar">,
        box_ivar: IvarDrop<Box<i32>, "_box_ivar">,
        maybe_box_ivar: IvarDrop<Option<Box<i32>>, "_maybe_box_ivar">,
        id_ivar: IvarDrop<Id<NSString>, "_id_ivar">,
        maybe_id_ivar: IvarDrop<Option<Id<NSString>>, "_maybe_id_ivar">,
    }

    mod ivars;

    unsafe impl ClassType for CustomAppDelegate {
        #[inherits(NSObject)]
        type Super = icrate::AppKit::NSResponder;
        type Mutability = mutability::InteriorMutable;
        const NAME: &'static str = "MyCustomAppDelegate";
    }

    unsafe impl CustomAppDelegate {
        #[method(initWith:another:)]
        unsafe fn init_with(
            this: *mut Self,
            ivar: u8,
            another_ivar: bool,
        ) -> Option<NonNull<Self>> {
            let this: Option<&mut Self> = unsafe { msg_send![super(this), init] };

            // TODO: `ns_string` can't be used inside closures; investigate!
            let s = ns_string!("def");

            this.map(|this| {
                Ivar::write(&mut this.ivar, ivar);
                *this.another_ivar = another_ivar;
                *this.maybe_box_ivar = None;
                *this.maybe_id_ivar = Some(s.copy());
                Ivar::write(&mut this.box_ivar, Box::new(2));
                Ivar::write(&mut this.id_ivar, NSString::from_str("abc"));
                NonNull::from(this)
            })
        }

        #[method(myClassMethod)]
        fn my_class_method() {
            println!("A class method!");
        }
    }

    // For some reason, `NSApplicationDelegate` is not a "real" protocol we
    // can retrieve using `objc_getProtocol` - it seems it is created by
    // `clang` only when used in Objective-C...
    //
    // TODO: Investigate this!
    unsafe impl CustomAppDelegate {
        /// This is `unsafe` because it expects `sender` to be valid
        #[method(applicationDidFinishLaunching:)]
        unsafe fn did_finish_launching(&self, sender: *mut AnyObject) {
            println!("Did finish launching!");
            // Do something with `sender`
            dbg!(sender);
        }

        /// Some comment before `sel`.
        #[method(applicationWillTerminate:)]
        /// Some comment after `sel`.
        fn will_terminate(&self, _: *mut AnyObject) {
            println!("Will terminate!");
        }
    }
);

#[cfg(target_os = "macos")]
impl CustomAppDelegate {
    pub fn new(ivar: u8, another_ivar: bool) -> Id<Self> {
        unsafe { msg_send_id![Self::alloc(), initWith: ivar, another: another_ivar] }
    }
}

#[cfg(target_os = "macos")]
fn main() {
    let delegate = CustomAppDelegate::new(42, true);

    println!("{delegate:?}");
    println!("{:?}", delegate.ivar);
    println!("{:?}", delegate.another_ivar);
    println!("{:?}", delegate.box_ivar);
    println!("{:?}", delegate.maybe_box_ivar);
    println!("{:?}", delegate.id_ivar);
    println!("{:?}", delegate.maybe_id_ivar);
}

#[cfg(not(target_os = "macos"))]
fn main() {
    panic!("This example uses AppKit, which is only present on macOS");
}
