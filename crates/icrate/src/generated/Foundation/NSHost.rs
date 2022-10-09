use super::__exported::NSArray;
use super::__exported::NSMutableArray;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSHost;
    unsafe impl ClassType for NSHost {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSHost {
        #[method_id(currentHost)]
        pub unsafe fn currentHost() -> Id<Self, Shared>;
        #[method_id(hostWithName:)]
        pub unsafe fn hostWithName(name: Option<&NSString>) -> Id<Self, Shared>;
        #[method_id(hostWithAddress:)]
        pub unsafe fn hostWithAddress(address: &NSString) -> Id<Self, Shared>;
        #[method(isEqualToHost:)]
        pub unsafe fn isEqualToHost(&self, aHost: &NSHost) -> bool;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(names)]
        pub unsafe fn names(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(address)]
        pub unsafe fn address(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(addresses)]
        pub unsafe fn addresses(&self) -> Id<NSArray<NSString>, Shared>;
        #[method_id(localizedName)]
        pub unsafe fn localizedName(&self) -> Option<Id<NSString, Shared>>;
        #[method(setHostCacheEnabled:)]
        pub unsafe fn setHostCacheEnabled(flag: bool);
        #[method(isHostCacheEnabled)]
        pub unsafe fn isHostCacheEnabled() -> bool;
        #[method(flushHostCache)]
        pub unsafe fn flushHostCache();
    }
);
