#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSMediaLibraryBrowserController;
    unsafe impl ClassType for NSMediaLibraryBrowserController {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSMediaLibraryBrowserController {
        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;
        #[method(setVisible:)]
        pub unsafe fn setVisible(&self, visible: bool);
        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;
        #[method(setFrame:)]
        pub unsafe fn setFrame(&self, frame: NSRect);
        #[method(mediaLibraries)]
        pub unsafe fn mediaLibraries(&self) -> NSMediaLibrary;
        #[method(setMediaLibraries:)]
        pub unsafe fn setMediaLibraries(&self, mediaLibraries: NSMediaLibrary);
        #[method_id(sharedMediaLibraryBrowserController)]
        pub unsafe fn sharedMediaLibraryBrowserController(
        ) -> Id<NSMediaLibraryBrowserController, Shared>;
        #[method(togglePanel:)]
        pub unsafe fn togglePanel(&self, sender: Option<&Object>);
    }
);
