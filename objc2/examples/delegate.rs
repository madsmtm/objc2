#![cfg_attr(not(all(feature = "apple", target_os = "macos")), allow(unused))]
use objc2::foundation::NSObject;
use objc2::rc::{Id, Shared};
use objc2::runtime::{Bool, Object};
use objc2::{declare_class, extern_class, msg_send, msg_send_id, ClassType};

#[cfg(all(feature = "apple", target_os = "macos"))]
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

#[cfg(all(feature = "apple", target_os = "macos"))]
extern_class!(
    struct NSResponder;

    unsafe impl ClassType for NSResponder {
        type Superclass = NSObject;
    }
);

#[cfg(all(feature = "apple", target_os = "macos"))]
declare_class!(
    struct CustomAppDelegate {
        pub ivar: u8,
        another_ivar: Bool,
    }

    unsafe impl ClassType for CustomAppDelegate {
        #[inherits(NSObject)]
        type Superclass = NSResponder;
    }

    unsafe impl CustomAppDelegate {
        #[sel(initWith:another:)]
        fn init_with(self: &mut Self, ivar: u8, another_ivar: Bool) -> *mut Self {
            let this: *mut Self = unsafe { msg_send![super(self, NSResponder::class()), init] };
            if let Some(this) = unsafe { this.as_mut() } {
                // TODO: Allow initialization through MaybeUninit
                *this.ivar = ivar;
                *this.another_ivar = another_ivar;
            }
            this
        }

        #[sel(myClassMethod)]
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
        #[sel(applicationDidFinishLaunching:)]
        unsafe fn did_finish_launching(&self, sender: *mut Object) {
            println!("Did finish launching!");
            // Do something with `sender`
            dbg!(sender);
        }

        /// Some comment before `sel`.
        #[sel(applicationWillTerminate:)]
        /// Some comment after `sel`.
        fn will_terminate(&self, _: *mut Object) {
            println!("Will terminate!");
        }
    }
);

#[cfg(all(feature = "apple", target_os = "macos"))]
impl CustomAppDelegate {
    pub fn new(ivar: u8, another_ivar: bool) -> Id<Self, Shared> {
        let cls = Self::class();
        unsafe {
            msg_send_id![
                msg_send_id![cls, alloc],
                initWith: ivar,
                another: Bool::from(another_ivar),
            ]
            .unwrap()
        }
    }
}

#[cfg(all(feature = "apple", target_os = "macos"))]
fn main() {
    let delegate = CustomAppDelegate::new(42, true);

    println!("{}", delegate.ivar);
    println!("{}", delegate.another_ivar.as_bool());
}

#[cfg(not(all(feature = "apple", target_os = "macos")))]
fn main() {
    panic!("This example uses AppKit, which is only present on macOS");
}
