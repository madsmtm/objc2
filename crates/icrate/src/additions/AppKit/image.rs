use crate::common::*;

use super::TARGET_ABI_USES_IOS_VALUES;

ns_enum!(
    #[underlying(NSInteger)]
    #[allow(clippy::bool_to_int_with_if)]
    pub enum NSImageResizingMode {
        #[doc(alias = "NSImageResizingModeStretch")]
        Stretch = if TARGET_ABI_USES_IOS_VALUES { 0 } else { 1 },
        #[doc(alias = "NSImageResizingModeTile")]
        Tile = if TARGET_ABI_USES_IOS_VALUES { 1 } else { 0 },
    }
);

#[cfg(feature = "Foundation_NSObject")]
unsafe impl crate::Foundation::NSCoding for crate::AppKit::NSImage {}
