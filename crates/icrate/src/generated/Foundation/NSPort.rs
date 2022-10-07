use super::__exported::NSConnection;
use super::__exported::NSData;
use super::__exported::NSDate;
use super::__exported::NSMutableArray;
use super::__exported::NSPortMessage;
use super::__exported::NSRunLoop;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRunLoop::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPort;
    unsafe impl ClassType for NSPort {
        type Super = NSObject;
    }
);
impl NSPort {
    pub unsafe fn port() -> Id<NSPort, Shared> {
        msg_send_id![Self::class(), port]
    }
    pub unsafe fn invalidate(&self) {
        msg_send![self, invalidate]
    }
    pub unsafe fn isValid(&self) -> bool {
        msg_send![self, isValid]
    }
    pub unsafe fn setDelegate(&self, anObject: Option<&NSPortDelegate>) {
        msg_send![self, setDelegate: anObject]
    }
    pub unsafe fn delegate(&self) -> Option<Id<NSPortDelegate, Shared>> {
        msg_send_id![self, delegate]
    }
    pub unsafe fn scheduleInRunLoop_forMode(&self, runLoop: &NSRunLoop, mode: &NSRunLoopMode) {
        msg_send![self, scheduleInRunLoop: runLoop, forMode: mode]
    }
    pub unsafe fn removeFromRunLoop_forMode(&self, runLoop: &NSRunLoop, mode: &NSRunLoopMode) {
        msg_send![self, removeFromRunLoop: runLoop, forMode: mode]
    }
    pub unsafe fn reservedSpaceLength(&self) -> NSUInteger {
        msg_send![self, reservedSpaceLength]
    }
    pub unsafe fn sendBeforeDate_components_from_reserved(
        &self,
        limitDate: &NSDate,
        components: Option<&NSMutableArray>,
        receivePort: Option<&NSPort>,
        headerSpaceReserved: NSUInteger,
    ) -> bool {
        msg_send![
            self,
            sendBeforeDate: limitDate,
            components: components,
            from: receivePort,
            reserved: headerSpaceReserved
        ]
    }
    pub unsafe fn sendBeforeDate_msgid_components_from_reserved(
        &self,
        limitDate: &NSDate,
        msgID: NSUInteger,
        components: Option<&NSMutableArray>,
        receivePort: Option<&NSPort>,
        headerSpaceReserved: NSUInteger,
    ) -> bool {
        msg_send![
            self,
            sendBeforeDate: limitDate,
            msgid: msgID,
            components: components,
            from: receivePort,
            reserved: headerSpaceReserved
        ]
    }
    pub unsafe fn addConnection_toRunLoop_forMode(
        &self,
        conn: &NSConnection,
        runLoop: &NSRunLoop,
        mode: &NSRunLoopMode,
    ) {
        msg_send![self, addConnection: conn, toRunLoop: runLoop, forMode: mode]
    }
    pub unsafe fn removeConnection_fromRunLoop_forMode(
        &self,
        conn: &NSConnection,
        runLoop: &NSRunLoop,
        mode: &NSRunLoopMode,
    ) {
        msg_send![
            self,
            removeConnection: conn,
            fromRunLoop: runLoop,
            forMode: mode
        ]
    }
}
pub type NSPortDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSMachPort;
    unsafe impl ClassType for NSMachPort {
        type Super = NSPort;
    }
);
impl NSMachPort {
    pub unsafe fn portWithMachPort(machPort: u32) -> Id<NSPort, Shared> {
        msg_send_id![Self::class(), portWithMachPort: machPort]
    }
    pub unsafe fn initWithMachPort(&self, machPort: u32) -> Id<Self, Shared> {
        msg_send_id![self, initWithMachPort: machPort]
    }
    pub unsafe fn setDelegate(&self, anObject: Option<&NSMachPortDelegate>) {
        msg_send![self, setDelegate: anObject]
    }
    pub unsafe fn delegate(&self) -> Option<Id<NSMachPortDelegate, Shared>> {
        msg_send_id![self, delegate]
    }
    pub unsafe fn portWithMachPort_options(
        machPort: u32,
        f: NSMachPortOptions,
    ) -> Id<NSPort, Shared> {
        msg_send_id![Self::class(), portWithMachPort: machPort, options: f]
    }
    pub unsafe fn initWithMachPort_options(
        &self,
        machPort: u32,
        f: NSMachPortOptions,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithMachPort: machPort, options: f]
    }
    pub unsafe fn machPort(&self) -> u32 {
        msg_send![self, machPort]
    }
    pub unsafe fn scheduleInRunLoop_forMode(&self, runLoop: &NSRunLoop, mode: &NSRunLoopMode) {
        msg_send![self, scheduleInRunLoop: runLoop, forMode: mode]
    }
    pub unsafe fn removeFromRunLoop_forMode(&self, runLoop: &NSRunLoop, mode: &NSRunLoopMode) {
        msg_send![self, removeFromRunLoop: runLoop, forMode: mode]
    }
}
pub type NSMachPortDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSMessagePort;
    unsafe impl ClassType for NSMessagePort {
        type Super = NSPort;
    }
);
impl NSMessagePort {}
extern_class!(
    #[derive(Debug)]
    pub struct NSSocketPort;
    unsafe impl ClassType for NSSocketPort {
        type Super = NSPort;
    }
);
impl NSSocketPort {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithTCPPort(&self, port: c_ushort) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithTCPPort: port]
    }
    pub unsafe fn initWithProtocolFamily_socketType_protocol_address(
        &self,
        family: c_int,
        type_: c_int,
        protocol: c_int,
        address: &NSData,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithProtocolFamily: family,
            socketType: type_,
            protocol: protocol,
            address: address
        ]
    }
    pub unsafe fn initWithProtocolFamily_socketType_protocol_socket(
        &self,
        family: c_int,
        type_: c_int,
        protocol: c_int,
        sock: NSSocketNativeHandle,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithProtocolFamily: family,
            socketType: type_,
            protocol: protocol,
            socket: sock
        ]
    }
    pub unsafe fn initRemoteWithTCPPort_host(
        &self,
        port: c_ushort,
        hostName: Option<&NSString>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initRemoteWithTCPPort: port, host: hostName]
    }
    pub unsafe fn initRemoteWithProtocolFamily_socketType_protocol_address(
        &self,
        family: c_int,
        type_: c_int,
        protocol: c_int,
        address: &NSData,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initRemoteWithProtocolFamily: family,
            socketType: type_,
            protocol: protocol,
            address: address
        ]
    }
    pub unsafe fn protocolFamily(&self) -> c_int {
        msg_send![self, protocolFamily]
    }
    pub unsafe fn socketType(&self) -> c_int {
        msg_send![self, socketType]
    }
    pub unsafe fn protocol(&self) -> c_int {
        msg_send![self, protocol]
    }
    pub unsafe fn address(&self) -> Id<NSData, Shared> {
        msg_send_id![self, address]
    }
    pub unsafe fn socket(&self) -> NSSocketNativeHandle {
        msg_send![self, socket]
    }
}
