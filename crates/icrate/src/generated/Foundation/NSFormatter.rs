use super::__exported::NSAttributedString;
use super::__exported::NSDictionary;
use super::__exported::NSString;
use crate::Foundation::generated::NSAttributedString::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSFormatter;
    unsafe impl ClassType for NSFormatter {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSFormatter {
        #[method_id(stringForObjectValue:)]
        pub unsafe fn stringForObjectValue(
            &self,
            obj: Option<&Object>,
        ) -> Option<Id<NSString, Shared>>;
        #[method_id(attributedStringForObjectValue:withDefaultAttributes:)]
        pub unsafe fn attributedStringForObjectValue_withDefaultAttributes(
            &self,
            obj: &Object,
            attrs: Option<&NSDictionary<NSAttributedStringKey, Object>>,
        ) -> Option<Id<NSAttributedString, Shared>>;
        #[method_id(editingStringForObjectValue:)]
        pub unsafe fn editingStringForObjectValue(
            &self,
            obj: &Object,
        ) -> Option<Id<NSString, Shared>>;
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<Object, Shared>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString, Shared>>>,
        ) -> bool;
        #[method(isPartialStringValid:newEditingString:errorDescription:)]
        pub unsafe fn isPartialStringValid_newEditingString_errorDescription(
            &self,
            partialString: &NSString,
            newString: Option<&mut Option<Id<NSString, Shared>>>,
            error: Option<&mut Option<Id<NSString, Shared>>>,
        ) -> bool;
        #[method(isPartialStringValid:proposedSelectedRange:originalString:originalSelectedRange:errorDescription:)]
        pub unsafe fn isPartialStringValid_proposedSelectedRange_originalString_originalSelectedRange_errorDescription(
            &self,
            partialStringPtr: &mut Id<NSString, Shared>,
            proposedSelRangePtr: NSRangePointer,
            origString: &NSString,
            origSelRange: NSRange,
            error: Option<&mut Option<Id<NSString, Shared>>>,
        ) -> bool;
    }
);
