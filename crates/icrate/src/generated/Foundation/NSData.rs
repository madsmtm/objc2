//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

pub type NSDataReadingOptions = NSUInteger;
pub const NSDataReadingMappedIfSafe: NSDataReadingOptions = 1 << 0;
pub const NSDataReadingUncached: NSDataReadingOptions = 1 << 1;
pub const NSDataReadingMappedAlways: NSDataReadingOptions = 1 << 3;
pub const NSDataReadingMapped: NSDataReadingOptions = NSDataReadingMappedIfSafe;
pub const NSMappedRead: NSDataReadingOptions = NSDataReadingMapped;
pub const NSUncachedRead: NSDataReadingOptions = NSDataReadingUncached;

pub type NSDataWritingOptions = NSUInteger;
pub const NSDataWritingAtomic: NSDataWritingOptions = 1 << 0;
pub const NSDataWritingWithoutOverwriting: NSDataWritingOptions = 1 << 1;
pub const NSDataWritingFileProtectionNone: NSDataWritingOptions = 0x10000000;
pub const NSDataWritingFileProtectionComplete: NSDataWritingOptions = 0x20000000;
pub const NSDataWritingFileProtectionCompleteUnlessOpen: NSDataWritingOptions = 0x30000000;
pub const NSDataWritingFileProtectionCompleteUntilFirstUserAuthentication: NSDataWritingOptions =
    0x40000000;
pub const NSDataWritingFileProtectionMask: NSDataWritingOptions = 0xf0000000;
pub const NSAtomicWrite: NSDataWritingOptions = NSDataWritingAtomic;

pub type NSDataSearchOptions = NSUInteger;
pub const NSDataSearchBackwards: NSDataSearchOptions = 1 << 0;
pub const NSDataSearchAnchored: NSDataSearchOptions = 1 << 1;

pub type NSDataBase64EncodingOptions = NSUInteger;
pub const NSDataBase64Encoding64CharacterLineLength: NSDataBase64EncodingOptions = 1 << 0;
pub const NSDataBase64Encoding76CharacterLineLength: NSDataBase64EncodingOptions = 1 << 1;
pub const NSDataBase64EncodingEndLineWithCarriageReturn: NSDataBase64EncodingOptions = 1 << 4;
pub const NSDataBase64EncodingEndLineWithLineFeed: NSDataBase64EncodingOptions = 1 << 5;

pub type NSDataBase64DecodingOptions = NSUInteger;
pub const NSDataBase64DecodingIgnoreUnknownCharacters: NSDataBase64DecodingOptions = 1 << 0;

