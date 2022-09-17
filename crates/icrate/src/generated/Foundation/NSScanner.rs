extern_class!(
    #[derive(Debug)]
    struct NSScanner;
    unsafe impl ClassType for NSScanner {
        type Super = NSObject;
    }
);
impl NSScanner {
    pub unsafe fn initWithString(&self, string: &NSString) -> Id<Self, Shared> {
        msg_send_id![self, initWithString: string]
    }
    pub unsafe fn string(&self) -> Id<NSString, Shared> {
        msg_send_id![self, string]
    }
    pub unsafe fn scanLocation(&self) -> NSUInteger {
        msg_send![self, scanLocation]
    }
    pub unsafe fn setScanLocation(&self, scanLocation: NSUInteger) {
        msg_send![self, setScanLocation: scanLocation]
    }
    pub unsafe fn charactersToBeSkipped(&self) -> Option<Id<NSCharacterSet, Shared>> {
        msg_send_id![self, charactersToBeSkipped]
    }
    pub unsafe fn setCharactersToBeSkipped(&self, charactersToBeSkipped: Option<&NSCharacterSet>) {
        msg_send![self, setCharactersToBeSkipped: charactersToBeSkipped]
    }
    pub unsafe fn caseSensitive(&self) -> bool {
        msg_send![self, caseSensitive]
    }
    pub unsafe fn setCaseSensitive(&self, caseSensitive: bool) {
        msg_send![self, setCaseSensitive: caseSensitive]
    }
    pub unsafe fn locale(&self) -> Option<Id<Object, Shared>> {
        msg_send_id![self, locale]
    }
    pub unsafe fn setLocale(&self, locale: Option<&Object>) {
        msg_send![self, setLocale: locale]
    }
}
#[doc = "NSExtendedScanner"]
impl NSScanner {
    pub unsafe fn scanInt(&self, result: *mut c_int) -> bool {
        msg_send![self, scanInt: result]
    }
    pub unsafe fn scanInteger(&self, result: *mut NSInteger) -> bool {
        msg_send![self, scanInteger: result]
    }
    pub unsafe fn scanLongLong(&self, result: *mut c_longlong) -> bool {
        msg_send![self, scanLongLong: result]
    }
    pub unsafe fn scanUnsignedLongLong(&self, result: *mut c_ulonglong) -> bool {
        msg_send![self, scanUnsignedLongLong: result]
    }
    pub unsafe fn scanFloat(&self, result: *mut c_float) -> bool {
        msg_send![self, scanFloat: result]
    }
    pub unsafe fn scanDouble(&self, result: *mut c_double) -> bool {
        msg_send![self, scanDouble: result]
    }
    pub unsafe fn scanHexInt(&self, result: *mut c_uint) -> bool {
        msg_send![self, scanHexInt: result]
    }
    pub unsafe fn scanHexLongLong(&self, result: *mut c_ulonglong) -> bool {
        msg_send![self, scanHexLongLong: result]
    }
    pub unsafe fn scanHexFloat(&self, result: *mut c_float) -> bool {
        msg_send![self, scanHexFloat: result]
    }
    pub unsafe fn scanHexDouble(&self, result: *mut c_double) -> bool {
        msg_send![self, scanHexDouble: result]
    }
    pub unsafe fn scanString_intoString(
        &self,
        string: &NSString,
        result: *mut Option<&NSString>,
    ) -> bool {
        msg_send![self, scanString: string, intoString: result]
    }
    pub unsafe fn scanCharactersFromSet_intoString(
        &self,
        set: &NSCharacterSet,
        result: *mut Option<&NSString>,
    ) -> bool {
        msg_send![self, scanCharactersFromSet: set, intoString: result]
    }
    pub unsafe fn scanUpToString_intoString(
        &self,
        string: &NSString,
        result: *mut Option<&NSString>,
    ) -> bool {
        msg_send![self, scanUpToString: string, intoString: result]
    }
    pub unsafe fn scanUpToCharactersFromSet_intoString(
        &self,
        set: &NSCharacterSet,
        result: *mut Option<&NSString>,
    ) -> bool {
        msg_send![self, scanUpToCharactersFromSet: set, intoString: result]
    }
    pub unsafe fn scannerWithString(string: &NSString) -> Id<Self, Shared> {
        msg_send_id![Self::class(), scannerWithString: string]
    }
    pub unsafe fn localizedScannerWithString(string: &NSString) -> Id<Object, Shared> {
        msg_send_id![Self::class(), localizedScannerWithString: string]
    }
    pub unsafe fn isAtEnd(&self) -> bool {
        msg_send![self, isAtEnd]
    }
}
