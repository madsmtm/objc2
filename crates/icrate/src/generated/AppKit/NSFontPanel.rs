#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSFontChanging = NSObject;
extern_methods!(
    #[doc = "NSFontPanelValidationAdditions"]
    unsafe impl NSObject {
        #[method(validModesForFontPanel:)]
        pub unsafe fn validModesForFontPanel(&self, fontPanel: &NSFontPanel)
            -> NSFontPanelModeMask;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSFontPanel;
    unsafe impl ClassType for NSFontPanel {
        type Super = NSPanel;
    }
);
extern_methods!(
    unsafe impl NSFontPanel {
        #[method_id(sharedFontPanel)]
        pub unsafe fn sharedFontPanel() -> Id<NSFontPanel, Shared>;
        #[method(sharedFontPanelExists)]
        pub unsafe fn sharedFontPanelExists() -> bool;
        #[method_id(accessoryView)]
        pub unsafe fn accessoryView(&self) -> Option<Id<NSView, Shared>>;
        #[method(setAccessoryView:)]
        pub unsafe fn setAccessoryView(&self, accessoryView: Option<&NSView>);
        #[method(setPanelFont:isMultiple:)]
        pub unsafe fn setPanelFont_isMultiple(&self, fontObj: &NSFont, flag: bool);
        #[method_id(panelConvertFont:)]
        pub unsafe fn panelConvertFont(&self, fontObj: &NSFont) -> Id<NSFont, Shared>;
        #[method(worksWhenModal)]
        pub unsafe fn worksWhenModal(&self) -> bool;
        #[method(setWorksWhenModal:)]
        pub unsafe fn setWorksWhenModal(&self, worksWhenModal: bool);
        #[method(isEnabled)]
        pub unsafe fn isEnabled(&self) -> bool;
        #[method(setEnabled:)]
        pub unsafe fn setEnabled(&self, enabled: bool);
        #[method(reloadDefaultFontFamilies)]
        pub unsafe fn reloadDefaultFontFamilies(&self);
    }
);