extern_class!(
    #[derive(Debug)]
    pub struct NSData;

    unsafe impl ClassType for NSData {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSData {
        #[method(length)]
        pub unsafe fn length(&self) -> NSUInteger;

        #[method(bytes)]
        pub unsafe fn bytes(&self) -> NonNull<c_void>;
    }
);

extern_methods!(
    /// NSExtendedData
    unsafe impl NSData {
        #[method_id(description)]
        pub unsafe fn description(&self) -> Id<NSString, Shared>;

        #[method(getBytes:length:)]
        pub unsafe fn getBytes_length(&self, buffer: NonNull<c_void>, length: NSUInteger);

        #[method(getBytes:range:)]
        pub unsafe fn getBytes_range(&self, buffer: NonNull<c_void>, range: NSRange);

        #[method(isEqualToData:)]
        pub unsafe fn isEqualToData(&self, other: &NSData) -> bool;

        #[method_id(subdataWithRange:)]
        pub unsafe fn subdataWithRange(&self, range: NSRange) -> Id<NSData, Shared>;

        #[method(writeToFile:atomically:)]
        pub unsafe fn writeToFile_atomically(
            &self,
            path: &NSString,
            useAuxiliaryFile: bool,
        ) -> bool;

        #[method(writeToURL:atomically:)]
        pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool;

        #[method(writeToFile:options:error:)]
        pub unsafe fn writeToFile_options_error(
            &self,
            path: &NSString,
            writeOptionsMask: NSDataWritingOptions,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(writeToURL:options:error:)]
        pub unsafe fn writeToURL_options_error(
            &self,
            url: &NSURL,
            writeOptionsMask: NSDataWritingOptions,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(rangeOfData:options:range:)]
        pub unsafe fn rangeOfData_options_range(
            &self,
            dataToFind: &NSData,
            mask: NSDataSearchOptions,
            searchRange: NSRange,
        ) -> NSRange;

        #[method(enumerateByteRangesUsingBlock:)]
        pub unsafe fn enumerateByteRangesUsingBlock(&self, block: TodoBlock);
    }
);

extern_methods!(
    /// NSDataCreation
    unsafe impl NSData {
        #[method_id(data)]
        pub unsafe fn data() -> Id<Self, Shared>;

        #[method_id(dataWithBytes:length:)]
        pub unsafe fn dataWithBytes_length(
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(dataWithBytesNoCopy:length:)]
        pub unsafe fn dataWithBytesNoCopy_length(
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(dataWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn dataWithBytesNoCopy_length_freeWhenDone(
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self, Shared>;

        #[method_id(dataWithContentsOfFile:options:error:)]
        pub unsafe fn dataWithContentsOfFile_options_error(
            path: &NSString,
            readOptionsMask: NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(dataWithContentsOfURL:options:error:)]
        pub unsafe fn dataWithContentsOfURL_options_error(
            url: &NSURL,
            readOptionsMask: NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(dataWithContentsOfFile:)]
        pub unsafe fn dataWithContentsOfFile(path: &NSString) -> Option<Id<Self, Shared>>;

        #[method_id(dataWithContentsOfURL:)]
        pub unsafe fn dataWithContentsOfURL(url: &NSURL) -> Option<Id<Self, Shared>>;

        #[method_id(initWithBytes:length:)]
        pub unsafe fn initWithBytes_length(
            &self,
            bytes: *mut c_void,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(initWithBytesNoCopy:length:)]
        pub unsafe fn initWithBytesNoCopy_length(
            &self,
            bytes: NonNull<c_void>,
            length: NSUInteger,
        ) -> Id<Self, Shared>;

        #[method_id(initWithBytesNoCopy:length:freeWhenDone:)]
        pub unsafe fn initWithBytesNoCopy_length_freeWhenDone(
            &self,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            b: bool,
        ) -> Id<Self, Shared>;

        #[method_id(initWithBytesNoCopy:length:deallocator:)]
        pub unsafe fn initWithBytesNoCopy_length_deallocator(
            &self,
            bytes: NonNull<c_void>,
            length: NSUInteger,
            deallocator: TodoBlock,
        ) -> Id<Self, Shared>;

        #[method_id(initWithContentsOfFile:options:error:)]
        pub unsafe fn initWithContentsOfFile_options_error(
            &self,
            path: &NSString,
            readOptionsMask: NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(initWithContentsOfURL:options:error:)]
        pub unsafe fn initWithContentsOfURL_options_error(
            &self,
            url: &NSURL,
            readOptionsMask: NSDataReadingOptions,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(initWithContentsOfFile:)]
        pub unsafe fn initWithContentsOfFile(&self, path: &NSString) -> Option<Id<Self, Shared>>;

        #[method_id(initWithContentsOfURL:)]
        pub unsafe fn initWithContentsOfURL(&self, url: &NSURL) -> Option<Id<Self, Shared>>;

        #[method_id(initWithData:)]
        pub unsafe fn initWithData(&self, data: &NSData) -> Id<Self, Shared>;

        #[method_id(dataWithData:)]
        pub unsafe fn dataWithData(data: &NSData) -> Id<Self, Shared>;
    }
);

extern_methods!(
    /// NSDataBase64Encoding
    unsafe impl NSData {
        #[method_id(initWithBase64EncodedString:options:)]
        pub unsafe fn initWithBase64EncodedString_options(
            &self,
            base64String: &NSString,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(base64EncodedStringWithOptions:)]
        pub unsafe fn base64EncodedStringWithOptions(
            &self,
            options: NSDataBase64EncodingOptions,
        ) -> Id<NSString, Shared>;

        #[method_id(initWithBase64EncodedData:options:)]
        pub unsafe fn initWithBase64EncodedData_options(
            &self,
            base64Data: &NSData,
            options: NSDataBase64DecodingOptions,
        ) -> Option<Id<Self, Shared>>;

        #[method_id(base64EncodedDataWithOptions:)]
        pub unsafe fn base64EncodedDataWithOptions(
            &self,
            options: NSDataBase64EncodingOptions,
        ) -> Id<NSData, Shared>;
    }
);

pub type NSDataCompressionAlgorithm = NSInteger;
pub const NSDataCompressionAlgorithmLZFSE: NSDataCompressionAlgorithm = 0;
pub const NSDataCompressionAlgorithmLZ4: NSDataCompressionAlgorithm = 1;
pub const NSDataCompressionAlgorithmLZMA: NSDataCompressionAlgorithm = 2;
pub const NSDataCompressionAlgorithmZlib: NSDataCompressionAlgorithm = 3;

extern_methods!(
    /// NSDataCompression
    unsafe impl NSData {
        #[method_id(decompressedDataUsingAlgorithm:error:)]
        pub unsafe fn decompressedDataUsingAlgorithm_error(
            &self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;

        #[method_id(compressedDataUsingAlgorithm:error:)]
        pub unsafe fn compressedDataUsingAlgorithm_error(
            &self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<Id<Self, Shared>, Id<NSError, Shared>>;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSData {
        #[method(getBytes:)]
        pub unsafe fn getBytes(&self, buffer: NonNull<c_void>);

        #[method_id(dataWithContentsOfMappedFile:)]
        pub unsafe fn dataWithContentsOfMappedFile(path: &NSString) -> Option<Id<Object, Shared>>;

        #[method_id(initWithContentsOfMappedFile:)]
        pub unsafe fn initWithContentsOfMappedFile(
            &self,
            path: &NSString,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(initWithBase64Encoding:)]
        pub unsafe fn initWithBase64Encoding(
            &self,
            base64String: &NSString,
        ) -> Option<Id<Object, Shared>>;

        #[method_id(base64Encoding)]
        pub unsafe fn base64Encoding(&self) -> Id<NSString, Shared>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSMutableData;

    unsafe impl ClassType for NSMutableData {
        type Super = NSData;
    }
);

extern_methods!(
    unsafe impl NSMutableData {
        #[method(mutableBytes)]
        pub unsafe fn mutableBytes(&self) -> NonNull<c_void>;

        #[method(length)]
        pub unsafe fn length(&self) -> NSUInteger;

        #[method(setLength:)]
        pub unsafe fn setLength(&self, length: NSUInteger);
    }
);

extern_methods!(
    /// NSExtendedMutableData
    unsafe impl NSMutableData {
        #[method(appendBytes:length:)]
        pub unsafe fn appendBytes_length(&self, bytes: NonNull<c_void>, length: NSUInteger);

        #[method(appendData:)]
        pub unsafe fn appendData(&self, other: &NSData);

        #[method(increaseLengthBy:)]
        pub unsafe fn increaseLengthBy(&self, extraLength: NSUInteger);

        #[method(replaceBytesInRange:withBytes:)]
        pub unsafe fn replaceBytesInRange_withBytes(&self, range: NSRange, bytes: NonNull<c_void>);

        #[method(resetBytesInRange:)]
        pub unsafe fn resetBytesInRange(&self, range: NSRange);

        #[method(setData:)]
        pub unsafe fn setData(&self, data: &NSData);

        #[method(replaceBytesInRange:withBytes:length:)]
        pub unsafe fn replaceBytesInRange_withBytes_length(
            &self,
            range: NSRange,
            replacementBytes: *mut c_void,
            replacementLength: NSUInteger,
        );
    }
);

extern_methods!(
    /// NSMutableDataCreation
    unsafe impl NSMutableData {
        #[method_id(dataWithCapacity:)]
        pub unsafe fn dataWithCapacity(aNumItems: NSUInteger) -> Option<Id<Self, Shared>>;

        #[method_id(dataWithLength:)]
        pub unsafe fn dataWithLength(length: NSUInteger) -> Option<Id<Self, Shared>>;

        #[method_id(initWithCapacity:)]
        pub unsafe fn initWithCapacity(&self, capacity: NSUInteger) -> Option<Id<Self, Shared>>;

        #[method_id(initWithLength:)]
        pub unsafe fn initWithLength(&self, length: NSUInteger) -> Option<Id<Self, Shared>>;
    }
);

extern_methods!(
    /// NSMutableDataCompression
    unsafe impl NSMutableData {
        #[method(decompressUsingAlgorithm:error:)]
        pub unsafe fn decompressUsingAlgorithm_error(
            &self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<(), Id<NSError, Shared>>;

        #[method(compressUsingAlgorithm:error:)]
        pub unsafe fn compressUsingAlgorithm_error(
            &self,
            algorithm: NSDataCompressionAlgorithm,
        ) -> Result<(), Id<NSError, Shared>>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSPurgeableData;

    unsafe impl ClassType for NSPurgeableData {
        type Super = NSMutableData;
    }
);

extern_methods!(
    unsafe impl NSPurgeableData {}
);
