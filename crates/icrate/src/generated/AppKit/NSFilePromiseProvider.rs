#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFilePromiseProvider;
    unsafe impl ClassType for NSFilePromiseProvider {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFilePromiseProvider {
        #[method_id(fileType)]
        pub unsafe fn fileType(&self) -> Id<NSString, Shared>;
        #[method(setFileType:)]
        pub unsafe fn setFileType(&self, fileType: &NSString);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSFilePromiseProviderDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSFilePromiseProviderDelegate>);
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<Object, Shared>>;
        #[method(setUserInfo:)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&Object>);
        #[method_id(initWithFileType:delegate:)]
        pub unsafe fn initWithFileType_delegate(
            &self,
            fileType: &NSString,
            delegate: &NSFilePromiseProviderDelegate,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
    }
);
pub type NSFilePromiseProviderDelegate = NSObject;
