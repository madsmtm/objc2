extern_class!(
    #[derive(Debug)]
    struct NSURLResponse;
    unsafe impl ClassType for NSURLResponse {
        type Super = NSObject;
    }
);
impl NSURLResponse {
    pub unsafe fn initWithURL_MIMEType_expectedContentLength_textEncodingName(
        &self,
        URL: &NSURL,
        MIMEType: Option<&NSString>,
        length: NSInteger,
        name: Option<&NSString>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithURL: URL,
            MIMEType: MIMEType,
            expectedContentLength: length,
            textEncodingName: name
        ]
    }
    pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>> {
        msg_send_id![self, URL]
    }
    pub unsafe fn MIMEType(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, MIMEType]
    }
    pub unsafe fn expectedContentLength(&self) -> c_longlong {
        msg_send![self, expectedContentLength]
    }
    pub unsafe fn textEncodingName(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, textEncodingName]
    }
    pub unsafe fn suggestedFilename(&self) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, suggestedFilename]
    }
}
extern_class!(
    #[derive(Debug)]
    struct NSHTTPURLResponse;
    unsafe impl ClassType for NSHTTPURLResponse {
        type Super = NSURLResponse;
    }
);
impl NSHTTPURLResponse {
    pub unsafe fn initWithURL_statusCode_HTTPVersion_headerFields(
        &self,
        url: &NSURL,
        statusCode: NSInteger,
        HTTPVersion: Option<&NSString>,
        headerFields: TodoGenerics,
    ) -> Option<Id<Self, Shared>> {
        msg_send_id![
            self,
            initWithURL: url,
            statusCode: statusCode,
            HTTPVersion: HTTPVersion,
            headerFields: headerFields
        ]
    }
    pub unsafe fn valueForHTTPHeaderField(&self, field: &NSString) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, valueForHTTPHeaderField: field]
    }
    pub unsafe fn localizedStringForStatusCode(statusCode: NSInteger) -> Id<NSString, Shared> {
        msg_send_id![Self::class(), localizedStringForStatusCode: statusCode]
    }
    pub unsafe fn statusCode(&self) -> NSInteger {
        msg_send![self, statusCode]
    }
    pub unsafe fn allHeaderFields(&self) -> Id<NSDictionary, Shared> {
        msg_send_id![self, allHeaderFields]
    }
}
