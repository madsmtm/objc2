use objc2::{
    msg_send, msg_send_id,
    rc::{Id, Shared},
    runtime::{Bool, Object},
};
use objc2_foundation::{declare_class, extern_class, NSObject};

#[cfg(all(feature = "apple", target_os = "macos"))]
#[link(name = "AppKit", kind = "framework")]
extern "C" {}

extern_class! {
    unsafe struct NSResponder: NSObject;
}

declare_class! {
    // For some reason, `NSApplicationDelegate` is not a "real" protocol we
    // can retrieve using `objc_getProtocol` - it seems it is created by
    // `clang` only when used in Objective-C...
    //
    // TODO: Investigate this!
    unsafe struct CustomAppDelegate: NSResponder, NSObject {
        pub ivar: u8,
        another_ivar: Bool,
    }

    unsafe impl {
        @sel(initWith:another:)
        fn init_with(
            self: &mut Self,
            ivar: u8,
            another_ivar: Bool,
        ) -> *mut Self {
            let this: *mut Self = unsafe {
                msg_send![super(self, NSResponder::class()), init]
            };
            if let Some(this) = unsafe { this.as_mut() } {
                // TODO: Allow initialization through MaybeUninit
                *this.ivar = ivar;
                *this.another_ivar = another_ivar;
            }
            this
        }

        #[allow(unused)] // TMP
        @sel(applicationDidFinishLaunching:)
        fn did_finish_launching(&self, _sender: *mut Object) {
            println!("Did finish launching!");
        }

        #[allow(unused)] // TMP
        @sel(applicationWillTerminate:)
        fn will_terminate(&self, _sender: *mut Object) {
            println!("Will terminate!");
        }

        #[allow(unused)] // TMP
        @sel(myClassMethod)
        fn my_class_method() {
            println!("A class method!");
        }
    }
}

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

fn main() {
    let _cls = CustomAppDelegate::create_class(); // TMP

    let delegate = CustomAppDelegate::new(42, true);

    println!("{}", delegate.ivar);
    println!("{}", delegate.another_ivar.as_bool());
}
