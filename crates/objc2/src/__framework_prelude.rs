//! Helper import prelude for framework crates.

// Note: While this is not public, it is still a breaking change to remove
// entries in here, since framework crates rely on it.

pub use core::ffi::{
    c_char, c_double, c_float, c_int, c_long, c_longlong, c_schar, c_short, c_uchar, c_uint,
    c_ulong, c_ulonglong, c_ushort, c_void,
};
pub use core::marker::PhantomData;
pub use core::ptr::NonNull;

pub use crate::encode::{Encode, Encoding, RefEncode};
pub use crate::ffi::{NSInteger, NSIntegerMax, NSUInteger, NSUIntegerMax};
pub use crate::rc::{Allocated, DefaultRetained, Retained};
pub use crate::runtime::{
    AnyClass, AnyObject, AnyProtocol, Bool, NSObject, NSObjectProtocol, ProtocolObject, Sel,
};
pub use crate::{
    extern_category, extern_class, extern_methods, extern_protocol, ClassType, MainThreadMarker,
    MainThreadOnly, Message, ProtocolType,
};

pub type TodoFunction = *const c_void;
pub type TodoProtocols = AnyObject;
pub type IMP = Option<crate::runtime::Imp>;
