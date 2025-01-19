use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};
use objc2_foundation::MainThreadMarker;

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

extern_methods!(
    unsafe impl MyObject {
        #[method(takesBox:)]
        fn takes_box(&self, arg: Box<i32>);
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(returnsReferenceToRetained)]
        fn returns_reference_to_retained(&self) -> &Retained<Self>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method(mainThreadMarkerAsReturn)]
        fn main_thread_marker_as_return() -> MainThreadMarker;
    }
);

fn main() {}
