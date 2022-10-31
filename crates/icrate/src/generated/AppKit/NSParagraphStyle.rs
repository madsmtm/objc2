//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

pub type NSLineBreakMode = NSUInteger;
pub const NSLineBreakByWordWrapping: NSLineBreakMode = 0;
pub const NSLineBreakByCharWrapping: NSLineBreakMode = 1;
pub const NSLineBreakByClipping: NSLineBreakMode = 2;
pub const NSLineBreakByTruncatingHead: NSLineBreakMode = 3;
pub const NSLineBreakByTruncatingTail: NSLineBreakMode = 4;
pub const NSLineBreakByTruncatingMiddle: NSLineBreakMode = 5;

pub type NSLineBreakStrategy = NSUInteger;
pub const NSLineBreakStrategyNone: NSLineBreakStrategy = 0;
pub const NSLineBreakStrategyPushOut: NSLineBreakStrategy = 1 << 0;
pub const NSLineBreakStrategyHangulWordPriority: NSLineBreakStrategy = 1 << 1;
pub const NSLineBreakStrategyStandard: NSLineBreakStrategy = 0xFFFF;

pub type NSTextTabOptionKey = NSString;

extern "C" {
    static NSTabColumnTerminatorsAttributeName: &'static NSTextTabOptionKey;
}

