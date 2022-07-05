use core::fmt;

use crate::parse;

/// An Objective-C type-encoding.
///
/// Can be retrieved in Objective-C for a type `T` using the `@encode(T)`
/// directive.
/// ```objc
/// NSLog(@"Encoding of NSException: %s", @encode(NSException));
/// ```
///
/// The [`Display`][`fmt::Display`] implementation converts the [`Encoding`]
/// into its string representation, that the the `@encode` directive would
/// return. This can be used conveniently through the `to_string` method:
///
/// ```
/// use objc2_encode::Encoding;
/// assert_eq!(Encoding::Int.to_string(), "i");
/// ```
///
/// For more information on the string value of an encoding, see [Apple's
/// documentation][ocrtTypeEncodings].
///
/// [ocrtTypeEncodings]: https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/ObjCRuntimeGuide/Articles/ocrtTypeEncodings.html
///
/// # Examples
///
/// Comparing an encoding to a string from the Objective-C runtime:
///
/// ```
/// use objc2_encode::Encoding;
/// assert!(Encoding::Array(10, &Encoding::FloatComplex).equivalent_to_str("[10jf]"));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive] // Maybe we're missing some encodings?
pub enum Encoding<'a> {
    /// A C `char`. Corresponds to the `c` code.
    Char,
    /// A C `short`. Corresponds to the `s` code.
    Short,
    /// A C `int`. Corresponds to the `i` code.
    Int,
    /// A C `long`. Corresponds to the `l` code.
    ///
    /// This is treated as a 32-bit quantity in 64-bit programs.
    // TODO: What does that mean??
    Long,
    /// A C `long long`. Corresponds to the `q` code.
    LongLong,
    /// A C `unsigned char`. Corresponds to the `C` code.
    UChar,
    /// A C `unsigned short`. Corresponds to the `S` code.
    UShort,
    /// A C `unsigned int`. Corresponds to the `I` code.
    UInt,
    /// A C `unsigned long`. Corresponds to the `L` code.
    ULong,
    /// A C `unsigned long long`. Corresponds to the `Q` code.
    ULongLong,
    /// A C `float`. Corresponds to the `f` code.
    Float,
    /// A C `double`. Corresponds to the `d` code.
    Double,
    /// A C `long double`. Corresponds to the `D` code.
    LongDouble,
    /// A C `float _Complex`. Corresponds to the `jf` code.
    FloatComplex,
    /// A C `_Complex` or `double _Complex`. Corresponds to the `jd` code.
    DoubleComplex,
    /// A C `long double _Complex`. Corresponds to the `jD` code.
    LongDoubleComplex,
    // TODO: Complex(&Encoding) ???
    /// A C++ `bool` / C99 `_Bool`. Corresponds to the `B` code.
    Bool,
    /// A C `void`. Corresponds to the `v` code.
    Void,
    /// A C `char *`. Corresponds to the `*` code.
    String,
    /// An Objective-C object (`id`). Corresponds to the `@` code.
    Object,
    /// An Objective-C block. Corresponds to the `@?` code.
    Block,
    /// An Objective-C class (`Class`). Corresponds to the `#` code.
    Class,
    /// An Objective-C selector (`SEL`). Corresponds to the `:` code.
    Sel,
    /// An unknown type. Corresponds to the `?` code.
    ///
    /// This is usually used to encode functions.
    Unknown,
    /// A bitfield with the given number of bits, and the given type.
    ///
    /// The type is not currently used, but may be in the future for better
    /// compatibility with Objective-C runtimes.
    ///
    /// Corresponds to the `b`num code.
    BitField(u8, &'a Encoding<'a>),
    /// A pointer to the given type.
    ///
    /// Corresponds to the `^`type code.
    Pointer(&'a Encoding<'a>),
    /// An array with the given length and type.
    ///
    /// Corresponds to the `[len type]` code.
    Array(usize, &'a Encoding<'a>),
    /// A struct with the given name and fields.
    ///
    /// The order of the fields must match the order of the order in this.
    ///
    /// It is not uncommon for the name to be `"?"`.
    ///
    /// Corresponds to the `{name=fields...}` code.
    Struct(&'a str, &'a [Encoding<'a>]),
    /// A union with the given name and fields.
    ///
    /// The order of the fields must match the order of the order in this.
    ///
    /// Corresponds to the `(name=fields...)` code.
    Union(&'a str, &'a [Encoding<'a>]),
    // "Vector" types have the '!' encoding, but are not implemented in clang

    // TODO: Atomic, const and other such specifiers
    // typedef struct x {
    //     int a;
    //     void* b;
    // } x_t;
    // NSLog(@"Encoding: %s", @encode(_Atomic x_t)); // -> A{x}
    // NSLog(@"Encoding: %s", @encode(const int*)); // -> r^i
    //
    // Note that const only applies to the outermost pointer!
    //
    // And how does atomic objects work?
    // core::sync::atomic::AtomicPtr<T: Message>?

    // TODO: `t` and `T` codes for i128 and u128?
}

impl Encoding<'_> {
    /// The encoding of [`c_long`](`std::os::raw::c_long`).
    ///
    /// Ideally the encoding of `long` would just be the same as `int` when
    /// it's 32 bits wide and the same as `long long` when it is 64 bits wide;
    /// then `c_long::ENCODING` would just work.
    ///
    /// Unfortunately, `long` have a different encoding than `int` when it is
    /// 32 bits wide; the [`l`][`Encoding::Long`] encoding.
    pub const C_LONG: Self = {
        // Alternative: `mem::size_of::<c_long>() == 4`
        // That would exactly match what `clang` does:
        // https://github.com/llvm/llvm-project/blob/release/13.x/clang/lib/AST/ASTContext.cpp#L7245
        if cfg!(any(target_pointer_width = "32", windows)) {
            // @encode(long) = 'l'
            Self::Long
        } else {
            // @encode(long) = 'q'
            Self::LongLong
        }
    };

    /// The encoding of [`c_ulong`](`std::os::raw::c_ulong`).
    ///
    /// See [`Encoding::C_LONG`] for explanation.
    pub const C_ULONG: Self = {
        if cfg!(any(target_pointer_width = "32", windows)) {
            // @encode(unsigned long) = 'L'
            Self::ULong
        } else {
            // @encode(unsigned long) = 'Q'
            Self::ULongLong
        }
    };

    /// Check if one encoding is equivalent to another.
    ///
    /// Currently, equivalence testing requires that the encodings are equal,
    /// except for qualifiers. This may be changed in the future to e.g.
    /// ignore struct names, to allow structs behind multiple pointers to be
    /// considered equivalent, or similar changes that may be required because
    /// of limitations in Objective-C compiler implementations.
    ///
    /// For example, you should not rely on two equivalent encodings to have
    /// the same size or ABI - that is provided on a best-effort basis.
    pub fn equivalent_to(&self, other: &Self) -> bool {
        // For now, because we don't allow representing qualifiers
        self == other
    }

    /// Check if an encoding is equivalent to the given string representation.
    ///
    /// See [`Encoding::equivalent_to`] for details about the meaning of
    /// "equivalence".
    pub fn equivalent_to_str(&self, s: &str) -> bool {
        // if the given encoding can be successfully removed from the start
        // and an empty string remains, they were fully equivalent!
        if let Some(res) = self.equivalent_to_start_of_str(s) {
            res.is_empty()
        } else {
            false
        }
    }

    /// Check if an encoding is equivalent to the start of the given string
    /// representation.
    ///
    /// If it is equivalent, the remaining part of the string is returned.
    /// Otherwise this returns [`None`].
    ///
    /// See [`Encoding::equivalent_to`] for details about the meaning of
    /// "equivalence".
    pub fn equivalent_to_start_of_str<'a>(&self, s: &'a str) -> Option<&'a str> {
        // strip leading qualifiers
        let s = s.trim_start_matches(parse::QUALIFIERS);

        // TODO: Allow missing/"?" names in structs and unions?

        parse::rm_enc_prefix(s, self)
    }
}

