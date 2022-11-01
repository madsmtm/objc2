//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;

pub type NSButtonType = NSUInteger;
pub const NSButtonTypeMomentaryLight: NSButtonType = 0;
pub const NSButtonTypePushOnPushOff: NSButtonType = 1;
pub const NSButtonTypeToggle: NSButtonType = 2;
pub const NSButtonTypeSwitch: NSButtonType = 3;
pub const NSButtonTypeRadio: NSButtonType = 4;
pub const NSButtonTypeMomentaryChange: NSButtonType = 5;
pub const NSButtonTypeOnOff: NSButtonType = 6;
pub const NSButtonTypeMomentaryPushIn: NSButtonType = 7;
pub const NSButtonTypeAccelerator: NSButtonType = 8;
pub const NSButtonTypeMultiLevelAccelerator: NSButtonType = 9;

pub type NSBezelStyle = NSUInteger;
pub const NSBezelStyleRounded: NSBezelStyle = 1;
pub const NSBezelStyleRegularSquare: NSBezelStyle = 2;
pub const NSBezelStyleDisclosure: NSBezelStyle = 5;
pub const NSBezelStyleShadowlessSquare: NSBezelStyle = 6;
pub const NSBezelStyleCircular: NSBezelStyle = 7;
pub const NSBezelStyleTexturedSquare: NSBezelStyle = 8;
pub const NSBezelStyleHelpButton: NSBezelStyle = 9;
pub const NSBezelStyleSmallSquare: NSBezelStyle = 10;
pub const NSBezelStyleTexturedRounded: NSBezelStyle = 11;
pub const NSBezelStyleRoundRect: NSBezelStyle = 12;
pub const NSBezelStyleRecessed: NSBezelStyle = 13;
pub const NSBezelStyleRoundedDisclosure: NSBezelStyle = 14;
pub const NSBezelStyleInline: NSBezelStyle = 15;

extern_class!(
    #[derive(Debug)]
    pub struct NSButtonCell;

    unsafe impl ClassType for NSButtonCell {
        type Super = NSActionCell;
    }
);

extern_methods!(
    unsafe impl NSButtonCell {
        #[method_id(initTextCell:)]
        pub unsafe fn initTextCell(
            this: Option<Allocated<Self>>,
            string: &NSString,
        ) -> Id<Self, Shared>;

        #[method_id(initImageCell:)]
        pub unsafe fn initImageCell(
            this: Option<Allocated<Self>>,
            image: Option<&NSImage>,
        ) -> Id<Self, Shared>;

        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(
            this: Option<Allocated<Self>>,
            coder: &NSCoder,
        ) -> Id<Self, Shared>;

        #[method(bezelStyle)]
        pub unsafe fn bezelStyle(&self) -> NSBezelStyle;

        #[method(setBezelStyle:)]
        pub unsafe fn setBezelStyle(&self, bezelStyle: NSBezelStyle);

        #[method(setButtonType:)]
        pub unsafe fn setButtonType(&self, type_: NSButtonType);

        #[method(highlightsBy)]
        pub unsafe fn highlightsBy(&self) -> NSCellStyleMask;

        #[method(setHighlightsBy:)]
        pub unsafe fn setHighlightsBy(&self, highlightsBy: NSCellStyleMask);

        #[method(showsStateBy)]
        pub unsafe fn showsStateBy(&self) -> NSCellStyleMask;

        #[method(setShowsStateBy:)]
        pub unsafe fn setShowsStateBy(&self, showsStateBy: NSCellStyleMask);

        #[method_id(title)]
        pub unsafe fn title(&self) -> Id<NSString, Shared>;

        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

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

        #[method(isTransparent)]
        pub unsafe fn isTransparent(&self) -> bool;

        #[method(setTransparent:)]
        pub unsafe fn setTransparent(&self, transparent: bool);

        #[method(isOpaque)]
        pub unsafe fn isOpaque(&self) -> bool;

        #[method(imageDimsWhenDisabled)]
        pub unsafe fn imageDimsWhenDisabled(&self) -> bool;

        #[method(setImageDimsWhenDisabled:)]
        pub unsafe fn setImageDimsWhenDisabled(&self, imageDimsWhenDisabled: bool);

        #[method(showsBorderOnlyWhileMouseInside)]
        pub unsafe fn showsBorderOnlyWhileMouseInside(&self) -> bool;

        #[method(setShowsBorderOnlyWhileMouseInside:)]
        pub unsafe fn setShowsBorderOnlyWhileMouseInside(
            &self,
            showsBorderOnlyWhileMouseInside: bool,
        );

        #[method_id(sound)]
        pub unsafe fn sound(&self) -> Option<Id<NSSound, Shared>>;

        #[method(setSound:)]
        pub unsafe fn setSound(&self, sound: Option<&NSSound>);

        #[method_id(backgroundColor)]
        pub unsafe fn backgroundColor(&self) -> Option<Id<NSColor, Shared>>;

        #[method(setBackgroundColor:)]
        pub unsafe fn setBackgroundColor(&self, backgroundColor: Option<&NSColor>);

        #[method(setPeriodicDelay:interval:)]
        pub unsafe fn setPeriodicDelay_interval(&self, delay: c_float, interval: c_float);

        #[method(getPeriodicDelay:interval:)]
        pub unsafe fn getPeriodicDelay_interval(
            &self,
            delay: NonNull<c_float>,
            interval: NonNull<c_float>,
        );

        #[method(performClick:)]
        pub unsafe fn performClick(&self, sender: Option<&Object>);

        #[method(mouseEntered:)]
        pub unsafe fn mouseEntered(&self, event: &NSEvent);

        #[method(mouseExited:)]
        pub unsafe fn mouseExited(&self, event: &NSEvent);

        #[method(drawBezelWithFrame:inView:)]
        pub unsafe fn drawBezelWithFrame_inView(&self, frame: NSRect, controlView: &NSView);

        #[method(drawImage:withFrame:inView:)]
        pub unsafe fn drawImage_withFrame_inView(
            &self,
            image: &NSImage,
            frame: NSRect,
            controlView: &NSView,
        );

        #[method(drawTitle:withFrame:inView:)]
        pub unsafe fn drawTitle_withFrame_inView(
            &self,
            title: &NSAttributedString,
            frame: NSRect,
            controlView: &NSView,
        ) -> NSRect;
    }
);

