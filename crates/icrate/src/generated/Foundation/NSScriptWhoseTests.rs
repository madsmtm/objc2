use super::__exported::NSArray;
use super::__exported::NSScriptObjectSpecifier;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptWhoseTest;
    unsafe impl ClassType for NSScriptWhoseTest {
        type Super = NSObject;
    }
);
impl NSScriptWhoseTest {
    pub unsafe fn isTrue(&self) -> bool {
        msg_send![self, isTrue]
    }
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: inCoder]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSLogicalTest;
    unsafe impl ClassType for NSLogicalTest {
        type Super = NSScriptWhoseTest;
    }
);
impl NSLogicalTest {
    pub unsafe fn initAndTestWithTests(
        &self,
        subTests: &NSArray<NSSpecifierTest>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initAndTestWithTests: subTests]
    }
    pub unsafe fn initOrTestWithTests(
        &self,
        subTests: &NSArray<NSSpecifierTest>,
    ) -> Id<Self, Shared> {
        msg_send_id![self, initOrTestWithTests: subTests]
    }
    pub unsafe fn initNotTestWithTest(&self, subTest: &NSScriptWhoseTest) -> Id<Self, Shared> {
        msg_send_id![self, initNotTestWithTest: subTest]
    }
}
extern_class!(
    #[derive(Debug)]
    pub struct NSSpecifierTest;
    unsafe impl ClassType for NSSpecifierTest {
        type Super = NSScriptWhoseTest;
    }
);
impl NSSpecifierTest {
    pub unsafe fn init(&self) -> Id<Self, Shared> {
        msg_send_id![self, init]
    }
    pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>> {
        msg_send_id![self, initWithCoder: inCoder]
    }
    pub unsafe fn initWithObjectSpecifier_comparisonOperator_testObject(
        &self,
        obj1: Option<&NSScriptObjectSpecifier>,
        compOp: NSTestComparisonOperation,
        obj2: Option<&Object>,
    ) -> Id<Self, Shared> {
        msg_send_id![
            self,
            initWithObjectSpecifier: obj1,
            comparisonOperator: compOp,
            testObject: obj2
        ]
    }
}
#[doc = "NSComparisonMethods"]
impl NSObject {
    pub unsafe fn isEqualTo(&self, object: Option<&Object>) -> bool {
        msg_send![self, isEqualTo: object]
    }
    pub unsafe fn isLessThanOrEqualTo(&self, object: Option<&Object>) -> bool {
        msg_send![self, isLessThanOrEqualTo: object]
    }
    pub unsafe fn isLessThan(&self, object: Option<&Object>) -> bool {
        msg_send![self, isLessThan: object]
    }
    pub unsafe fn isGreaterThanOrEqualTo(&self, object: Option<&Object>) -> bool {
        msg_send![self, isGreaterThanOrEqualTo: object]
    }
    pub unsafe fn isGreaterThan(&self, object: Option<&Object>) -> bool {
        msg_send![self, isGreaterThan: object]
    }
    pub unsafe fn isNotEqualTo(&self, object: Option<&Object>) -> bool {
        msg_send![self, isNotEqualTo: object]
    }
    pub unsafe fn doesContain(&self, object: &Object) -> bool {
        msg_send![self, doesContain: object]
    }
    pub unsafe fn isLike(&self, object: &NSString) -> bool {
        msg_send![self, isLike: object]
    }
    pub unsafe fn isCaseInsensitiveLike(&self, object: &NSString) -> bool {
        msg_send![self, isCaseInsensitiveLike: object]
    }
}
#[doc = "NSScriptingComparisonMethods"]
impl NSObject {
    pub unsafe fn scriptingIsEqualTo(&self, object: &Object) -> bool {
        msg_send![self, scriptingIsEqualTo: object]
    }
    pub unsafe fn scriptingIsLessThanOrEqualTo(&self, object: &Object) -> bool {
        msg_send![self, scriptingIsLessThanOrEqualTo: object]
    }
    pub unsafe fn scriptingIsLessThan(&self, object: &Object) -> bool {
        msg_send![self, scriptingIsLessThan: object]
    }
    pub unsafe fn scriptingIsGreaterThanOrEqualTo(&self, object: &Object) -> bool {
        msg_send![self, scriptingIsGreaterThanOrEqualTo: object]
    }
    pub unsafe fn scriptingIsGreaterThan(&self, object: &Object) -> bool {
        msg_send![self, scriptingIsGreaterThan: object]
    }
    pub unsafe fn scriptingBeginsWith(&self, object: &Object) -> bool {
        msg_send![self, scriptingBeginsWith: object]
    }
    pub unsafe fn scriptingEndsWith(&self, object: &Object) -> bool {
        msg_send![self, scriptingEndsWith: object]
    }
    pub unsafe fn scriptingContains(&self, object: &Object) -> bool {
        msg_send![self, scriptingContains: object]
    }
}
