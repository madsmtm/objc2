#[doc = "NSCoderMethods"]
impl NSObject {
    pub unsafe fn version() -> NSInteger {
        msg_send![Self::class(), version]
    }
    pub unsafe fn setVersion(aVersion: NSInteger) {
        msg_send![Self::class(), setVersion: aVersion]
    }
    pub unsafe fn replacementObjectForCoder(&self, coder: &NSCoder) -> Option<Id<Object, Shared>> {
        msg_send_id![self, replacementObjectForCoder: coder]
    }
    pub unsafe fn awakeAfterUsingCoder(&self, coder: &NSCoder) -> Option<Id<Object, Shared>> {
        msg_send_id![self, awakeAfterUsingCoder: coder]
    }
    pub unsafe fn classForCoder(&self) -> &Class {
        msg_send![self, classForCoder]
    }
}
#[doc = "NSDeprecatedMethods"]
impl NSObject {
    pub unsafe fn poseAsClass(aClass: &Class) {
        msg_send![Self::class(), poseAsClass: aClass]
    }
}
#[doc = "NSDiscardableContentProxy"]
impl NSObject {
    pub unsafe fn autoContentAccessingProxy(&self) -> Id<Object, Shared> {
        msg_send_id![self, autoContentAccessingProxy]
    }
}
