mod NSUUID;
#[path = "NSDecimal.rs"]
mod __NSDecimal;
#[path = "NSNotFound.rs"]
mod __NSNotFound;
mod copy;
mod debug;
mod generic_return;
mod gnustep;
mod ns_consumed;

pub use self::__NSDecimal::NSDecimal;
pub use self::__NSNotFound::NSNotFound;
#[cfg(feature = "Foundation_NSMapTable")]
pub use self::ns_consumed::NSFreeMapTable;
#[cfg(feature = "Foundation_NSUUID")]
pub(crate) use self::NSUUID::UuidBytes;
