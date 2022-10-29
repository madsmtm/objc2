#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};
extern_class!(
    #[derive(Debug)]
    pub struct NSColor;
    unsafe impl ClassType for NSColor {
        type Super = NSObject;
    }
);
extern_methods!(
    unsafe impl NSColor {
        #[method_id(init)]
        pub unsafe fn init(&self) -> Id<Self, Shared>;
        #[method_id(initWithCoder:)]
        pub unsafe fn initWithCoder(&self, coder: &NSCoder) -> Option<Id<Self, Shared>>;
        #[method_id(colorWithColorSpace:components:count:)]
        pub unsafe fn colorWithColorSpace_components_count(
            space: &NSColorSpace,
            components: NonNull<CGFloat>,
            numberOfComponents: NSInteger,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithSRGBRed:green:blue:alpha:)]
        pub unsafe fn colorWithSRGBRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        # [method_id (colorWithGenericGamma22White : alpha :)]
        pub unsafe fn colorWithGenericGamma22White_alpha(
            white: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        # [method_id (colorWithDisplayP3Red : green : blue : alpha :)]
        pub unsafe fn colorWithDisplayP3Red_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithWhite:alpha:)]
        pub unsafe fn colorWithWhite_alpha(white: CGFloat, alpha: CGFloat) -> Id<NSColor, Shared>;
        #[method_id(colorWithRed:green:blue:alpha:)]
        pub unsafe fn colorWithRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithHue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithHue_saturation_brightness_alpha(
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithColorSpace:hue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithColorSpace_hue_saturation_brightness_alpha(
            space: &NSColorSpace,
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithCatalogName:colorName:)]
        pub unsafe fn colorWithCatalogName_colorName(
            listName: &NSColorListName,
            colorName: &NSColorName,
        ) -> Option<Id<NSColor, Shared>>;
        #[method_id(colorNamed:bundle:)]
        pub unsafe fn colorNamed_bundle(
            name: &NSColorName,
            bundle: Option<&NSBundle>,
        ) -> Option<Id<NSColor, Shared>>;
        #[method_id(colorNamed:)]
        pub unsafe fn colorNamed(name: &NSColorName) -> Option<Id<NSColor, Shared>>;
        #[method_id(colorWithName:dynamicProvider:)]
        pub unsafe fn colorWithName_dynamicProvider(
            colorName: Option<&NSColorName>,
            dynamicProvider: TodoBlock,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithDeviceWhite:alpha:)]
        pub unsafe fn colorWithDeviceWhite_alpha(
            white: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithDeviceRed:green:blue:alpha:)]
        pub unsafe fn colorWithDeviceRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithDeviceHue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithDeviceHue_saturation_brightness_alpha(
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithDeviceCyan:magenta:yellow:black:alpha:)]
        pub unsafe fn colorWithDeviceCyan_magenta_yellow_black_alpha(
            cyan: CGFloat,
            magenta: CGFloat,
            yellow: CGFloat,
            black: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithCalibratedWhite:alpha:)]
        pub unsafe fn colorWithCalibratedWhite_alpha(
            white: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithCalibratedRed:green:blue:alpha:)]
        pub unsafe fn colorWithCalibratedRed_green_blue_alpha(
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithCalibratedHue:saturation:brightness:alpha:)]
        pub unsafe fn colorWithCalibratedHue_saturation_brightness_alpha(
            hue: CGFloat,
            saturation: CGFloat,
            brightness: CGFloat,
            alpha: CGFloat,
        ) -> Id<NSColor, Shared>;
        #[method_id(colorWithPatternImage:)]
        pub unsafe fn colorWithPatternImage(image: &NSImage) -> Id<NSColor, Shared>;
        #[method(type)]
        pub unsafe fn type_(&self) -> NSColorType;
        #[method_id(colorUsingType:)]
        pub unsafe fn colorUsingType(&self, type_: NSColorType) -> Option<Id<NSColor, Shared>>;
        #[method_id(colorUsingColorSpace:)]
        pub unsafe fn colorUsingColorSpace(
            &self,
            space: &NSColorSpace,
        ) -> Option<Id<NSColor, Shared>>;
        #[method_id(blackColor)]
        pub unsafe fn blackColor() -> Id<NSColor, Shared>;
        #[method_id(darkGrayColor)]
        pub unsafe fn darkGrayColor() -> Id<NSColor, Shared>;
        #[method_id(lightGrayColor)]
        pub unsafe fn lightGrayColor() -> Id<NSColor, Shared>;
        #[method_id(whiteColor)]
        pub unsafe fn whiteColor() -> Id<NSColor, Shared>;
        #[method_id(grayColor)]
        pub unsafe fn grayColor() -> Id<NSColor, Shared>;
        #[method_id(redColor)]
        pub unsafe fn redColor() -> Id<NSColor, Shared>;
        #[method_id(greenColor)]
        pub unsafe fn greenColor() -> Id<NSColor, Shared>;
        #[method_id(blueColor)]
        pub unsafe fn blueColor() -> Id<NSColor, Shared>;
        #[method_id(cyanColor)]
        pub unsafe fn cyanColor() -> Id<NSColor, Shared>;
        #[method_id(yellowColor)]
        pub unsafe fn yellowColor() -> Id<NSColor, Shared>;
        #[method_id(magentaColor)]
        pub unsafe fn magentaColor() -> Id<NSColor, Shared>;
        #[method_id(orangeColor)]
        pub unsafe fn orangeColor() -> Id<NSColor, Shared>;
        #[method_id(purpleColor)]
        pub unsafe fn purpleColor() -> Id<NSColor, Shared>;
        #[method_id(brownColor)]
        pub unsafe fn brownColor() -> Id<NSColor, Shared>;
        #[method_id(clearColor)]
        pub unsafe fn clearColor() -> Id<NSColor, Shared>;
        #[method_id(labelColor)]
        pub unsafe fn labelColor() -> Id<NSColor, Shared>;
        #[method_id(secondaryLabelColor)]
        pub unsafe fn secondaryLabelColor() -> Id<NSColor, Shared>;
        #[method_id(tertiaryLabelColor)]
        pub unsafe fn tertiaryLabelColor() -> Id<NSColor, Shared>;
        #[method_id(quaternaryLabelColor)]
        pub unsafe fn quaternaryLabelColor() -> Id<NSColor, Shared>;
        #[method_id(linkColor)]
        pub unsafe fn linkColor() -> Id<NSColor, Shared>;
        #[method_id(placeholderTextColor)]
        pub unsafe fn placeholderTextColor() -> Id<NSColor, Shared>;
        #[method_id(windowFrameTextColor)]
        pub unsafe fn windowFrameTextColor() -> Id<NSColor, Shared>;
        #[method_id(selectedMenuItemTextColor)]
        pub unsafe fn selectedMenuItemTextColor() -> Id<NSColor, Shared>;
        #[method_id(alternateSelectedControlTextColor)]
        pub unsafe fn alternateSelectedControlTextColor() -> Id<NSColor, Shared>;
        #[method_id(headerTextColor)]
        pub unsafe fn headerTextColor() -> Id<NSColor, Shared>;
        #[method_id(separatorColor)]
        pub unsafe fn separatorColor() -> Id<NSColor, Shared>;
        #[method_id(gridColor)]
        pub unsafe fn gridColor() -> Id<NSColor, Shared>;
        #[method_id(windowBackgroundColor)]
        pub unsafe fn windowBackgroundColor() -> Id<NSColor, Shared>;
        #[method_id(underPageBackgroundColor)]
        pub unsafe fn underPageBackgroundColor() -> Id<NSColor, Shared>;
        #[method_id(controlBackgroundColor)]
        pub unsafe fn controlBackgroundColor() -> Id<NSColor, Shared>;
        #[method_id(selectedContentBackgroundColor)]
        pub unsafe fn selectedContentBackgroundColor() -> Id<NSColor, Shared>;
        #[method_id(unemphasizedSelectedContentBackgroundColor)]
        pub unsafe fn unemphasizedSelectedContentBackgroundColor() -> Id<NSColor, Shared>;
        #[method_id(alternatingContentBackgroundColors)]
        pub unsafe fn alternatingContentBackgroundColors() -> Id<NSArray<NSColor>, Shared>;
        #[method_id(findHighlightColor)]
        pub unsafe fn findHighlightColor() -> Id<NSColor, Shared>;
        #[method_id(textColor)]
        pub unsafe fn textColor() -> Id<NSColor, Shared>;
        #[method_id(textBackgroundColor)]
        pub unsafe fn textBackgroundColor() -> Id<NSColor, Shared>;
        #[method_id(selectedTextColor)]
        pub unsafe fn selectedTextColor() -> Id<NSColor, Shared>;
        #[method_id(selectedTextBackgroundColor)]
        pub unsafe fn selectedTextBackgroundColor() -> Id<NSColor, Shared>;
        #[method_id(unemphasizedSelectedTextBackgroundColor)]
        pub unsafe fn unemphasizedSelectedTextBackgroundColor() -> Id<NSColor, Shared>;
        #[method_id(unemphasizedSelectedTextColor)]
        pub unsafe fn unemphasizedSelectedTextColor() -> Id<NSColor, Shared>;
        #[method_id(controlColor)]
        pub unsafe fn controlColor() -> Id<NSColor, Shared>;
        #[method_id(controlTextColor)]
        pub unsafe fn controlTextColor() -> Id<NSColor, Shared>;
        #[method_id(selectedControlColor)]
        pub unsafe fn selectedControlColor() -> Id<NSColor, Shared>;
        #[method_id(selectedControlTextColor)]
        pub unsafe fn selectedControlTextColor() -> Id<NSColor, Shared>;
        #[method_id(disabledControlTextColor)]
        pub unsafe fn disabledControlTextColor() -> Id<NSColor, Shared>;
        #[method_id(keyboardFocusIndicatorColor)]
        pub unsafe fn keyboardFocusIndicatorColor() -> Id<NSColor, Shared>;
        #[method_id(scrubberTexturedBackgroundColor)]
        pub unsafe fn scrubberTexturedBackgroundColor() -> Id<NSColor, Shared>;
        #[method_id(systemRedColor)]
        pub unsafe fn systemRedColor() -> Id<NSColor, Shared>;
        #[method_id(systemGreenColor)]
        pub unsafe fn systemGreenColor() -> Id<NSColor, Shared>;
        #[method_id(systemBlueColor)]
        pub unsafe fn systemBlueColor() -> Id<NSColor, Shared>;
        #[method_id(systemOrangeColor)]
        pub unsafe fn systemOrangeColor() -> Id<NSColor, Shared>;
        #[method_id(systemYellowColor)]
        pub unsafe fn systemYellowColor() -> Id<NSColor, Shared>;
        #[method_id(systemBrownColor)]
        pub unsafe fn systemBrownColor() -> Id<NSColor, Shared>;
        #[method_id(systemPinkColor)]
        pub unsafe fn systemPinkColor() -> Id<NSColor, Shared>;
        #[method_id(systemPurpleColor)]
        pub unsafe fn systemPurpleColor() -> Id<NSColor, Shared>;
        #[method_id(systemGrayColor)]
        pub unsafe fn systemGrayColor() -> Id<NSColor, Shared>;
        #[method_id(systemTealColor)]
        pub unsafe fn systemTealColor() -> Id<NSColor, Shared>;
        #[method_id(systemIndigoColor)]
        pub unsafe fn systemIndigoColor() -> Id<NSColor, Shared>;
        #[method_id(systemMintColor)]
        pub unsafe fn systemMintColor() -> Id<NSColor, Shared>;
        #[method_id(systemCyanColor)]
        pub unsafe fn systemCyanColor() -> Id<NSColor, Shared>;
        #[method_id(controlAccentColor)]
        pub unsafe fn controlAccentColor() -> Id<NSColor, Shared>;
        #[method(currentControlTint)]
        pub unsafe fn currentControlTint() -> NSControlTint;
        #[method_id(colorForControlTint:)]
        pub unsafe fn colorForControlTint(controlTint: NSControlTint) -> Id<NSColor, Shared>;
        #[method_id(highlightColor)]
        pub unsafe fn highlightColor() -> Id<NSColor, Shared>;
        #[method_id(shadowColor)]
        pub unsafe fn shadowColor() -> Id<NSColor, Shared>;
        #[method_id(highlightWithLevel:)]
        pub unsafe fn highlightWithLevel(&self, val: CGFloat) -> Option<Id<NSColor, Shared>>;
        #[method_id(shadowWithLevel:)]
        pub unsafe fn shadowWithLevel(&self, val: CGFloat) -> Option<Id<NSColor, Shared>>;
        #[method_id(colorWithSystemEffect:)]
        pub unsafe fn colorWithSystemEffect(
            &self,
            systemEffect: NSColorSystemEffect,
        ) -> Id<NSColor, Shared>;
        #[method(set)]
        pub unsafe fn set(&self);
        #[method(setFill)]
        pub unsafe fn setFill(&self);
        #[method(setStroke)]
        pub unsafe fn setStroke(&self);
        #[method_id(blendedColorWithFraction:ofColor:)]
        pub unsafe fn blendedColorWithFraction_ofColor(
            &self,
            fraction: CGFloat,
            color: &NSColor,
        ) -> Option<Id<NSColor, Shared>>;
        #[method_id(colorWithAlphaComponent:)]
        pub unsafe fn colorWithAlphaComponent(&self, alpha: CGFloat) -> Id<NSColor, Shared>;
        #[method_id(catalogNameComponent)]
        pub unsafe fn catalogNameComponent(&self) -> Id<NSColorListName, Shared>;
        #[method_id(colorNameComponent)]
        pub unsafe fn colorNameComponent(&self) -> Id<NSColorName, Shared>;
        #[method_id(localizedCatalogNameComponent)]
        pub unsafe fn localizedCatalogNameComponent(&self) -> Id<NSString, Shared>;
        #[method_id(localizedColorNameComponent)]
        pub unsafe fn localizedColorNameComponent(&self) -> Id<NSString, Shared>;
        #[method(redComponent)]
        pub unsafe fn redComponent(&self) -> CGFloat;
        #[method(greenComponent)]
        pub unsafe fn greenComponent(&self) -> CGFloat;
        #[method(blueComponent)]
        pub unsafe fn blueComponent(&self) -> CGFloat;
        #[method(getRed:green:blue:alpha:)]
        pub unsafe fn getRed_green_blue_alpha(
            &self,
            red: *mut CGFloat,
            green: *mut CGFloat,
            blue: *mut CGFloat,
            alpha: *mut CGFloat,
        );
        #[method(hueComponent)]
        pub unsafe fn hueComponent(&self) -> CGFloat;
        #[method(saturationComponent)]
        pub unsafe fn saturationComponent(&self) -> CGFloat;
        #[method(brightnessComponent)]
        pub unsafe fn brightnessComponent(&self) -> CGFloat;
        #[method(getHue:saturation:brightness:alpha:)]
        pub unsafe fn getHue_saturation_brightness_alpha(
            &self,
            hue: *mut CGFloat,
            saturation: *mut CGFloat,
            brightness: *mut CGFloat,
            alpha: *mut CGFloat,
        );
        #[method(whiteComponent)]
        pub unsafe fn whiteComponent(&self) -> CGFloat;
        #[method(getWhite:alpha:)]
        pub unsafe fn getWhite_alpha(&self, white: *mut CGFloat, alpha: *mut CGFloat);
        #[method(cyanComponent)]
        pub unsafe fn cyanComponent(&self) -> CGFloat;
        #[method(magentaComponent)]
        pub unsafe fn magentaComponent(&self) -> CGFloat;
        #[method(yellowComponent)]
        pub unsafe fn yellowComponent(&self) -> CGFloat;
        #[method(blackComponent)]
        pub unsafe fn blackComponent(&self) -> CGFloat;
        #[method(getCyan:magenta:yellow:black:alpha:)]
        pub unsafe fn getCyan_magenta_yellow_black_alpha(
            &self,
            cyan: *mut CGFloat,
            magenta: *mut CGFloat,
            yellow: *mut CGFloat,
            black: *mut CGFloat,
            alpha: *mut CGFloat,
        );
        #[method_id(colorSpace)]
        pub unsafe fn colorSpace(&self) -> Id<NSColorSpace, Shared>;
        #[method(numberOfComponents)]
        pub unsafe fn numberOfComponents(&self) -> NSInteger;
        #[method(getComponents:)]
        pub unsafe fn getComponents(&self, components: NonNull<CGFloat>);
        #[method_id(patternImage)]
        pub unsafe fn patternImage(&self) -> Id<NSImage, Shared>;
        #[method(alphaComponent)]
        pub unsafe fn alphaComponent(&self) -> CGFloat;
        #[method_id(colorFromPasteboard:)]
        pub unsafe fn colorFromPasteboard(pasteBoard: &NSPasteboard)
            -> Option<Id<NSColor, Shared>>;
        #[method(writeToPasteboard:)]
        pub unsafe fn writeToPasteboard(&self, pasteBoard: &NSPasteboard);
        #[method(drawSwatchInRect:)]
        pub unsafe fn drawSwatchInRect(&self, rect: NSRect);
        #[method_id(colorWithCGColor:)]
        pub unsafe fn colorWithCGColor(cgColor: CGColorRef) -> Option<Id<NSColor, Shared>>;
        #[method(CGColor)]
        pub unsafe fn CGColor(&self) -> CGColorRef;
        #[method(ignoresAlpha)]
        pub unsafe fn ignoresAlpha() -> bool;
        #[method(setIgnoresAlpha:)]
        pub unsafe fn setIgnoresAlpha(ignoresAlpha: bool);
    }
);
extern_methods!(
    #[doc = "NSDeprecated"]
    unsafe impl NSColor {
        #[method_id(controlHighlightColor)]
        pub unsafe fn controlHighlightColor() -> Id<NSColor, Shared>;
        #[method_id(controlLightHighlightColor)]
        pub unsafe fn controlLightHighlightColor() -> Id<NSColor, Shared>;
        #[method_id(controlShadowColor)]
        pub unsafe fn controlShadowColor() -> Id<NSColor, Shared>;
        #[method_id(controlDarkShadowColor)]
        pub unsafe fn controlDarkShadowColor() -> Id<NSColor, Shared>;
        #[method_id(scrollBarColor)]
        pub unsafe fn scrollBarColor() -> Id<NSColor, Shared>;
        #[method_id(knobColor)]
        pub unsafe fn knobColor() -> Id<NSColor, Shared>;
        #[method_id(selectedKnobColor)]
        pub unsafe fn selectedKnobColor() -> Id<NSColor, Shared>;
        #[method_id(windowFrameColor)]
        pub unsafe fn windowFrameColor() -> Id<NSColor, Shared>;
        #[method_id(selectedMenuItemColor)]
        pub unsafe fn selectedMenuItemColor() -> Id<NSColor, Shared>;
        #[method_id(headerColor)]
        pub unsafe fn headerColor() -> Id<NSColor, Shared>;
        #[method_id(secondarySelectedControlColor)]
        pub unsafe fn secondarySelectedControlColor() -> Id<NSColor, Shared>;
        #[method_id(alternateSelectedControlColor)]
        pub unsafe fn alternateSelectedControlColor() -> Id<NSColor, Shared>;
        #[method_id(controlAlternatingRowBackgroundColors)]
        pub unsafe fn controlAlternatingRowBackgroundColors() -> Id<NSArray<NSColor>, Shared>;
        #[method_id(colorSpaceName)]
        pub unsafe fn colorSpaceName(&self) -> Id<NSColorSpaceName, Shared>;
        #[method_id(colorUsingColorSpaceName:device:)]
        pub unsafe fn colorUsingColorSpaceName_device(
            &self,
            name: Option<&NSColorSpaceName>,
            deviceDescription: Option<&NSDictionary<NSDeviceDescriptionKey, Object>>,
        ) -> Option<Id<NSColor, Shared>>;
        #[method_id(colorUsingColorSpaceName:)]
        pub unsafe fn colorUsingColorSpaceName(
            &self,
            name: &NSColorSpaceName,
        ) -> Option<Id<NSColor, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSQuartzCoreAdditions"]
    unsafe impl NSColor {
        #[method_id(colorWithCIColor:)]
        pub unsafe fn colorWithCIColor(color: &CIColor) -> Id<NSColor, Shared>;
    }
);
extern_methods!(
    #[doc = "NSAppKitAdditions"]
    unsafe impl CIColor {
        #[method_id(initWithColor:)]
        pub unsafe fn initWithColor(&self, color: &NSColor) -> Option<Id<Self, Shared>>;
    }
);
extern_methods!(
    #[doc = "NSAppKitColorExtensions"]
    unsafe impl NSCoder {
        #[method_id(decodeNXColor)]
        pub unsafe fn decodeNXColor(&self) -> Option<Id<NSColor, Shared>>;
    }
);