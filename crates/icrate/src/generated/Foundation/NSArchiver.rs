use super::__exported::NSData;
use super::__exported::NSMutableArray;
use super::__exported::NSMutableData;
use super::__exported::NSMutableDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSCoder::*;
use crate::Foundation::generated::NSException::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSArchiver;
    unsafe impl ClassType for NSArchiver {
        type Super = NSCoder;
    }
);
impl NSArchiver {
    pub unsafe fn initForWritingWithMutableData(&self, mdata: &NSMutableData) -> Id<Self, Shared> {
        msg_send_id![self, initForWritingWithMutableData: mdata]
    }
    pub unsafe fn encodeRootObject(&self, rootObject: &Object) {
        msg_send![self, encodeRootObject: rootObject]
    }
    pub unsafe fn encodeConditionalObject(&self, object: Option<&Object>) {
        msg_send![self, encodeConditionalObject: object]
    }
    pub unsafe fn archivedDataWithRootObject(rootObject: &Object) -> Id<NSData, Shared> {
        msg_send_id![Self::class(), archivedDataWithRootObject: rootObject]
    }
    pub unsafe fn archiveRootObject_toFile(rootObject: &Object, path: &NSString) -> bool {
        msg_send![Self::class(), archiveRootObject: rootObject, toFile: path]
    }
    pub unsafe fn encodeClassName_intoClassName(
        &self,
        trueName: &NSString,
        inArchiveName: &NSString,
    ) {
        msg_send![
            self,
            encodeClassName: trueName,
            intoClassName: inArchiveName
        ]
    }
    pub unsafe fn classNameEncodedForTrueClassName(
        &self,
        trueName: &NSString,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, classNameEncodedForTrueClassName: trueName]
    }
    pub unsafe fn replaceObject_withObject(&self, object: &Object, newObject: &Object) {
        msg_send![self, replaceObject: object, withObject: newObject]
    }
    pub unsafe fn archiverData(&self) -> Id<NSMutableData, Shared> {
        msg_send_id![self, archiverData]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSUnarchiver;
    unsafe impl ClassType for NSUnarchiver {
        type Super = NSCoder;
    }
);
impl NSUnarchiver {
    pub unsafe fn initForReadingWithData(&self, data: &NSData) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initForReadingWithData: data]
    }
    pub unsafe fn setObjectZone(&self, zone: *mut NSZone) {
        msg_send![self, setObjectZone: zone]
    }
    pub unsafe fn objectZone(&self) -> *mut NSZone {
        msg_send![self, objectZone]
    }
    pub unsafe fn unarchiveObjectWithData(data: &NSData) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), unarchiveObjectWithData: data]
    }
    pub unsafe fn unarchiveObjectWithFile(path: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), unarchiveObjectWithFile: path]
    }
    pub unsafe fn decodeClassName_asClassName(inArchiveName: &NSString, trueName: &NSString) {
        msg_send![
            Self::class(),
            decodeClassName: inArchiveName,
            asClassName: trueName
        ]
    }
    pub unsafe fn decodeClassName_asClassName(
        &self,
        inArchiveName: &NSString,
        trueName: &NSString,
    ) {
        msg_send![self, decodeClassName: inArchiveName, asClassName: trueName]
    }
    pub unsafe fn classNameDecodedForArchiveClassName(
        inArchiveName: &NSString,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            Self::class(),
            classNameDecodedForArchiveClassName: inArchiveName
        ]
    }
    pub unsafe fn classNameDecodedForArchiveClassName(
        &self,
        inArchiveName: &NSString,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, classNameDecodedForArchiveClassName: inArchiveName]
    }
    pub unsafe fn replaceObject_withObject(&self, object: &Object, newObject: &Object) {
        msg_send![self, replaceObject: object, withObject: newObject]
    }
    pub unsafe fn isAtEnd(&self) -> bool {
        msg_send![self, isAtEnd]
    }
    pub unsafe fn systemVersion(&self) -> c_uint {
        msg_send![self, systemVersion]
    }
}
#[doc = "NSArchiverCallback"]
impl NSObject {
    pub unsafe fn replacementObjectForArchiver(
        &self,
        archiver: &NSArchiver,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, replacementObjectForArchiver: archiver]
    }
    pub unsafe fn classForArchiver(&self) -> Option<&Class> {
        msg_send![self, classForArchiver]
    }
}
