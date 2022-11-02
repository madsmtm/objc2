//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSFileWrapperReadingOptions = NSUInteger;
pub const NSFileWrapperReadingImmediate: NSFileWrapperReadingOptions = 1 << 0;
pub const NSFileWrapperReadingWithoutMapping: NSFileWrapperReadingOptions = 1 << 1;

pub type NSFileWrapperWritingOptions = NSUInteger;
pub const NSFileWrapperWritingAtomic: NSFileWrapperWritingOptions = 1 << 0;
pub const NSFileWrapperWritingWithNameUpdating: NSFileWrapperWritingOptions = 1 << 1;

extern_class!(
    #[derive(Debug)]
    pub struct NSFileWrapper;

    unsafe impl ClassType for NSFileWrapper {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSFileWrapper {
        #[method_id(@__retain_semantics Init initWithURL:options:error:)]
        pub unsafe fn initWithURL_options_error(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            options: NSFileWrapperReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(@__retain_semantics Init initDirectoryWithFileWrappers:)]
        pub unsafe fn initDirectoryWithFileWrappers(
            this: Option<Allocated<Self>>,
            childrenByPreferredName: &NSDictionary<NSString, NSFileWrapper>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initRegularFileWithContents:)]
        pub unsafe fn initRegularFileWithContents(
            this: Option<Allocated<Self>>,
            contents: &NSData,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initSymbolicLinkWithDestinationURL:)]
        pub unsafe fn initSymbolicLinkWithDestinationURL(
            this: Option<Allocated<Self>>,
            url: &NSURL,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithSerializedRepresentation:)]
        pub unsafe fn initWithSerializedRepresentation(
            this: Option<Allocated<Self>>,
            serializeRepresentation: &NSData,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            inCoder: &NSCoder,
        ) -> Option<Id<Self, Shared>>;

        #[method(isDirectory)]
        pub unsafe fn isDirectory(&self) -> bool;

        #[method(isRegularFile)]
        pub unsafe fn isRegularFile(&self) -> bool;

        #[method(isSymbolicLink)]
        pub unsafe fn isSymbolicLink(&self) -> bool;

        #[method_id(@__retain_semantics Other preferredFilename)]
        pub unsafe fn preferredFilename(&self) -> Option<Id<NSString, Shared>>;

        #[method(setPreferredFilename:)]
        pub unsafe fn setPreferredFilename(&self, preferredFilename: Option<&NSString>);

        #[method_id(@__retain_semantics Other filename)]
        pub unsafe fn filename(&self) -> Option<Id<NSString, Shared>>;

        #[method(setFilename:)]
        pub unsafe fn setFilename(&self, filename: Option<&NSString>);

        #[method_id(@__retain_semantics Other fileAttributes)]
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

        #[method_id(@__retain_semantics Other serializedRepresentation)]
        pub unsafe fn serializedRepresentation(&self) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other addFileWrapper:)]
        pub unsafe fn addFileWrapper(&self, child: &NSFileWrapper) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other addRegularFileWithContents:preferredFilename:)]
        pub unsafe fn addRegularFileWithContents_preferredFilename(
            &self,
            data: &NSData,
            fileName: &NSString,
        ) -> Id<NSString, Shared>;

        #[method(removeFileWrapper:)]
        pub unsafe fn removeFileWrapper(&self, child: &NSFileWrapper);

        #[method_id(@__retain_semantics Other fileWrappers)]
        pub unsafe fn fileWrappers(
            &self,
        ) -> Option<Id<NSDictionary<NSString, NSFileWrapper>, Shared>>;

        #[method_id(@__retain_semantics Other keyForFileWrapper:)]
        pub unsafe fn keyForFileWrapper(
            &self,
            child: &NSFileWrapper,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(@__retain_semantics Other regularFileContents)]
        pub unsafe fn regularFileContents(&self) -> Option<Id<NSData, Shared>>;

        #[method_id(@__retain_semantics Other symbolicLinkDestinationURL)]
        pub unsafe fn symbolicLinkDestinationURL(&self) -> Option<Id<NSURL, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSFileWrapper {
        #[method_id(@__retain_semantics Init initWithPath:)]
        pub unsafe fn initWithPath(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(@__retain_semantics Init initSymbolicLinkWithDestination:)]
        pub unsafe fn initSymbolicLinkWithDestination(
            this: Option<Allocated<Self>>,
            path: &NSString,
        ) -> Id<Self, Shared>;

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

        #[method_id(@__retain_semantics Other addFileWithPath:)]
        pub unsafe fn addFileWithPath(&self, path: &NSString) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other addSymbolicLinkWithDestination:preferredFilename:)]
        pub unsafe fn addSymbolicLinkWithDestination_preferredFilename(
            &self,
            path: &NSString,
            filename: &NSString,
        ) -> Id<NSString, Shared>;

        #[method_id(@__retain_semantics Other symbolicLinkDestination)]
        pub unsafe fn symbolicLinkDestination(&self) -> Id<NSString, Shared>;
    }
);
