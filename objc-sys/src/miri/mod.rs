//! A Rust implementation of the parts of Objective-C runtime that we use.
//!
//! So that Miri works on the pure-Rust Objective-C code that people write.

mod class;
mod message;
mod selector;

pub use message::{custom_msg_send, custom_msg_send_super};
