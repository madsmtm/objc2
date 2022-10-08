use super::__exported::NSError;
use super::__exported::NSLock;
use super::__exported::NSMutableDictionary;
use super::__exported::NSOperationQueue;
use super::__exported::NSSet;
use super::__exported::NSString;
use crate::bsm::generated::audit::*;
use crate::dispatch::generated::dispatch::*;
use crate::xpc::generated::xpc::*;
use crate::CoreFoundation::generated::CFDictionary::*;
use crate::Foundation::generated::NSCoder::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSXPCProxyCreating = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSXPCConnection;
    unsafe impl ClassType for NSXPCConnection {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSXPCConnection {
        # [method_id (initWithServiceName :)]
        pub unsafe fn initWithServiceName(&self, serviceName: &NSString) -> Id<Self, Shared>;
        #[method_id(serviceName)]
        pub unsafe fn serviceName(&self) -> Option<Id<NSString, Shared>>;
        # [method_id (initWithMachServiceName : options :)]
        pub unsafe fn initWithMachServiceName_options(
            &self,
            name: &NSString,
            options: NSXPCConnectionOptions,
        ) -> Id<Self, Shared>;
        # [method_id (initWithListenerEndpoint :)]
        pub unsafe fn initWithListenerEndpoint(
            &self,
            endpoint: &NSXPCListenerEndpoint,
        ) -> Id<Self, Shared>;
        #[method_id(endpoint)]
        pub unsafe fn endpoint(&self) -> Id<NSXPCListenerEndpoint, Shared>;
        #[method_id(exportedInterface)]
        pub unsafe fn exportedInterface(&self) -> Option<Id<NSXPCInterface, Shared>>;
        # [method (setExportedInterface :)]
        pub unsafe fn setExportedInterface(&self, exportedInterface: Option<&NSXPCInterface>);
        #[method_id(exportedObject)]
        pub unsafe fn exportedObject(&self) -> Option<Id<Object, Shared>>;
        # [method (setExportedObject :)]
        pub unsafe fn setExportedObject(&self, exportedObject: Option<&Object>);
        #[method_id(remoteObjectInterface)]
        pub unsafe fn remoteObjectInterface(&self) -> Option<Id<NSXPCInterface, Shared>>;
        # [method (setRemoteObjectInterface :)]
        pub unsafe fn setRemoteObjectInterface(
            &self,
            remoteObjectInterface: Option<&NSXPCInterface>,
        );
        #[method_id(remoteObjectProxy)]
        pub unsafe fn remoteObjectProxy(&self) -> Id<Object, Shared>;
        # [method_id (remoteObjectProxyWithErrorHandler :)]
        pub unsafe fn remoteObjectProxyWithErrorHandler(
            &self,
            handler: TodoBlock,
        ) -> Id<Object, Shared>;
        # [method_id (synchronousRemoteObjectProxyWithErrorHandler :)]
        pub unsafe fn synchronousRemoteObjectProxyWithErrorHandler(
            &self,
            handler: TodoBlock,
        ) -> Id<Object, Shared>;
        #[method(interruptionHandler)]
        pub unsafe fn interruptionHandler(&self) -> TodoBlock;
        # [method (setInterruptionHandler :)]
        pub unsafe fn setInterruptionHandler(&self, interruptionHandler: TodoBlock);
        #[method(invalidationHandler)]
        pub unsafe fn invalidationHandler(&self) -> TodoBlock;
        # [method (setInvalidationHandler :)]
        pub unsafe fn setInvalidationHandler(&self, invalidationHandler: TodoBlock);
        #[method(resume)]
        pub unsafe fn resume(&self);
        #[method(suspend)]
        pub unsafe fn suspend(&self);
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
        #[method(auditSessionIdentifier)]
        pub unsafe fn auditSessionIdentifier(&self) -> au_asid_t;
        #[method(processIdentifier)]
        pub unsafe fn processIdentifier(&self) -> pid_t;
        #[method(effectiveUserIdentifier)]
        pub unsafe fn effectiveUserIdentifier(&self) -> uid_t;
        #[method(effectiveGroupIdentifier)]
        pub unsafe fn effectiveGroupIdentifier(&self) -> gid_t;
        #[method_id(currentConnection)]
        pub unsafe fn currentConnection() -> Option<Id<NSXPCConnection, Shared>>;
        # [method (scheduleSendBarrierBlock :)]
        pub unsafe fn scheduleSendBarrierBlock(&self, block: TodoBlock);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSXPCListener;
    unsafe impl ClassType for NSXPCListener {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSXPCListener {
        #[method_id(serviceListener)]
        pub unsafe fn serviceListener() -> Id<NSXPCListener, Shared>;
        #[method_id(anonymousListener)]
        pub unsafe fn anonymousListener() -> Id<NSXPCListener, Shared>;
        # [method_id (initWithMachServiceName :)]
        pub unsafe fn initWithMachServiceName(&self, name: &NSString) -> Id<Self, Shared>;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSXPCListenerDelegate, Shared>>;
        # [method (setDelegate :)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSXPCListenerDelegate>);
        #[method_id(endpoint)]
        pub unsafe fn endpoint(&self) -> Id<NSXPCListenerEndpoint, Shared>;
        #[method(resume)]
        pub unsafe fn resume(&self);
        #[method(suspend)]
        pub unsafe fn suspend(&self);
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
    }
);
pub type NSXPCListenerDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSXPCInterface;
    unsafe impl ClassType for NSXPCInterface {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSXPCInterface {
        # [method_id (interfaceWithProtocol :)]
        pub unsafe fn interfaceWithProtocol(protocol: &Protocol) -> Id<NSXPCInterface, Shared>;
        #[method_id(protocol)]
        pub unsafe fn protocol(&self) -> Id<Protocol, Shared>;
        # [method (setProtocol :)]
        pub unsafe fn setProtocol(&self, protocol: &Protocol);
        # [method (setClasses : forSelector : argumentIndex : ofReply :)]
        pub unsafe fn setClasses_forSelector_argumentIndex_ofReply(
            &self,
            classes: &NSSet<TodoClass>,
            sel: Sel,
            arg: NSUInteger,
            ofReply: bool,
        );
        # [method_id (classesForSelector : argumentIndex : ofReply :)]
        pub unsafe fn classesForSelector_argumentIndex_ofReply(
            &self,
            sel: Sel,
            arg: NSUInteger,
            ofReply: bool,
        ) -> Id<NSSet<TodoClass>, Shared>;
        # [method (setInterface : forSelector : argumentIndex : ofReply :)]
        pub unsafe fn setInterface_forSelector_argumentIndex_ofReply(
            &self,
            ifc: &NSXPCInterface,
            sel: Sel,
            arg: NSUInteger,
            ofReply: bool,
        );
        # [method_id (interfaceForSelector : argumentIndex : ofReply :)]
        pub unsafe fn interfaceForSelector_argumentIndex_ofReply(
            &self,
            sel: Sel,
            arg: NSUInteger,
            ofReply: bool,
        ) -> Option<Id<NSXPCInterface, Shared>>;
        # [method (setXPCType : forSelector : argumentIndex : ofReply :)]
        pub unsafe fn setXPCType_forSelector_argumentIndex_ofReply(
            &self,
            type_: xpc_type_t,
            sel: Sel,
            arg: NSUInteger,
            ofReply: bool,
        );
        # [method (XPCTypeForSelector : argumentIndex : ofReply :)]
        pub unsafe fn XPCTypeForSelector_argumentIndex_ofReply(
            &self,
            sel: Sel,
            arg: NSUInteger,
            ofReply: bool,
        ) -> xpc_type_t;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSXPCListenerEndpoint;
    unsafe impl ClassType for NSXPCListenerEndpoint {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSXPCListenerEndpoint {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSXPCCoder;
    unsafe impl ClassType for NSXPCCoder {
        type Super = NSCoder;
    }
);
extern_methods!(
    unsafe impl NSXPCCoder {
        # [method (encodeXPCObject : forKey :)]
        pub unsafe fn encodeXPCObject_forKey(&self, xpcObject: &xpc_object_t, key: &NSString);
        # [method_id (decodeXPCObjectOfType : forKey :)]
        pub unsafe fn decodeXPCObjectOfType_forKey(
            &self,
            type_: xpc_type_t,
            key: &NSString,
        ) -> Option<Id<xpc_object_t, Shared>>;
        #[method_id(userInfo)]
        pub unsafe fn userInfo(&self) -> Option<Id<NSObject, Shared>>;
        # [method (setUserInfo :)]
        pub unsafe fn setUserInfo(&self, userInfo: Option<&NSObject>);
        #[method_id(connection)]
        pub unsafe fn connection(&self) -> Option<Id<NSXPCConnection, Shared>>;
    }
);
