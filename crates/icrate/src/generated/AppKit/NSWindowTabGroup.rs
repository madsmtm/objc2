#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSWindowTabGroup;
    unsafe impl ClassType for NSWindowTabGroup {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSWindowTabGroup {
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Id<NSWindowTabbingIdentifier, Shared>;
        #[method_id(windows)]
        pub unsafe fn windows(&self) -> Id<NSArray<NSWindow>, Shared>;
        #[method(isOverviewVisible)]
        pub unsafe fn isOverviewVisible(&self) -> bool;
        #[method(setOverviewVisible:)]
        pub unsafe fn setOverviewVisible(&self, overviewVisible: bool);
        #[method(isTabBarVisible)]
        pub unsafe fn isTabBarVisible(&self) -> bool;
        #[method_id(selectedWindow)]
        pub unsafe fn selectedWindow(&self) -> Option<Id<NSWindow, Shared>>;
        #[method(setSelectedWindow:)]
        pub unsafe fn setSelectedWindow(&self, selectedWindow: Option<&NSWindow>);
        #[method(addWindow:)]
        pub unsafe fn addWindow(&self, window: &NSWindow);
        #[method(insertWindow:atIndex:)]
        pub unsafe fn insertWindow_atIndex(&self, window: &NSWindow, index: NSInteger);
        #[method(removeWindow:)]
        pub unsafe fn removeWindow(&self, window: &NSWindow);
    }
);
