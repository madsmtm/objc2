use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSWindow::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPanel;
    unsafe impl ClassType for NSPanel {
        type Super = NSWindow;
    }
);
extern_methods!(
    unsafe impl NSPanel {
        #[method(isFloatingPanel)]
        pub unsafe fn isFloatingPanel(&self) -> bool;
        #[method(setFloatingPanel:)]
        pub unsafe fn setFloatingPanel(&self, floatingPanel: bool);
        #[method(becomesKeyOnlyIfNeeded)]
        pub unsafe fn becomesKeyOnlyIfNeeded(&self) -> bool;
        #[method(setBecomesKeyOnlyIfNeeded:)]
        pub unsafe fn setBecomesKeyOnlyIfNeeded(&self, becomesKeyOnlyIfNeeded: bool);
        #[method(worksWhenModal)]
        pub unsafe fn worksWhenModal(&self) -> bool;
        #[method(setWorksWhenModal:)]
        pub unsafe fn setWorksWhenModal(&self, worksWhenModal: bool);
    }
);
