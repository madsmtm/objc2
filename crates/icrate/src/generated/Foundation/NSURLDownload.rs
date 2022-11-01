//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSURLDownload;

    unsafe impl ClassType for NSURLDownload {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSURLDownload {
        #[method(canResumeDownloadDecodedWithEncodingMIMEType:)]
        pub unsafe fn canResumeDownloadDecodedWithEncodingMIMEType(MIMEType: &NSString) -> bool;

        #[method_id(@__retain_semantics Init initWithRequest:delegate:)]
        pub unsafe fn initWithRequest_delegate(
            this: Option<Allocated<Self>>,
            request: &NSURLRequest,
            delegate: Option<&NSURLDownloadDelegate>,
        ) -> Id<Self, Shared>;

        #[method_id(@__retain_semantics Init initWithResumeData:delegate:path:)]
        pub unsafe fn initWithResumeData_delegate_path(
            this: Option<Allocated<Self>>,
            resumeData: &NSData,
            delegate: Option<&NSURLDownloadDelegate>,
            path: &NSString,
        ) -> Id<Self, Shared>;

        #[method(cancel)]
        pub unsafe fn cancel(&self);

        #[method(setDestination:allowOverwrite:)]
        pub unsafe fn setDestination_allowOverwrite(&self, path: &NSString, allowOverwrite: bool);

        #[method_id(@__retain_semantics Other request)]
        pub unsafe fn request(&self) -> Id<NSURLRequest, Shared>;

        #[method_id(@__retain_semantics Other resumeData)]
        pub unsafe fn resumeData(&self) -> Option<Id<NSData, Shared>>;

        #[method(deletesFileUponFailure)]
        pub unsafe fn deletesFileUponFailure(&self) -> bool;

        #[method(setDeletesFileUponFailure:)]
        pub unsafe fn setDeletesFileUponFailure(&self, deletesFileUponFailure: bool);
    }
);

pub type NSURLDownloadDelegate = NSObject;
