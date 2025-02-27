fn main() {
    // FIXME: We need a better way to determine this.
    // https://github.com/rust-lang/rust/issues/73628#issuecomment-2467241918
    println!("cargo:rustc-check-cfg=cfg(arm64e)");
    if std::env::var("TARGET").unwrap().contains("arm64e") {
        println!("cargo::rustc-cfg=arm64e");
    }
}
