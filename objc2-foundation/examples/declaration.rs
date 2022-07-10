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
    // TODO: Protocol NSApplicationDelegate
    unsafe struct CustomAppDelegate: NSResponder, NSObject {
        pub ivar: u8,
        another_ivar: Bool,
    }

    unsafe impl {
        // #[selector(initWith:another:)]
        fn init_with(
            this: &mut Self,
            ivar: u8,
            another_ivar: Bool,
        ) -> *mut Self {
            let this: *mut Self = unsafe {
                msg_send![super(this, NSResponder::class()), init]
            };
            if let Some(this) = unsafe { this.as_mut() } {
                // TODO: Allow initialization through MaybeUninit
                *this.ivar = ivar;
                *this.another_ivar = another_ivar;
            }
            this
        }

        // #[selector(applicationDidFinishLaunching:)]
        #[allow(unused)] // TMP
        fn did_finish_launching(&self, _sender: *mut Object) {
            println!("Did finish launching!");
        }

        // #[selector(applicationWillTerminate:)]
        #[allow(unused)] // TMP
        fn will_terminate(&self, _sender: *mut Object) {
            println!("Will terminate!");
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
