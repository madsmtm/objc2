mod cached;
#[cfg(feature = "Foundation_NSString")]
mod ns_string;

pub use self::cached::CachedId;
#[cfg(feature = "Foundation_NSString")]
pub use self::ns_string::*;
