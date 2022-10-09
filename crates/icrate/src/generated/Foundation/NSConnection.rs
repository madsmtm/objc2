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
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSConnection;
    unsafe impl ClassType for NSConnection {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSConnection {
        #[method_id(statistics)]
        pub unsafe fn statistics(&self) -> Id<NSDictionary<NSString, NSNumber>, Shared>;
        #[method_id(allConnections)]
        pub unsafe fn allConnections() -> Id<NSArray<NSConnection>, Shared>;
        #[method_id(defaultConnection)]
        pub unsafe fn defaultConnection() -> Id<NSConnection, Shared>;
        #[method_id(connectionWithRegisteredName:host:)]
        pub unsafe fn connectionWithRegisteredName_host(
            name: &NSString,
            hostName: Option<&NSString>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(connectionWithRegisteredName:host:usingNameServer:)]
        pub unsafe fn connectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            hostName: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(rootProxyForConnectionWithRegisteredName:host:)]
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host(
            name: &NSString,
            hostName: Option<&NSString>,
        ) -> Option<Id<NSDistantObject, Shared>>;
        #[method_id(rootProxyForConnectionWithRegisteredName:host:usingNameServer:)]
        pub unsafe fn rootProxyForConnectionWithRegisteredName_host_usingNameServer(
            name: &NSString,
            hostName: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> Option<Id<NSDistantObject, Shared>>;
        #[method_id(serviceConnectionWithName:rootObject:usingNameServer:)]
        pub unsafe fn serviceConnectionWithName_rootObject_usingNameServer(
            name: &NSString,
            root: &Object,
            server: &NSPortNameServer,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(serviceConnectionWithName:rootObject:)]
        pub unsafe fn serviceConnectionWithName_rootObject(
            name: &NSString,
            root: &Object,
        ) -> Option<Id<Self, Shared>>;
        #[method(requestTimeout)]
        pub unsafe fn requestTimeout(&self) -> NSTimeInterval;
        #[method(setRequestTimeout:)]
        pub unsafe fn setRequestTimeout(&self, requestTimeout: NSTimeInterval);
        #[method(replyTimeout)]
        pub unsafe fn replyTimeout(&self) -> NSTimeInterval;
        #[method(setReplyTimeout:)]
        pub unsafe fn setReplyTimeout(&self, replyTimeout: NSTimeInterval);
        #[method_id(rootObject)]
        pub unsafe fn rootObject(&self) -> Option<Id<Object, Shared>>;
        #[method(setRootObject:)]
        pub unsafe fn setRootObject(&self, rootObject: Option<&Object>);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSConnectionDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSConnectionDelegate>);
        #[method(independentConversationQueueing)]
        pub unsafe fn independentConversationQueueing(&self) -> bool;
        #[method(setIndependentConversationQueueing:)]
        pub unsafe fn setIndependentConversationQueueing(
            &self,
            independentConversationQueueing: bool,
        );
        #[method(isValid)]
        pub unsafe fn isValid(&self) -> bool;
        #[method_id(rootProxy)]
        pub unsafe fn rootProxy(&self) -> Id<NSDistantObject, Shared>;
        #[method(invalidate)]
        pub unsafe fn invalidate(&self);
        #[method(addRequestMode:)]
        pub unsafe fn addRequestMode(&self, rmode: &NSString);
        #[method(removeRequestMode:)]
        pub unsafe fn removeRequestMode(&self, rmode: &NSString);
        #[method_id(requestModes)]
        pub unsafe fn requestModes(&self) -> Id<NSArray<NSString>, Shared>;
        #[method(registerName:)]
        pub unsafe fn registerName(&self, name: Option<&NSString>) -> bool;
        #[method(registerName:withNameServer:)]
        pub unsafe fn registerName_withNameServer(
            &self,
            name: Option<&NSString>,
            server: &NSPortNameServer,
        ) -> bool;
        #[method_id(connectionWithReceivePort:sendPort:)]
        pub unsafe fn connectionWithReceivePort_sendPort(
            receivePort: Option<&NSPort>,
            sendPort: Option<&NSPort>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(currentConversation)]
        pub unsafe fn currentConversation() -> Option<Id<Object, Shared>>;
        #[method_id(initWithReceivePort:sendPort:)]
        pub unsafe fn initWithReceivePort_sendPort(
            &self,
            receivePort: Option<&NSPort>,
            sendPort: Option<&NSPort>,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(sendPort)]
        pub unsafe fn sendPort(&self) -> Id<NSPort, Shared>;
        #[method_id(receivePort)]
        pub unsafe fn receivePort(&self) -> Id<NSPort, Shared>;
        #[method(enableMultipleThreads)]
        pub unsafe fn enableMultipleThreads(&self);
        #[method(multipleThreadsEnabled)]
        pub unsafe fn multipleThreadsEnabled(&self) -> bool;
        #[method(addRunLoop:)]
        pub unsafe fn addRunLoop(&self, runloop: &NSRunLoop);
        #[method(removeRunLoop:)]
        pub unsafe fn removeRunLoop(&self, runloop: &NSRunLoop);
        #[method(runInNewThread)]
        pub unsafe fn runInNewThread(&self);
        #[method_id(remoteObjects)]
        pub unsafe fn remoteObjects(&self) -> Id<NSArray, Shared>;
        #[method_id(localObjects)]
        pub unsafe fn localObjects(&self) -> Id<NSArray, Shared>;
        #[method(dispatchWithComponents:)]
        pub unsafe fn dispatchWithComponents(&self, components: &NSArray);
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
        #[method_id(invocation)]
        pub unsafe fn invocation(&self) -> Id<NSInvocation, Shared>;
        #[method_id(connection)]
        pub unsafe fn connection(&self) -> Id<NSConnection, Shared>;
        #[method_id(conversation)]
        pub unsafe fn conversation(&self) -> Id<Object, Shared>;
        #[method(replyWithException:)]
        pub unsafe fn replyWithException(&self, exception: Option<&NSException>);
    }
);
