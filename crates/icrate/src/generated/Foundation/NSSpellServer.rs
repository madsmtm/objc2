use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSOrthography;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
use crate::Foundation::generated::NSTextCheckingResult::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSpellServer;
    unsafe impl ClassType for NSSpellServer {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSSpellServer {
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSpellServerDelegate, Shared>>;
        # [method (setDelegate :)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSpellServerDelegate>);
        # [method (registerLanguage : byVendor :)]
        pub unsafe fn registerLanguage_byVendor(
            &self,
            language: Option<&NSString>,
            vendor: Option<&NSString>,
        ) -> bool;
        # [method (isWordInUserDictionaries : caseSensitive :)]
        pub unsafe fn isWordInUserDictionaries_caseSensitive(
            &self,
            word: &NSString,
            flag: bool,
        ) -> bool;
        #[method(run)]
        pub unsafe fn run(&self);
    }
);
pub type NSSpellServerDelegate = NSObject;
