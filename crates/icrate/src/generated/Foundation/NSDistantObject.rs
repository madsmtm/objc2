#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSDistantObject;
    unsafe impl ClassType for NSDistantObject {
        type Super = NSProxy;
    }
);
impl NSDistantObject {
    pub unsafe fn proxyWithTarget_connection(
        target: &Object,
        connection: &NSConnection,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            proxyWithTarget: target,
            connection: connection
        ]
    }
    pub unsafe fn initWithTarget_connection(
        &self,
        target: &Object,
        connection: &NSConnection,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithTarget: target, connection: connection]
    }
    pub unsafe fn proxyWithLocal_connection(
        target: &Object,
        connection: &NSConnection,
    ) -> Id<Object, Shared> {
        msg_send_id![
            Self::class(),
            proxyWithLocal: target,
            connection: connection
        ]
    }
    pub unsafe fn initWithLocal_connection(
        &self,
        target: &Object,
        connection: &NSConnection,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithLocal: target, connection: connection]
    }
    pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: inCoder]
    }
    pub unsafe fn setProtocolForProxy(&self, proto: Option<&Protocol>) {
        msg_send![self, setProtocolForProxy: proto]
    }
    pub unsafe fn connectionForProxy(&self) -> Id<NSConnection, Shared> {
        msg_send_id![self, connectionForProxy]
    }
}
