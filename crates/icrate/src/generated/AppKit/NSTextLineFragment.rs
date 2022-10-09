use crate::CoreGraphics::generated::CoreGraphics::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSAttributedString::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextLineFragment;
    unsafe impl ClassType for NSTextLineFragment {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextLineFragment {
        #[method_id(initWithAttributedString:range:)]
        pub unsafe fn initWithAttributedString_range(
            &self,
            attributedString: &NSAttributedString,
            range: NSRange,
        ) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, aDecoder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(initWithString:attributes:range:)]
        pub unsafe fn initWithString_attributes_range(
            &self,
            string: &NSString,
            attributes: &NSDictionary<NSAttributedStringKey, Object>,
            range: NSRange,
        ) -> Id<Self, Shared>;
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(attributedString)]
        pub unsafe fn attributedString(&self) -> Id<NSAttributedString, Shared>;
        #[method(characterRange)]
        pub unsafe fn characterRange(&self) -> NSRange;
        #[method(typographicBounds)]
        pub unsafe fn typographicBounds(&self) -> CGRect;
        #[method(glyphOrigin)]
        pub unsafe fn glyphOrigin(&self) -> CGPoint;
        #[method(drawAtPoint:inContext:)]
        pub unsafe fn drawAtPoint_inContext(&self, point: CGPoint, context: CGContextRef);
        #[method(locationForCharacterAtIndex:)]
        pub unsafe fn locationForCharacterAtIndex(&self, index: NSInteger) -> CGPoint;
        #[method(characterIndexForPoint:)]
        pub unsafe fn characterIndexForPoint(&self, point: CGPoint) -> NSInteger;
        #[method(fractionOfDistanceThroughGlyphForPoint:)]
        pub unsafe fn fractionOfDistanceThroughGlyphForPoint(&self, point: CGPoint) -> CGFloat;
    }
);
