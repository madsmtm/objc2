#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSDistantObject;
    unsafe impl ClassType for NSDistantObject {
        type Super = NSProxy;
    }
);
extern_methods!(
    unsafe impl NSDistantObject {
        #[method_id(proxyWithTarget:connection:)]
        pub unsafe fn proxyWithTarget_connection(
            target: &Object,
            connection: &NSConnection,
        ) -> Option<Id<Object, Shared>>;
        #[method_id(initWithTarget:connection:)]
        pub unsafe fn initWithTarget_connection(
            &self,
            target: &Object,
            connection: &NSConnection,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(proxyWithLocal:connection:)]
        pub unsafe fn proxyWithLocal_connection(
            target: &Object,
            connection: &NSConnection,
        ) -> Id<Object, Shared>;
        #[method_id(initWithLocal:connection:)]
        pub unsafe fn initWithLocal_connection(
            &self,
            target: &Object,
            connection: &NSConnection,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method(setProtocolForProxy:)]
        pub unsafe fn setProtocolForProxy(&self, proto: Option<&Protocol>);
        #[method_id(connectionForProxy)]
        pub unsafe fn connectionForProxy(&self) -> Id<NSConnection, Shared>;
    }
);
