use super::__exported::NSData;
use super::__exported::NSURL;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSPasteboard::*;
use crate::Foundation::generated::NSArray::*;
use crate::Foundation::generated::NSBundle::*;
use crate::Foundation::generated::NSDate::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
pub type NSSoundName = NSString;
pub type NSSoundPlaybackDeviceIdentifier = NSString;
extern_class!(
    #[derive(Debug)]
    pub struct NSSound;
    unsafe impl ClassType for NSSound {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSSound {
        #[method_id(soundNamed:)]
        pub unsafe fn soundNamed(name: &NSSoundName) -> Option<Id<NSSound, Shared>>;
        #[method_id(initWithContentsOfURL:byReference:)]
        pub unsafe fn initWithContentsOfURL_byReference(
            &self,
            url: &NSURL,
            byRef: bool,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithContentsOfFile:byReference:)]
        pub unsafe fn initWithContentsOfFile_byReference(
            &self,
            path: &NSString,
            byRef: bool,
        ) -> Option<Id<Self, Shared>>;
        #[method_id(initWithData:)]
        pub unsafe fn initWithData(&self, data: &NSData) -> Option<Id<Self, Shared>>;
        #[method(setName:)]
        pub unsafe fn setName(&self, string: Option<&NSSoundName>) -> bool;
        #[method_id(name)]
        pub unsafe fn name(&self) -> Option<Id<NSSoundName, Shared>>;
        #[method(canInitWithPasteboard:)]
        pub unsafe fn canInitWithPasteboard(pasteboard: &NSPasteboard) -> bool;
        #[method_id(soundUnfilteredTypes)]
        pub unsafe fn soundUnfilteredTypes() -> Id<NSArray<NSString>, Shared>;
        #[method_id(initWithPasteboard:)]
        pub unsafe fn initWithPasteboard(
            &self,
            pasteboard: &NSPasteboard,
        ) -> Option<Id<Self, Shared>>;
        #[method(writeToPasteboard:)]
        pub unsafe fn writeToPasteboard(&self, pasteboard: &NSPasteboard);
        #[method(play)]
        pub unsafe fn play(&self) -> bool;
        #[method(pause)]
        pub unsafe fn pause(&self) -> bool;
        #[method(resume)]
        pub unsafe fn resume(&self) -> bool;
        #[method(stop)]
        pub unsafe fn stop(&self) -> bool;
        #[method(isPlaying)]
        pub unsafe fn isPlaying(&self) -> bool;
        #[method_id(delegate)]
        pub unsafe fn delegate(&self) -> Option<Id<NSSoundDelegate, Shared>>;
        #[method(setDelegate:)]
        pub unsafe fn setDelegate(&self, delegate: Option<&NSSoundDelegate>);
        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;
        #[method(volume)]
        pub unsafe fn volume(&self) -> c_float;
        #[method(setVolume:)]
        pub unsafe fn setVolume(&self, volume: c_float);
        #[method(currentTime)]
        pub unsafe fn currentTime(&self) -> NSTimeInterval;
        #[method(setCurrentTime:)]
        pub unsafe fn setCurrentTime(&self, currentTime: NSTimeInterval);
        #[method(loops)]
        pub unsafe fn loops(&self) -> bool;
        #[method(setLoops:)]
        pub unsafe fn setLoops(&self, loops: bool);
        #[method_id(playbackDeviceIdentifier)]
        pub unsafe fn playbackDeviceIdentifier(
            &self,
        ) -> Option<Id<NSSoundPlaybackDeviceIdentifier, Shared>>;
        #[method(setPlaybackDeviceIdentifier:)]
        pub unsafe fn setPlaybackDeviceIdentifier(
            &self,
            playbackDeviceIdentifier: Option<&NSSoundPlaybackDeviceIdentifier>,
        );
        #[method(setChannelMapping:)]
        pub unsafe fn setChannelMapping(&self, channelMapping: Option<&NSArray>);
        #[method_id(channelMapping)]
        pub unsafe fn channelMapping(&self) -> Option<Id<NSArray, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSSound {
        #[method_id(soundUnfilteredFileTypes)]
        pub unsafe fn soundUnfilteredFileTypes() -> Option<Id<NSArray, Shared>>;
        #[method_id(soundUnfilteredPasteboardTypes)]
        pub unsafe fn soundUnfilteredPasteboardTypes() -> Option<Id<NSArray, Shared>>;
    }
);
pub type NSSoundDelegate = NSObject;
extern_methods!(
    #[doc = "NSBundleSoundExtensions"]
    unsafe impl NSBundle {
        #[method_id(pathForSoundResource:)]
        pub unsafe fn pathForSoundResource(
            &self,
            name: &NSSoundName,
        ) -> Option<Id<NSString, Shared>>;
    }
);
