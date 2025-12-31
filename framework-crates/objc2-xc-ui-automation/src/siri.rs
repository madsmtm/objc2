#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use crate::XCUIDevice;
use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, NSObjectProtocol},
    MainThreadMarker, MainThreadOnly,
};
use objc2_foundation::NSString;

extern_class!(
    /// Represents a device's Siri interface and allows issuing textual queries
    /// and producing element queries for UI shown by Siri.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/xcuiautomation/xcuisiriservice?language=objc)
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct XCUISiriService;
);
extern_conformance!(
    unsafe impl NSObjectProtocol for XCUISiriService {}
);

impl XCUISiriService {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new(mtm: MainThreadMarker) -> Retained<Self>;

        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Provides debugging information about the element representing the root of the Siri UI.
        ///
        /// See also: XCUIElement
        #[unsafe(method(debugDescription))]
        #[unsafe(method_family = none)]
        pub fn debugDescription(&self) -> Retained<NSString>;

        /// Presents the Siri UI, if it is not currently active, and accepts a string
        /// which is then processed as if it were recognized speech.
        ///
        ///
        /// Parameter `text`: The string to pass to Siri for processing.
        #[unsafe(method(activateWithVoiceRecognitionText:))]
        #[unsafe(method_family = none)]
        pub fn activateWithVoiceRecognitionText(&self, text: &NSString);
    );
}
impl XCUIDevice {
    extern_methods!(
        /// Provides access to an object representing the Siri interface on the device.
        #[unsafe(method(siriService))]
        #[unsafe(method_family = none)]
        pub fn siriService(&self) -> Retained<XCUISiriService>;
    );
}
