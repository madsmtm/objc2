use super::__exported::NSData;
use super::__exported::NSError;
use super::__exported::NSString;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSException::*;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
use crate::Foundation::generated::NSRunLoop::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFileHandle;
    unsafe impl ClassType for NSFileHandle {
        type Super = NSObject;
    }
);
impl NSFileHandle {
    pub unsafe fn availableData(&self) -> Id<NSData, Shared> {
        msg_send_id![self, availableData]
    }
    pub unsafe fn initWithFileDescriptor_closeOnDealloc(
        &self,
        fd: c_int,
        closeopt: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithFileDescriptor: fd, closeOnDealloc: closeopt]
    }
    pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: coder]
    }
    pub unsafe fn readDataToEndOfFileAndReturnError(
        &self,
    ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>> {
        msg_send_id![self, readDataToEndOfFileAndReturnError: _]
    }
    pub unsafe fn readDataUpToLength_error(
        &self,
        length: NSUInteger,
    ) -> Result<Id<NSData, Shared>, Id<NSError, Shared>> {
        msg_send_id![self, readDataUpToLength: length, error: _]
    }
    pub unsafe fn writeData_error(&self, data: &NSData) -> Result<(), Id<NSError, Shared>> {
        msg_send![self, writeData: data, error: _]
    }
    pub unsafe fn getOffset_error(
        &self,
        offsetInFile: NonNull<c_ulonglong>,
    ) -> Result<(), Id<NSError, Shared>> {
        msg_send![self, getOffset: offsetInFile, error: _]
    }
    pub unsafe fn seekToEndReturningOffset_error(
        &self,
        offsetInFile: *mut c_ulonglong,
    ) -> Result<(), Id<NSError, Shared>> {
        msg_send![self, seekToEndReturningOffset: offsetInFile, error: _]
    }
    pub unsafe fn seekToOffset_error(
        &self,
        offset: c_ulonglong,
    ) -> Result<(), Id<NSError, Shared>> {
        msg_send![self, seekToOffset: offset, error: _]
    }
    pub unsafe fn truncateAtOffset_error(
        &self,
        offset: c_ulonglong,
    ) -> Result<(), Id<NSError, Shared>> {
        msg_send![self, truncateAtOffset: offset, error: _]
    }
    pub unsafe fn synchronizeAndReturnError(&self) -> Result<(), Id<NSError, Shared>> {
        msg_send![self, synchronizeAndReturnError: _]
    }
    pub unsafe fn closeAndReturnError(&self) -> Result<(), Id<NSError, Shared>> {
        msg_send![self, closeAndReturnError: _]
    }
}
#[doc = "NSFileHandleCreation"]
impl NSFileHandle {
    pub unsafe fn fileHandleWithStandardInput() -> Id<NSFileHandle, Shared> {
        msg_send_id![Self::class(), fileHandleWithStandardInput]
    }
    pub unsafe fn fileHandleWithStandardOutput() -> Id<NSFileHandle, Shared> {
        msg_send_id![Self::class(), fileHandleWithStandardOutput]
    }
    pub unsafe fn fileHandleWithStandardError() -> Id<NSFileHandle, Shared> {
        msg_send_id![Self::class(), fileHandleWithStandardError]
    }
    pub unsafe fn fileHandleWithNullDevice() -> Id<NSFileHandle, Shared> {
        msg_send_id![Self::class(), fileHandleWithNullDevice]
    }
    pub unsafe fn fileHandleForReadingAtPath(path: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), fileHandleForReadingAtPath: path]
    }
    pub unsafe fn fileHandleForWritingAtPath(path: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), fileHandleForWritingAtPath: path]
    }
    pub unsafe fn fileHandleForUpdatingAtPath(path: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), fileHandleForUpdatingAtPath: path]
    }
    pub unsafe fn fileHandleForReadingFromURL_error(
        url: &NSURL,
    ) -> Result<Id<Self, Shared>, Id<NSError, Shared>> {
        msg_send_id![Self::class(), fileHandleForReadingFromURL: url, error: _]
    }
    pub unsafe fn fileHandleForWritingToURL_error(
        url: &NSURL,
    ) -> Result<Id<Self, Shared>, Id<NSError, Shared>> {
        msg_send_id![Self::class(), fileHandleForWritingToURL: url, error: _]
    }
    pub unsafe fn fileHandleForUpdatingURL_error(
        url: &NSURL,
    ) -> Result<Id<Self, Shared>, Id<NSError, Shared>> {
        msg_send_id![Self::class(), fileHandleForUpdatingURL: url, error: _]
    }
}
#[doc = "NSFileHandleAsynchronousAccess"]
impl NSFileHandle {
    pub unsafe fn readInBackgroundAndNotifyForModes(&self, modes: Option<&NSArray<NSRunLoopMode>>) {
        msg_send![self, readInBackgroundAndNotifyForModes: modes]
    }
    pub unsafe fn readInBackgroundAndNotify(&self) {
        msg_send![self, readInBackgroundAndNotify]
    }
    pub unsafe fn readToEndOfFileInBackgroundAndNotifyForModes(
        &self,
        modes: Option<&NSArray<NSRunLoopMode>>,
    ) {
        msg_send![self, readToEndOfFileInBackgroundAndNotifyForModes: modes]
    }
    pub unsafe fn readToEndOfFileInBackgroundAndNotify(&self) {
        msg_send![self, readToEndOfFileInBackgroundAndNotify]
    }
    pub unsafe fn acceptConnectionInBackgroundAndNotifyForModes(
        &self,
        modes: Option<&NSArray<NSRunLoopMode>>,
    ) {
        msg_send![self, acceptConnectionInBackgroundAndNotifyForModes: modes]
    }
    pub unsafe fn acceptConnectionInBackgroundAndNotify(&self) {
        msg_send![self, acceptConnectionInBackgroundAndNotify]
    }
    pub unsafe fn waitForDataInBackgroundAndNotifyForModes(
        &self,
        modes: Option<&NSArray<NSRunLoopMode>>,
    ) {
        msg_send![self, waitForDataInBackgroundAndNotifyForModes: modes]
    }
    pub unsafe fn waitForDataInBackgroundAndNotify(&self) {
        msg_send![self, waitForDataInBackgroundAndNotify]
    }
    pub unsafe fn readabilityHandler(&self) -> TodoBlock {
        msg_send![self, readabilityHandler]
    }
    pub unsafe fn setReadabilityHandler(&self, readabilityHandler: TodoBlock) {
        msg_send![self, setReadabilityHandler: readabilityHandler]
    }
    pub unsafe fn writeabilityHandler(&self) -> TodoBlock {
        msg_send![self, writeabilityHandler]
    }
    pub unsafe fn setWriteabilityHandler(&self, writeabilityHandler: TodoBlock) {
        msg_send![self, setWriteabilityHandler: writeabilityHandler]
    }
}
#[doc = "NSFileHandlePlatformSpecific"]
impl NSFileHandle {
    pub unsafe fn initWithFileDescriptor(&self, fd: c_int) -> Id<Self, Shared> {
        msg_send_id![self, initWithFileDescriptor: fd]
    }
    pub unsafe fn fileDescriptor(&self) -> c_int {
        msg_send![self, fileDescriptor]
    }
}
impl NSFileHandle {
    pub unsafe fn readDataToEndOfFile(&self) -> Id<NSData, Shared> {
        msg_send_id![self, readDataToEndOfFile]
    }
    pub unsafe fn readDataOfLength(&self, length: NSUInteger) -> Id<NSData, Shared> {
        msg_send_id![self, readDataOfLength: length]
    }
    pub unsafe fn writeData(&self, data: &NSData) {
        msg_send![self, writeData: data]
    }
    pub unsafe fn offsetInFile(&self) -> c_ulonglong {
        msg_send![self, offsetInFile]
    }
    pub unsafe fn seekToEndOfFile(&self) -> c_ulonglong {
        msg_send![self, seekToEndOfFile]
    }
    pub unsafe fn seekToFileOffset(&self, offset: c_ulonglong) {
        msg_send![self, seekToFileOffset: offset]
    }
    pub unsafe fn truncateFileAtOffset(&self, offset: c_ulonglong) {
        msg_send![self, truncateFileAtOffset: offset]
    }
    pub unsafe fn synchronizeFile(&self) {
        msg_send![self, synchronizeFile]
    }
    pub unsafe fn closeFile(&self) {
        msg_send![self, closeFile]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSPipe;
    unsafe impl ClassType for NSPipe {
        type Super = NSObject;
    }
);
impl NSPipe {
    pub unsafe fn fileHandleForReading(&self) -> Id<NSFileHandle, Shared> {
        msg_send_id![self, fileHandleForReading]
    }
    pub unsafe fn fileHandleForWriting(&self) -> Id<NSFileHandle, Shared> {
        msg_send_id![self, fileHandleForWriting]
    }
    pub unsafe fn pipe() -> Id<NSPipe, Shared> {
        msg_send_id![Self::class(), pipe]
    }
}
