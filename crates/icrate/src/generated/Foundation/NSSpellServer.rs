use super::__exported::NSArray;
use super::__exported::NSDictionary;
use super::__exported::NSOrthography;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
use crate::Foundation::generated::NSTextCheckingResult::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSpellServer;
    unsafe impl ClassType for NSSpellServer {
        type Super = NSObject;
    }
);
impl NSSpellServer {
    pub unsafe fn registerLanguage_byVendor(
        &self,
        language: Option<&NSString>,
        vendor: Option<&NSString>,
    ) -> bool {
        msg_send![self, registerLanguage: language, byVendor: vendor]
    }
    pub unsafe fn isWordInUserDictionaries_caseSensitive(
        &self,
        word: &NSString,
        flag: bool,
    ) -> bool {
        msg_send![self, isWordInUserDictionaries: word, caseSensitive: flag]
    }
    pub unsafe fn run(&self) {
        msg_send![self, run]
    }
    pub unsafe fn delegate(&self) -> Option<Id<id, Shared>> {
        msg_send_id![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: Option<&id>) {
        msg_send![self, setDelegate: delegate]
    }
}
pub type NSSpellServerDelegate = NSObject;
