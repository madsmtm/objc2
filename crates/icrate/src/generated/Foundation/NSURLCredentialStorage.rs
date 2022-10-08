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
use objc2::{extern_class, extern_methods, ClassType};
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
        # [method_id (credentialsForProtectionSpace :)]
        pub unsafe fn credentialsForProtectionSpace(
            &self,
            space: &NSURLProtectionSpace,
        ) -> Option<Id<NSDictionary<NSString, NSURLCredential>, Shared>>;
        #[method_id(allCredentials)]
        pub unsafe fn allCredentials(
            &self,
        ) -> Id<NSDictionary<NSURLProtectionSpace, NSDictionary<NSString, NSURLCredential>>, Shared>;
        # [method (setCredential : forProtectionSpace :)]
        pub unsafe fn setCredential_forProtectionSpace(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
        );
        # [method (removeCredential : forProtectionSpace :)]
        pub unsafe fn removeCredential_forProtectionSpace(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
        );
        # [method (removeCredential : forProtectionSpace : options :)]
        pub unsafe fn removeCredential_forProtectionSpace_options(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
            options: Option<&NSDictionary<NSString, Object>>,
        );
        # [method_id (defaultCredentialForProtectionSpace :)]
        pub unsafe fn defaultCredentialForProtectionSpace(
            &self,
            space: &NSURLProtectionSpace,
        ) -> Option<Id<NSURLCredential, Shared>>;
        # [method (setDefaultCredential : forProtectionSpace :)]
        pub unsafe fn setDefaultCredential_forProtectionSpace(
            &self,
            credential: &NSURLCredential,
            space: &NSURLProtectionSpace,
        );
    }
);
extern_methods!(
    #[doc = "NSURLSessionTaskAdditions"]
    unsafe impl NSURLCredentialStorage {
        # [method (getCredentialsForProtectionSpace : task : completionHandler :)]
        pub unsafe fn getCredentialsForProtectionSpace_task_completionHandler(
            &self,
            protectionSpace: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
            completionHandler: TodoBlock,
        );
        # [method (setCredential : forProtectionSpace : task :)]
        pub unsafe fn setCredential_forProtectionSpace_task(
            &self,
            credential: &NSURLCredential,
            protectionSpace: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
        );
        # [method (removeCredential : forProtectionSpace : options : task :)]
        pub unsafe fn removeCredential_forProtectionSpace_options_task(
            &self,
            credential: &NSURLCredential,
            protectionSpace: &NSURLProtectionSpace,
            options: Option<&NSDictionary<NSString, Object>>,
            task: &NSURLSessionTask,
        );
        # [method (getDefaultCredentialForProtectionSpace : task : completionHandler :)]
        pub unsafe fn getDefaultCredentialForProtectionSpace_task_completionHandler(
            &self,
            space: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
            completionHandler: TodoBlock,
        );
        # [method (setDefaultCredential : forProtectionSpace : task :)]
        pub unsafe fn setDefaultCredential_forProtectionSpace_task(
            &self,
            credential: &NSURLCredential,
            protectionSpace: &NSURLProtectionSpace,
            task: &NSURLSessionTask,
        );
    }
);
