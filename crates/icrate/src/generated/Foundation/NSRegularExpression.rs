extern_class!(
    #[derive(Debug)]
    struct NSRegularExpression;
    unsafe impl ClassType for NSRegularExpression {
        type Super = NSObject;
    }
);
impl NSRegularExpression {
    pub unsafe fn regularExpressionWithPattern_options_error(
        pattern: &NSString,
        options: NSRegularExpressionOptions,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSRegularExpression, Shared>> {
        msg_send_id![
            Self::class(),
            regularExpressionWithPattern: pattern,
            options: options,
            error: error
        ]
    }
    pub unsafe fn initWithPattern_options_error(
        &self,
        pattern: &NSString,
        options: NSRegularExpressionOptions,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithPattern: pattern,
            options: options,
            error: error
        ]
    }
    pub unsafe fn escapedPatternForString(string: &NSString) -> Id<NSString, Shared> {
        msg_send_id![Self::class(), escapedPatternForString: string]
    }
    pub unsafe fn pattern(&self) -> Id<NSString, Shared> {
        msg_send_id![self, pattern]
    }
    pub unsafe fn options(&self) -> NSRegularExpressionOptions {
        msg_send![self, options]
    }
    pub unsafe fn numberOfCaptureGroups(&self) -> NSUInteger {
        msg_send![self, numberOfCaptureGroups]
    }
}
#[doc = "NSMatching"]
impl NSRegularExpression {
    pub unsafe fn enumerateMatchesInString_options_range_usingBlock(
        &self,
        string: &NSString,
        options: NSMatchingOptions,
        range: NSRange,
        block: TodoBlock,
    ) {
        msg_send![
            self,
            enumerateMatchesInString: string,
            options: options,
            range: range,
            usingBlock: block
        ]
    }
    pub unsafe fn matchesInString_options_range(
        &self,
        string: &NSString,
        options: NSMatchingOptions,
        range: NSRange,
    ) -> TodoGenerics {
        msg_send![
            self,
            matchesInString: string,
            options: options,
            range: range
        ]
    }
    pub unsafe fn numberOfMatchesInString_options_range(
        &self,
        string: &NSString,
        options: NSMatchingOptions,
        range: NSRange,
    ) -> NSUInteger {
        msg_send![
            self,
            numberOfMatchesInString: string,
            options: options,
            range: range
        ]
    }
    pub unsafe fn firstMatchInString_options_range(
        &self,
        string: &NSString,
        options: NSMatchingOptions,
        range: NSRange,
    ) -> Option<Id<NSTextCheckingResult, Shared>> {
        msg_send_id![
            self,
            firstMatchInString: string,
            options: options,
            range: range
        ]
    }
    pub unsafe fn rangeOfFirstMatchInString_options_range(
        &self,
        string: &NSString,
        options: NSMatchingOptions,
        range: NSRange,
    ) -> NSRange {
        msg_send![
            self,
            rangeOfFirstMatchInString: string,
            options: options,
            range: range
        ]
    }
}
#[doc = "NSReplacement"]
impl NSRegularExpression {
    pub unsafe fn stringByReplacingMatchesInString_options_range_withTemplate(
        &self,
        string: &NSString,
        options: NSMatchingOptions,
        range: NSRange,
        templ: &NSString,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            self,
            stringByReplacingMatchesInString: string,
            options: options,
            range: range,
            withTemplate: templ
        ]
    }
    pub unsafe fn replaceMatchesInString_options_range_withTemplate(
        &self,
        string: &NSMutableString,
        options: NSMatchingOptions,
        range: NSRange,
        templ: &NSString,
    ) -> NSUInteger {
        msg_send![
            self,
            replaceMatchesInString: string,
            options: options,
            range: range,
            withTemplate: templ
        ]
    }
    pub unsafe fn replacementStringForResult_inString_offset_template(
        &self,
        result: &NSTextCheckingResult,
        string: &NSString,
        offset: NSInteger,
        templ: &NSString,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            self,
            replacementStringForResult: result,
            inString: string,
            offset: offset,
            template: templ
        ]
    }
    pub unsafe fn escapedTemplateForString(string: &NSString) -> Id<NSString, Shared> {
        msg_send_id![Self::class(), escapedTemplateForString: string]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSDataDetector;
    unsafe impl ClassType for NSDataDetector {
        type Super = NSRegularExpression;
    }
);
impl NSDataDetector {
    pub unsafe fn dataDetectorWithTypes_error(
        checkingTypes: NSTextCheckingTypes,
        error: *mut Option<&NSError>,
    ) -> Option<Id<NSDataDetector, Shared>> {
        msg_send_id![
            Self::class(),
            dataDetectorWithTypes: checkingTypes,
            error: error
        ]
    }
    pub unsafe fn initWithTypes_error(
        &self,
        checkingTypes: NSTextCheckingTypes,
        error: *mut Option<&NSError>,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithTypes: checkingTypes, error: error]
    }
    pub unsafe fn checkingTypes(&self) -> NSTextCheckingTypes {
        msg_send![self, checkingTypes]
    }
}
