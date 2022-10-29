#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSShadow;
    unsafe impl ClassType for NSShadow {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSShadow {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method(shadowOffset)]
        pub unsafe fn shadowOffset(&self) -> NSSize;
        #[method(setShadowOffset:)]
        pub unsafe fn setShadowOffset(&self, shadowOffset: NSSize);
        #[method(shadowBlurRadius)]
        pub unsafe fn shadowBlurRadius(&self) -> CGFloat;
        #[method(setShadowBlurRadius:)]
        pub unsafe fn setShadowBlurRadius(&self, shadowBlurRadius: CGFloat);
        #[method_id(shadowColor)]
        pub unsafe fn shadowColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setShadowColor:)]
        pub unsafe fn setShadowColor(&self, shadowColor: Option<&NSColor>);
        #[method(set)]
        pub unsafe fn set(&self);
    }
);
