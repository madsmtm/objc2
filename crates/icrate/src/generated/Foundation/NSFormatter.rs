#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, msg_send, msg_send_id, ClassType};
extern_class!(
    #[derive(Debug)]
    struct NSFormatter;
    unsafe impl ClassType for NSFormatter {
        type Super = NSObject;
    }
);
impl NSFormatter {
    pub unsafe fn stringForObjectValue(
        &self,
        obj: Option<&Object>,
    ) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, stringForObjectValue: obj]
    }
    pub unsafe fn attributedStringForObjectValue_withDefaultAttributes(
        &self,
        obj: &Object,
        attrs: TodoGenerics,
    ) -> Option<Id<NSAttributedString, Shared>> {
        msg_send_id![
            self,
            attributedStringForObjectValue: obj,
            withDefaultAttributes: attrs
        ]
    }
    pub unsafe fn editingStringForObjectValue(&self, obj: &Object) -> Option<Id<NSString, Shared>> {
        msg_send_id![self, editingStringForObjectValue: obj]
    }
    pub unsafe fn getObjectValue_forString_errorDescription(
        &self,
        obj: *mut Option<&Object>,
        string: &NSString,
        error: *mut Option<&NSString>,
    ) -> bool {
        msg_send![
            self,
            getObjectValue: obj,
            forString: string,
            errorDescription: error
        ]
    }
    pub unsafe fn isPartialStringValid_newEditingString_errorDescription(
        &self,
        partialString: &NSString,
        newString: *mut Option<&NSString>,
        error: *mut Option<&NSString>,
    ) -> bool {
        msg_send![
            self,
            isPartialStringValid: partialString,
            newEditingString: newString,
            errorDescription: error
        ]
    }
    pub unsafe fn isPartialStringValid_proposedSelectedRange_originalString_originalSelectedRange_errorDescription(
        &self,
        partialStringPtr: NonNull<&NSString>,
        proposedSelRangePtr: NSRangePointer,
        origString: &NSString,
        origSelRange: NSRange,
        error: *mut Option<&NSString>,
    ) -> bool {
        msg_send![
            self,
            isPartialStringValid: partialStringPtr,
            proposedSelectedRange: proposedSelRangePtr,
            originalString: origString,
            originalSelectedRange: origSelRange,
            errorDescription: error
        ]
    }
}
