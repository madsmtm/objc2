use super::__exported::NSArray;
use super::__exported::NSConnection;
use super::__exported::NSPort;
use crate::Foundation::generated::NSCoder::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPortCoder;
    unsafe impl ClassType for NSPortCoder {
        type Super = NSCoder;
    }
);
extern_methods!(
    unsafe impl NSPortCoder {
        #[method(isBycopy)]
        pub unsafe fn isBycopy(&self) -> bool;
        #[method(isByref)]
        pub unsafe fn isByref(&self) -> bool;
        # [method (encodePortObject :)]
        pub unsafe fn encodePortObject(&self, aport: &NSPort);
        #[method_id(decodePortObject)]
        pub unsafe fn decodePortObject(&self) -> Option<Id<NSPort, Shared>>;
        #[method_id(connection)]
        pub unsafe fn connection(&self) -> Option<Id<NSConnection, Shared>>;
        # [method_id (portCoderWithReceivePort : sendPort : components :)]
        pub unsafe fn portCoderWithReceivePort_sendPort_components(
            rcvPort: Option<&NSPort>,
            sndPort: Option<&NSPort>,
            comps: Option<&NSArray>,
        ) -> Id<Object, Shared>;
        # [method_id (initWithReceivePort : sendPort : components :)]
        pub unsafe fn initWithReceivePort_sendPort_components(
            &self,
            rcvPort: Option<&NSPort>,
            sndPort: Option<&NSPort>,
            comps: Option<&NSArray>,
        ) -> Id<Object, Shared>;
        #[method(dispatch)]
        pub unsafe fn dispatch(&self);
    }
);
extern_methods!(
    #[doc = "NSDistributedObjects"]
    unsafe impl NSObject {
        #[method(classForPortCoder)]
        pub unsafe fn classForPortCoder(&self) -> &Class;
        # [method_id (replacementObjectForPortCoder :)]
        pub unsafe fn replacementObjectForPortCoder(
            &self,
            coder: &NSPortCoder,
        ) -> Option<Id<Object, Shared>>;
    }
);
