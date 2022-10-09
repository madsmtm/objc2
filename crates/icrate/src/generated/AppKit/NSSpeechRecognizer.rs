use super::__exported::NSString;
use crate::AppKit::generated::AppKitDefines::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSObject::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSSpeechRecognizer;
    unsafe impl ClassType for NSSpeechRecognizer {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSSpeechRecognizer {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Option<Id<Self, Shared>>;
        #[method(startListening)]
        pub unsafe fn startListening(&self);
        #[method(stopListening)]
        pub unsafe fn stopListening(&self);
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSpeechRecognizerDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSpeechRecognizerDelegate>);
        #[method_id(commands)]
        pub unsafe fn commands(&self) -> Option<Id<NSArray<NSString>, Shared>>;
        #[method(setCommands:)]
        pub unsafe fn setCommands(&self, commands: Option<&NSArray<NSString>>);
        #[method_id(displayedCommandsTitle)]
        pub unsafe fn displayedCommandsTitle(&self) -> Option<Id<NSString, Shared>>;
        #[method(setDisplayedCommandsTitle:)]
        pub unsafe fn setDisplayedCommandsTitle(&self, displayedCommandsTitle: Option<&NSString>);
        #[method(listensInForegroundOnly)]
        pub unsafe fn listensInForegroundOnly(&self) -> bool;
        #[method(setListensInForegroundOnly:)]
        pub unsafe fn setListensInForegroundOnly(&self, listensInForegroundOnly: bool);
        #[method(blocksOtherRecognizers)]
        pub unsafe fn blocksOtherRecognizers(&self) -> bool;
        #[method(setBlocksOtherRecognizers:)]
        pub unsafe fn setBlocksOtherRecognizers(&self, blocksOtherRecognizers: bool);
    }
);
pub type NSSpeechRecognizerDelegate = NSObject;
