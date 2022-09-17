extern_class!(
    #[derive(Debug)]
    struct NSURLDownload;
    unsafe impl ClassType for NSURLDownload {
        type Super = NSObject;
    }
);
impl NSURLDownload {
    pub unsafe fn canResumeDownloadDecodedWithEncodingMIMEType(MIMEType: &NSString) -> bool {
        msg_send![
            Self::class(),
            canResumeDownloadDecodedWithEncodingMIMEType: MIMEType
        ]
    }
    pub unsafe fn initWithRequest_delegate(
        &self,
        request: &NSURLRequest,
        delegate: TodoGenerics,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initWithRequest: request, delegate: delegate]
    }
    pub unsafe fn initWithResumeData_delegate_path(
        &self,
        resumeData: &NSData,
        delegate: TodoGenerics,
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
