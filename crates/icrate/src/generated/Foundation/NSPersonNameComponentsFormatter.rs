extern_class!(
    #[derive(Debug)]
    struct NSPersonNameComponentsFormatter;
    unsafe impl ClassType for NSPersonNameComponentsFormatter {
        type Super = NSFormatter;
    }
);
impl NSPersonNameComponentsFormatter {
    pub unsafe fn localizedStringFromPersonNameComponents_style_options(
        components: &NSPersonNameComponents,
        nameFormatStyle: NSPersonNameComponentsFormatterStyle,
        nameOptions: NSPersonNameComponentsFormatterOptions,
    ) -> Id<NSString, Shared> {
        msg_send_id![
            Self::class(),
            localizedStringFromPersonNameComponents: components,
            style: nameFormatStyle,
            options: nameOptions
        ]
    }
    pub unsafe fn stringFromPersonNameComponents(
        &self,
        components: &NSPersonNameComponents,
    ) -> Id<NSString, Shared> {
        msg_send_id![self, stringFromPersonNameComponents: components]
    }
    pub unsafe fn annotatedStringFromPersonNameComponents(
        &self,
        components: &NSPersonNameComponents,
    ) -> Id<NSAttributedString, Shared> {
        msg_send_id![self, annotatedStringFromPersonNameComponents: components]
    }
    pub unsafe fn personNameComponentsFromString(
        &self,
        string: &NSString,
    ) -> Option<Id<NSPersonNameComponents, Shared>> {
        msg_send_id![self, personNameComponentsFromString: string]
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
    pub unsafe fn style(&self) -> NSPersonNameComponentsFormatterStyle {
        msg_send![self, style]
    }
    pub unsafe fn setStyle(&self, style: NSPersonNameComponentsFormatterStyle) {
        msg_send![self, setStyle: style]
    }
    pub unsafe fn isPhonetic(&self) -> bool {
        msg_send![self, isPhonetic]
    }
    pub unsafe fn setPhonetic(&self, phonetic: bool) {
        msg_send![self, setPhonetic: phonetic]
    }
    pub unsafe fn locale(&self) -> Id<NSLocale, Shared> {
        msg_send_id![self, locale]
    }
    pub unsafe fn setLocale(&self, locale: Option<&NSLocale>) {
        msg_send![self, setLocale: locale]
    }
}
