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
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSXPCProxyCreating = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSXPCConnection;
    unsafe impl ClassType for NSXPCConnection {
        type Super = NSObject;
    }
);
impl NSXPCConnection {
    pub unsafe fn initWithServiceName(&self, serviceName: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initWithServiceName: serviceName]
    }
    pub unsafe fn initWithMachServiceName_options(
        &self,
        name: &NSString,
        options: NSXPCConnectionOptions,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithMachServiceName: name, options: options]
    }
    pub unsafe fn initWithListenerEndpoint(
        &self,
        endpoint: &NSXPCListenerEndpoint,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithListenerEndpoint: endpoint]
    }
    pub unsafe fn remoteObjectProxyWithErrorHandler(
        &self,
        handler: TodoBlock,
    ) -> Id<Object, Shared> {
        msg_send_id![self, remoteObjectProxyWithErrorHandler: handler]
    }
    pub unsafe fn synchronousRemoteObjectProxyWithErrorHandler(
        &self,
        handler: TodoBlock,
    ) -> Id<Object, Shared> {
        msg_send_id![self, synchronousRemoteObjectProxyWithErrorHandler: handler]
    }
    pub unsafe fn resume(&self) {
        msg_send![self, resume]
    }
    pub unsafe fn suspend(&self) {
        msg_send![self, suspend]
    }
    pub unsafe fn invalidate(&self) {
        msg_send![self, invalidate]
    }
    pub unsafe fn currentConnection() -> Option<Id<NSXPCConnection, Shared>> {
        msg_send_id![Self::class(), currentConnection]
    }
    pub unsafe fn scheduleSendBarrierBlock(&self, block: TodoBlock) {
        msg_send![self, scheduleSendBarrierBlock: block]
    }
    pub unsafe fn serviceName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, serviceName]
    }
    pub unsafe fn endpoint(&self) -> Id<NSXPCListenerEndpoint, Shared> {
        msg_send_id![self, endpoint]
    }
    pub unsafe fn exportedInterface(&self) -> Option<Id<NSXPCInterface, Shared>> {
        msg_send_id![self, exportedInterface]
    }
    pub unsafe fn setExportedInterface(&self, exportedInterface: Option<&NSXPCInterface>) {
        msg_send![self, setExportedInterface: exportedInterface]
    }
    pub unsafe fn exportedObject(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, exportedObject]
    }
    pub unsafe fn setExportedObject(&self, exportedObject: Option<&Object>) {
        msg_send![self, setExportedObject: exportedObject]
    }
    pub unsafe fn remoteObjectInterface(&self) -> Option<Id<NSXPCInterface, Shared>> {
        msg_send_id![self, remoteObjectInterface]
    }
    pub unsafe fn setRemoteObjectInterface(&self, remoteObjectInterface: Option<&NSXPCInterface>) {
        msg_send![self, setRemoteObjectInterface: remoteObjectInterface]
    }
    pub unsafe fn remoteObjectProxy(&self) -> Id<Object, Shared> {
        msg_send_id![self, remoteObjectProxy]
    }
    pub unsafe fn interruptionHandler(&self) -> TodoBlock {
        msg_send![self, interruptionHandler]
    }
    pub unsafe fn setInterruptionHandler(&self, interruptionHandler: TodoBlock) {
        msg_send![self, setInterruptionHandler: interruptionHandler]
    }
    pub unsafe fn invalidationHandler(&self) -> TodoBlock {
        msg_send![self, invalidationHandler]
    }
    pub unsafe fn setInvalidationHandler(&self, invalidationHandler: TodoBlock) {
        msg_send![self, setInvalidationHandler: invalidationHandler]
    }
    pub unsafe fn auditSessionIdentifier(&self) -> au_asid_t {
        msg_send![self, auditSessionIdentifier]
    }
    pub unsafe fn processIdentifier(&self) -> pid_t {
        msg_send![self, processIdentifier]
    }
    pub unsafe fn effectiveUserIdentifier(&self) -> uid_t {
        msg_send![self, effectiveUserIdentifier]
    }
    pub unsafe fn effectiveGroupIdentifier(&self) -> gid_t {
        msg_send![self, effectiveGroupIdentifier]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSXPCListener;
    unsafe impl ClassType for NSXPCListener {
        type Super = NSObject;
    }
);
impl NSXPCListener {
    pub unsafe fn serviceListener() -> Id<NSXPCListener, Shared> {
        msg_send_id![Self::class(), serviceListener]
    }
    pub unsafe fn anonymousListener() -> Id<NSXPCListener, Shared> {
        msg_send_id![Self::class(), anonymousListener]
    }
    pub unsafe fn initWithMachServiceName(&self, name: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initWithMachServiceName: name]
    }
    pub unsafe fn resume(&self) {
        msg_send![self, resume]
    }
    pub unsafe fn suspend(&self) {
        msg_send![self, suspend]
    }
    pub unsafe fn invalidate(&self) {
        msg_send![self, invalidate]
    }
    pub unsafe fn delegate(&self) -> Option<Id<NSXPCListenerDelegate, Shared>> {
        msg_send_id![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: Option<&NSXPCListenerDelegate>) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn endpoint(&self) -> Id<NSXPCListenerEndpoint, Shared> {
        msg_send_id![self, endpoint]
    }
}
pub type NSXPCListenerDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSXPCInterface;
    unsafe impl ClassType for NSXPCInterface {
        type Super = NSObject;
    }
);
impl NSXPCInterface {
    pub unsafe fn interfaceWithProtocol(protocol: &Protocol) -> Id<NSXPCInterface, Shared> {
        msg_send_id![Self::class(), interfaceWithProtocol: protocol]
    }
    pub unsafe fn setClasses_forSelector_argumentIndex_ofReply(
        &self,
        classes: &NSSet<TodoClass>,
        sel: Sel,
        arg: NSUInteger,
        ofReply: bool,
    ) {
        msg_send![
            self,
            setClasses: classes,
            forSelector: sel,
            argumentIndex: arg,
            ofReply: ofReply
        ]
    }
    pub unsafe fn classesForSelector_argumentIndex_ofReply(
        &self,
        sel: Sel,
        arg: NSUInteger,
        ofReply: bool,
    ) -> Id<NSSet<TodoClass>, Shared> {
        msg_send_id![
            self,
            classesForSelector: sel,
            argumentIndex: arg,
            ofReply: ofReply
        ]
    }
    pub unsafe fn setInterface_forSelector_argumentIndex_ofReply(
        &self,
        ifc: &NSXPCInterface,
        sel: Sel,
        arg: NSUInteger,
        ofReply: bool,
    ) {
        msg_send![
            self,
            setInterface: ifc,
            forSelector: sel,
            argumentIndex: arg,
            ofReply: ofReply
        ]
    }
    pub unsafe fn interfaceForSelector_argumentIndex_ofReply(
        &self,
        sel: Sel,
        arg: NSUInteger,
        ofReply: bool,
    ) -> Option<Id<NSXPCInterface, Shared>> {
        msg_send_id![
            self,
            interfaceForSelector: sel,
            argumentIndex: arg,
            ofReply: ofReply
        ]
    }
    pub unsafe fn setXPCType_forSelector_argumentIndex_ofReply(
        &self,
        type_: xpc_type_t,
        sel: Sel,
        arg: NSUInteger,
        ofReply: bool,
    ) {
        msg_send![
            self,
            setXPCType: type_,
            forSelector: sel,
            argumentIndex: arg,
            ofReply: ofReply
        ]
    }
    pub unsafe fn XPCTypeForSelector_argumentIndex_ofReply(
        &self,
        sel: Sel,
        arg: NSUInteger,
        ofReply: bool,
    ) -> xpc_type_t {
        msg_send![
            self,
            XPCTypeForSelector: sel,
            argumentIndex: arg,
            ofReply: ofReply
        ]
    }
    pub unsafe fn protocol(&self) -> Id<Protocol, Shared> {
        msg_send_id![self, protocol]
    }
    pub unsafe fn setProtocol(&self, protocol: &Protocol) {
        msg_send![self, setProtocol: protocol]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSXPCListenerEndpoint;
    unsafe impl ClassType for NSXPCListenerEndpoint {
        type Super = NSObject;
    }
);
impl NSXPCListenerEndpoint {}
extern_class!(
    #[derive(Debug)]
    pub struct NSXPCCoder;
    unsafe impl ClassType for NSXPCCoder {
        type Super = NSCoder;
    }
);
impl NSXPCCoder {
    pub unsafe fn encodeXPCObject_forKey(&self, xpcObject: &xpc_object_t, key: &NSString) {
        msg_send![self, encodeXPCObject: xpcObject, forKey: key]
    }
    pub unsafe fn decodeXPCObjectOfType_forKey(
        &self,
        type_: xpc_type_t,
        key: &NSString,
    ) -> Option<Id<xpc_object_t, Shared>> {
        msg_send_id![self, decodeXPCObjectOfType: type_, forKey: key]
    }
    pub unsafe fn userInfo(&self) -> Option<Id<NSObject, Shared>> {
        msg_send_id![self, userInfo]
    }
    pub unsafe fn setUserInfo(&self, userInfo: Option<&NSObject>) {
        msg_send![self, setUserInfo: userInfo]
    }
    pub unsafe fn connection(&self) -> Option<Id<NSXPCConnection, Shared>> {
        msg_send_id![self, connection]
    }
}
