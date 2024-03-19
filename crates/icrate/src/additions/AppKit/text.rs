use objc2::ffi::NSInteger;

use super::TARGET_ABI_USES_IOS_VALUES;

ns_enum!(
    #[underlying(NSInteger)]
    #[allow(clippy::bool_to_int_with_if)]
    pub enum NSTextAlignment {
        #[doc(alias = "NSTextAlignmentLeft")]
        Left = 0,
        #[doc(alias = "NSTextAlignmentRight")]
        Right = if TARGET_ABI_USES_IOS_VALUES { 2 } else { 1 },
        #[doc(alias = "NSTextAlignmentCenter")]
        Center = if TARGET_ABI_USES_IOS_VALUES { 1 } else { 2 },
        #[doc(alias = "NSTextAlignmentJustified")]
        Justified = 3,
        #[doc(alias = "NSTextAlignmentNatural")]
        Natural = 4,
    }
);
