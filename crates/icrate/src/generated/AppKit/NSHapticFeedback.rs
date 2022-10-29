#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSHapticFeedbackPerformer = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSHapticFeedbackManager;
    unsafe impl ClassType for NSHapticFeedbackManager {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSHapticFeedbackManager {
        #[method_id(defaultPerformer)]
        pub unsafe fn defaultPerformer() -> Id<NSHapticFeedbackPerformer, Shared>;
    }
);
