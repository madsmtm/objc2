use super::__exported::NSCoder;
use super::__exported::NSEnumerator;
use super::__exported::NSInvocation;
use super::__exported::NSMethodSignature;
use super::__exported::NSString;
use super::__exported::Protocol;
use crate::objc::generated::NSObject::*;
use crate::Foundation::generated::NSObjCRuntime::*;
use crate::Foundation::generated::NSZone::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
pub type NSCopying = NSObject;
pub type NSMutableCopying = NSObject;
pub type NSCoding = NSObject;
pub type NSSecureCoding = NSObject;
#[doc = "NSCoderMethods"]
impl NSObject {
    pub unsafe fn version() -> NSInteger {
        msg_send![Self::class(), version]
    }
    pub unsafe fn setVersion(aVersion: NSInteger) {
        msg_send![Self::class(), setVersion: aVersion]
    }
    pub unsafe fn classForCoder(&self) -> &Class {
        msg_send![self, classForCoder]
    }
    pub unsafe fn replacementObjectForCoder(&self, coder: &NSCoder) -> Option<Id<Object, Shared>> {
        msg_send_id![self, replacementObjectForCoder: coder]
    }
}
#[doc = "NSDeprecatedMethods"]
impl NSObject {
    pub unsafe fn poseAsClass(aClass: &Class) {
        msg_send![Self::class(), poseAsClass: aClass]
    }
}
pub type NSDiscardableContent = NSObject;
#[doc = "NSDiscardableContentProxy"]
impl NSObject {
    pub unsafe fn autoContentAccessingProxy(&self) -> Id<Object, Shared> {
        msg_send_id![self, autoContentAccessingProxy]
    }
}
