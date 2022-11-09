//! Run UI tests.
//!
//! Use as:
//! ```
//! TRYBUILD=overwrite cargo +nightly run --features=run --bin test-ui
//! ```
//!
//! These are not part of the normal test suite, because trybuild doesn't pass
//! feature flags (like --features gnustep-1-9) to its cargo invocation, and
//! hence they don't always work.
//!
//! Also, they're slower than most tests.

#[cfg(feature = "run")]
fn main() {
    let t = trybuild::TestCases::new();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("ui")
        .join("*.rs");
    t.compile_fail(path);
}

#[cfg(not(feature = "run"))]
fn main() {
    panic!("`run` feature must be enabled");
}
