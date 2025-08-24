//! Test that the `available!` macro is optimized as expected.
use objc2::available;

#[export_name = "fn1_always"]
fn always() -> bool {
    // Can elide the version check here
    available!(..)
}

#[export_name = "fn2_never"]
fn never() -> bool {
    // Can elide the version check here
    available!()
}

#[export_name = "fn3_low"]
fn low() -> bool {
    // Can elide the version check here
    available!(
        macos = 10.7,
        ios = 5.0,
        tvos = 5.0,
        watchos = 3.0,
        visionos = 1.0,
        ..
    )
}

#[export_name = "fn4_high"]
fn high() -> bool {
    // Has to insert a runtime check here
    available!(
        macos = 15.0,
        ios = 18.0,
        tvos = 18.0,
        watchos = 11.0,
        visionos = 2.0,
        ..
    )
}

#[export_name = "fn5_only_ios"]
fn only_ios() -> bool {
    // Can elide the version check here on macOS
    available!(ios = 5.0)
}

#[export_name = "fn6_two_checks"]
fn two_checks() -> bool {
    // Ideally only has to insert one runtime check here, but currently does two.

    let in_14 = available!(
        macos = 14.0,
        ios = 17.0,
        tvos = 17.0,
        watchos = 10.0,
        visionos = 1.0,
        ..
    );
    let in_15 = available!(
        macos = 15.0,
        ios = 18.0,
        tvos = 18.0,
        watchos = 11.0,
        visionos = 2.0,
        ..
    );
    in_14 && in_15
}
