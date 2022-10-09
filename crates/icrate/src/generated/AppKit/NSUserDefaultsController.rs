use super::__exported::NSUserDefaults;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSController::*;
use crate::Foundation::generated::NSDictionary::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUserDefaultsController;
    unsafe impl ClassType for NSUserDefaultsController {
        type Super = NSController;
    }
);
extern_methods!(
    unsafe impl NSUserDefaultsController {
        #[method_id(sharedUserDefaultsController)]
        pub unsafe fn sharedUserDefaultsController() -> Id<NSUserDefaultsController, Shared>;
        #[method_id(initWithDefaults:initialValues:)]
        pub unsafe fn initWithDefaults_initialValues(
            &self,
            defaults: Option<&NSUserDefaults>,
            initialValues: Option<&NSDictionary<NSString, Object>>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(defaults)]
        pub unsafe fn defaults(&self) -> Id<NSUserDefaults, Shared>;
        #[method_id(initialValues)]
        pub unsafe fn initialValues(&self) -> Option<Id<NSDictionary<NSString, Object>, Shared>>;
        #[method(setInitialValues:)]
        pub unsafe fn setInitialValues(
            &self,
            initialValues: Option<&NSDictionary<NSString, Object>>,
        );
        #[method(appliesImmediately)]
        pub unsafe fn appliesImmediately(&self) -> bool;
        #[method(setAppliesImmediately:)]
        pub unsafe fn setAppliesImmediately(&self, appliesImmediately: bool);
        #[method(hasUnappliedChanges)]
        pub unsafe fn hasUnappliedChanges(&self) -> bool;
        #[method_id(values)]
        pub unsafe fn values(&self) -> Id<Object, Shared>;
        #[method(revert:)]
        pub unsafe fn revert(&self, sender: Option<&Object>);
        #[method(save:)]
        pub unsafe fn save(&self, sender: Option<&Object>);
        #[method(revertToInitialValues:)]
        pub unsafe fn revertToInitialValues(&self, sender: Option<&Object>);
    }
);