/// Formats this [`Encoding`] in a similar way that the `@encode` directive
/// would ordinarily do.
///
/// You should not rely on the output of this to be stable across versions. It
/// may change if found to be required to be compatible with exisiting
/// Objective-C compilers.
impl fmt::Display for Encoding<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Encoding::*;
        let code = match *self {
            Char => "c",
            Short => "s",
            Int => "i",
            Long => "l",
            LongLong => "q",
            UChar => "C",
            UShort => "S",
            UInt => "I",
            ULong => "L",
            ULongLong => "Q",
            Float => "f",
            Double => "d",
            LongDouble => "D",
            FloatComplex => "jf",
            DoubleComplex => "jd",
            LongDoubleComplex => "jD",
            Bool => "B",
            Void => "v",
            String => "*",
            Object => "@",
            Block => "@?",
            Class => "#",
            Sel => ":",
            Unknown => "?",
            BitField(b, _type) => {
                // TODO: Use the type on GNUStep
                return write!(formatter, "b{}", b);
            }
            Pointer(t) => {
                return write!(formatter, "^{}", t);
            }
            Array(len, item) => {
                return write!(formatter, "[{}{}]", len, item);
            }
            Struct(name, fields) => {
                write!(formatter, "{{{}=", name)?;
                for field in fields {
                    fmt::Display::fmt(field, formatter)?;
                }
                return formatter.write_str("}");
            }
            Union(name, members) => {
                write!(formatter, "({}=", name)?;
                for member in members {
                    fmt::Display::fmt(member, formatter)?;
                }
                return formatter.write_str(")");
            }
        };
        formatter.write_str(code)
    }
}

