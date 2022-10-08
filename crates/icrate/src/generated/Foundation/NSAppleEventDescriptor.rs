use super::__exported::NSData;
use crate::CoreServices::generated::CoreServices::*;
use crate::Foundation::generated::NSDate::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSAppleEventDescriptor;
    unsafe impl ClassType for NSAppleEventDescriptor {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSAppleEventDescriptor {
        #[method_id(nullDescriptor)]
        pub unsafe fn nullDescriptor() -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (descriptorWithDescriptorType : bytes : length :)]
        pub unsafe fn descriptorWithDescriptorType_bytes_length(
            descriptorType: DescType,
            bytes: *mut c_void,
            byteCount: NSUInteger,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        # [method_id (descriptorWithDescriptorType : data :)]
        pub unsafe fn descriptorWithDescriptorType_data(
            descriptorType: DescType,
            data: Option<&NSData>,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        # [method_id (descriptorWithBoolean :)]
        pub unsafe fn descriptorWithBoolean(boolean: Boolean)
            -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (descriptorWithEnumCode :)]
        pub unsafe fn descriptorWithEnumCode(
            enumerator: OSType,
        ) -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (descriptorWithInt32 :)]
        pub unsafe fn descriptorWithInt32(signedInt: SInt32) -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (descriptorWithDouble :)]
        pub unsafe fn descriptorWithDouble(
            doubleValue: c_double,
        ) -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (descriptorWithTypeCode :)]
        pub unsafe fn descriptorWithTypeCode(
            typeCode: OSType,
        ) -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (descriptorWithString :)]
        pub unsafe fn descriptorWithString(string: &NSString)
            -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (descriptorWithDate :)]
        pub unsafe fn descriptorWithDate(date: &NSDate) -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (descriptorWithFileURL :)]
        pub unsafe fn descriptorWithFileURL(fileURL: &NSURL) -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (appleEventWithEventClass : eventID : targetDescriptor : returnID : transactionID :)]
        pub unsafe fn appleEventWithEventClass_eventID_targetDescriptor_returnID_transactionID(
            eventClass: AEEventClass,
            eventID: AEEventID,
            targetDescriptor: Option<&NSAppleEventDescriptor>,
            returnID: AEReturnID,
            transactionID: AETransactionID,
        ) -> Id<NSAppleEventDescriptor, Shared>;
        #[method_id(listDescriptor)]
        pub unsafe fn listDescriptor() -> Id<NSAppleEventDescriptor, Shared>;
        #[method_id(recordDescriptor)]
        pub unsafe fn recordDescriptor() -> Id<NSAppleEventDescriptor, Shared>;
        #[method_id(currentProcessDescriptor)]
        pub unsafe fn currentProcessDescriptor() -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (descriptorWithProcessIdentifier :)]
        pub unsafe fn descriptorWithProcessIdentifier(
            processIdentifier: pid_t,
        ) -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (descriptorWithBundleIdentifier :)]
        pub unsafe fn descriptorWithBundleIdentifier(
            bundleIdentifier: &NSString,
        ) -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (descriptorWithApplicationURL :)]
        pub unsafe fn descriptorWithApplicationURL(
            applicationURL: &NSURL,
        ) -> Id<NSAppleEventDescriptor, Shared>;
        # [method_id (initWithAEDescNoCopy :)]
        pub unsafe fn initWithAEDescNoCopy(&self, aeDesc: NonNull<AEDesc>) -> Id<Self, Shared>;
        # [method_id (initWithDescriptorType : bytes : length :)]
        pub unsafe fn initWithDescriptorType_bytes_length(
            &self,
            descriptorType: DescType,
            bytes: *mut c_void,
            byteCount: NSUInteger,
        ) -> Option<Id<Self, Shared>>;
        # [method_id (initWithDescriptorType : data :)]
        pub unsafe fn initWithDescriptorType_data(
            &self,
            descriptorType: DescType,
            data: Option<&NSData>,
        ) -> Option<Id<Self, Shared>>;
        # [method_id (initWithEventClass : eventID : targetDescriptor : returnID : transactionID :)]
        pub unsafe fn initWithEventClass_eventID_targetDescriptor_returnID_transactionID(
            &self,
            eventClass: AEEventClass,
            eventID: AEEventID,
            targetDescriptor: Option<&NSAppleEventDescriptor>,
            returnID: AEReturnID,
            transactionID: AETransactionID,
        ) -> Id<Self, Shared>;
        #[method_id(initListDescriptor)]
        pub unsafe fn initListDescriptor(&self) -> Id<Self, Shared>;
        #[method_id(initRecordDescriptor)]
        pub unsafe fn initRecordDescriptor(&self) -> Id<Self, Shared>;
        #[method(aeDesc)]
        pub unsafe fn aeDesc(&self) -> *mut AEDesc;
        #[method(descriptorType)]
        pub unsafe fn descriptorType(&self) -> DescType;
        #[method_id(data)]
        pub unsafe fn data(&self) -> Id<NSData, Shared>;
        #[method(booleanValue)]
        pub unsafe fn booleanValue(&self) -> Boolean;
        #[method(enumCodeValue)]
        pub unsafe fn enumCodeValue(&self) -> OSType;
        #[method(int32Value)]
        pub unsafe fn int32Value(&self) -> SInt32;
        #[method(doubleValue)]
        pub unsafe fn doubleValue(&self) -> c_double;
        #[method(typeCodeValue)]
        pub unsafe fn typeCodeValue(&self) -> OSType;
        #[method_id(stringValue)]
        pub unsafe fn stringValue(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(dateValue)]
        pub unsafe fn dateValue(&self) -> Option<Id<NSDate, Shared>>;
        #[method_id(fileURLValue)]
        pub unsafe fn fileURLValue(&self) -> Option<Id<NSURL, Shared>>;
        #[method(eventClass)]
        pub unsafe fn eventClass(&self) -> AEEventClass;
        #[method(eventID)]
        pub unsafe fn eventID(&self) -> AEEventID;
        #[method(returnID)]
        pub unsafe fn returnID(&self) -> AEReturnID;
        #[method(transactionID)]
        pub unsafe fn transactionID(&self) -> AETransactionID;
        # [method (setParamDescriptor : forKeyword :)]
        pub unsafe fn setParamDescriptor_forKeyword(
            &self,
            descriptor: &NSAppleEventDescriptor,
            keyword: AEKeyword,
        );
        # [method_id (paramDescriptorForKeyword :)]
        pub unsafe fn paramDescriptorForKeyword(
            &self,
            keyword: AEKeyword,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        # [method (removeParamDescriptorWithKeyword :)]
        pub unsafe fn removeParamDescriptorWithKeyword(&self, keyword: AEKeyword);
        # [method (setAttributeDescriptor : forKeyword :)]
        pub unsafe fn setAttributeDescriptor_forKeyword(
            &self,
            descriptor: &NSAppleEventDescriptor,
            keyword: AEKeyword,
        );
        # [method_id (attributeDescriptorForKeyword :)]
        pub unsafe fn attributeDescriptorForKeyword(
            &self,
            keyword: AEKeyword,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        # [method_id (sendEventWithOptions : timeout : error :)]
        pub unsafe fn sendEventWithOptions_timeout_error(
            &self,
            sendOptions: NSAppleEventSendOptions,
            timeoutInSeconds: NSTimeInterval,
        ) -> Result<Id<NSAppleEventDescriptor, Shared>, Id<NSError, Shared>>;
        #[method(isRecordDescriptor)]
        pub unsafe fn isRecordDescriptor(&self) -> bool;
        #[method(numberOfItems)]
        pub unsafe fn numberOfItems(&self) -> NSInteger;
        # [method (insertDescriptor : atIndex :)]
        pub unsafe fn insertDescriptor_atIndex(
            &self,
            descriptor: &NSAppleEventDescriptor,
            index: NSInteger,
        );
        # [method_id (descriptorAtIndex :)]
        pub unsafe fn descriptorAtIndex(
            &self,
            index: NSInteger,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        # [method (removeDescriptorAtIndex :)]
        pub unsafe fn removeDescriptorAtIndex(&self, index: NSInteger);
        # [method (setDescriptor : forKeyword :)]
        pub unsafe fn setDescriptor_forKeyword(
            &self,
            descriptor: &NSAppleEventDescriptor,
            keyword: AEKeyword,
        );
        # [method_id (descriptorForKeyword :)]
        pub unsafe fn descriptorForKeyword(
            &self,
            keyword: AEKeyword,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;
        # [method (removeDescriptorWithKeyword :)]
        pub unsafe fn removeDescriptorWithKeyword(&self, keyword: AEKeyword);
        # [method (keywordForDescriptorAtIndex :)]
        pub unsafe fn keywordForDescriptorAtIndex(&self, index: NSInteger) -> AEKeyword;
        # [method_id (coerceToDescriptorType :)]
        pub unsafe fn coerceToDescriptorType(
            &self,
            descriptorType: DescType,
        ) -> Option<Id<NSAppleEventDescriptor, Shared>>;
    }
);
