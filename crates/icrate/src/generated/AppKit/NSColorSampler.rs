#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSColorSampler;
    unsafe impl ClassType for NSColorSampler {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSColorSampler {
        #[method(showSamplerWithSelectionHandler:)]
        pub unsafe fn showSamplerWithSelectionHandler(&self, selectionHandler: TodoBlock);
    }
);