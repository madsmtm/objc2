fn main() {
    cc::Build::new()
        .file("extern/exception.m")
        .flag("-fobjc-exceptions")
        .compile("librustobjcexception.a");
}