extern_class!(
    #[derive(Debug)]
    pub struct NSTextTab;

    unsafe impl ClassType for NSTextTab {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSTextTab {
        #[method_id(columnTerminatorsForLocale:)]
        pub unsafe fn columnTerminatorsForLocale(
            aLocale: Option<&NSLocale>,
        ) -> Id<NSCharacterSet, Shared>;

        #[method_id(initWithTextAlignment:location:options:)]
        pub unsafe fn initWithTextAlignment_location_options(
            &self,
            alignment: NSTextAlignment,
            loc: CGFloat,
            options: &NSDictionary<NSTextTabOptionKey, Object>,
        ) -> Id<Self, Shared>;

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(location)]
        pub unsafe fn location(&self) -> CGFloat;

        #[method_id(options)]
        pub unsafe fn options(&self) -> Id<NSDictionary<NSTextTabOptionKey, Object>, Shared>;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSParagraphStyle;

    unsafe impl ClassType for NSParagraphStyle {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSParagraphStyle {
        #[method_id(defaultParagraphStyle)]
        pub unsafe fn defaultParagraphStyle() -> Id<NSParagraphStyle, Shared>;

        #[method(defaultWritingDirectionForLanguage:)]
        pub unsafe fn defaultWritingDirectionForLanguage(
            languageName: Option<&NSString>,
        ) -> NSWritingDirection;

        #[method(lineSpacing)]
        pub unsafe fn lineSpacing(&self) -> CGFloat;

        #[method(paragraphSpacing)]
        pub unsafe fn paragraphSpacing(&self) -> CGFloat;

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(headIndent)]
        pub unsafe fn headIndent(&self) -> CGFloat;

        #[method(tailIndent)]
        pub unsafe fn tailIndent(&self) -> CGFloat;

        #[method(firstLineHeadIndent)]
        pub unsafe fn firstLineHeadIndent(&self) -> CGFloat;

        #[method(minimumLineHeight)]
        pub unsafe fn minimumLineHeight(&self) -> CGFloat;

        #[method(maximumLineHeight)]
        pub unsafe fn maximumLineHeight(&self) -> CGFloat;

        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[method(lineHeightMultiple)]
        pub unsafe fn lineHeightMultiple(&self) -> CGFloat;

        #[method(paragraphSpacingBefore)]
        pub unsafe fn paragraphSpacingBefore(&self) -> CGFloat;

        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> c_float;

        #[method(usesDefaultHyphenation)]
        pub unsafe fn usesDefaultHyphenation(&self) -> bool;

        #[method_id(tabStops)]
        pub unsafe fn tabStops(&self) -> Id<NSArray<NSTextTab>, Shared>;

        #[method(defaultTabInterval)]
        pub unsafe fn defaultTabInterval(&self) -> CGFloat;

        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;

        #[method(tighteningFactorForTruncation)]
        pub unsafe fn tighteningFactorForTruncation(&self) -> c_float;

        #[method_id(textBlocks)]
        pub unsafe fn textBlocks(&self) -> Id<NSArray<NSTextBlock>, Shared>;

        #[method_id(textLists)]
        pub unsafe fn textLists(&self) -> Id<NSArray<NSTextList>, Shared>;

        #[method(headerLevel)]
        pub unsafe fn headerLevel(&self) -> NSInteger;

        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;
    }
);

extern_class!(
    #[derive(Debug)]
    pub struct NSMutableParagraphStyle;

    unsafe impl ClassType for NSMutableParagraphStyle {
        type Super = NSParagraphStyle;
    }
);

extern_methods!(
    unsafe impl NSMutableParagraphStyle {
        #[method(lineSpacing)]
        pub unsafe fn lineSpacing(&self) -> CGFloat;

        #[method(setLineSpacing:)]
        pub unsafe fn setLineSpacing(&self, lineSpacing: CGFloat);

        #[method(paragraphSpacing)]
        pub unsafe fn paragraphSpacing(&self) -> CGFloat;

        #[method(setParagraphSpacing:)]
        pub unsafe fn setParagraphSpacing(&self, paragraphSpacing: CGFloat);

        #[method(alignment)]
        pub unsafe fn alignment(&self) -> NSTextAlignment;

        #[method(setAlignment:)]
        pub unsafe fn setAlignment(&self, alignment: NSTextAlignment);

        #[method(firstLineHeadIndent)]
        pub unsafe fn firstLineHeadIndent(&self) -> CGFloat;

        #[method(setFirstLineHeadIndent:)]
        pub unsafe fn setFirstLineHeadIndent(&self, firstLineHeadIndent: CGFloat);

        #[method(headIndent)]
        pub unsafe fn headIndent(&self) -> CGFloat;

        #[method(setHeadIndent:)]
        pub unsafe fn setHeadIndent(&self, headIndent: CGFloat);

        #[method(tailIndent)]
        pub unsafe fn tailIndent(&self) -> CGFloat;

        #[method(setTailIndent:)]
        pub unsafe fn setTailIndent(&self, tailIndent: CGFloat);

        #[method(lineBreakMode)]
        pub unsafe fn lineBreakMode(&self) -> NSLineBreakMode;

        #[method(setLineBreakMode:)]
        pub unsafe fn setLineBreakMode(&self, lineBreakMode: NSLineBreakMode);

        #[method(minimumLineHeight)]
        pub unsafe fn minimumLineHeight(&self) -> CGFloat;

        #[method(setMinimumLineHeight:)]
        pub unsafe fn setMinimumLineHeight(&self, minimumLineHeight: CGFloat);

        #[method(maximumLineHeight)]
        pub unsafe fn maximumLineHeight(&self) -> CGFloat;

        #[method(setMaximumLineHeight:)]
        pub unsafe fn setMaximumLineHeight(&self, maximumLineHeight: CGFloat);

        #[method(baseWritingDirection)]
        pub unsafe fn baseWritingDirection(&self) -> NSWritingDirection;

        #[method(setBaseWritingDirection:)]
        pub unsafe fn setBaseWritingDirection(&self, baseWritingDirection: NSWritingDirection);

        #[method(lineHeightMultiple)]
        pub unsafe fn lineHeightMultiple(&self) -> CGFloat;

        #[method(setLineHeightMultiple:)]
        pub unsafe fn setLineHeightMultiple(&self, lineHeightMultiple: CGFloat);

        #[method(paragraphSpacingBefore)]
        pub unsafe fn paragraphSpacingBefore(&self) -> CGFloat;

        #[method(setParagraphSpacingBefore:)]
        pub unsafe fn setParagraphSpacingBefore(&self, paragraphSpacingBefore: CGFloat);

        #[method(hyphenationFactor)]
        pub unsafe fn hyphenationFactor(&self) -> c_float;

        #[method(setHyphenationFactor:)]
        pub unsafe fn setHyphenationFactor(&self, hyphenationFactor: c_float);

        #[method(usesDefaultHyphenation)]
        pub unsafe fn usesDefaultHyphenation(&self) -> bool;

        #[method(setUsesDefaultHyphenation:)]
        pub unsafe fn setUsesDefaultHyphenation(&self, usesDefaultHyphenation: bool);

        #[method_id(tabStops)]
        pub unsafe fn tabStops(&self) -> Id<NSArray<NSTextTab>, Shared>;

        #[method(setTabStops:)]
        pub unsafe fn setTabStops(&self, tabStops: Option<&NSArray<NSTextTab>>);

        #[method(defaultTabInterval)]
        pub unsafe fn defaultTabInterval(&self) -> CGFloat;

        #[method(setDefaultTabInterval:)]
        pub unsafe fn setDefaultTabInterval(&self, defaultTabInterval: CGFloat);

        #[method(allowsDefaultTighteningForTruncation)]
        pub unsafe fn allowsDefaultTighteningForTruncation(&self) -> bool;

        #[method(setAllowsDefaultTighteningForTruncation:)]
        pub unsafe fn setAllowsDefaultTighteningForTruncation(
            &self,
            allowsDefaultTighteningForTruncation: bool,
        );

        #[method(addTabStop:)]
        pub unsafe fn addTabStop(&self, anObject: &NSTextTab);

        #[method(removeTabStop:)]
        pub unsafe fn removeTabStop(&self, anObject: &NSTextTab);

        #[method(setParagraphStyle:)]
        pub unsafe fn setParagraphStyle(&self, obj: &NSParagraphStyle);

        #[method(tighteningFactorForTruncation)]
        pub unsafe fn tighteningFactorForTruncation(&self) -> c_float;

        #[method(setTighteningFactorForTruncation:)]
        pub unsafe fn setTighteningFactorForTruncation(
            &self,
            tighteningFactorForTruncation: c_float,
        );

        #[method_id(textBlocks)]
        pub unsafe fn textBlocks(&self) -> Id<NSArray<NSTextBlock>, Shared>;

        #[method(setTextBlocks:)]
        pub unsafe fn setTextBlocks(&self, textBlocks: &NSArray<NSTextBlock>);

        #[method_id(textLists)]
        pub unsafe fn textLists(&self) -> Id<NSArray<NSTextList>, Shared>;

        #[method(setTextLists:)]
        pub unsafe fn setTextLists(&self, textLists: &NSArray<NSTextList>);

        #[method(headerLevel)]
        pub unsafe fn headerLevel(&self) -> NSInteger;

        #[method(setHeaderLevel:)]
        pub unsafe fn setHeaderLevel(&self, headerLevel: NSInteger);

        #[method(lineBreakStrategy)]
        pub unsafe fn lineBreakStrategy(&self) -> NSLineBreakStrategy;

        #[method(setLineBreakStrategy:)]
        pub unsafe fn setLineBreakStrategy(&self, lineBreakStrategy: NSLineBreakStrategy);
    }
);

pub type NSTextTabType = NSUInteger;
pub const NSLeftTabStopType: NSTextTabType = 0;
pub const NSRightTabStopType: NSTextTabType = 1;
pub const NSCenterTabStopType: NSTextTabType = 2;
pub const NSDecimalTabStopType: NSTextTabType = 3;

extern_methods!(
    /// NSTextTabDeprecated
    unsafe impl NSTextTab {
        #[method_id(initWithType:location:)]
        pub unsafe fn initWithType_location(
            &self,
            type_: NSTextTabType,
            loc: CGFloat,
        ) -> Id<Self, Shared>;

        #[method(tabStopType)]
        pub unsafe fn tabStopType(&self) -> NSTextTabType;
    }
);
