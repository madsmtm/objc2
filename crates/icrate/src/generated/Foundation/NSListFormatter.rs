#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSListFormatter;
    unsafe impl ClassType for NSListFormatter {
        type Super = NSFormatter;
    }
);
impl NSListFormatter {
    pub unsafe fn localizedStringByJoiningStrings(strings: TodoGenerics) -> Id<NSString, Shared> {
        msg_send_id![Self::class(), localizedStringByJoiningStrings: strings]
    }
    pub unsafe fn stringFromItems(&self, items: &NSArray) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringFromItems: items]
    }
    pub unsafe fn stringForObjectValue(
        &self,
        obj: Option<&Object>,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringForObjectValue: obj]
    }
    pub unsafe fn locale(&self) -> Id<NSLocale, Shared> {
        msg_send_id![self, locale]
    }
    pub unsafe fn setLocale(&self, locale: Option<&NSLocale>) {
        msg_send![self, setLocale: locale]
    }
    pub unsafe fn itemFormatter(&self) -> Option<Id<NSFormatter, Shared>> {
        msg_send_id![self, itemFormatter]
    }
    pub unsafe fn setItemFormatter(&self, itemFormatter: Option<&NSFormatter>) {
        msg_send![self, setItemFormatter: itemFormatter]
    }
}
