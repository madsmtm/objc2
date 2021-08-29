#[macro_use]
extern crate objc;

pub use objc::*;
use objc::runtime::*;
use objc::rc::*;

#[path = "../src/test_utils.rs"]
mod test_utils;
