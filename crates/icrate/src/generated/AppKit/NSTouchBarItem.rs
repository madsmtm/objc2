#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTouchBarItemIdentifier = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSTouchBarItem;
    unsafe impl ClassType for NSTouchBarItem {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTouchBarItem {
        #[method_id(initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            &self,
            identifier: &NSTouchBarItemIdentifier,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(identifier)]
        pub unsafe fn identifier(&self) -> Id<NSTouchBarItemIdentifier, Shared>;
        #[method(visibilityPriority)]
        pub unsafe fn visibilityPriority(&self) -> NSTouchBarItemPriority;
        #[method(setVisibilityPriority:)]
        pub unsafe fn setVisibilityPriority(&self, visibilityPriority: NSTouchBarItemPriority);
        #[method_id(view)]
        pub unsafe fn view(&self) -> Option<Id<NSView, Shared>>;
        #[method_id(viewController)]
        pub unsafe fn viewController(&self) -> Option<Id<NSViewController, Shared>>;
        #[method_id(customizationLabel)]
        pub unsafe fn customizationLabel(&self) -> Id<NSString, Shared>;
        #[method(isVisible)]
        pub unsafe fn isVisible(&self) -> bool;
    }
);