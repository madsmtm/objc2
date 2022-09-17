use super::__exported::NSDictionary;
use super::__exported::NSString;
use super::__exported::NSURLCredential;
use super::__exported::NSURLCredentialStorageInternal;
use super::__exported::NSURLSessionTask;
use crate::Foundation::generated::NSNotification::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSURLProtectionSpace::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSURLCredentialStorage;
    unsafe impl ClassType for NSURLCredentialStorage {
        type Super = NSObject;
    }
);
impl NSURLCredentialStorage {
    pub unsafe fn credentialsForProtectionSpace(
        &self,
        space: &NSURLProtectionSpace,
    ) -> TodoGenerics {
        msg_send![self, credentialsForProtectionSpace: space]
    }
    pub unsafe fn setCredential_forProtectionSpace(
        &self,
        credential: &NSURLCredential,
        space: &NSURLProtectionSpace,
    ) {
        msg_send![self, setCredential: credential, forProtectionSpace: space]
    }
    pub unsafe fn removeCredential_forProtectionSpace(
        &self,
        credential: &NSURLCredential,
        space: &NSURLProtectionSpace,
    ) {
        msg_send![
            self,
            removeCredential: credential,
            forProtectionSpace: space
        ]
    }
    pub unsafe fn removeCredential_forProtectionSpace_options(
        &self,
        credential: &NSURLCredential,
        space: &NSURLProtectionSpace,
        options: TodoGenerics,
    ) {
        msg_send![
            self,
            removeCredential: credential,
            forProtectionSpace: space,
            options: options
        ]
    }
    pub unsafe fn defaultCredentialForProtectionSpace(
        &self,
        space: &NSURLProtectionSpace,
    ) -> Option<Id<NSURLCredential, Shared>> {
        msg_send_id![self, defaultCredentialForProtectionSpace: space]
    }
    pub unsafe fn setDefaultCredential_forProtectionSpace(
        &self,
        credential: &NSURLCredential,
        space: &NSURLProtectionSpace,
    ) {
        msg_send![
            self,
            setDefaultCredential: credential,
            forProtectionSpace: space
        ]
    }
    pub unsafe fn sharedCredentialStorage() -> Id<NSURLCredentialStorage, Shared> {
        msg_send_id![Self::class(), sharedCredentialStorage]
    }
    pub unsafe fn allCredentials(&self) -> TodoGenerics {
        msg_send![self, allCredentials]
    }
}
#[doc = "NSURLSessionTaskAdditions"]
impl NSURLCredentialStorage {
    pub unsafe fn getCredentialsForProtectionSpace_task_completionHandler(
        &self,
        protectionSpace: &NSURLProtectionSpace,
        task: &NSURLSessionTask,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            self,
            getCredentialsForProtectionSpace: protectionSpace,
            task: task,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn setCredential_forProtectionSpace_task(
        &self,
        credential: &NSURLCredential,
        protectionSpace: &NSURLProtectionSpace,
        task: &NSURLSessionTask,
    ) {
        msg_send![
            self,
            setCredential: credential,
            forProtectionSpace: protectionSpace,
            task: task
        ]
    }
    pub unsafe fn removeCredential_forProtectionSpace_options_task(
        &self,
        credential: &NSURLCredential,
        protectionSpace: &NSURLProtectionSpace,
        options: TodoGenerics,
        task: &NSURLSessionTask,
    ) {
        msg_send![
            self,
            removeCredential: credential,
            forProtectionSpace: protectionSpace,
            options: options,
            task: task
        ]
    }
    pub unsafe fn getDefaultCredentialForProtectionSpace_task_completionHandler(
        &self,
        space: &NSURLProtectionSpace,
        task: &NSURLSessionTask,
        completionHandler: TodoBlock,
    ) {
        msg_send![
            self,
            getDefaultCredentialForProtectionSpace: space,
            task: task,
            completionHandler: completionHandler
        ]
    }
    pub unsafe fn setDefaultCredential_forProtectionSpace_task(
        &self,
        credential: &NSURLCredential,
        protectionSpace: &NSURLProtectionSpace,
        task: &NSURLSessionTask,
    ) {
        msg_send![
            self,
            setDefaultCredential: credential,
            forProtectionSpace: protectionSpace,
            task: task
        ]
    }
}
