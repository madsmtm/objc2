#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSUUID;
    unsafe impl ClassType for NSUUID {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSUUID {
        #[method_id(UUID)]
        pub unsafe fn UUID() -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithUUIDString:)]
        pub unsafe fn initWithUUIDString(&self, string: &NSString) -> Option<Id<Self, Shared>>;
        #[method_id(initWithUUIDBytes:)]
        pub unsafe fn initWithUUIDBytes(&self, bytes: uuid_t) -> Id<Self, Shared>;
        #[method(getUUIDBytes:)]
        pub unsafe fn getUUIDBytes(&self, uuid: uuid_t);
        #[method(compare:)]
        pub unsafe fn compare(&self, otherUUID: &NSUUID) -> NSComparisonResult;
        #[method_id(UUIDString)]
        pub unsafe fn UUIDString(&self) -> Id<NSString, Shared>;
    }
);
