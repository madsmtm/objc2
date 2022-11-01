//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSURLResponse;

    unsafe impl ClassType for NSURLResponse {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSURLResponse {
        #[method_id(initWithURL:MIMEType:expectedContentLength:textEncodingName:)]
        pub unsafe fn initWithURL_MIMEType_expectedContentLength_textEncodingName(
            this: Option<Allocated<Self>>,
            URL: &NSURL,
            MIMEType: Option<&NSString>,
            length: NSInteger,
            name: Option<&NSString>,
        ) -> Id<Self, Shared>;

        #[method_id(URL)]
        pub unsafe fn URL(&self) -> Option<Id<NSURL, Shared>>;

        #[method_id(MIMEType)]
        pub unsafe fn MIMEType(&self) -> Option<Id<NSString, Shared>>;

        #[method(expectedContentLength)]
        pub unsafe fn expectedContentLength(&self) -> c_longlong;

        #[method_id(textEncodingName)]
        pub unsafe fn textEncodingName(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(suggestedFilename)]
        pub unsafe fn suggestedFilename(&self) -> Option<Id<NSString, Shared>>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSHTTPURLResponse;

    unsafe impl ClassType for NSHTTPURLResponse {
        type Super = NSURLResponse;
    }
);

extern_methods!(
    unsafe impl NSHTTPURLResponse {
        #[method_id(initWithURL:statusCode:HTTPVersion:headerFields:)]
        pub unsafe fn initWithURL_statusCode_HTTPVersion_headerFields(
            this: Option<Allocated<Self>>,
            url: &NSURL,
            statusCode: NSInteger,
            HTTPVersion: Option<&NSString>,
            headerFields: Option<&NSDictionary<NSString, NSString>>,
        ) -> Option<Id<Self, Shared>>;

        #[method(statusCode)]
        pub unsafe fn statusCode(&self) -> NSInteger;

        #[method_id(allHeaderFields)]
        pub unsafe fn allHeaderFields(&self) -> Id<NSDictionary, Shared>;

        #[method_id(valueForHTTPHeaderField:)]
        pub unsafe fn valueForHTTPHeaderField(
            &self,
            field: &NSString,
        ) -> Option<Id<NSString, Shared>>;

        #[method_id(localizedStringForStatusCode:)]
        pub unsafe fn localizedStringForStatusCode(statusCode: NSInteger) -> Id<NSString, Shared>;
    }
);
