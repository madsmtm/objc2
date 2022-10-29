//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
#[allow(unused_imports)]
use objc2::rc::{Id, Shared};
#[allow(unused_imports)]
use objc2::{extern_class, extern_methods, ClassType};

extern_class!(
    #[derive(Debug)]
    pub struct NSScreen;

    unsafe impl ClassType for NSScreen {
        type Super = NSObject;
    }
);

extern_methods!(
    unsafe impl NSScreen {
        #[method_id(screens)]
        pub unsafe fn screens() -> Id<NSArray<NSScreen>, Shared>;

        #[method_id(mainScreen)]
        pub unsafe fn mainScreen() -> Option<Id<NSScreen, Shared>>;

        #[method_id(deepestScreen)]
        pub unsafe fn deepestScreen() -> Option<Id<NSScreen, Shared>>;

        #[method(screensHaveSeparateSpaces)]
        pub unsafe fn screensHaveSeparateSpaces() -> bool;

        #[method(depth)]
        pub unsafe fn depth(&self) -> NSWindowDepth;

        #[method(frame)]
        pub unsafe fn frame(&self) -> NSRect;

        #[method(visibleFrame)]
        pub unsafe fn visibleFrame(&self) -> NSRect;

        #[method_id(deviceDescription)]
        pub unsafe fn deviceDescription(
            &self,
        ) -> Id<NSDictionary<NSDeviceDescriptionKey, Object>, Shared>;

        #[method_id(colorSpace)]
        pub unsafe fn colorSpace(&self) -> Option<Id<NSColorSpace, Shared>>;

        #[method(supportedWindowDepths)]
        pub unsafe fn supportedWindowDepths(&self) -> NonNull<NSWindowDepth>;

        #[method(canRepresentDisplayGamut:)]
        pub unsafe fn canRepresentDisplayGamut(&self, displayGamut: NSDisplayGamut) -> bool;

        #[method(convertRectToBacking:)]
        pub unsafe fn convertRectToBacking(&self, rect: NSRect) -> NSRect;

        #[method(convertRectFromBacking:)]
        pub unsafe fn convertRectFromBacking(&self, rect: NSRect) -> NSRect;

        #[method(backingAlignedRect:options:)]
        pub unsafe fn backingAlignedRect_options(
            &self,
            rect: NSRect,
            options: NSAlignmentOptions,
        ) -> NSRect;

        #[method(backingScaleFactor)]
        pub unsafe fn backingScaleFactor(&self) -> CGFloat;

        #[method_id(localizedName)]
        pub unsafe fn localizedName(&self) -> Id<NSString, Shared>;

        #[method(safeAreaInsets)]
        pub unsafe fn safeAreaInsets(&self) -> NSEdgeInsets;

        #[method(auxiliaryTopLeftArea)]
        pub unsafe fn auxiliaryTopLeftArea(&self) -> NSRect;

        #[method(auxiliaryTopRightArea)]
        pub unsafe fn auxiliaryTopRightArea(&self) -> NSRect;
    }
);

extern_methods!(
    unsafe impl NSScreen {
        #[method(maximumExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumExtendedDynamicRangeColorComponentValue(&self) -> CGFloat;

        #[method(maximumPotentialExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumPotentialExtendedDynamicRangeColorComponentValue(&self) -> CGFloat;

        #[method(maximumReferenceExtendedDynamicRangeColorComponentValue)]
        pub unsafe fn maximumReferenceExtendedDynamicRangeColorComponentValue(&self) -> CGFloat;
    }
);

extern_methods!(
    unsafe impl NSScreen {
        #[method(maximumFramesPerSecond)]
        pub unsafe fn maximumFramesPerSecond(&self) -> NSInteger;

        #[method(minimumRefreshInterval)]
        pub unsafe fn minimumRefreshInterval(&self) -> NSTimeInterval;

        #[method(maximumRefreshInterval)]
        pub unsafe fn maximumRefreshInterval(&self) -> NSTimeInterval;

        #[method(displayUpdateGranularity)]
        pub unsafe fn displayUpdateGranularity(&self) -> NSTimeInterval;

        #[method(lastDisplayUpdateTimestamp)]
        pub unsafe fn lastDisplayUpdateTimestamp(&self) -> NSTimeInterval;
    }
);

extern_methods!(
    /// NSDeprecated
    unsafe impl NSScreen {
        #[method(userSpaceScaleFactor)]
        pub unsafe fn userSpaceScaleFactor(&self) -> CGFloat;
    }
);
