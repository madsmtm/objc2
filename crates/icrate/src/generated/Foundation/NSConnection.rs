use super::__exported::NSArray;
use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSDistantObject;
use super::__exported::NSException;
use super::__exported::NSMutableData;
use super::__exported::NSNumber;
use super::__exported::NSPort;
use super::__exported::NSPortNameServer;
use super::__exported::NSRunLoop;
use super::__exported::NSString;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSConnection;
    unsafe impl ClassType for NSConnection {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSConnection {
        pub unsafe fn statistics(&self) -> Id<NSDictionary<NSString, NSNumber>, Shared> {
            msg_send_id![self, statistics]
        }
        pub unsafe fn allConnections() -> Id<NSArray<NSConnection>, Shared> {
            msg_send_id![Self::class(), allConnections]
        }
        pub unsafe fn defaultConnection() -> Id<NSConnection, Shared> {
            msg_send_id![Self::class(), defaultConnection]
        }
        pub unsafe fn connectionWithRegisteredName_host(
            name: &NSString,
            hostName: Option<&NSString>,
        ) -> Option<Id<Self, Shared>> {
            msg_send_id![
                Self::class(),
                connectionWithRegisteredName: name,
                host: hostName
            ]
        }
        pub unsafe fn connectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            hostName: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Id<Self, Shared>> {
            msg_send_id![
                Self::class(),
                connectionWithRegisteredName: name,
                host: hostName,
                usingNameServer: server
            ]
        }
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host(
            name: &NSString,
            hostName: Option<&NSString>,
        ) -> Option<Id<NSDistantObject, Shared>> {
            msg_send_id![
                Self::class(),
                rootProxyForConnectionWithRegisteredName: name,
                host: hostName
            ]
        }
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            hostName: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Id<NSDistantObject, Shared>> {
            msg_send_id![
                Self::class(),
                rootProxyForConnectionWithRegisteredName: name,
                host: hostName,
                usingNameServer: server
            ]
        }
        pub unsafe fn serviceConnectionWithName_rootObject_usingNameServer(
            name: &NSString,
            root: &Object,
            server: &NSPortNameServer,
        ) -> Option<Id<Self, Shared>> {
            msg_send_id![
                Self::class(),
                serviceConnectionWithName: name,
                rootObject: root,
                usingNameServer: server
            ]
        }
        pub unsafe fn serviceConnectionWithName_rootObject(
            name: &NSString,
            root: &Object,
        ) -> Option<Id<Self, Shared>> {
            msg_send_id![
                Self::class(),
                serviceConnectionWithName: name,
                rootObject: root
            ]
        }
        pub unsafe fn requestTimeout(&self) -> NSTimeInterval {
            msg_send![self, requestTimeout]
        }
        pub unsafe fn setRequestTimeout(&self, requestTimeout: NSTimeInterval) {
            msg_send![self, setRequestTimeout: requestTimeout]
        }
        pub unsafe fn replyTimeout(&self) -> NSTimeInterval {
            msg_send![self, replyTimeout]
        }
        pub unsafe fn setReplyTimeout(&self, replyTimeout: NSTimeInterval) {
            msg_send![self, setReplyTimeout: replyTimeout]
        }
        pub unsafe fn rootObject(&self) -> Option<Id<Object, Shared>> {
            msg_send_id![self, rootObject]
        }
        pub unsafe fn setRootObject(&self, rootObject: Option<&Object>) {
            msg_send![self, setRootObject: rootObject]
        }
        pub unsafe fn delegate(&self) -> Option<Id<NSConnectionDelegate, Shared>> {
            msg_send_id![self, delegate]
        }
        pub unsafe fn setDelegate(&self, delegate: Option<&NSConnectionDelegate>) {
            msg_send![self, setDelegate: delegate]
        }
        pub unsafe fn independentConversationQueueing(&self) -> bool {
            msg_send![self, independentConversationQueueing]
        }
        pub unsafe fn setIndependentConversationQueueing(
            &self,
            independentConversationQueueing: bool,
        ) {
            msg_send![
                self,
                setIndependentConversationQueueing: independentConversationQueueing
            ]
        }
        pub unsafe fn isValid(&self) -> bool {
            msg_send![self, isValid]
        }
        pub unsafe fn rootProxy(&self) -> Id<NSDistantObject, Shared> {
            msg_send_id![self, rootProxy]
        }
        pub unsafe fn invalidate(&self) {
            msg_send![self, invalidate]
        }
        pub unsafe fn addRequestMode(&self, rmode: &NSString) {
            msg_send![self, addRequestMode: rmode]
        }
        pub unsafe fn removeRequestMode(&self, rmode: &NSString) {
            msg_send![self, removeRequestMode: rmode]
        }
        pub unsafe fn requestModes(&self) -> Id<NSArray<NSString>, Shared> {
            msg_send_id![self, requestModes]
        }
        pub unsafe fn registerName(&self, name: Option<&NSString>) -> bool {
            msg_send![self, registerName: name]
        }
        pub unsafe fn registerName_withNameServer(
            &self,
            name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> bool {
            msg_send![self, registerName: name, withNameServer: server]
        }
        pub unsafe fn connectionWithReceivePort_sendPort(
            receivePort: Option<&NSPort>,
            sendPort: Option<&NSPort>,
        ) -> Option<Id<Self, Shared>> {
            msg_send_id![
                Self::class(),
                connectionWithReceivePort: receivePort,
                sendPort: sendPort
            ]
        }
        pub unsafe fn currentConversation() -> Option<Id<Object, Shared>> {
            msg_send_id![Self::class(), currentConversation]
        }
        pub unsafe fn initWithReceivePort_sendPort(
            &self,
            receivePort: Option<&NSPort>,
            sendPort: Option<&NSPort>,
        ) -> Option<Id<Self, Shared>> {
            msg_send_id![self, initWithReceivePort: receivePort, sendPort: sendPort]
        }
        pub unsafe fn sendPort(&self) -> Id<NSPort, Shared> {
            msg_send_id![self, sendPort]
        }
        pub unsafe fn receivePort(&self) -> Id<NSPort, Shared> {
            msg_send_id![self, receivePort]
        }
        pub unsafe fn enableMultipleThreads(&self) {
            msg_send![self, enableMultipleThreads]
        }
        pub unsafe fn multipleThreadsEnabled(&self) -> bool {
            msg_send![self, multipleThreadsEnabled]
        }
        pub unsafe fn addRunLoop(&self, runloop: &NSRunLoop) {
            msg_send![self, addRunLoop: runloop]
        }
        pub unsafe fn removeRunLoop(&self, runloop: &NSRunLoop) {
            msg_send![self, removeRunLoop: runloop]
        }
        pub unsafe fn runInNewThread(&self) {
            msg_send![self, runInNewThread]
        }
        pub unsafe fn remoteObjects(&self) -> Id<NSArray, Shared> {
            msg_send_id![self, remoteObjects]
        }
        pub unsafe fn localObjects(&self) -> Id<NSArray, Shared> {
            msg_send_id![self, localObjects]
        }
        pub unsafe fn dispatchWithComponents(&self, components: &NSArray) {
            msg_send![self, dispatchWithComponents: components]
        }
    }
);
pub type NSConnectionDelegate = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSDistantObjectRequest;
    unsafe impl ClassType for NSDistantObjectRequest {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSDistantObjectRequest {
        pub unsafe fn invocation(&self) -> Id<NSInvocation, Shared> {
            msg_send_id![self, invocation]
        }
        pub unsafe fn connection(&self) -> Id<NSConnection, Shared> {
            msg_send_id![self, connection]
        }
        pub unsafe fn conversation(&self) -> Id<Object, Shared> {
            msg_send_id![self, conversation]
        }
        pub unsafe fn replyWithException(&self, exception: Option<&NSException>) {
            msg_send![self, replyWithException: exception]
        }
    }
);