pub type NSGradientType = NSUInteger;
pub const NSGradientNone: NSGradientType = 0;
pub const NSGradientConcaveWeak: NSGradientType = 1;
pub const NSGradientConcaveStrong: NSGradientType = 2;
pub const NSGradientConvexWeak: NSGradientType = 3;
pub const NSGradientConvexStrong: NSGradientType = 4;

pub static NSMomentaryLightButton: NSButtonType = NSButtonTypeMomentaryLight;

pub static NSPushOnPushOffButton: NSButtonType = NSButtonTypePushOnPushOff;

pub static NSToggleButton: NSButtonType = NSButtonTypeToggle;

pub static NSSwitchButton: NSButtonType = NSButtonTypeSwitch;

pub static NSRadioButton: NSButtonType = NSButtonTypeRadio;

pub static NSMomentaryChangeButton: NSButtonType = NSButtonTypeMomentaryChange;

pub static NSOnOffButton: NSButtonType = NSButtonTypeOnOff;

pub static NSMomentaryPushInButton: NSButtonType = NSButtonTypeMomentaryPushIn;

pub static NSAcceleratorButton: NSButtonType = NSButtonTypeAccelerator;

pub static NSMultiLevelAcceleratorButton: NSButtonType = NSButtonTypeMultiLevelAccelerator;

pub static NSMomentaryPushButton: NSButtonType = NSButtonTypeMomentaryLight;

pub static NSMomentaryLight: NSButtonType = NSButtonTypeMomentaryPushIn;

pub static NSRoundedBezelStyle: NSBezelStyle = NSBezelStyleRounded;

pub static NSRegularSquareBezelStyle: NSBezelStyle = NSBezelStyleRegularSquare;

pub static NSDisclosureBezelStyle: NSBezelStyle = NSBezelStyleDisclosure;

pub static NSShadowlessSquareBezelStyle: NSBezelStyle = NSBezelStyleShadowlessSquare;

pub static NSCircularBezelStyle: NSBezelStyle = NSBezelStyleCircular;

pub static NSTexturedSquareBezelStyle: NSBezelStyle = NSBezelStyleTexturedSquare;

pub static NSHelpButtonBezelStyle: NSBezelStyle = NSBezelStyleHelpButton;

pub static NSSmallSquareBezelStyle: NSBezelStyle = NSBezelStyleSmallSquare;

pub static NSTexturedRoundedBezelStyle: NSBezelStyle = NSBezelStyleTexturedRounded;

pub static NSRoundRectBezelStyle: NSBezelStyle = NSBezelStyleRoundRect;

pub static NSRecessedBezelStyle: NSBezelStyle = NSBezelStyleRecessed;

pub static NSRoundedDisclosureBezelStyle: NSBezelStyle = NSBezelStyleRoundedDisclosure;

pub static NSInlineBezelStyle: NSBezelStyle = NSBezelStyleInline;

pub static NSSmallIconButtonBezelStyle: NSBezelStyle = 2;

pub static NSThickSquareBezelStyle: NSBezelStyle = 3;

pub static NSThickerSquareBezelStyle: NSBezelStyle = 4;

extern_methods!(
    /// NSDeprecated
    unsafe impl NSButtonCell {
        #[method(gradientType)]
        pub unsafe fn gradientType(&self) -> NSGradientType;

        #[method(setGradientType:)]
        pub unsafe fn setGradientType(&self, gradientType: NSGradientType);

        #[method(setTitleWithMnemonic:)]
        pub unsafe fn setTitleWithMnemonic(&self, stringWithAmpersand: Option<&NSString>);

        #[method(setAlternateTitleWithMnemonic:)]
        pub unsafe fn setAlternateTitleWithMnemonic(&self, stringWithAmpersand: Option<&NSString>);

        #[method(setAlternateMnemonicLocation:)]
        pub unsafe fn setAlternateMnemonicLocation(&self, location: NSUInteger);

        #[method(alternateMnemonicLocation)]
        pub unsafe fn alternateMnemonicLocation(&self) -> NSUInteger;

        #[method_id(alternateMnemonic)]
        pub unsafe fn alternateMnemonic(&self) -> Option<Id<NSString, Shared>>;

        #[method_id(keyEquivalentFont)]
        pub unsafe fn keyEquivalentFont(&self) -> Option<Id<NSFont, Shared>>;

        #[method(setKeyEquivalentFont:)]
        pub unsafe fn setKeyEquivalentFont(&self, keyEquivalentFont: Option<&NSFont>);

        #[method(setKeyEquivalentFont:size:)]
        pub unsafe fn setKeyEquivalentFont_size(&self, fontName: &NSString, fontSize: CGFloat);
    }
);
