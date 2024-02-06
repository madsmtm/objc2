mod capture;
mod device;
mod packed;
#[cfg(feature = "unstable-private")]
mod private;
mod shift_mask;
mod slice;

pub use self::packed::{MTLPackedFloat3, __MTLPackedFloat3};
pub use self::shift_mask::*;

pub type CFTimeInterval = std::os::raw::c_double;
