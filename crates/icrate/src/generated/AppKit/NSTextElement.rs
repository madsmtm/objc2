#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSTextElement;
    unsafe impl ClassType for NSTextElement {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSTextElement {
        #[method_id(initWithTextContentManager:)]
        pub unsafe fn initWithTextContentManager(
            &self,
            textContentManager: Option<&NSTextContentManager>,
        ) -> Id<Self, Shared>;
        #[method_id(textContentManager)]
        pub unsafe fn textContentManager(&self) -> Option<Id<NSTextContentManager, Shared>>;
        #[method(setTextContentManager:)]
        pub unsafe fn setTextContentManager(
            &self,
            textContentManager: Option<&NSTextContentManager>,
        );
        #[method_id(elementRange)]
        pub unsafe fn elementRange(&self) -> Option<Id<NSTextRange, Shared>>;
        #[method(setElementRange:)]
        pub unsafe fn setElementRange(&self, elementRange: Option<&NSTextRange>);
    }
);
extern_class!(
    #[derive(Debug)]
    pub struct NSTextParagraph;
    unsafe impl ClassType for NSTextParagraph {
        type Super = NSTextElement;
    }
);
extern_methods!(
    unsafe impl NSTextParagraph {
        #[method_id(initWithAttributedString:)]
        pub unsafe fn initWithAttributedString(
            &self,
            attributedString: Option<&NSAttributedString>,
        ) -> Id<Self, Shared>;
        #[method_id(attributedString)]
        pub unsafe fn attributedString(&self) -> Id<NSAttributedString, Shared>;
        #[method_id(paragraphContentRange)]
        pub unsafe fn paragraphContentRange(&self) -> Option<Id<NSTextRange, Shared>>;
        #[method_id(paragraphSeparatorRange)]
        pub unsafe fn paragraphSeparatorRange(&self) -> Option<Id<NSTextRange, Shared>>;
    }
);