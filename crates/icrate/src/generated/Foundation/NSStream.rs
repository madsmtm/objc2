#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSStreamPropertyKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSStream;
    unsafe impl ClassType for NSStream {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSStream {
        #[method(open)]
        pub unsafe fn open(&self);
        #[method(close)]
        pub unsafe fn close(&self);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSStreamDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSStreamDelegate>);
        #[method_id(propertyForKey:)]
        pub unsafe fn propertyForKey(
            &self,
            key: &NSStreamPropertyKey,
        ) -> Option<Id<Object, Shared>>;
        #[method(setProperty:forKey:)]
        pub unsafe fn setProperty_forKey(
            &self,
            property: Option<&Object>,
            key: &NSStreamPropertyKey,
        ) -> bool;
        #[method(scheduleInRunLoop:forMode:)]
        pub unsafe fn scheduleInRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);
        #[method(removeFromRunLoop:forMode:)]
        pub unsafe fn removeFromRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode);
        #[method(streamStatus)]
        pub unsafe fn streamStatus(&self) -> NSStreamStatus;
        #[method_id(streamError)]
        pub unsafe fn streamError(&self) -> Option<Id<NSError, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSInputStream;
    unsafe impl ClassType for NSInputStream {
        type Super = NSStream;
    }
);
extern_methods!(
    unsafe impl NSInputStream {
        #[method(read:maxLength:)]
        pub unsafe fn read_maxLength(&self, buffer: NonNull<u8>, len: NSUInteger) -> NSInteger;
        #[method(getBuffer:length:)]
        pub unsafe fn getBuffer_length(
            &self,
            buffer: NonNull<*mut u8>,
            len: NonNull<NSUInteger>,
        ) -> bool;
        #[method(hasBytesAvailable)]
        pub unsafe fn hasBytesAvailable(&self) -> bool;
        #[method_id(initWithData:)]
        pub unsafe fn initWithData(&self, data: &NSData) -> Id<Self, Shared>;
        #[method_id(initWithURL:)]
        pub unsafe fn initWithURL(&self, url: &NSURL) -> Option<Id<Self, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSOutputStream;
    unsafe impl ClassType for NSOutputStream {
        type Super = NSStream;
    }
);
extern_methods!(
    unsafe impl NSOutputStream {
        #[method(write:maxLength:)]
        pub unsafe fn write_maxLength(&self, buffer: NonNull<u8>, len: NSUInteger) -> NSInteger;
        #[method(hasSpaceAvailable)]
        pub unsafe fn hasSpaceAvailable(&self) -> bool;
        #[method_id(initToMemory)]
        pub unsafe fn initToMemory(&self) -> Id<Self, Shared>;
        #[method_id(initToBuffer:capacity:)]
        pub unsafe fn initToBuffer_capacity(
            &self,
            buffer: NonNull<u8>,
            capacity: NSUInteger,
        ) -> Id<Self, Shared>;
        #[method_id(initWithURL:append:)]
        pub unsafe fn initWithURL_append(
            &self,
            url: &NSURL,
            shouldAppend: bool,
        ) -> Option<Id<Self, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSSocketStreamCreationExtensions"]
    unsafe impl NSStream {
        #[method(getStreamsToHostWithName:port:inputStream:outputStream:)]
        pub unsafe fn getStreamsToHostWithName_port_inputStream_outputStream(
            hostname: &NSString,
            port: NSInteger,
            inputStream: Option<&mut Option<Id<NSInputStream, Shared>>>,
            outputStream: Option<&mut Option<Id<NSOutputStream, Shared>>>,
        );
        #[method(getStreamsToHost:port:inputStream:outputStream:)]
        pub unsafe fn getStreamsToHost_port_inputStream_outputStream(
            host: &NSHost,
            port: NSInteger,
            inputStream: Option<&mut Option<Id<NSInputStream, Shared>>>,
            outputStream: Option<&mut Option<Id<NSOutputStream, Shared>>>,
        );
    }
);
extern_methods!(
    #[doc = "NSStreamBoundPairCreationExtensions"]
    unsafe impl NSStream {
        #[method(getBoundStreamsWithBufferSize:inputStream:outputStream:)]
        pub unsafe fn getBoundStreamsWithBufferSize_inputStream_outputStream(
            bufferSize: NSUInteger,
            inputStream: Option<&mut Option<Id<NSInputStream, Shared>>>,
            outputStream: Option<&mut Option<Id<NSOutputStream, Shared>>>,
        );
    }
);
extern_methods!(
    #[doc = "NSInputStreamExtensions"]
    unsafe impl NSInputStream {
        #[method_id(initWithFileAtPath:)]
        pub unsafe fn initWithFileAtPath(&self, path: &NSString) -> Option<Id<Self, Shared>>;
        #[method_id(inputStreamWithData:)]
        pub unsafe fn inputStreamWithData(data: &NSData) -> Option<Id<Self, Shared>>;
        #[method_id(inputStreamWithFileAtPath:)]
        pub unsafe fn inputStreamWithFileAtPath(path: &NSString) -> Option<Id<Self, Shared>>;
        #[method_id(inputStreamWithURL:)]
        pub unsafe fn inputStreamWithURL(url: &NSURL) -> Option<Id<Self, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSOutputStreamExtensions"]
    unsafe impl NSOutputStream {
        #[method_id(initToFileAtPath:append:)]
        pub unsafe fn initToFileAtPath_append(
            &self,
            path: &NSString,
            shouldAppend: bool,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(outputStreamToMemory)]
        pub unsafe fn outputStreamToMemory() -> Id<Self, Shared>;
        #[method_id(outputStreamToBuffer:capacity:)]
        pub unsafe fn outputStreamToBuffer_capacity(
            buffer: NonNull<u8>,
            capacity: NSUInteger,
        ) -> Id<Self, Shared>;
        #[method_id(outputStreamToFileAtPath:append:)]
        pub unsafe fn outputStreamToFileAtPath_append(
            path: &NSString,
            shouldAppend: bool,
        ) -> Id<Self, Shared>;
        #[method_id(outputStreamWithURL:append:)]
        pub unsafe fn outputStreamWithURL_append(
            url: &NSURL,
            shouldAppend: bool,
        ) -> Option<Id<Self, Shared>>;
    }
);
pub type NSStreamDelegate = NSObject;
pub type NSStreamSocketSecurityLevel = NSString;
pub type NSStreamSOCKSProxyConfiguration = NSString;
pub type NSStreamSOCKSProxyVersion = NSString;
pub type NSStreamNetworkServiceTypeValue = NSString;
