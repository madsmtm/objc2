use objc2::rc::Retained;
use objc2::runtime::AnyObject;
use objc2::{define_class, msg_send, Ivars, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{NSResponder, NSViewController};
use objc2_foundation::{NSObject, NSObjectProtocol};

define_class!(
    // SAFETY:
    // - We correctly override `NSViewController` methods.
    // - `ViewController` does not implement `Drop`.
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    #[thread_kind = MainThreadOnly]
    pub struct ViewController;

    // SAFETY: No problematic methods on `NSObjectProtocol` are implemented.
    unsafe impl NSObjectProtocol for ViewController {}

    impl ViewController {
        // SAFETY: The signature is correct.
        #[unsafe(method(viewDidLoad))]
        fn view_did_load(&self) {
            // SAFETY: Messaging super is safe, and the signature is correct.
            let _: () = unsafe { msg_send![super(self), viewDidLoad] };

            // Do any additional setup after loading the view.
        }

        // SAFETY: The signature is correct.
        #[unsafe(method(setRepresentedObject:))]
        fn set_represented_object(&self, represented_object: Option<&AnyObject>) {
            // SAFETY: Messaging super is safe, and the signature is correct.
            let _: () = unsafe { msg_send![super(self), setRepresentedObject: represented_object] };

            // Update the view, if already loaded.
        }
    }
);

impl ViewController {
    // FIXME: Make it possible to avoid this boilerplate.
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm);
        let this = this.set_ivars(Ivars::<Self> {});
        // SAFETY: `ViewController` is safe to initialize.
        unsafe { msg_send![super(this), init] }
    }
}
