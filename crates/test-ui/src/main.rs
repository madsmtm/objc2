//! Run UI tests.
//!
//! Use as:
//! ```
//! TRYBUILD=overwrite RUSTFLAGS="--deny=warnings" cargo run --features=run --bin test-ui
//! ```
//!
//! These are not part of the normal test suite, because trybuild doesn't pass
//! feature flags (like --features gnustep-1-9) to its cargo invocation, and
//! hence they don't always work.
//!
//! Also, they're slower than most tests.

fn main() {
    let t = trybuild::TestCases::new();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("ui")
        .join("*.rs");
    t.compile_fail(path);
    // Make trybuild use `cargo build` instead of `cargo check`
    //
    // Workaround for https://github.com/dtolnay/trybuild/issues/241
    t.pass(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src")
            .join("hack.rs"),
    );
}
