#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
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
        #[method_id(initWithRequest:delegate:)]
        pub unsafe fn initWithRequest_delegate(
            &self,
            request: &NSURLRequest,
            delegate: Option<&NSURLDownloadDelegate>,
        ) -> Id<Self, Shared>;
        #[method_id(initWithResumeData:delegate:path:)]
        pub unsafe fn initWithResumeData_delegate_path(
            &self,
            resumeData: &NSData,
            delegate: Option<&NSURLDownloadDelegate>,
            path: &NSString,
        ) -> Id<Self, Shared>;
        #[method(cancel)]
        pub unsafe fn cancel(&self);
        #[method(setDestination:allowOverwrite:)]
        pub unsafe fn setDestination_allowOverwrite(&self, path: &NSString, allowOverwrite: bool);
        #[method_id(request)]
        pub unsafe fn request(&self) -> Id<NSURLRequest, Shared>;
        #[method_id(resumeData)]
        pub unsafe fn resumeData(&self) -> Option<Id<NSData, Shared>>;
        #[method(deletesFileUponFailure)]
        pub unsafe fn deletesFileUponFailure(&self) -> bool;
        #[method(setDeletesFileUponFailure:)]
        pub unsafe fn setDeletesFileUponFailure(&self, deletesFileUponFailure: bool);
    }
);
pub type NSURLDownloadDelegate = NSObject;
