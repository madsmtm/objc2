use super::__exported::NSArray;
use super::__exported::NSScriptObjectSpecifier;
use super::__exported::NSString;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSScriptWhoseTest;
    unsafe impl ClassType for NSScriptWhoseTest {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSScriptWhoseTest {
        #[method(isTrue)]
        pub unsafe fn isTrue(&self) -> bool;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        # [method_id (initWithCoder :)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSLogicalTest;
    unsafe impl ClassType for NSLogicalTest {
        type Super = NSScriptWhoseTest;
    }
);
extern_methods!(
    unsafe impl NSLogicalTest {
        # [method_id (initAndTestWithTests :)]
        pub unsafe fn initAndTestWithTests(
            &self,
            subTests: &NSArray<NSSpecifierTest>,
        ) -> Id<Self, Shared>;
        # [method_id (initOrTestWithTests :)]
        pub unsafe fn initOrTestWithTests(
            &self,
            subTests: &NSArray<NSSpecifierTest>,
        ) -> Id<Self, Shared>;
        # [method_id (initNotTestWithTest :)]
        pub unsafe fn initNotTestWithTest(&self, subTest: &NSScriptWhoseTest) -> Id<Self, Shared>;
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSSpecifierTest;
    unsafe impl ClassType for NSSpecifierTest {
        type Super = NSScriptWhoseTest;
    }
);
extern_methods!(
    unsafe impl NSSpecifierTest {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        # [method_id (initWithCoder :)]
        pub unsafe fn initWithCoder(&self, inCoder: &NSCoder) -> Option<Id<Self, Shared>>;
        # [method_id (initWithObjectSpecifier : comparisonOperator : testObject :)]
        pub unsafe fn initWithObjectSpecifier_comparisonOperator_testObject(
            &self,
            obj1: Option<&NSScriptObjectSpecifier>,
            compOp: NSTestComparisonOperation,
            obj2: Option<&Object>,
        ) -> Id<Self, Shared>;
    }
);
extern_methods!(
    #[doc = "NSComparisonMethods"]
    unsafe impl NSObject {
        # [method (isEqualTo :)]
        pub unsafe fn isEqualTo(&self, object: Option<&Object>) -> bool;
        # [method (isLessThanOrEqualTo :)]
        pub unsafe fn isLessThanOrEqualTo(&self, object: Option<&Object>) -> bool;
        # [method (isLessThan :)]
        pub unsafe fn isLessThan(&self, object: Option<&Object>) -> bool;
        # [method (isGreaterThanOrEqualTo :)]
        pub unsafe fn isGreaterThanOrEqualTo(&self, object: Option<&Object>) -> bool;
        # [method (isGreaterThan :)]
        pub unsafe fn isGreaterThan(&self, object: Option<&Object>) -> bool;
        # [method (isNotEqualTo :)]
        pub unsafe fn isNotEqualTo(&self, object: Option<&Object>) -> bool;
        # [method (doesContain :)]
        pub unsafe fn doesContain(&self, object: &Object) -> bool;
        # [method (isLike :)]
        pub unsafe fn isLike(&self, object: &NSString) -> bool;
        # [method (isCaseInsensitiveLike :)]
        pub unsafe fn isCaseInsensitiveLike(&self, object: &NSString) -> bool;
    }
);
extern_methods!(
    #[doc = "NSScriptingComparisonMethods"]
    unsafe impl NSObject {
        # [method (scriptingIsEqualTo :)]
        pub unsafe fn scriptingIsEqualTo(&self, object: &Object) -> bool;
        # [method (scriptingIsLessThanOrEqualTo :)]
        pub unsafe fn scriptingIsLessThanOrEqualTo(&self, object: &Object) -> bool;
        # [method (scriptingIsLessThan :)]
        pub unsafe fn scriptingIsLessThan(&self, object: &Object) -> bool;
        # [method (scriptingIsGreaterThanOrEqualTo :)]
        pub unsafe fn scriptingIsGreaterThanOrEqualTo(&self, object: &Object) -> bool;
        # [method (scriptingIsGreaterThan :)]
        pub unsafe fn scriptingIsGreaterThan(&self, object: &Object) -> bool;
        # [method (scriptingBeginsWith :)]
        pub unsafe fn scriptingBeginsWith(&self, object: &Object) -> bool;
        # [method (scriptingEndsWith :)]
        pub unsafe fn scriptingEndsWith(&self, object: &Object) -> bool;
        # [method (scriptingContains :)]
        pub unsafe fn scriptingContains(&self, object: &Object) -> bool;
    }
);
