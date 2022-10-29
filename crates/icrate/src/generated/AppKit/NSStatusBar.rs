#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSStatusBar;
    unsafe impl ClassType for NSStatusBar {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSStatusBar {
        #[method_id(systemStatusBar)]
        pub unsafe fn systemStatusBar() -> Id<NSStatusBar, Shared>;
        #[method_id(statusItemWithLength:)]
        pub unsafe fn statusItemWithLength(&self, length: CGFloat) -> Id<NSStatusItem, Shared>;
        #[method(removeStatusItem:)]
        pub unsafe fn removeStatusItem(&self, item: &NSStatusItem);
        #[method(isVertical)]
        pub unsafe fn isVertical(&self) -> bool;
        #[method(thickness)]
        pub unsafe fn thickness(&self) -> CGFloat;
    }
);