#[cfg(test)]
mod tests {
    use super::Encoding;
    use crate::static_str::{static_encoding_str_array, static_encoding_str_len};
    use alloc::string::ToString;

    fn send_sync<T: Send + Sync>() {}

    #[test]
    fn test_send_sync() {
        send_sync::<Encoding<'_>>();
    }

    #[test]
    fn smoke() {
        assert!(Encoding::Short.equivalent_to_str("s"));
    }

    #[test]
    fn qualifiers() {
        assert!(Encoding::Void.equivalent_to_str("v"));
        assert!(Encoding::Void.equivalent_to_str("Vv"));
        assert!(Encoding::String.equivalent_to_str("*"));
        assert!(Encoding::String.equivalent_to_str("r*"));
    }

    macro_rules! assert_enc {
        ($(
            fn $name:ident() {
                $encoding:expr;
                $(
                    ~$equivalent_encoding:expr;
                )*
                $(
                    !$not_encoding:expr;
                )*
                $string:literal;
                $(
                    ~$equivalent_string:expr;
                )*
                $(
                    !$not_string:literal;
                )*
            }
        )+) => {$(
            #[test]
            fn $name() {
                const E: Encoding<'static> = $encoding;

                // Check PartialEq
                assert_eq!(E, E);

                // Check Display
                assert_eq!(E.to_string(), $string);

                // Check equivalence comparisons
                assert!(E.equivalent_to(&E));
                assert!(E.equivalent_to_str($string));
                assert_eq!(E.equivalent_to_start_of_str(concat!($string, "xyz")), Some("xyz"));
                $(
                    assert!(E.equivalent_to(&$equivalent_encoding));
                    assert!(E.equivalent_to_str(&$equivalent_encoding.to_string()));
                )*
                $(
                    assert!(E.equivalent_to_str($equivalent_string));
                )*

                // Negative checks
                $(
                    assert_ne!(E, $not_encoding);
                    assert!(!E.equivalent_to(&$not_encoding));
                    assert!(!E.equivalent_to_str(&$not_encoding.to_string()));
                )*
                $(
                    assert!(!E.equivalent_to_str(&$not_string));
                )*

                // Check static str
                const STATIC_ENCODING_DATA: [u8; static_encoding_str_len(E)] = static_encoding_str_array(E);
                const STATIC_ENCODING_STR: &str = unsafe { core::str::from_utf8_unchecked(&STATIC_ENCODING_DATA) };
                assert_eq!(STATIC_ENCODING_STR, $string);
            }
        )+};
    }

