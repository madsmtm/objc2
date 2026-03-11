fn main() {
    // Make Cargo not re-run the build-script unnecessarily.
    println!("cargo:rerun-if-changed=build.rs");

    let extension_bin = "fskit-example-extension";

    // Pass `-e _NSExtensionMain` to change the entry point for the extension.
    //
    // Combined with `#![no_main]`, this makes our binary an app extension.
    println!("cargo:rustc-link-arg-bin={extension_bin}=-e");
    println!("cargo:rustc-link-arg-bin={extension_bin}=_NSExtensionMain");

    // Configure the linker to give earlier diagnostics if linking dylibs that
    // aren't supported in application extensions. There currently aren't any
    // public frameworks / libraries where this is the case, so this doesn't
    // matter that much, but there might be in the future.
    println!("cargo:rustc-link-arg-bin={extension_bin}=-Wl,-application_extension");
}
