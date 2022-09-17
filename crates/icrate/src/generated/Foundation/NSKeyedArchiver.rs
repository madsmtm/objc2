extern_class!(
    #[derive(Debug)]
    struct NSKeyedArchiver;
    unsafe impl ClassType for NSKeyedArchiver {
        type Super = NSCoder;
    }
);
impl NSKeyedArchiver {
    pub unsafe fn initRequiringSecureCoding(&self, requiresSecureCoding: bool) -> Id<Self, Shared> {
        msg_send_id![self, initRequiringSecureCoding: requiresSecureCoding]
    }
    pub unsafe fn archivedDataWithRootObject_requiringSecureCoding_error(
        object: &Object,
        requiresSecureCoding: bool,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSData, Shared>> {
        msg_send_id![
            Self::class(),
            archivedDataWithRootObject: object,
            requiringSecureCoding: requiresSecureCoding,
            error: error
        ]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initForWritingWithMutableData(&self, data: &NSMutableData) -> Id<Self, Shared> {
        msg_send_id![self, initForWritingWithMutableData: data]
    }
    pub unsafe fn archivedDataWithRootObject(rootObject: &Object) -> Id<NSData, Shared> {
        msg_send_id![Self::class(), archivedDataWithRootObject: rootObject]
    }
    pub unsafe fn archiveRootObject_toFile(rootObject: &Object, path: &NSString) -> bool {
        msg_send![Self::class(), archiveRootObject: rootObject, toFile: path]
    }
    pub unsafe fn finishEncoding(&self) {
        msg_send![self, finishEncoding]
    }
    pub unsafe fn setClassName_forClass(codedName: Option<&NSString>, cls: &Class) {
        msg_send![Self::class(), setClassName: codedName, forClass: cls]
    }
    pub unsafe fn setClassName_forClass(&self, codedName: Option<&NSString>, cls: &Class) {
        msg_send![self, setClassName: codedName, forClass: cls]
    }
    pub unsafe fn classNameForClass(cls: &Class) -> Option<Id<NSString, Shared>> {
        msg_send_id![Self::class(), classNameForClass: cls]
    }
    pub unsafe fn classNameForClass(&self, cls: &Class) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, classNameForClass: cls]
    }
    pub unsafe fn encodeObject_forKey(&self, object: Option<&Object>, key: &NSString) {
        msg_send![self, encodeObject: object, forKey: key]
    }
    pub unsafe fn encodeConditionalObject_forKey(&self, object: Option<&Object>, key: &NSString) {
        msg_send![self, encodeConditionalObject: object, forKey: key]
    }
    pub unsafe fn encodeBool_forKey(&self, value: bool, key: &NSString) {
        msg_send![self, encodeBool: value, forKey: key]
    }
    pub unsafe fn encodeInt_forKey(&self, value: c_int, key: &NSString) {
        msg_send![self, encodeInt: value, forKey: key]
    }
    pub unsafe fn encodeInt32_forKey(&self, value: int32_t, key: &NSString) {
        msg_send![self, encodeInt32: value, forKey: key]
    }
    pub unsafe fn encodeInt64_forKey(&self, value: int64_t, key: &NSString) {
        msg_send![self, encodeInt64: value, forKey: key]
    }
    pub unsafe fn encodeFloat_forKey(&self, value: c_float, key: &NSString) {
        msg_send![self, encodeFloat: value, forKey: key]
    }
    pub unsafe fn encodeDouble_forKey(&self, value: c_double, key: &NSString) {
        msg_send![self, encodeDouble: value, forKey: key]
    }
    pub unsafe fn encodeBytes_length_forKey(
        &self,
        bytes: *mut uint8_t,
        length: NSUInteger,
        key: &NSString,
    ) {
        msg_send![self, encodeBytes: bytes, length: length, forKey: key]
    }
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: TodoGenerics) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn outputFormat(&self) -> NSPropertyListFormat {
        msg_send![self, outputFormat]
    }
    pub unsafe fn setOutputFormat(&self, outputFormat: NSPropertyListFormat) {
        msg_send![self, setOutputFormat: outputFormat]
    }
    pub unsafe fn encodedData(&self) -> Id<NSData, Shared> {
        msg_send_id![self, encodedData]
    }
    pub unsafe fn requiresSecureCoding(&self) -> bool {
        msg_send![self, requiresSecureCoding]
    }
    pub unsafe fn setRequiresSecureCoding(&self, requiresSecureCoding: bool) {
        msg_send![self, setRequiresSecureCoding: requiresSecureCoding]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSKeyedUnarchiver;
    unsafe impl ClassType for NSKeyedUnarchiver {
        type Super = NSCoder;
    }
);
impl NSKeyedUnarchiver {
    pub unsafe fn initForReadingFromData_error(
        &self,
        data: &NSData,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initForReadingFromData: data, error: error]
    }
    pub unsafe fn unarchivedObjectOfClass_fromData_error(
        cls: &Class,
        data: &NSData,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            unarchivedObjectOfClass: cls,
            fromData: data,
            error: error
        ]
    }
    pub unsafe fn unarchivedArrayOfObjectsOfClass_fromData_error(
        cls: &Class,
        data: &NSData,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSArray, Shared>> {
        msg_send_id![
            Self::class(),
            unarchivedArrayOfObjectsOfClass: cls,
            fromData: data,
            error: error
        ]
    }
    pub unsafe fn unarchivedDictionaryWithKeysOfClass_objectsOfClass_fromData_error(
        keyCls: &Class,
        valueCls: &Class,
        data: &NSData,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![
            Self::class(),
            unarchivedDictionaryWithKeysOfClass: keyCls,
            objectsOfClass: valueCls,
            fromData: data,
            error: error
        ]
    }
    pub unsafe fn unarchivedObjectOfClasses_fromData_error(
        classes: TodoGenerics,
        data: &NSData,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            unarchivedObjectOfClasses: classes,
            fromData: data,
            error: error
        ]
    }
    pub unsafe fn unarchivedArrayOfObjectsOfClasses_fromData_error(
        classes: TodoGenerics,
        data: &NSData,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSArray, Shared>> {
        msg_send_id![
            Self::class(),
            unarchivedArrayOfObjectsOfClasses: classes,
            fromData: data,
            error: error
        ]
    }
    pub unsafe fn unarchivedDictionaryWithKeysOfClasses_objectsOfClasses_fromData_error(
        keyClasses: TodoGenerics,
        valueClasses: TodoGenerics,
        data: &NSData,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSDictionary, Shared>> {
        msg_send_id![
            Self::class(),
            unarchivedDictionaryWithKeysOfClasses: keyClasses,
            objectsOfClasses: valueClasses,
            fromData: data,
            error: error
        ]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initForReadingWithData(&self, data: &NSData) -> Id<Self, Shared> {
        msg_send_id![self, initForReadingWithData: data]
    }
    pub unsafe fn unarchiveObjectWithData(data: &NSData) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), unarchiveObjectWithData: data]
    }
    pub unsafe fn unarchiveTopLevelObjectWithData_error(
        data: &NSData,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![
            Self::class(),
            unarchiveTopLevelObjectWithData: data,
            error: error
        ]
    }
    pub unsafe fn unarchiveObjectWithFile(path: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![Self::class(), unarchiveObjectWithFile: path]
    }
    pub unsafe fn finishDecoding(&self) {
        msg_send![self, finishDecoding]
    }
    pub unsafe fn setClass_forClassName(cls: Option<&Class>, codedName: &NSString) {
        msg_send![Self::class(), setClass: cls, forClassName: codedName]
    }
    pub unsafe fn setClass_forClassName(&self, cls: Option<&Class>, codedName: &NSString) {
        msg_send![self, setClass: cls, forClassName: codedName]
    }
    pub unsafe fn classForClassName(codedName: &NSString) -> Option<&Class> {
        msg_send![Self::class(), classForClassName: codedName]
    }
    pub unsafe fn classForClassName(&self, codedName: &NSString) -> Option<&Class> {
        msg_send![self, classForClassName: codedName]
    }
    pub unsafe fn containsValueForKey(&self, key: &NSString) -> bool {
        msg_send![self, containsValueForKey: key]
    }
    pub unsafe fn decodeObjectForKey(&self, key: &NSString) -> Option<Id<Object, Shared>> {
        msg_send_id![self, decodeObjectForKey: key]
    }
    pub unsafe fn decodeBoolForKey(&self, key: &NSString) -> bool {
        msg_send![self, decodeBoolForKey: key]
    }
    pub unsafe fn decodeIntForKey(&self, key: &NSString) -> c_int {
        msg_send![self, decodeIntForKey: key]
    }
    pub unsafe fn decodeInt32ForKey(&self, key: &NSString) -> int32_t {
        msg_send![self, decodeInt32ForKey: key]
    }
    pub unsafe fn decodeInt64ForKey(&self, key: &NSString) -> int64_t {
        msg_send![self, decodeInt64ForKey: key]
    }
    pub unsafe fn decodeFloatForKey(&self, key: &NSString) -> c_float {
        msg_send![self, decodeFloatForKey: key]
    }
    pub unsafe fn decodeDoubleForKey(&self, key: &NSString) -> c_double {
        msg_send![self, decodeDoubleForKey: key]
    }
    pub unsafe fn decodeBytesForKey_returnedLength(
        &self,
        key: &NSString,
        lengthp: *mut NSUInteger,
    ) -> *mut uint8_t {
        msg_send![self, decodeBytesForKey: key, returnedLength: lengthp]
    }
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: TodoGenerics) {
        msg_send![self, setDelegate: delegate]
    }
    pub unsafe fn requiresSecureCoding(&self) -> bool {
        msg_send![self, requiresSecureCoding]
    }
    pub unsafe fn setRequiresSecureCoding(&self, requiresSecureCoding: bool) {
        msg_send![self, setRequiresSecureCoding: requiresSecureCoding]
    }
    pub unsafe fn decodingFailurePolicy(&self) -> NSDecodingFailurePolicy {
        msg_send![self, decodingFailurePolicy]
    }
    pub unsafe fn setDecodingFailurePolicy(&self, decodingFailurePolicy: NSDecodingFailurePolicy) {
        msg_send![self, setDecodingFailurePolicy: decodingFailurePolicy]
    }
}
#[doc = "NSKeyedArchiverObjectSubstitution"]
impl NSObject {
    pub unsafe fn replacementObjectForKeyedArchiver(
        &self,
        archiver: &NSKeyedArchiver,
    ) -> Option<Id<Object, Shared>> {
        msg_send_id![self, replacementObjectForKeyedArchiver: archiver]
    }
    pub unsafe fn classFallbacksForKeyedArchiver() -> TodoGenerics {
        msg_send![Self::class(), classFallbacksForKeyedArchiver]
    }
    pub unsafe fn classForKeyedArchiver(&self) -> Option<&Class> {
        msg_send![self, classForKeyedArchiver]
    }
}
#[doc = "NSKeyedUnarchiverObjectSubstitution"]
impl NSObject {
    pub unsafe fn classForKeyedUnarchiver() -> &Class {
        msg_send![Self::class(), classForKeyedUnarchiver]
    }
}
