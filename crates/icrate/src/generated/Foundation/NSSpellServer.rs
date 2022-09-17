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
    pub unsafe fn delegate(&self) -> TodoGenerics {
        msg_send![self, delegate]
    }
    pub unsafe fn setDelegate(&self, delegate: TodoGenerics) {
        msg_send![self, setDelegate: delegate]
    }
}
