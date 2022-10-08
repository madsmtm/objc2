use super::__exported::NSData;
use super::__exported::NSError;
use super::__exported::NSString;
use super::__exported::NSURLAuthenticationChallenge;
use super::__exported::NSURLDownloadInternal;
use super::__exported::NSURLProtectionSpace;
use super::__exported::NSURLRequest;
use super::__exported::NSURLResponse;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLDownload;
    unsafe impl ClassType for NSURLDownload {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSURLDownload {
        pub unsafe fn canResumeDownloadDecodedWithEncodingMIMEType(MIMEType: &NSString) -> bool {
            msg_send![
                Self::class(),
                canResumeDownloadDecodedWithEncodingMIMEType: MIMEType
            ]
        }
        pub unsafe fn initWithRequest_delegate(
            &self,
            request: &NSURLRequest,
            delegate: Option<&NSURLDownloadDelegate>,
        ) -> Id<Self, Shared> {
            msg_send_id![self, initWithRequest: request, delegate: delegate]
        }
        pub unsafe fn initWithResumeData_delegate_path(
            &self,
            resumeData: &NSData,
            delegate: Option<&NSURLDownloadDelegate>,
            path: &NSString,
        ) -> Id<Self, Shared> {
            msg_send_id![
                self,
                initWithResumeData: resumeData,
                delegate: delegate,
                path: path
            ]
        }
        pub unsafe fn cancel(&self) {
            msg_send![self, cancel]
        }
        pub unsafe fn setDestination_allowOverwrite(&self, path: &NSString, allowOverwrite: bool) {
            msg_send![self, setDestination: path, allowOverwrite: allowOverwrite]
        }
        pub unsafe fn request(&self) -> Id<NSURLRequest, Shared> {
            msg_send_id![self, request]
        }
        pub unsafe fn resumeData(&self) -> Option<Id<NSData, Shared>> {
            msg_send_id![self, resumeData]
        }
        pub unsafe fn deletesFileUponFailure(&self) -> bool {
            msg_send![self, deletesFileUponFailure]
        }
        pub unsafe fn setDeletesFileUponFailure(&self, deletesFileUponFailure: bool) {
            msg_send![self, setDeletesFileUponFailure: deletesFileUponFailure]
        }
    }
);
pub type NSURLDownloadDelegate = NSObject;
