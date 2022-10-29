#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSUserActivityRestoring = NSObject;
extern_methods!(
    #[doc = "NSUserActivity"]
    unsafe impl NSResponder {
        #[method_id(userActivity)]
        pub unsafe fn userActivity(&self) -> Option<Id<NSUserActivity, Shared>>;
        #[method(setUserActivity:)]
        pub unsafe fn setUserActivity(&self, userActivity: Option<&NSUserActivity>);
        #[method(updateUserActivityState:)]
        pub unsafe fn updateUserActivityState(&self, userActivity: &NSUserActivity);
    }
);
extern_methods!(
    #[doc = "NSUserActivity"]
    unsafe impl NSDocument {
        #[method_id(userActivity)]
        pub unsafe fn userActivity(&self) -> Option<Id<NSUserActivity, Shared>>;
        #[method(setUserActivity:)]
        pub unsafe fn setUserActivity(&self, userActivity: Option<&NSUserActivity>);
        #[method(updateUserActivityState:)]
        pub unsafe fn updateUserActivityState(&self, activity: &NSUserActivity);
    }
);