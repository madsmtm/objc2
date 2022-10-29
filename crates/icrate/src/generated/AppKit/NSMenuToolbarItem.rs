#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMenuToolbarItem;
    unsafe impl ClassType for NSMenuToolbarItem {
        type Super = NSToolbarItem;
    }
);
extern_methods!(
    unsafe impl NSMenuToolbarItem {
        #[method_id(menu)]
        pub unsafe fn menu(&self) -> Id<NSMenu, Shared>;
        #[method(setMenu:)]
        pub unsafe fn setMenu(&self, menu: &NSMenu);
        #[method(showsIndicator)]
        pub unsafe fn showsIndicator(&self) -> bool;
        #[method(setShowsIndicator:)]
        pub unsafe fn setShowsIndicator(&self, showsIndicator: bool);
    }
);
