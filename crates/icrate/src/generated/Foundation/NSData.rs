#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSData;
    unsafe impl ClassType for NSData {
        type Super = NSObject;
    }
);
impl NSData {
    pub unsafe fn length(&self) -> NSUInteger {
        msg_send![self, length]
    }
    pub unsafe fn bytes(&self) -> NonNull<c_void> {
        msg_send![self, bytes]
    }
}
#[doc = "NSExtendedData"]
impl NSData {
    pub unsafe fn getBytes_length(&self, buffer: NonNull<c_void>, length: NSUInteger) {
        msg_send![self, getBytes: buffer, length: length]
    }
    pub unsafe fn getBytes_range(&self, buffer: NonNull<c_void>, range: NSRange) {
        msg_send![self, getBytes: buffer, range: range]
    }
    pub unsafe fn isEqualToData(&self, other: &NSData) -> bool {
        msg_send![self, isEqualToData: other]
    }
    pub unsafe fn subdataWithRange(&self, range: NSRange) -> Id<NSData, Shared> {
        msg_send_id![self, subdataWithRange: range]
    }
    pub unsafe fn writeToFile_atomically(&self, path: &NSString, useAuxiliaryFile: bool) -> bool {
        msg_send![self, writeToFile: path, atomically: useAuxiliaryFile]
    }
    pub unsafe fn writeToURL_atomically(&self, url: &NSURL, atomically: bool) -> bool {
        msg_send![self, writeToURL: url, atomically: atomically]
    }
    pub unsafe fn writeToFile_options_error(
        &self,
        path: &NSString,
        writeOptionsMask: NSDataWritingOptions,
        errorPtr: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            writeToFile: path,
            options: writeOptionsMask,
            error: errorPtr
        ]
    }
    pub unsafe fn writeToURL_options_error(
        &self,
        url: &NSURL,
        writeOptionsMask: NSDataWritingOptions,
        errorPtr: *mut Option<&NSError>,
    ) -> bool {
        msg_send![
            self,
            writeToURL: url,
            options: writeOptionsMask,
            error: errorPtr
        ]
    }
    pub unsafe fn rangeOfData_options_range(
        &self,
        dataToFind: &NSData,
        mask: NSDataSearchOptions,
        searchRange: NSRange,
    ) -> NSRange {
        msg_send![
            self,
            rangeOfData: dataToFind,
            options: mask,
            range: searchRange
        ]
    }
    pub unsafe fn enumerateByteRangesUsingBlock(&self, block: TodoBlock) {
        msg_send![self, enumerateByteRangesUsingBlock: block]
    }
    pub unsafe fn description(&self) -> Id<NSString, Shared> {
        msg_send_id![self, description]
    }
}
#[doc = "NSDataCreation"]
impl NSData {
    pub unsafe fn data() -> Id<Self, Shared> {
        msg_send_id![Self::class(), data]
    }
    pub unsafe fn dataWithBytes_length(bytes: *mut c_void, length: NSUInteger) -> Id<Self, Shared> {
        msg_send_id![Self::class(), dataWithBytes: bytes, length: length]
    }
    pub unsafe fn dataWithBytesNoCopy_length(
        bytes: NonNull<c_void>,
        length: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![Self::class(), dataWithBytesNoCopy: bytes, length: length]
    }
    pub unsafe fn dataWithBytesNoCopy_length_freeWhenDone(
        bytes: NonNull<c_void>,
        length: NSUInteger,
        b: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![
            Self::class(),
            dataWithBytesNoCopy: bytes,
            length: length,
            freeWhenDone: b
        ]
    }
    pub unsafe fn dataWithContentsOfFile_options_error(
        path: &NSString,
        readOptionsMask: NSDataReadingOptions,
        errorPtr: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            Self::class(),
            dataWithContentsOfFile: path,
            options: readOptionsMask,
            error: errorPtr
        ]
    }
    pub unsafe fn dataWithContentsOfURL_options_error(
        url: &NSURL,
        readOptionsMask: NSDataReadingOptions,
        errorPtr: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            Self::class(),
            dataWithContentsOfURL: url,
            options: readOptionsMask,
            error: errorPtr
        ]
    }
    pub unsafe fn dataWithContentsOfFile(path: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), dataWithContentsOfFile: path]
    }
    pub unsafe fn dataWithContentsOfURL(url: &NSURL) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), dataWithContentsOfURL: url]
    }
    pub unsafe fn initWithBytes_length(
        &self,
        bytes: *mut c_void,
        length: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithBytes: bytes, length: length]
    }
    pub unsafe fn initWithBytesNoCopy_length(
        &self,
        bytes: NonNull<c_void>,
        length: NSUInteger,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithBytesNoCopy: bytes, length: length]
    }
    pub unsafe fn initWithBytesNoCopy_length_freeWhenDone(
        &self,
        bytes: NonNull<c_void>,
        length: NSUInteger,
        b: bool,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithBytesNoCopy: bytes,
            length: length,
            freeWhenDone: b
        ]
    }
    pub unsafe fn initWithBytesNoCopy_length_deallocator(
        &self,
        bytes: NonNull<c_void>,
        length: NSUInteger,
        deallocator: TodoBlock,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithBytesNoCopy: bytes,
            length: length,
            deallocator: deallocator
        ]
    }
    pub unsafe fn initWithContentsOfFile_options_error(
        &self,
        path: &NSString,
        readOptionsMask: NSDataReadingOptions,
        errorPtr: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithContentsOfFile: path,
            options: readOptionsMask,
            error: errorPtr
        ]
    }
    pub unsafe fn initWithContentsOfURL_options_error(
        &self,
        url: &NSURL,
        readOptionsMask: NSDataReadingOptions,
        errorPtr: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithContentsOfURL: url,
            options: readOptionsMask,
            error: errorPtr
        ]
    }
    pub unsafe fn initWithContentsOfFile(&self, path: &NSString) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithContentsOfFile: path]
    }
    pub unsafe fn initWithContentsOfURL(&self, url: &NSURL) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithContentsOfURL: url]
    }
    pub unsafe fn initWithData(&self, data: &NSData) -> Id<Self, Shared> {
        msg_send_id![self, initWithData: data]
    }
    pub unsafe fn dataWithData(data: &NSData) -> Id<Self, Shared> {
        msg_send_id![Self::class(), dataWithData: data]
    }
}
#[doc = "NSDataBase64Encoding"]
impl NSData {
    pub unsafe fn initWithBase64EncodedString_options(
        &self,
        base64String: &NSString,
        options: NSDataBase64DecodingOptions,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithBase64EncodedString: base64String,
            options: options
        ]
    }
    pub unsafe fn base64EncodedStringWithOptions(
        &self,
        options: NSDataBase64EncodingOptions,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, base64EncodedStringWithOptions: options]
    }
    pub unsafe fn initWithBase64EncodedData_options(
        &self,
        base64Data: &NSData,
        options: NSDataBase64DecodingOptions,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithBase64EncodedData: base64Data,
            options: options
        ]
    }
    pub unsafe fn base64EncodedDataWithOptions(
        &self,
        options: NSDataBase64EncodingOptions,
    ) -> Id<NSData, Shared> {
        msg_send_id![self, base64EncodedDataWithOptions: options]
    }
}
#[doc = "NSDataCompression"]
impl NSData {
    pub unsafe fn decompressedDataUsingAlgorithm_error(
        &self,
        algorithm: NSDataCompressionAlgorithm,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            decompressedDataUsingAlgorithm: algorithm,
            error: error
        ]
    }
    pub unsafe fn compressedDataUsingAlgorithm_error(
        &self,
        algorithm: NSDataCompressionAlgorithm,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, compressedDataUsingAlgorithm: algorithm, error: error]
    }
}
#[doc = "NSDeprecated"]
impl NSData {
    pub unsafe fn getBytes(&self, buffer: NonNull<c_void>) {
        msg_send![self, getBytes: buffer]
    }
    pub unsafe fn dataWithContentsOfMappedFile(path: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), dataWithContentsOfMappedFile: path]
    }
    pub unsafe fn initWithContentsOfMappedFile(
        &self,
        path: &NSString,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithContentsOfMappedFile: path]
    }
    pub unsafe fn initWithBase64Encoding(
        &self,
        base64String: &NSString,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, initWithBase64Encoding: base64String]
    }
    pub unsafe fn base64Encoding(&self) -> Id<NSString, Shared> {
        msg_send_id![self, base64Encoding]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSMutableData;
    unsafe impl ClassType for NSMutableData {
        type Super = NSData;
    }
);
impl NSMutableData {
    pub unsafe fn mutableBytes(&self) -> NonNull<c_void> {
        msg_send![self, mutableBytes]
    }
    pub unsafe fn length(&self) -> NSUInteger {
        msg_send![self, length]
    }
    pub unsafe fn setLength(&self, length: NSUInteger) {
        msg_send![self, setLength: length]
    }
}
#[doc = "NSExtendedMutableData"]
impl NSMutableData {
    pub unsafe fn appendBytes_length(&self, bytes: NonNull<c_void>, length: NSUInteger) {
        msg_send![self, appendBytes: bytes, length: length]
    }
    pub unsafe fn appendData(&self, other: &NSData) {
        msg_send![self, appendData: other]
    }
    pub unsafe fn increaseLengthBy(&self, extraLength: NSUInteger) {
        msg_send![self, increaseLengthBy: extraLength]
    }
    pub unsafe fn replaceBytesInRange_withBytes(&self, range: NSRange, bytes: NonNull<c_void>) {
        msg_send![self, replaceBytesInRange: range, withBytes: bytes]
    }
    pub unsafe fn resetBytesInRange(&self, range: NSRange) {
        msg_send![self, resetBytesInRange: range]
    }
    pub unsafe fn setData(&self, data: &NSData) {
        msg_send![self, setData: data]
    }
    pub unsafe fn replaceBytesInRange_withBytes_length(
        &self,
        range: NSRange,
        replacementBytes: *mut c_void,
        replacementLength: NSUInteger,
    ) {
        msg_send![
            self,
            replaceBytesInRange: range,
            withBytes: replacementBytes,
            length: replacementLength
        ]
    }
}
#[doc = "NSMutableDataCreation"]
impl NSMutableData {
    pub unsafe fn dataWithCapacity(aNumItems: NSUInteger) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), dataWithCapacity: aNumItems]
    }
    pub unsafe fn dataWithLength(length: NSUInteger) -> Option<Id<Self, Shared>> {
        msg_send_id![Self::class(), dataWithLength: length]
    }
    pub unsafe fn initWithCapacity(&self, capacity: NSUInteger) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCapacity: capacity]
    }
    pub unsafe fn initWithLength(&self, length: NSUInteger) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithLength: length]
    }
}
#[doc = "NSMutableDataCompression"]
impl NSMutableData {
    pub unsafe fn decompressUsingAlgorithm_error(
        &self,
        algorithm: NSDataCompressionAlgorithm,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, decompressUsingAlgorithm: algorithm, error: error]
    }
    pub unsafe fn compressUsingAlgorithm_error(
        &self,
        algorithm: NSDataCompressionAlgorithm,
        error: *mut Option<&NSError>,
    ) -> bool {
        msg_send![self, compressUsingAlgorithm: algorithm, error: error]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSPurgeableData;
    unsafe impl ClassType for NSPurgeableData {
        type Super = NSMutableData;
    }
);
impl NSPurgeableData {}
