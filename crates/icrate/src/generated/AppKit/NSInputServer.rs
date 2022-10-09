use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSInputServiceProvider = NSObject;
pub type NSInputServerMouseTracker = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSInputServer;
    unsafe impl ClassType for NSInputServer {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSInputServer {
        #[method_id(initWithDelegate:name:)]
        pub unsafe fn initWithDelegate_name(
            &self,
            delegate: Option<&Object>,
            name: Option<&NSString>,
        ) -> Id<Self, Shared>;
    }
);
