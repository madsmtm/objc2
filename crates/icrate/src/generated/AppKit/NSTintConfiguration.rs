#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTintConfiguration;
    unsafe impl ClassType for NSTintConfiguration {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTintConfiguration {
        #[method_id(defaultTintConfiguration)]
        pub unsafe fn defaultTintConfiguration() -> Id<NSTintConfiguration, Shared>;
        #[method_id(monochromeTintConfiguration)]
        pub unsafe fn monochromeTintConfiguration() -> Id<NSTintConfiguration, Shared>;
        #[method_id(tintConfigurationWithPreferredColor:)]
        pub unsafe fn tintConfigurationWithPreferredColor(color: &NSColor) -> Id<Self, Shared>;
        #[method_id(tintConfigurationWithFixedColor:)]
        pub unsafe fn tintConfigurationWithFixedColor(color: &NSColor) -> Id<Self, Shared>;
        #[method_id(baseTintColor)]
        pub unsafe fn baseTintColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method_id(equivalentContentTintColor)]
        pub unsafe fn equivalentContentTintColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(adaptsToUserAccentColor)]
        pub unsafe fn adaptsToUserAccentColor(&self) -> bool;
    }
);
