#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSAppearanceName = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSAppearance;
    unsafe impl ClassType for NSAppearance {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAppearance {
        #[method_id(name)]
        pub unsafe fn name(&self) -> Id<NSAppearanceName, Shared>;
        #[method_id(currentAppearance)]
        pub unsafe fn currentAppearance() -> Option<Id<NSAppearance, Shared>>;
        #[method(setCurrentAppearance:)]
        pub unsafe fn setCurrentAppearance(currentAppearance: Option<&NSAppearance>);
        #[method_id(currentDrawingAppearance)]
        pub unsafe fn currentDrawingAppearance() -> Id<NSAppearance, Shared>;
        #[method(performAsCurrentDrawingAppearance:)]
        pub unsafe fn performAsCurrentDrawingAppearance(&self, block: TodoBlock);
        #[method_id(appearanceNamed:)]
        pub unsafe fn appearanceNamed(name: &NSAppearanceName) -> Option<Id<NSAppearance, Shared>>;
        #[method_id(initWithAppearanceNamed:bundle:)]
        pub unsafe fn initWithAppearanceNamed_bundle(
            &self,
            name: &NSAppearanceName,
            bundle: Option<&NSBundle>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method(allowsVibrancy)]
        pub unsafe fn allowsVibrancy(&self) -> bool;
        #[method_id(bestMatchFromAppearancesWithNames:)]
        pub unsafe fn bestMatchFromAppearancesWithNames(
            &self,
            appearances: &NSArray<NSAppearanceName>,
        ) -> Option<Id<NSAppearanceName, Shared>>;
    }
);
pub type NSAppearanceCustomization = NSObject;
