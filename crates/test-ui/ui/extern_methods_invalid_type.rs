use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};
use objc2_foundation::MainThreadMarker;

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

impl MyObject {
    extern_methods!(
        #[unsafe(method(takesBox:))]
        fn takes_box(&self, arg: Box<i32>);
    );

    extern_methods!(
        #[unsafe(method(returnsReferenceToRetained))]
        fn returns_reference_to_retained(&self) -> &Retained<Self>;
    );

    extern_methods!(
        #[unsafe(method(mainThreadMarkerAsReturn))]
        fn main_thread_marker_as_return() -> MainThreadMarker;
    );
}

fn main() {}
