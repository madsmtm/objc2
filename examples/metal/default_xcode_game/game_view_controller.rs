use std::cell::OnceCell;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, Ivars, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{NSResponder, NSViewController};
use objc2_core_foundation::{CGPoint, CGRect, CGSize};
use objc2_foundation::NSObject;
use objc2_metal::MTLCreateSystemDefaultDevice;
use objc2_metal_kit::{MTKView, MTKViewDelegate};

use crate::renderer::Renderer;

define_class!(
    // SAFETY:
    // - We correctly override `NSViewController` methods.
    // - `GameViewController` does not implement `Drop`.
    #[unsafe(super(NSViewController, NSResponder, NSObject))]
    pub struct GameViewController{
        renderer: OnceCell<Retained<Renderer>>,
    }

    impl GameViewController {
        // SAFETY: The signature is correct.
        #[unsafe(method(viewDidLoad))]
        fn view_did_load(&self) {
            let _: () = unsafe { msg_send![super(self), viewDidLoad] };
            let mtm = self.mtm();

            // Create a new MTKView and configure Renderer to render into it.

            let device =
                MTLCreateSystemDefaultDevice().expect("Metal is not supported on this device");

            let view = MTKView::initWithFrame_device(
                MTKView::alloc(mtm),
                CGRect::new(CGPoint::ZERO, CGSize::new(800.0, 600.0)),
                Some(&device),
            );
            self.setView(&view);

            let renderer = Renderer::new(&view);
            renderer.mtkView_drawableSizeWillChange(&view, view.drawableSize());
            view.setDelegate(Some(ProtocolObject::from_ref(&*renderer)));
            let _ = self.renderer().set(renderer);
        }
    }
);

impl GameViewController {
    // FIXME: Make it possible to avoid this boilerplate.
    pub fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = Self::alloc(mtm);
        let this = this.set_ivars(Ivars::<Self> {
            renderer: Default::default(),
        });
        // SAFETY: `GameViewController` is safe to initialize.
        unsafe { msg_send![super(this), init] }
    }
}
