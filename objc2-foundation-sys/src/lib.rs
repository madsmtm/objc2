#![no_std]
#![allow(non_camel_case_types)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![doc(html_root_url = "https://docs.rs/objc2-foundation-sys/0.0.1")]

extern crate std;

mod generated;

#[allow(improper_ctypes)] // TODO
mod generated_nsstring;

pub use generated::*;
pub use generated_nsstring::*;
