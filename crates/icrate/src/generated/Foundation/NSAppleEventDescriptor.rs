#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAppleEventDescriptor;
    unsafe impl ClassType for NSAppleEventDescriptor {
        type Super = NSObject;
    }
);
impl NSAppleEventDescriptor {
    pub unsafe fn nullDescriptor() -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), nullDescriptor]
    }
    pub unsafe fn descriptorWithDescriptorType_bytes_length(
        descriptorType: DescType,
        bytes: *mut c_void,
        byteCount: NSUInteger,
    ) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![
            Self::class(),
            descriptorWithDescriptorType: descriptorType,
            bytes: bytes,
            length: byteCount
        ]
    }
    pub unsafe fn descriptorWithDescriptorType_data(
        descriptorType: DescType,
        data: Option<&NSData>,
    ) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![
            Self::class(),
            descriptorWithDescriptorType: descriptorType,
            data: data
        ]
    }
    pub unsafe fn descriptorWithBoolean(boolean: Boolean) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), descriptorWithBoolean: boolean]
    }
    pub unsafe fn descriptorWithEnumCode(enumerator: OSType) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), descriptorWithEnumCode: enumerator]
    }
    pub unsafe fn descriptorWithInt32(signedInt: SInt32) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), descriptorWithInt32: signedInt]
    }
    pub unsafe fn descriptorWithDouble(
        doubleValue: c_double,
    ) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), descriptorWithDouble: doubleValue]
    }
    pub unsafe fn descriptorWithTypeCode(typeCode: OSType) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), descriptorWithTypeCode: typeCode]
    }
    pub unsafe fn descriptorWithString(string: &NSString) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), descriptorWithString: string]
    }
    pub unsafe fn descriptorWithDate(date: &NSDate) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), descriptorWithDate: date]
    }
    pub unsafe fn descriptorWithFileURL(fileURL: &NSURL) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), descriptorWithFileURL: fileURL]
    }
    pub unsafe fn appleEventWithEventClass_eventID_targetDescriptor_returnID_transactionID(
        eventClass: AEEventClass,
        eventID: AEEventID,
        targetDescriptor: Option<&NSAppleEventDescriptor>,
        returnID: AEReturnID,
        transactionID: AETransactionID,
    ) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![
            Self::class(),
            appleEventWithEventClass: eventClass,
            eventID: eventID,
            targetDescriptor: targetDescriptor,
            returnID: returnID,
            transactionID: transactionID
        ]
    }
    pub unsafe fn listDescriptor() -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), listDescriptor]
    }
    pub unsafe fn recordDescriptor() -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), recordDescriptor]
    }
    pub unsafe fn currentProcessDescriptor() -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), currentProcessDescriptor]
    }
    pub unsafe fn descriptorWithProcessIdentifier(
        processIdentifier: pid_t,
    ) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![
            Self::class(),
            descriptorWithProcessIdentifier: processIdentifier
        ]
    }
    pub unsafe fn descriptorWithBundleIdentifier(
        bundleIdentifier: &NSString,
    ) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![
            Self::class(),
            descriptorWithBundleIdentifier: bundleIdentifier
        ]
    }
    pub unsafe fn descriptorWithApplicationURL(
        applicationURL: &NSURL,
    ) -> Id<NSAppleEventDescriptor, Shared> {
        msg_send_id![Self::class(), descriptorWithApplicationURL: applicationURL]
    }
    pub unsafe fn initWithAEDescNoCopy(&self, aeDesc: NonNull<AEDesc>) -> Id<Self, Shared> {
        msg_send_id![self, initWithAEDescNoCopy: aeDesc]
    }
    pub unsafe fn initWithDescriptorType_bytes_length(
        &self,
        descriptorType: DescType,
        bytes: *mut c_void,
        byteCount: NSUInteger,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithDescriptorType: descriptorType,
            bytes: bytes,
            length: byteCount
        ]
    }
    pub unsafe fn initWithDescriptorType_data(
        &self,
        descriptorType: DescType,
        data: Option<&NSData>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithDescriptorType: descriptorType, data: data]
    }
    pub unsafe fn initWithEventClass_eventID_targetDescriptor_returnID_transactionID(
        &self,
        eventClass: AEEventClass,
        eventID: AEEventID,
        targetDescriptor: Option<&NSAppleEventDescriptor>,
        returnID: AEReturnID,
        transactionID: AETransactionID,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithEventClass: eventClass,
            eventID: eventID,
            targetDescriptor: targetDescriptor,
            returnID: returnID,
            transactionID: transactionID
        ]
    }
    pub unsafe fn initListDescriptor(&self) -> Id<Self, Shared> {
        msg_send_id![self, initListDescriptor]
    }
    pub unsafe fn initRecordDescriptor(&self) -> Id<Self, Shared> {
        msg_send_id![self, initRecordDescriptor]
    }
    pub unsafe fn setParamDescriptor_forKeyword(
        &self,
        descriptor: &NSAppleEventDescriptor,
        keyword: AEKeyword,
    ) {
        msg_send![self, setParamDescriptor: descriptor, forKeyword: keyword]
    }
    pub unsafe fn paramDescriptorForKeyword(
        &self,
        keyword: AEKeyword,
    ) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![self, paramDescriptorForKeyword: keyword]
    }
    pub unsafe fn removeParamDescriptorWithKeyword(&self, keyword: AEKeyword) {
        msg_send![self, removeParamDescriptorWithKeyword: keyword]
    }
    pub unsafe fn setAttributeDescriptor_forKeyword(
        &self,
        descriptor: &NSAppleEventDescriptor,
        keyword: AEKeyword,
    ) {
        msg_send![
            self,
            setAttributeDescriptor: descriptor,
            forKeyword: keyword
        ]
    }
    pub unsafe fn attributeDescriptorForKeyword(
        &self,
        keyword: AEKeyword,
    ) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![self, attributeDescriptorForKeyword: keyword]
    }
    pub unsafe fn sendEventWithOptions_timeout_error(
        &self,
        sendOptions: NSAppleEventSendOptions,
        timeoutInSeconds: NSTimeInterval,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![
            self,
            sendEventWithOptions: sendOptions,
            timeout: timeoutInSeconds,
            error: error
        ]
    }
    pub unsafe fn insertDescriptor_atIndex(
        &self,
        descriptor: &NSAppleEventDescriptor,
        index: NSInteger,
    ) {
        msg_send![self, insertDescriptor: descriptor, atIndex: index]
    }
    pub unsafe fn descriptorAtIndex(
        &self,
        index: NSInteger,
    ) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![self, descriptorAtIndex: index]
    }
    pub unsafe fn removeDescriptorAtIndex(&self, index: NSInteger) {
        msg_send![self, removeDescriptorAtIndex: index]
    }
    pub unsafe fn setDescriptor_forKeyword(
        &self,
        descriptor: &NSAppleEventDescriptor,
        keyword: AEKeyword,
    ) {
        msg_send![self, setDescriptor: descriptor, forKeyword: keyword]
    }
    pub unsafe fn descriptorForKeyword(
        &self,
        keyword: AEKeyword,
    ) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![self, descriptorForKeyword: keyword]
    }
    pub unsafe fn removeDescriptorWithKeyword(&self, keyword: AEKeyword) {
        msg_send![self, removeDescriptorWithKeyword: keyword]
    }
    pub unsafe fn keywordForDescriptorAtIndex(&self, index: NSInteger) -> AEKeyword {
        msg_send![self, keywordForDescriptorAtIndex: index]
    }
    pub unsafe fn coerceToDescriptorType(
        &self,
        descriptorType: DescType,
    ) -> Option<Id<NSAppleEventDescriptor, Shared>> {
        msg_send_id![self, coerceToDescriptorType: descriptorType]
    }
    pub unsafe fn aeDesc(&self) -> *mut AEDesc {
        msg_send![self, aeDesc]
    }
    pub unsafe fn descriptorType(&self) -> DescType {
        msg_send![self, descriptorType]
    }
    pub unsafe fn data(&self) -> Id<NSData, Shared> {
        msg_send_id![self, data]
    }
    pub unsafe fn booleanValue(&self) -> Boolean {
        msg_send![self, booleanValue]
    }
    pub unsafe fn enumCodeValue(&self) -> OSType {
        msg_send![self, enumCodeValue]
    }
    pub unsafe fn int32Value(&self) -> SInt32 {
        msg_send![self, int32Value]
    }
    pub unsafe fn doubleValue(&self) -> c_double {
        msg_send![self, doubleValue]
    }
    pub unsafe fn typeCodeValue(&self) -> OSType {
        msg_send![self, typeCodeValue]
    }
    pub unsafe fn stringValue(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringValue]
    }
    pub unsafe fn dateValue(&self) -> Option<Id<NSDate, Shared>> {
        msg_send_id![self, dateValue]
    }
    pub unsafe fn fileURLValue(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, fileURLValue]
    }
    pub unsafe fn eventClass(&self) -> AEEventClass {
        msg_send![self, eventClass]
    }
    pub unsafe fn eventID(&self) -> AEEventID {
        msg_send![self, eventID]
    }
    pub unsafe fn returnID(&self) -> AEReturnID {
        msg_send![self, returnID]
    }
    pub unsafe fn transactionID(&self) -> AETransactionID {
        msg_send![self, transactionID]
    }
    pub unsafe fn isRecordDescriptor(&self) -> bool {
        msg_send![self, isRecordDescriptor]
    }
    pub unsafe fn numberOfItems(&self) -> NSInteger {
        msg_send![self, numberOfItems]
    }
}
