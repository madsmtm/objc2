// Using a wrapper.h isn't strictly necessary for this example
// since there is only one header file to wrap - so this is just
// for illustration on what you might do in a larger project where
// you might have more than one header file to import.
//
// See the bindgen crate for more information on creating a
// wrapper header when generating Rust types from C headers.
//
// https://rust-lang.github.io/rust-bindgen/tutorial-2.html

#import "./shader_types.h"
