#[macro_use]
extern crate objc;

pub use id::{Id, Owned, Ownership, Shared, ShareId};
pub use weak::WeakId;

mod id;
mod weak;
