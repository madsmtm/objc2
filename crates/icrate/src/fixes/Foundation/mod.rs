mod copy;
mod copying;
mod debug;
mod decimal;
mod enumerator;
mod exception;
mod generics;
mod gnustep;
mod not_found;
mod ns_consumed;
#[cfg(feature = "Foundation_NSUUID")]
mod uuid;

pub use self::copying::{NSCopying, NSMutableCopying};
pub use self::decimal::NSDecimal;
pub use self::enumerator::NSFastEnumerationState;
pub use self::generics::*;
pub use self::not_found::NSNotFound;
#[cfg(feature = "Foundation_NSMapTable")]
pub use self::ns_consumed::NSFreeMapTable;
#[cfg(feature = "Foundation_NSUUID")]
pub(crate) use self::uuid::UuidBytes;
