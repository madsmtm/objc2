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
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPort;
    unsafe impl ClassType for NSPort {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSPort {
        #[method_id(port)]
        pub unsafe fn port() -> Id<NSPort, Shared>;
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, anObject: Option<&NSPortDelegate>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSPortDelegate, Shared>>;
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(&self, runLoop: &NSRunLoop, mode: &NSRunLoopMode);
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(&self, runLoop: &NSRunLoop, mode: &NSRunLoopMode);
        #[method(reservedSpaceLength)]
        pub unsafe fn reservedSpaceLength(&self) -> NSUInteger;
        #[method(sendBeforeDate:components:from:reserved:)]
        pub unsafe fn sendBeforeDate_components_from_reserved(
            &self,
            limitDate: &NSDate,
            components: Option<&NSMutableArray>,
            receivePort: Option<&NSPort>,
            headerSpaceReserved: NSUInteger,
        ) -> bool;
        #[method(sendBeforeDate:msgid:components:from:reserved:)]
        pub unsafe fn sendBeforeDate_msgid_components_from_reserved(
            &self,
            limitDate: &NSDate,
            msgID: NSUInteger,
            components: Option<&NSMutableArray>,
            receivePort: Option<&NSPort>,
            headerSpaceReserved: NSUInteger,
        ) -> bool;
        #[method(addConnection:toRunLoop:forMode:)]
        pub unsafe fn addConnection_toRunLoop_forMode(
            &self,
            conn: &NSConnection,
            runLoop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );
        #[method(removeConnection:fromRunLoop:forMode:)]
        pub unsafe fn removeConnection_fromRunLoop_forMode(
            &self,
            conn: &NSConnection,
            runLoop: &NSRunLoop,
            mode: &NSRunLoopMode,
        );
    }
);
pub type NSPortDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSMachPort;
    unsafe impl ClassType for NSMachPort {
        type Super = NSPort;
    }
);
extern_methods!(
    unsafe impl NSMachPort {
        #[method_id(portWithMachPort:)]
        pub unsafe fn portWithMachPort(machPort: u32) -> Id<NSPort, Shared>;
        #[method_id(initWithMachPort:)]
        pub unsafe fn initWithMachPort(&self, machPort: u32) -> Id<Self, Shared>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, anObject: Option<&NSMachPortDelegate>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSMachPortDelegate, Shared>>;
        #[method_id(portWithMachPort:options:)]
        pub unsafe fn portWithMachPort_options(
            machPort: u32,
            f: NSMachPortOptions,
        ) -> Id<NSPort, Shared>;
        #[method_id(initWithMachPort:options:)]
        pub unsafe fn initWithMachPort_options(
            &self,
            machPort: u32,
            f: NSMachPortOptions,
        ) -> Id<Self, Shared>;
        #[method(machPort)]
        pub unsafe fn machPort(&self) -> u32;
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(&self, runLoop: &NSRunLoop, mode: &NSRunLoopMode);
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(&self, runLoop: &NSRunLoop, mode: &NSRunLoopMode);
    }
);
pub type NSMachPortDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSMessagePort;
    unsafe impl ClassType for NSMessagePort {
        type Super = NSPort;
    }
);
extern_methods!(
    unsafe impl NSMessagePort {}
);
extern_class!(
    #[derive(Debug)]
    pub struct NSSocketPort;
    unsafe impl ClassType for NSSocketPort {
        type Super = NSPort;
    }
);
extern_methods!(
    unsafe impl NSSocketPort {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithTCPPort:)]
        pub unsafe fn initWithTCPPort(&self, port: c_ushort) -> Option<Id<Self, Shared>>;
        #[method_id(initWithProtocolFamily:socketType:protocol:address:)]
        pub unsafe fn initWithProtocolFamily_socketType_protocol_address(
            &self,
            family: c_int,
            type_: c_int,
            protocol: c_int,
            address: &NSData,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithProtocolFamily:socketType:protocol:socket:)]
        pub unsafe fn initWithProtocolFamily_socketType_protocol_socket(
            &self,
            family: c_int,
            type_: c_int,
            protocol: c_int,
            sock: NSSocketNativeHandle,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initRemoteWithTCPPort:host:)]
        pub unsafe fn initRemoteWithTCPPort_host(
            &self,
            port: c_ushort,
            hostName: Option<&NSString>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initRemoteWithProtocolFamily:socketType:protocol:address:)]
        pub unsafe fn initRemoteWithProtocolFamily_socketType_protocol_address(
            &self,
            family: c_int,
            type_: c_int,
            protocol: c_int,
            address: &NSData,
        ) -> Id<Self, Shared>;
        #[method(protocolFamily)]
        pub unsafe fn protocolFamily(&self) -> c_int;
        #[method(socketType)]
        pub unsafe fn socketType(&self) -> c_int;
        #[method(protocol)]
        pub unsafe fn protocol(&self) -> c_int;
        #[method_id(address)]
        pub unsafe fn address(&self) -> Id<NSData, Shared>;
        #[method(socket)]
        pub unsafe fn socket(&self) -> NSSocketNativeHandle;
    }
);
