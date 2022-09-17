#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSFileWrapper;
    unsafe impl ClassType for NSFileWrapper {
        type Super = NSObject;
    }
);
impl NSFileWrapper {
    pub unsafe fn initWithURL_options_error(
        &self,
        url: &NSURL,
        options: NSFileWrapperReadingOptions,
        outError: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithURL: url, options: options, error: outError]
    }
    pub unsafe fn initDirectoryWithFileWrappers(
        &self,
        childrenByPreferredName: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initDirectoryWithFileWrappers: childrenByPreferredName]
    }
    pub unsafe fn initRegularFileWithContents(&self, contents: &NSData) -> Id<Self, Shared> {
        msg_send_id![self, initRegularFileWithContents: contents]
    }
    pub unsafe fn initSymbolicLinkWithDestinationURL(&self, url: &NSURL) -> Id<Self, Shared> {
        msg_send_id![self, initSymbolicLinkWithDestinationURL: url]
    }
    pub unsafe fn initWithSerializedRepresentation(
        &self,
        serializeRepresentation: &NSData,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithSerializedRepresentation: serializeRepresentation
        ]
    }
    pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: inCoder]
    }
    pub unsafe fn matchesContentsOfURL(&self, url: &NSURL) -> bool {
        msg_send![self, matchesContentsOfURL: url]
    }
    pub unsafe fn readFromURL_options_error(
        &self,
        url: &NSURL,
        options: NSFileWrapperReadingOptions,
        outError: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, readFromURL: url, options: options, error: outError]
    }
    pub unsafe fn writeToURL_options_originalContentsURL_error(
        &self,
        url: &NSURL,
        options: NSFileWrapperWritingOptions,
        originalContentsURL: Option<&NSURL>,
        outError: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            writeToURL: url,
            options: options,
            originalContentsURL: originalContentsURL,
            error: outError
        ]
    }
    pub unsafe fn addFileWrapper(&self, child: &NSFileWrapper) -> Id<NSString, Shared> {
        msg_send_id![self, addFileWrapper: child]
    }
    pub unsafe fn addRegularFileWithContents_preferredFilename(
        &self,
        data: &NSData,
        fileName: &NSString,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            self,
            addRegularFileWithContents: data,
            preferredFilename: fileName
        ]
    }
    pub unsafe fn removeFileWrapper(&self, child: &NSFileWrapper) {
        msg_send![self, removeFileWrapper: child]
    }
    pub unsafe fn keyForFileWrapper(&self, child: &NSFileWrapper) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, keyForFileWrapper: child]
    }
    pub unsafe fn isDirectory(&self) -> bool {
        msg_send![self, isDirectory]
    }
    pub unsafe fn isRegularFile(&self) -> bool {
        msg_send![self, isRegularFile]
    }
    pub unsafe fn isSymbolicLink(&self) -> bool {
        msg_send![self, isSymbolicLink]
    }
    pub unsafe fn preferredFilename(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, preferredFilename]
    }
    pub unsafe fn setPreferredFilename(&self, preferredFilename: Option<&NSString>) {
        msg_send![self, setPreferredFilename: preferredFilename]
    }
    pub unsafe fn filename(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, filename]
    }
    pub unsafe fn setFilename(&self, filename: Option<&NSString>) {
        msg_send![self, setFilename: filename]
    }
    pub unsafe fn fileAttributes(&self) -> TodoGenerics {
        msg_send![self, fileAttributes]
    }
    pub unsafe fn setFileAttributes(&self, fileAttributes: TodoGenerics) {
        msg_send![self, setFileAttributes: fileAttributes]
    }
    pub unsafe fn serializedRepresentation(&self) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, serializedRepresentation]
    }
    pub unsafe fn fileWrappers(&self) -> TodoGenerics {
        msg_send![self, fileWrappers]
    }
    pub unsafe fn regularFileContents(&self) -> Option<Id<NSData, Shared>> {
        msg_send_id![self, regularFileContents]
    }
    pub unsafe fn symbolicLinkDestinationURL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, symbolicLinkDestinationURL]
    }
}
#[doc = "NSDeprecated"]
impl NSFileWrapper {
    pub unsafe fn initWithPath(&self, path: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithPath: path]
    }
    pub unsafe fn initSymbolicLinkWithDestination(&self, path: &NSString) -> Id<Object, Shared> {
        msg_send_id![self, initSymbolicLinkWithDestination: path]
    }
    pub unsafe fn needsToBeUpdatedFromPath(&self, path: &NSString) -> bool {
        msg_send![self, needsToBeUpdatedFromPath: path]
    }
    pub unsafe fn updateFromPath(&self, path: &NSString) -> bool {
        msg_send![self, updateFromPath: path]
    }
    pub unsafe fn writeToFile_atomically_updateFilenames(
        &self,
        path: &NSString,
        atomicFlag: bool,
        updateFilenamesFlag: bool,
    ) -> bool {
        msg_send![
            self,
            writeToFile: path,
            atomically: atomicFlag,
            updateFilenames: updateFilenamesFlag
        ]
    }
    pub unsafe fn addFileWithPath(&self, path: &NSString) -> Id<NSString, Shared> {
        msg_send_id![self, addFileWithPath: path]
    }
    pub unsafe fn addSymbolicLinkWithDestination_preferredFilename(
        &self,
        path: &NSString,
        filename: &NSString,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            self,
            addSymbolicLinkWithDestination: path,
            preferredFilename: filename
        ]
    }
    pub unsafe fn symbolicLinkDestination(&self) -> Id<NSString, Shared> {
        msg_send_id![self, symbolicLinkDestination]
    }
}
