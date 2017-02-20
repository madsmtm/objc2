Objective-C type encoding creation and parsing in Rust.

The Objective-C compiler encodes types as strings for usage in the runtime.
This crate aims to provide a strongly-typed (rather than stringly-typed) way
to create and describe these type encodings without memory allocation in Rust.

# Implementing Encode

This crate declares an `Encode` trait that can be implemented for types that
the Objective-C compiler can encode. Implementing this trait looks like:

``` rust
unsafe impl Encode for CGPoint {
    type Encoding = Struct<&'static str, (Primitive, Primitive)>;

    fn encode() -> Self::Encoding {
        Struct::new("CGPoint", (CGFloat::encode(), CGFloat::encode()))
    }
}
```

For an example of how this works with more complex types, like structs
containing structs, see the `core_graphics` example.

# Comparing with encoding strings

If you have an encoding string from the Objective-C runtime, it can be parsed
and compared with another encoding through a `StrEncoding`:

``` rust
let parsed = StrEncoding::from_str("i").unwrap();
assert!(parsed == &i32::encode());
```

# Generating encoding strings

Every `Encoding` implements `Display` as its string representation.
This can be generated conveniently through the `to_string` method:

``` rust
assert_eq!(i32::encode().to_string(), "i");
```
