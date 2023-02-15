#[cfg(feature = "objc2-proc-macros")]
extern crate self as objc2;

#[cfg(feature = "objc2-proc-macros")]
#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub use core::{
    borrow::{Borrow, BorrowMut},
    clone::Clone,
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    convert::{AsMut, AsRef, From, TryFrom},
    default::Default,
    fmt::Debug,
    hash::Hash,
    marker::{Copy, PhantomData, Sized},
    mem::size_of,
    ops::{Deref, DerefMut},
    option::Option::{self, None, Some},
    panic,
    panic::UnwindSafe,
    primitive::str,
    result::Result::{self, Err, Ok},
    unreachable,
};

#[cfg(feature = "objc2-proc-macros")]
#[cfg(feature = "std")]
#[doc(hidden)]
pub use std::{
    borrow::{Borrow, BorrowMut},
    clone::Clone,
    cmp::{Eq, Ord, PartialEq, PartialOrd},
    convert::{AsMut, AsRef, From, TryFrom},
    default::Default,
    fmt::Debug,
    hash::Hash,
    marker::{Copy, PhantomData, Sized},
    mem::size_of,
    ops::{Deref, DerefMut},
    option::Option::{self, None, Some},
    panic,
    panic::UnwindSafe,
    primitive::str,
    result::Result::{self, Err, Ok},
    unreachable,
};

#[cfg(feature = "objc2-proc-macros")]
#[doc(hidden)]
pub use ::{bitflags, num_enum};
