#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTabViewItem;
    unsafe impl ClassType for NSTabViewItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTabViewItem {
        #[method_id(tabViewItemWithViewController:)]
        pub unsafe fn tabViewItemWithViewController(
            viewController: &NSViewController,
        ) -> Id<Self, Shared>;
        #[method_id(initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(&self, identifier: Option<&Object>) -> Id<Self, Shared>;
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Option<Id<Object, Shared>>;
        #[method(setIdentifier:)]
        pub unsafe fn setIdentifier(&self, identifier: Option<&Object>);
        #[method_id(color)]
        pub unsafe fn color(&self) -> Id<NSColor, Shared>;
        #[method(setColor:)]
        pub unsafe fn setColor(&self, color: &NSColor);
        #[method_id(label)]
        pub unsafe fn label(&self) -> Id<NSString, Shared>;
        #[method(setLabel:)]
        pub unsafe fn setLabel(&self, label: &NSString);
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
        #[method_id(view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;
        #[method(setView:)]
        pub unsafe fn setView(&self, view: Option<&NSView>);
        #[method_id(viewController)]
        pub unsafe fn viewController(&self) -> Option<Id<NSViewController, Shared>>;
        #[method(setViewController:)]
        pub unsafe fn setViewController(&self, viewController: Option<&NSViewController>);
        #[method(tabState)]
        pub unsafe fn tabState(&self) -> NSTabState;
        #[method_id(tabView)]
        pub unsafe fn tabView(&self) -> Option<Id<NSTabView, Shared>>;
        #[method_id(initialFirstResponder)]
        pub unsafe fn initialFirstResponder(&self) -> Option<Id<NSView, Shared>>;
        #[method(setInitialFirstResponder:)]
        pub unsafe fn setInitialFirstResponder(&self, initialFirstResponder: Option<&NSView>);
        #[method_id(toolTip)]
        pub unsafe fn toolTip(&self) -> Option<Id<NSString, Shared>>;
        #[method(setToolTip:)]
        pub unsafe fn setToolTip(&self, toolTip: Option<&NSString>);
        #[method(drawLabel:inRect:)]
        pub unsafe fn drawLabel_inRect(&self, shouldTruncateLabel: bool, labelRect: NSRect);
        #[method(sizeOfLabel:)]
        pub unsafe fn sizeOfLabel(&self, computeMin: bool) -> NSSize;
    }
);
