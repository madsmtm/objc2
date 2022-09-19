use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSError;
use super::__exported::NSHost;
use super::__exported::NSRunLoop;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::Foundation::generated::NSError::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSStreamPropertyKey = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSStream;
    unsafe impl ClassType for NSStream {
        type Super = NSObject;
    }
);
impl NSStream {
    pub unsafe fn open(&self) {
        msg_send![self, open]
    }
    pub unsafe fn close(&self) {
        msg_send![self, close]
    }
    pub unsafe fn propertyForKey(&self, key: &NSStreamPropertyKey) -> Option<Id<Object, Shared>> {
        msg_send_id![self, propertyForKey: key]
    }
    pub unsafe fn setProperty_forKey(
        &self,
        property: Option<&Object>,
        key: &NSStreamPropertyKey,
    ) -> bool {
        msg_send![self, setProperty: property, forKey: key]
    }
    pub unsafe fn scheduleInRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode) {
        msg_send![self, scheduleInRunLoop: aRunLoop, forMode: mode]
    }
    pub unsafe fn removeFromRunLoop_forMode(&self, aRunLoop: &NSRunLoop, mode: &NSRunLoopMode) {
        msg_send![self, removeFromRunLoop: aRunLoop, forMode: mode]
    }
    pub unsafe fn delegate(&self) -> Option<Id<id, Shared>> {
        msg_send_id![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: Option<&id>) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn streamStatus(&self) -> NSStreamStatus {
        msg_send![self, streamStatus]
    }
    pub unsafe fn streamError(&self) -> Option<Id<NSError, Shared>> {
        msg_send_id![self, streamError]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSInputStream;
    unsafe impl ClassType for NSInputStream {
        type Super = NSStream;
    }
);
impl NSInputStream {
    pub unsafe fn read_maxLength(&self, buffer: NonNull<uint8_t>, len: NSUInteger) -> NSInteger {
        msg_send![self, read: buffer, maxLength: len]
    }
    pub unsafe fn getBuffer_length(
        &self,
        buffer: NonNull<*mut uint8_t>,
        len: NonNull<NSUInteger>,
    ) -> bool {
        msg_send![self, getBuffer: buffer, length: len]
    }
    pub unsafe fn initWithData(&self, data: &NSData) -> Id<Self, Shared> {
        msg_send_id![self, initWithData: data]
    }
    pub unsafe fn initWithURL(&self, url: &NSURL) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithURL: url]
    }
    pub unsafe fn hasBytesAvailable(&self) -> bool {
        msg_send![self, hasBytesAvailable]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSOutputStream;
    unsafe impl ClassType for NSOutputStream {
        type Super = NSStream;
    }
);
impl NSOutputStream {
    pub unsafe fn write_maxLength(&self, buffer: NonNull<uint8_t>, len: NSUInteger) -> NSInteger {
        msg_send![self, write: buffer, maxLength: len]
    }
    pub unsafe fn initToMemory(&self) -> Id<Self, Shared> {
        msg_send_id![self, initToMemory]
    }
    pub unsafe fn initToBuffer_capacity(
        &self,
        buffer: NonNull<uint8_t>,
        capacity: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initToBuffer: buffer, capacity: capacity]
    }
    pub unsafe fn initWithURL_append(
        &self,
        url: &NSURL,
        shouldAppend: bool,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithURL: url, append: shouldAppend]
    }
    pub unsafe fn hasSpaceAvailable(&self) -> bool {
        msg_send![self, hasSpaceAvailable]
    }
}
#[doc = "NSSocketStreamCreationExtensions"]
impl NSStream {
    pub unsafe fn getStreamsToHostWithName_port_inputStream_outputStream(
        hostname: &NSString,
        port: NSInteger,
        inputStream: *mut *mut NSInputStream,
        outputStream: *mut *mut NSOutputStream,
    ) {
        msg_send![
            Self::class(),
            getStreamsToHostWithName: hostname,
            port: port,
            inputStream: inputStream,
            outputStream: outputStream
        ]
    }
    pub unsafe fn getStreamsToHost_port_inputStream_outputStream(
        host: &NSHost,
        port: NSInteger,
        inputStream: *mut *mut NSInputStream,
        outputStream: *mut *mut NSOutputStream,
    ) {
        msg_send![
            Self::class(),
            getStreamsToHost: host,
            port: port,
            inputStream: inputStream,
            outputStream: outputStream
        ]
    }
}
#[doc = "NSStreamBoundPairCreationExtensions"]
impl NSStream {
    pub unsafe fn getBoundStreamsWithBufferSize_inputStream_outputStream(
        bufferSize: NSUInteger,
        inputStream: *mut *mut NSInputStream,
        outputStream: *mut *mut NSOutputStream,
    ) {
        msg_send![
            Self::class(),
            getBoundStreamsWithBufferSize: bufferSize,
            inputStream: inputStream,
            outputStream: outputStream
        ]
    }
}
#[doc = "NSInputStreamExtensions"]
impl NSInputStream {
    pub unsafe fn initWithFileAtPath(&self, path: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithFileAtPath: path]
    }
    pub unsafe fn inputStreamWithData(data: &NSData) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), inputStreamWithData: data]
    }
    pub unsafe fn inputStreamWithFileAtPath(path: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), inputStreamWithFileAtPath: path]
    }
    pub unsafe fn inputStreamWithURL(url: &NSURL) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), inputStreamWithURL: url]
    }
}
#[doc = "NSOutputStreamExtensions"]
impl NSOutputStream {
    pub unsafe fn initToFileAtPath_append(
        &self,
        path: &NSString,
        shouldAppend: bool,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initToFileAtPath: path, append: shouldAppend]
    }
    pub unsafe fn outputStreamToMemory() -> Id<Self, Shared> {
        msg_send_id![Self::class(), outputStreamToMemory]
    }
    pub unsafe fn outputStreamToBuffer_capacity(
        buffer: NonNull<uint8_t>,
        capacity: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            outputStreamToBuffer: buffer,
            capacity: capacity
        ]
    }
    pub unsafe fn outputStreamToFileAtPath_append(
        path: &NSString,
        shouldAppend: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            outputStreamToFileAtPath: path,
            append: shouldAppend
        ]
    }
    pub unsafe fn outputStreamWithURL_append(
        url: &NSURL,
        shouldAppend: bool,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            Self::class(),
            outputStreamWithURL: url,
            append: shouldAppend
        ]
    }
}
pub type NSStreamDelegate = NSObject;
pub type NSStreamSocketSecurityLevel = NSString;
pub type NSStreamSOCKSProxyConfiguration = NSString;
pub type NSStreamSOCKSProxyVersion = NSString;
pub type NSStreamNetworkServiceTypeValue = NSString;
