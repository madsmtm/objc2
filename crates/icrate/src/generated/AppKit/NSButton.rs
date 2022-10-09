use super::__exported::NSImageSymbolConfiguration;
use super::__exported::NSSound;
use crate::AppKit::generated::AppKitDefines::*;
use crate::AppKit::generated::NSButtonCell::*;
use crate::AppKit::generated::NSControl::*;
use crate::AppKit::generated::NSUserInterfaceCompression::*;
use crate::AppKit::generated::NSUserInterfaceValidation::*;
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSButton;
    unsafe impl ClassType for NSButton {
        type Super = NSControl;
    }
);
extern_methods!(
    unsafe impl NSButton {
        #[method_id(buttonWithTitle:image:target:action:)]
        pub unsafe fn buttonWithTitle_image_target_action(
            title: &NSString,
            image: &NSImage,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method_id(buttonWithTitle:target:action:)]
        pub unsafe fn buttonWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method_id(buttonWithImage:target:action:)]
        pub unsafe fn buttonWithImage_target_action(
            image: &NSImage,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method_id(checkboxWithTitle:target:action:)]
        pub unsafe fn checkboxWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method_id(radioButtonWithTitle:target:action:)]
        pub unsafe fn radioButtonWithTitle_target_action(
            title: &NSString,
            target: Option<&Object>,
            action: Option<Sel>,
        ) -> Id<Self, Shared>;
        #[method(setButtonType:)]
        pub unsafe fn setButtonType(&self, type_: NSButtonType);
        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: &NSString);
        #[method_id(attributedTitle)]
        pub unsafe fn attributedTitle(&self) -> Id<NSAttributedString, Shared>;
        #[method(setAttributedTitle:)]
        pub unsafe fn setAttributedTitle(&self, attributedTitle: &NSAttributedString);
        #[method_id(alternateTitle)]
        pub unsafe fn alternateTitle(&self) -> Id<NSString, Shared>;
        #[method(setAlternateTitle:)]
        pub unsafe fn setAlternateTitle(&self, alternateTitle: &NSString);
        #[method_id(attributedAlternateTitle)]
        pub unsafe fn attributedAlternateTitle(&self) -> Id<NSAttributedString, Shared>;
        #[method(setAttributedAlternateTitle:)]
        pub unsafe fn setAttributedAlternateTitle(
            &self,
            attributedAlternateTitle: &NSAttributedString,
        );
        #[method(hasDestructiveAction)]
        pub unsafe fn hasDestructiveAction(&self) -> bool;
        #[method(setHasDestructiveAction:)]
        pub unsafe fn setHasDestructiveAction(&self, hasDestructiveAction: bool);
        #[method_id(sound)]
        pub unsafe fn sound(&self) -> Option<Id<NSSound, Shared>>;
        #[method(setSound:)]
        pub unsafe fn setSound(&self, sound: Option<&NSSound>);
        #[method(isSpringLoaded)]
        pub unsafe fn isSpringLoaded(&self) -> bool;
        #[method(setSpringLoaded:)]
        pub unsafe fn setSpringLoaded(&self, springLoaded: bool);
        #[method(maxAcceleratorLevel)]
        pub unsafe fn maxAcceleratorLevel(&self) -> NSInteger;
        #[method(setMaxAcceleratorLevel:)]
        pub unsafe fn setMaxAcceleratorLevel(&self, maxAcceleratorLevel: NSInteger);
        #[method(setPeriodicDelay:interval:)]
        pub unsafe fn setPeriodicDelay_interval(&self, delay: c_float, interval: c_float);
        #[method(getPeriodicDelay:interval:)]
        pub unsafe fn getPeriodicDelay_interval(
            &self,
            delay: NonNull<c_float>,
            interval: NonNull<c_float>,
        );
        #[method(bezelStyle)]
        pub unsafe fn bezelStyle(&self) -> NSBezelStyle;
        #[method(setBezelStyle:)]
        pub unsafe fn setBezelStyle(&self, bezelStyle: NSBezelStyle);
        #[method(isBordered)]
        pub unsafe fn isBordered(&self) -> bool;
        #[method(setBordered:)]
        pub unsafe fn setBordered(&self, bordered: bool);
        #[method(isTransparent)]
        pub unsafe fn isTransparent(&self) -> bool;
        #[method(setTransparent:)]
        pub unsafe fn setTransparent(&self, transparent: bool);
        #[method(showsBorderOnlyWhileMouseInside)]
        pub unsafe fn showsBorderOnlyWhileMouseInside(&self) -> bool;
        #[method(setShowsBorderOnlyWhileMouseInside:)]
        pub unsafe fn setShowsBorderOnlyWhileMouseInside(
            &self,
            showsBorderOnlyWhileMouseInside: bool,
        );
        #[method_id(image)]
        pub unsafe fn image(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setImage:)]
        pub unsafe fn setImage(&self, image: Option<&NSImage>);
        #[method_id(alternateImage)]
        pub unsafe fn alternateImage(&self) -> Option<Id<NSImage, Shared>>;
        #[method(setAlternateImage:)]
        pub unsafe fn setAlternateImage(&self, alternateImage: Option<&NSImage>);
        #[method(imagePosition)]
        pub unsafe fn imagePosition(&self) -> NSCellImagePosition;
        #[method(setImagePosition:)]
        pub unsafe fn setImagePosition(&self, imagePosition: NSCellImagePosition);
        #[method(imageScaling)]
        pub unsafe fn imageScaling(&self) -> NSImageScaling;
        #[method(setImageScaling:)]
        pub unsafe fn setImageScaling(&self, imageScaling: NSImageScaling);
        #[method(imageHugsTitle)]
        pub unsafe fn imageHugsTitle(&self) -> bool;
        #[method(setImageHugsTitle:)]
        pub unsafe fn setImageHugsTitle(&self, imageHugsTitle: bool);
        #[method_id(symbolConfiguration)]
        pub unsafe fn symbolConfiguration(&self) -> Option<Id<NSImageSymbolConfiguration, Shared>>;
        #[method(setSymbolConfiguration:)]
        pub unsafe fn setSymbolConfiguration(
            &self,
            symbolConfiguration: Option<&NSImageSymbolConfiguration>,
        );
        #[method_id(bezelColor)]
        pub unsafe fn bezelColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setBezelColor:)]
        pub unsafe fn setBezelColor(&self, bezelColor: Option<&NSColor>);
        #[method_id(contentTintColor)]
        pub unsafe fn contentTintColor(&self) -> Option<Id<NSColor, Shared>>;
        #[method(setContentTintColor:)]
        pub unsafe fn setContentTintColor(&self, contentTintColor: Option<&NSColor>);
        #[method(state)]
        pub unsafe fn state(&self) -> NSControlStateValue;
        #[method(setState:)]
        pub unsafe fn setState(&self, state: NSControlStateValue);
        #[method(allowsMixedState)]
        pub unsafe fn allowsMixedState(&self) -> bool;
        #[method(setAllowsMixedState:)]
        pub unsafe fn setAllowsMixedState(&self, allowsMixedState: bool);
        #[method(setNextState)]
        pub unsafe fn setNextState(&self);
        #[method(highlight:)]
        pub unsafe fn highlight(&self, flag: bool);
        #[method_id(keyEquivalent)]
        pub unsafe fn keyEquivalent(&self) -> Id<NSString, Shared>;
        #[method(setKeyEquivalent:)]
        pub unsafe fn setKeyEquivalent(&self, keyEquivalent: &NSString);
        #[method(keyEquivalentModifierMask)]
        pub unsafe fn keyEquivalentModifierMask(&self) -> NSEventModifierFlags;
        #[method(setKeyEquivalentModifierMask:)]
        pub unsafe fn setKeyEquivalentModifierMask(
            &self,
            keyEquivalentModifierMask: NSEventModifierFlags,
        );
        #[method(performKeyEquivalent:)]
        pub unsafe fn performKeyEquivalent(&self, key: &NSEvent) -> bool;
        #[method(compressWithPrioritizedCompressionOptions:)]
        pub unsafe fn compressWithPrioritizedCompressionOptions(
            &self,
            prioritizedOptions: &NSArray<NSUserInterfaceCompressionOptions>,
        );
        #[method(minimumSizeWithPrioritizedCompressionOptions:)]
        pub unsafe fn minimumSizeWithPrioritizedCompressionOptions(
            &self,
            prioritizedOptions: &NSArray<NSUserInterfaceCompressionOptions>,
        ) -> NSSize;
        #[method_id(activeCompressionOptions)]
        pub unsafe fn activeCompressionOptions(
            &self,
        ) -> Id<NSUserInterfaceCompressionOptions, Shared>;
    }
);
extern_methods!(
    #[doc = "NSButtonDeprecated"]
    unsafe impl NSButton {
        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, stringWithAmpersand: Option<&NSString>);
    }
);
