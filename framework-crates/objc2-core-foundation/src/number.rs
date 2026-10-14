use core::cmp::Ordering;
use core::ptr;

use crate::{kCFBooleanFalse, kCFBooleanTrue, CFBoolean, CFNumber, CFNumberType, CFRetained};

impl CFBoolean {
    #[inline]
    pub fn new(value: bool) -> &'static CFBoolean {
        if value {
            unsafe { kCFBooleanTrue }
        } else {
            unsafe { kCFBooleanFalse }
        }
    }

    #[inline]
    pub fn as_bool(&self) -> bool {
        self.value()
    }
}

macro_rules! def_new_fn {
    {$(
        $(#[$($m:meta)*])*
        ($fn_name:ident($fn_inp:ty); $type:ident),
    )*} => {$(
        $(#[$($m)*])*
        #[doc = concat!("Construct a new `CFNumber` from a [`", stringify!($fn_inp), "`].")]
        #[inline]
        pub fn $fn_name(val: $fn_inp) -> CFRetained<Self> {
            let ptr: *const $fn_inp = &val;
            unsafe { Self::new(None, CFNumberType::$type, ptr.cast()).expect("failed creating CFNumber") }
        }
    )*}
}

impl CFNumber {
    // NOTE: CFNumberType only has signed variants, but it's safe to us it for
    // the unsigned variants as well, since the only thing that matters is the
    // size and kind of number.
    //
    // (The primary other case where signed-ness matters is ABI, but that
    // doesn't apply here since the bytes are stored in memory behind pointers
    // instead of being passed around in function signatures).
    def_new_fn! {
        // `new_bool` not available, see `CFBoolean` instead.
        (new_i8(i8); SInt8Type),
        (new_u8(u8); SInt8Type),
        (new_i16(i16); SInt16Type),
        (new_u16(u16); SInt16Type),
        (new_i32(i32); SInt32Type),
        (new_u32(u32); SInt32Type),
        (new_i64(i64); SInt64Type),
        (new_u64(u64); SInt64Type),
        (new_isize(isize); CFIndexType), // Or NSIntegerType.
        (new_usize(usize); CFIndexType),

        (new_f32(f32); Float32Type),
        (new_f64(f64); Float64Type),
    }

    /// Construct a new `CFNumber` from a [`CGFloat`][crate::CGFloat].
    #[cfg(feature = "CFCGTypes")]
    #[inline]
    pub fn new_cgfloat(val: crate::CGFloat) -> CFRetained<Self> {
        #[cfg(not(target_pointer_width = "64"))]
        {
            Self::new_f32(val)
        }
        #[cfg(target_pointer_width = "64")]
        {
            Self::new_f64(val)
        }
    }
}

macro_rules! def_get_fn {
    {$(
        $(#[$($m:meta)*])*
        ($fn_name:ident -> $fn_ret:ty; $type:ident),
    )*} => {$(
        $(#[$($m)*])*
        #[doc = concat!("Extract the value of the `CFNumber` as a [`", stringify!($fn_ret), "`].")]
        #[inline]
        pub fn $fn_name(&self) -> Option<$fn_ret> {
            let mut value: $fn_ret = <$fn_ret>::default();
            let ptr: *mut $fn_ret = &mut value;
            let ret = unsafe { self.value(CFNumberType::$type, ptr.cast()) };
            if ret {
                Some(value)
            } else {
                None
            }
        }
    )*}
}

impl CFNumber {
    // See `def_new_fn` above for details.
    def_get_fn! {
        (as_i8 -> i8; SInt8Type),
        (as_u8 -> u8; SInt8Type),
        (as_i16 -> i16; SInt16Type),
        (as_u16 -> u16; SInt16Type),
        (as_i32 -> i32; SInt32Type),
        (as_u32 -> u32; SInt32Type),
        (as_i64 -> i64; SInt64Type),
        (as_u64 -> u64; SInt64Type),
        (as_isize -> isize; CFIndexType),
        (as_usize -> usize; CFIndexType),

        (as_f32 -> f32; Float32Type),
        (as_f64 -> f64; Float64Type),
    }

    /// Extract the value of the `CFNumber` as a [`CGFloat`][crate::CGFloat].
    #[cfg(feature = "CFCGTypes")]
    #[inline]
    pub fn as_cgfloat(&self) -> Option<crate::CGFloat> {
        #[cfg(not(target_pointer_width = "64"))]
        {
            self.as_f32()
        }
        #[cfg(target_pointer_width = "64")]
        {
            self.as_f64()
        }
    }
}

impl PartialOrd for CFNumber {
    #[inline]
    #[doc(alias = "CFNumberCompare")]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CFNumber {
    #[inline]
    #[doc(alias = "CFNumberCompare")]
    fn cmp(&self, other: &Self) -> Ordering {
        // Documented that one should pass NULL here.
        let context = ptr::null_mut();
        unsafe { self.compare(other, context) }.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_bool() {
        let cffalse = CFBoolean::new(false);
        let cftrue = CFBoolean::new(true);
        assert_ne!(cffalse, cftrue);
        assert_eq!(cftrue, CFBoolean::new(true));
        assert_eq!(cffalse, CFBoolean::new(false));
        assert!(!cffalse.as_bool());
        assert!(cftrue.as_bool());
    }

    #[test]
    fn new() {
        let n = CFNumber::new_i32(442);
        assert_ne!(n, CFNumber::new_i8(442i32 as i8));
        assert_ne!(n, CFNumber::new_u8(442u32 as u8));
        assert_eq!(n, CFNumber::new_i16(442));
        assert_eq!(n, CFNumber::new_u16(442));
        assert_eq!(n, CFNumber::new_i32(442));
        assert_eq!(n, CFNumber::new_u32(442));
        assert_eq!(n, CFNumber::new_i64(442));
        assert_eq!(n, CFNumber::new_u64(442));
        assert_eq!(n, CFNumber::new_isize(442));
        assert_eq!(n, CFNumber::new_usize(442));
        assert_eq!(n, CFNumber::new_f32(442.0));
        assert_eq!(n, CFNumber::new_f64(442.0));
    }

    #[test]
    fn value() {
        let n = CFNumber::new_i32(442);
        if cfg!(all(target_os = "macos", target_arch = "x86")) {
            // Seems to return `None` in the old runtime.
            assert_eq!(n.as_i8(), None);
            assert_eq!(n.as_u8(), None);
        } else {
            assert_eq!(n.as_i8(), Some(442i32 as i8));
            assert_eq!(n.as_u8(), Some(442u32 as u8));
        }
        assert_eq!(n.as_i16(), Some(442));
        assert_eq!(n.as_u16(), Some(442));
        assert_eq!(n.as_i32(), Some(442));
        assert_eq!(n.as_u32(), Some(442));
        assert_eq!(n.as_i64(), Some(442));
        assert_eq!(n.as_u64(), Some(442));
        assert_eq!(n.as_isize(), Some(442));
        assert_eq!(n.as_usize(), Some(442));
        assert_eq!(n.as_f32(), Some(442.0));
        assert_eq!(n.as_f64(), Some(442.0));
    }

    #[test]
    fn cmp_number() {
        assert!(CFNumber::new_i32(2) < CFNumber::new_i32(3));
        assert!(CFNumber::new_i32(3) == CFNumber::new_i32(3));
        assert!(CFNumber::new_i32(4) > CFNumber::new_i32(3));
    }

    #[test]
    fn signed_to_unsigned() {
        let n = CFNumber::new_u32(u32::MAX - 2);
        assert_eq!(n.as_i32(), Some((u32::MAX - 2) as i32));
    }

    #[test]
    fn weird_floats() {
        let n = CFNumber::new_f32(f32::INFINITY);
        assert_eq!(n.as_i32(), None);
        assert_eq!(n.as_f32(), Some(f32::INFINITY));

        let n = CFNumber::new_f32(f32::NEG_INFINITY);
        assert_eq!(n.as_i32(), None);
        assert_eq!(n.as_f32(), Some(f32::NEG_INFINITY));
    }
}
