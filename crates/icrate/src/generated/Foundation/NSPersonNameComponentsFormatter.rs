#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSPersonNameComponentsFormatter;
    unsafe impl ClassType for NSPersonNameComponentsFormatter {
        type Super = NSFormatter;
    }
);
extern_methods!(
    unsafe impl NSPersonNameComponentsFormatter {
        #[method(style)]
        pub unsafe fn style(&self) -> NSPersonNameComponentsFormatterStyle;
        #[method(setStyle:)]
        pub unsafe fn setStyle(&self, style: NSPersonNameComponentsFormatterStyle);
        #[method(isPhonetic)]
        pub unsafe fn isPhonetic(&self) -> bool;
        #[method(setPhonetic:)]
        pub unsafe fn setPhonetic(&self, phonetic: bool);
        #[method_id(locale)]
        pub unsafe fn locale(&self) -> Id<NSLocale, Shared>;
        #[method(setLocale:)]
        pub unsafe fn setLocale(&self, locale: Option<&NSLocale>);
        #[method_id(localizedStringFromPersonNameComponents:style:options:)]
        pub unsafe fn localizedStringFromPersonNameComponents_style_options(
            components: &NSPersonNameComponents,
            nameFormatStyle: NSPersonNameComponentsFormatterStyle,
            nameOptions: NSPersonNameComponentsFormatterOptions,
        ) -> Id<NSString, Shared>;
        #[method_id(stringFromPersonNameComponents:)]
        pub unsafe fn stringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Id<NSString, Shared>;
        #[method_id(annotatedStringFromPersonNameComponents:)]
        pub unsafe fn annotatedStringFromPersonNameComponents(
            &self,
            components: &NSPersonNameComponents,
        ) -> Id<NSAttributedString, Shared>;
        #[method_id(personNameComponentsFromString:)]
        pub unsafe fn personNameComponentsFromString(
            &self,
            string: &NSString,
        ) -> Option<Id<NSPersonNameComponents, Shared>>;
        #[method(getObjectValue:forString:errorDescription:)]
        pub unsafe fn getObjectValue_forString_errorDescription(
            &self,
            obj: Option<&mut Option<Id<Object, Shared>>>,
            string: &NSString,
            error: Option<&mut Option<Id<NSString, Shared>>>,
        ) -> bool;
    }
);
