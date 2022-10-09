use super::__exported::NSArray;
use super::__exported::NSAttributedString;
use super::__exported::NSEvent;
use super::__exported::NSImage;
use super::__exported::NSInputServer;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSGeometry::*;
use crate::Foundation::generated::NSObject::*;
use crate::Foundation::generated::NSRange::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSTextInput = NSObject;
extern_class!(
    #[derive(Debug)]
    pub struct NSInputManager;
    unsafe impl ClassType for NSInputManager {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSInputManager {
        #[method_id(currentInputManager)]
        pub unsafe fn currentInputManager() -> Option<Id<NSInputManager, Shared>>;
        #[method(cycleToNextInputLanguage:)]
        pub unsafe fn cycleToNextInputLanguage(sender: Option<&Object>);
        #[method(cycleToNextInputServerInLanguage:)]
        pub unsafe fn cycleToNextInputServerInLanguage(sender: Option<&Object>);
        #[method_id(initWithName:host:)]
        pub unsafe fn initWithName_host(
            &self,
            inputServerName: Option<&NSString>,
            hostName: Option<&NSString>,
        ) -> Option<Id<NSInputManager, Shared>>;
        #[method_id(localizedInputManagerName)]
        pub unsafe fn localizedInputManagerName(&self) -> Option<Id<NSString, Shared>>;
        #[method(markedTextAbandoned:)]
        pub unsafe fn markedTextAbandoned(&self, cli: Option<&Object>);
        #[method(markedTextSelectionChanged:client:)]
        pub unsafe fn markedTextSelectionChanged_client(
            &self,
            newSel: NSRange,
            cli: Option<&Object>,
        );
        #[method(wantsToInterpretAllKeystrokes)]
        pub unsafe fn wantsToInterpretAllKeystrokes(&self) -> bool;
        #[method_id(language)]
        pub unsafe fn language(&self) -> Option<Id<NSString, Shared>>;
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method_id(server)]
        pub unsafe fn server(&self) -> Option<Id<NSInputServer, Shared>>;
        #[method(wantsToHandleMouseEvents)]
        pub unsafe fn wantsToHandleMouseEvents(&self) -> bool;
        #[method(handleMouseEvent:)]
        pub unsafe fn handleMouseEvent(&self, mouseEvent: Option<&NSEvent>) -> bool;
        #[method(wantsToDelayTextChangeNotifications)]
        pub unsafe fn wantsToDelayTextChangeNotifications(&self) -> bool;
    }
);
