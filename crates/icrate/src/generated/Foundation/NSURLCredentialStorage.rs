//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::Foundation::*;

extern_class!(
    #[derive(Debug)]
    pub struct NSURLCredentialStorage;

    unsafe impl ClassType for NSURLCredentialStorage {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSURLCredentialStorage {
        #[method_id(sharedCredentialStorage)]
        pub unsafe fn sharedCredentialStorage() -> Id<NSURLCredentialStorage, Shared>;

        #[method_id(credentialsForProtectionSpace:)]
        pub unsafe fn credentialsForProtectionSpace(
            &self,
            space: &NSURLProtectionSpace,
        ) -> Option<Id<NSDictionary<NSString, NSURLCredential>, Shared>>;

        #[method_id(allCredentials)]
        pub unsafe fn allCredentials(
            &self,
        ) -> Id<NSDictionary<NSURLProtectionSpace, NSDictionary<NSString, NSURLCredential>>, Shared>;

        #[method(setCredential:forProtectionSpace:)]
        pub unsafe fn setCredential_forProtectionSpace(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
        );

        #[method(removeCredential:forProtectionSpace:)]
        pub unsafe fn removeCredential_forProtectionSpace(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
        );

        #[method(removeCredential:forProtectionSpace:options:)]
        pub unsafe fn removeCredential_forProtectionSpace_options(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
            options: Option<&NSDictionary<NSString, Object>>,
        );

        #[method_id(defaultCredentialForProtectionSpace:)]
        pub unsafe fn defaultCredentialForProtectionSpace(
            &self,
            space: &NSURLProtectionSpace,
        ) -> Option<Id<NSURLCredential, Shared>>;

        #[method(setDefaultCredential:forProtectionSpace:)]
        pub unsafe fn setDefaultCredential_forProtectionSpace(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
        );
    }
);

extern_methods!(
    /// NSURLSessionTaskAdditions
    unsafe impl NSURLCredentialStorage {
        #[method(getCredentialsForProtectionSpace:task:completionHandler:)]
        pub unsafe fn getCredentialsForProtectionSpace_task_completionHandler(
            &self,
            protectionSpace: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
            completionHandler: TodoBlock,
        );

        #[method(setCredential:forProtectionSpace:task:)]
        pub unsafe fn setCredential_forProtectionSpace_task(
            &self,
            credential: &NSURLCredential,
            protectionSpace: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
        );

        #[method(removeCredential:forProtectionSpace:options:task:)]
        pub unsafe fn removeCredential_forProtectionSpace_options_task(
            &self,
            credential: &NSURLCredential,
            protectionSpace: &NSURLProtectionSpace,
            options: Option<&NSDictionary<NSString, Object>>,
            task: &NSURLSessionTask,
        );

        #[method(getDefaultCredentialForProtectionSpace:task:completionHandler:)]
        pub unsafe fn getDefaultCredentialForProtectionSpace_task_completionHandler(
            &self,
            space: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
            completionHandler: TodoBlock,
        );

        #[method(setDefaultCredential:forProtectionSpace:task:)]
        pub unsafe fn setDefaultCredential_forProtectionSpace_task(
            &self,
            credential: &NSURLCredential,
            protectionSpace: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
        );
    }
);

extern "C" {
    static NSURLCredentialStorageChangedNotification: &'static NSNotificationName;
}

extern "C" {
    static NSURLCredentialStorageRemoveSynchronizableCredentials: &'static NSString;
}
