use objc2::rc::{Allocated, Retained};
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_methods};

extern_class!(
    #[unsafe(super(NSObject))]
    pub struct MyObject;
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(initWithMut)]
        fn init_with_mut(&mut self) -> Option<Retained<Self>>;
    }
);

extern_methods!(
    unsafe impl MyObject {
        #[method_id(initWithOptionAllocated)]
        fn init_with_option_allocated(this: Option<Allocated<Self>>) -> Option<Retained<Self>>;
    }
);

fn main() {}