    assert_enc! {
        fn int() {
            Encoding::Int;
            !Encoding::Char;
            "i";
        }

        fn char() {
            Encoding::Char;
            !Encoding::Int;
            "c";
            // Qualifiers
            ~"rc";
            ~"nc";
            ~"Nc";
            ~"oc";
            ~"Oc";
            ~"Rc";
            ~"Vc";

            !"ri";
        }

        fn block() {
            Encoding::Block;
            "@?";
        }

        fn object() {
            Encoding::Object;
            !Encoding::Block;
            "@";
            !"@?";
        }

        fn unknown() {
            Encoding::Unknown;
            !Encoding::Block;
            "?";
        }

        fn object_unknown_in_struct() {
            Encoding::Struct("S", &[Encoding::Object, Encoding::Unknown]);
            // TODO:
            // | Encoding::Struct("S", &[Encoding::Block]);
            "{S=@?}";
        }

        fn double() {
            Encoding::Double;
            "d";
        }

        fn bitfield() {
            Encoding::BitField(32, &Encoding::Int);
            !Encoding::Int;
            !Encoding::BitField(33, &Encoding::Int);
            "b32";
            !"b32a";
            !"b";
            !"b-32";
        }

        fn pointer() {
            Encoding::Pointer(&Encoding::Int);
            !Encoding::Pointer(&Encoding::Char);
            !Encoding::Pointer(&Encoding::Pointer(&Encoding::Int));
            "^i";
        }

        fn array() {
            Encoding::Array(12, &Encoding::Int);
            !Encoding::Int;
            !Encoding::Array(11, &Encoding::Int);
            !Encoding::Array(12, &Encoding::Char);
            "[12i]";
            !"[12i";
        }

        fn struct_() {
            Encoding::Struct("SomeStruct", &[Encoding::Char, Encoding::Int]);
            !Encoding::Union("SomeStruct", &[Encoding::Char, Encoding::Int]);
            !Encoding::Int;
            !Encoding::Struct("SomeStruct", &[Encoding::Int, Encoding::Char]);
            !Encoding::Struct("AnotherName", &[Encoding::Char, Encoding::Int]);
            "{SomeStruct=ci}";
            !"{SomeStruct=ci";
            !"{SomeStruct}";
            !"{SomeStruct=}";
        }

        fn struct_unicode() {
            Encoding::Struct("☃", &[Encoding::Char]);
            "{☃=c}";
        }

        fn empty_struct() {
            Encoding::Struct("SomeStruct", &[]);
            "{SomeStruct=}";
            // TODO: Unsure about this
            !"{SomeStruct}";
        }

        fn union_() {
            Encoding::Union("Onion", &[Encoding::Char, Encoding::Int]);
            !Encoding::Struct("Onion", &[Encoding::Char, Encoding::Int]);
            !Encoding::Int;
            !Encoding::Union("Onion", &[Encoding::Int, Encoding::Char]);
            !Encoding::Union("AnotherUnion", &[Encoding::Char, Encoding::Int]);
            "(Onion=ci)";
            !"(Onion=ci";
        }

        fn nested() {
            Encoding::Struct(
                "A",
                &[
                    Encoding::Struct("B", &[Encoding::Int]),
                    Encoding::Char,
                ],
            );
            "{A={B=i}c}";
            !"{A={B=i}c";
        }

        fn various() {
            Encoding::Struct(
                "abc",
                &[
                    Encoding::Pointer(&Encoding::Array(8, &Encoding::Bool)),
                    Encoding::Union("def", &[Encoding::Block]),
                    Encoding::Pointer(&Encoding::Pointer(&Encoding::BitField(255, &Encoding::Int))),
                    Encoding::Unknown,
                ]
            );
            "{abc=^[8B](def=@?)^^b255?}";
        }
    }
}
