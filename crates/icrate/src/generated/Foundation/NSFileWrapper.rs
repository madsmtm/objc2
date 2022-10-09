use super::__exported::NSData;
use super::__exported::NSDictionary;
use super::__exported::NSError;
use super::__exported::NSString;
use super::__exported::NSURL;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFileWrapper;
    unsafe impl ClassType for NSFileWrapper {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFileWrapper {
        #[method_id(initWithURL:options:error:)]
        pub unsafe fn initWithURL_options_error(
            &self,
            url: &NSURL,
            options: NSFileWrapperReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
        #[method_id(initDirectoryWithFileWrappers:)]
        pub unsafe fn initDirectoryWithFileWrappers(
            &self,
            childrenByPreferredName: &NSDictionary<NSString, NSFileWrapper>,
        ) -> Id<Self, Shared>;
        #[method_id(initRegularFileWithContents:)]
        pub unsafe fn initRegularFileWithContents(&self, contents: &NSData) -> Id<Self, Shared>;
        #[method_id(initSymbolicLinkWithDestinationURL:)]
        pub unsafe fn initSymbolicLinkWithDestinationURL(&self, url: &NSURL) -> Id<Self, Shared>;
        #[method_id(initWithSerializedRepresentation:)]
        pub unsafe fn initWithSerializedRepresentation(
            &self,
            serializeRepresentation: &NSData,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method(isDirectory)]
        pub unsafe fn isDirectory(&self) -> bool;
        #[method(isRegularFile)]
        pub unsafe fn isRegularFile(&self) -> bool;
        #[method(isSymbolicLink)]
        pub unsafe fn isSymbolicLink(&self) -> bool;
        #[method_id(preferredFilename)]
        pub unsafe fn preferredFilename(&self) -> Option<Id<NSString, Shared>>;
        #[method(setPreferredFilename:)]
        pub unsafe fn setPreferredFilename(&self, preferredFilename: Option<&NSString>);
        #[method_id(filename)]
        pub unsafe fn filename(&self) -> Option<Id<NSString, Shared>>;
        #[method(setFilename:)]
        pub unsafe fn setFilename(&self, filename: Option<&NSString>);
        #[method_id(fileAttributes)]
        pub unsafe fn fileAttributes(&self) -> Id<NSDictionary<NSString, Object>, Shared>;
        #[method(setFileAttributes:)]
        pub unsafe fn setFileAttributes(&self, fileAttributes: &NSDictionary<NSString, Object>);
        #[method(matchesContentsOfURL:)]
        pub unsafe fn matchesContentsOfURL(&self, url: &NSURL) -> bool;
        #[method(readFromURL:options:error:)]
        pub unsafe fn readFromURL_options_error(
            &self,
            url: &NSURL,
            options: NSFileWrapperReadingOptions,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method(writeToURL:options:originalContentsURL:error:)]
        pub unsafe fn writeToURL_options_originalContentsURL_error(
            &self,
            url: &NSURL,
            options: NSFileWrapperWritingOptions,
            originalContentsURL: Option<&NSURL>,
        ) -> Result<(), Id<NSError, Shared>>;
        #[method_id(serializedRepresentation)]
        pub unsafe fn serializedRepresentation(&self) -> Option<Id<NSData, Shared>>;
        #[method_id(addFileWrapper:)]
        pub unsafe fn addFileWrapper(&self, child: &NSFileWrapper) -> Id<NSString, Shared>;
        #[method_id(addRegularFileWithContents:preferredFilename:)]
        pub unsafe fn addRegularFileWithContents_preferredFilename(
            &self,
            data: &NSData,
            fileName: &NSString,
        ) -> Id<NSString, Shared>;
        #[method(removeFileWrapper:)]
        pub unsafe fn removeFileWrapper(&self, child: &NSFileWrapper);
        #[method_id(fileWrappers)]
        pub unsafe fn fileWrappers(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSFileWrapper>, Shared>>;
        #[method_id(keyForFileWrapper:)]
        pub unsafe fn keyForFileWrapper(
            &self,
            child: &NSFileWrapper,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(regularFileContents)]
        pub unsafe fn regularFileContents(&self) -> Option<Id<NSData, Shared>>;
        #[method_id(symbolicLinkDestinationURL)]
        pub unsafe fn symbolicLinkDestinationURL(&self) -> Option<Id<NSURL, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSFileWrapper {
        #[method_id(initWithPath:)]
        pub unsafe fn initWithPath(&self, path: &NSString) -> Option<Id<Object, Shared>>;
        #[method_id(initSymbolicLinkWithDestination:)]
        pub unsafe fn initSymbolicLinkWithDestination(&self, path: &NSString)
            -> Id<Object, Shared>;
        #[method(needsToBeUpdatedFromPath:)]
        pub unsafe fn needsToBeUpdatedFromPath(&self, path: &NSString) -> bool;
        #[method(updateFromPath:)]
        pub unsafe fn updateFromPath(&self, path: &NSString) -> bool;
        #[method(writeToFile:atomically:updateFilenames:)]
        pub unsafe fn writeToFile_atomically_updateFilenames(
            &self,
            path: &NSString,
            atomicFlag: bool,
            updateFilenamesFlag: bool,
        ) -> bool;
        #[method_id(addFileWithPath:)]
        pub unsafe fn addFileWithPath(&self, path: &NSString) -> Id<NSString, Shared>;
        #[method_id(addSymbolicLinkWithDestination:preferredFilename:)]
        pub unsafe fn addSymbolicLinkWithDestination_preferredFilename(
            &self,
            path: &NSString,
            filename: &NSString,
        ) -> Id<NSString, Shared>;
        #[method_id(symbolicLinkDestination)]
        pub unsafe fn symbolicLinkDestination(&self) -> Id<NSString, Shared>;
    }
);
