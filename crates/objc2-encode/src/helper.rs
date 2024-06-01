use core::fmt;
use core::write;

use crate::parse::verify_name;
use crate::Encoding;
use crate::EncodingBox;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum NestingLevel {
    Top,
    Within,
    Bottom,
}

impl NestingLevel {
    pub(crate) const fn new() -> Self {
        Self::Top
    }

    pub(crate) const fn bitfield(self) -> Self {
        // This is a bit irrelevant, since bitfields can only contain integral
        // types
        self
    }

    pub(crate) const fn indirection(self, kind: IndirectionKind) -> Self {
        match kind {
            // Move all the way down
            IndirectionKind::Atomic => Self::Bottom,
            // Move one step down
            IndirectionKind::Pointer => match self {
                Self::Top => Self::Within,
                Self::Bottom | Self::Within => Self::Bottom,
            },
        }
    }

    pub(crate) const fn array(self) -> Self {
        // TODO: Is this correct?
        self
    }

    pub(crate) const fn container_include_fields(self) -> Option<Self> {
        match self {
            Self::Top | Self::Within => {
                // Move top one step down
                Some(Self::Within)
            }
            Self::Bottom => None,
        }
    }
}

pub(crate) fn compare_encodings<E1: EncodingType, E2: EncodingType>(
    enc1: &E1,
    enc2: &E2,
    level: NestingLevel,
    include_all: bool,
) -> bool {
    use Helper::*;
    // Note: Ideally `Block` and sequence of `Object, Unknown` in struct
    // should compare equivalent, but we don't bother since in practice a
    // plain `Unknown` will never appear.

    let level = if include_all {
        NestingLevel::new()
    } else {
        level
    };

    match (enc1.helper(), enc2.helper()) {
        (Primitive(p1), Primitive(p2)) => p1 == p2,
        (BitField(size1, Some((offset1, type1))), BitField(size2, Some((offset2, type2)))) => {
            size1 == size2
                && offset1 == offset2
                && compare_encodings(type1, type2, level.bitfield(), include_all)
        }
        (BitField(size1, None), BitField(size2, None)) => size1 == size2,
        // The type-encoding of a bitfield is always either available, or it
        // is not (depends on platform); so if it was available in one, but
        // not the other, we should compare the encodings unequal.
        (BitField(_, _), BitField(_, _)) => false,
        (Indirection(kind1, t1), Indirection(kind2, t2)) => {
            kind1 == kind2 && compare_encodings(t1, t2, level.indirection(kind1), include_all)
        }
        (Array(len1, item1), Array(len2, item2)) => {
            len1 == len2 && compare_encodings(item1, item2, level.array(), include_all)
        }
        (Container(kind1, name1, items1), Container(kind2, name2, items2)) => {
            kind1 == kind2 && name1 == name2 && {
                if let Some(level) = level.container_include_fields() {
                    // If either container is empty, then they are equivalent
                    if items1.is_empty() || items2.is_empty() {
                        return true;
                    }
                    if items1.len() != items2.len() {
                        return false;
                    }
                    for (item1, item2) in items1.iter().zip(items2.iter()) {
                        if !compare_encodings(item1, item2, level, include_all) {
                            return false;
                        }
                    }
                    true
                } else {
                    true
                }
            }
        }
        (NoneInvalid, NoneInvalid) => true,
        (_, _) => false,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) enum Primitive {
    Char,
    Short,
    Int,
    Long,
    LongLong,
    UChar,
    UShort,
    UInt,
    ULong,
    ULongLong,
    Float,
    Double,
    LongDouble,
    FloatComplex,
    DoubleComplex,
    LongDoubleComplex,
    Bool,
    Void,
    String,
    Object,
    Block,
    Class,
    Sel,
    Unknown,
}

impl Primitive {
    pub(crate) const fn to_str(self) -> &'static str {
        use Primitive::*;
        match self {
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
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum IndirectionKind {
    Atomic,
    Pointer,
}

impl IndirectionKind {
    pub(crate) const fn prefix(self) -> char {
        self.prefix_byte() as char
    }

    pub(crate) const fn prefix_byte(self) -> u8 {
        match self {
            Self::Atomic => b'A',
            Self::Pointer => b'^',
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ContainerKind {
    Struct,
    Union,
}

impl ContainerKind {
    pub(crate) const fn start(self) -> char {
        self.start_byte() as char
    }

    pub(crate) const fn end(self) -> char {
        self.end_byte() as char
    }

    pub(crate) const fn start_byte(self) -> u8 {
        match self {
            Self::Struct => b'{',
            Self::Union => b'(',
        }
    }

    pub(crate) const fn end_byte(self) -> u8 {
        match self {
            Self::Struct => b'}',
            Self::Union => b')',
        }
    }
}

impl fmt::Display for ContainerKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Struct => write!(f, "struct"),
            Self::Union => write!(f, "union"),
        }
    }
}

pub(crate) trait EncodingType: Sized + fmt::Debug {
    fn helper(&self) -> Helper<'_, Self>;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) enum Helper<'a, E = Encoding> {
    Primitive(Primitive),
    BitField(u8, Option<&'a (u64, E)>),
    Indirection(IndirectionKind, &'a E),
    Array(u64, &'a E),
    Container(ContainerKind, &'a str, &'a [E]),
    NoneInvalid,
}

impl<E: EncodingType> Helper<'_, E> {
    pub(crate) fn fmt(&self, f: &mut fmt::Formatter<'_>, level: NestingLevel) -> fmt::Result {
        match self {
            Self::Primitive(primitive) => {
                write!(f, "{}", primitive.to_str())?;
            }
            Self::BitField(size, None) => {
                write!(f, "b{size}")?;
            }
            Self::BitField(size, Some((offset, t))) => {
                write!(f, "b{offset}")?;
                t.helper().fmt(f, level.bitfield())?;
                write!(f, "{size}")?;
            }
            Self::Indirection(kind, t) => {
                write!(f, "{}", kind.prefix())?;
                t.helper().fmt(f, level.indirection(*kind))?;
            }
            Self::Array(len, item) => {
                write!(f, "[")?;
                write!(f, "{len}")?;
                item.helper().fmt(f, level.array())?;
                write!(f, "]")?;
            }
            Self::Container(kind, name, items) => {
                write!(f, "{}", kind.start())?;
                write!(f, "{name}")?;
                if let Some(level) = level.container_include_fields() {
                    write!(f, "=")?;
                    for item in *items {
                        item.helper().fmt(f, level)?;
                    }
                }
                write!(f, "{}", kind.end())?;
            }
            Self::NoneInvalid => {}
        }
        Ok(())
    }
}

impl Helper<'_> {
    pub(crate) const fn new(encoding: &Encoding) -> Self {
        use Encoding::*;
        match encoding {
            Char => Self::Primitive(Primitive::Char),
            Short => Self::Primitive(Primitive::Short),
            Int => Self::Primitive(Primitive::Int),
            Long => Self::Primitive(Primitive::Long),
            LongLong => Self::Primitive(Primitive::LongLong),
            UChar => Self::Primitive(Primitive::UChar),
            UShort => Self::Primitive(Primitive::UShort),
            UInt => Self::Primitive(Primitive::UInt),
            ULong => Self::Primitive(Primitive::ULong),
            ULongLong => Self::Primitive(Primitive::ULongLong),
            Float => Self::Primitive(Primitive::Float),
            Double => Self::Primitive(Primitive::Double),
            LongDouble => Self::Primitive(Primitive::LongDouble),
            FloatComplex => Self::Primitive(Primitive::FloatComplex),
            DoubleComplex => Self::Primitive(Primitive::DoubleComplex),
            LongDoubleComplex => Self::Primitive(Primitive::LongDoubleComplex),
            Bool => Self::Primitive(Primitive::Bool),
            Void => Self::Primitive(Primitive::Void),
            String => Self::Primitive(Primitive::String),
            Object => Self::Primitive(Primitive::Object),
            Block => Self::Primitive(Primitive::Block),
            Class => Self::Primitive(Primitive::Class),
            Sel => Self::Primitive(Primitive::Sel),
            Unknown => Self::Primitive(Primitive::Unknown),
            BitField(b, t) => Self::BitField(*b, *t),
            Pointer(t) => Self::Indirection(IndirectionKind::Pointer, t),
            Atomic(t) => Self::Indirection(IndirectionKind::Atomic, t),
            Array(len, item) => Self::Array(*len, item),
            Struct(name, fields) => {
                if !verify_name(name) {
                    panic!("Struct name was not a valid identifier");
                }
                Self::Container(ContainerKind::Struct, name, fields)
            }
            Union(name, members) => {
                if !verify_name(name) {
                    panic!("Union name was not a valid identifier");
                }
                Self::Container(ContainerKind::Union, name, members)
            }
            None => Self::NoneInvalid,
        }
    }
}

impl<'a> Helper<'a, EncodingBox> {
    pub(crate) fn from_box(encoding: &'a EncodingBox) -> Self {
        use EncodingBox::*;
        match encoding {
            Char => Self::Primitive(Primitive::Char),
            Short => Self::Primitive(Primitive::Short),
            Int => Self::Primitive(Primitive::Int),
            Long => Self::Primitive(Primitive::Long),
            LongLong => Self::Primitive(Primitive::LongLong),
            UChar => Self::Primitive(Primitive::UChar),
            UShort => Self::Primitive(Primitive::UShort),
            UInt => Self::Primitive(Primitive::UInt),
            ULong => Self::Primitive(Primitive::ULong),
            ULongLong => Self::Primitive(Primitive::ULongLong),
            Float => Self::Primitive(Primitive::Float),
            Double => Self::Primitive(Primitive::Double),
            LongDouble => Self::Primitive(Primitive::LongDouble),
            FloatComplex => Self::Primitive(Primitive::FloatComplex),
            DoubleComplex => Self::Primitive(Primitive::DoubleComplex),
            LongDoubleComplex => Self::Primitive(Primitive::LongDoubleComplex),
            Bool => Self::Primitive(Primitive::Bool),
            Void => Self::Primitive(Primitive::Void),
            String => Self::Primitive(Primitive::String),
            Object => Self::Primitive(Primitive::Object),
            Block => Self::Primitive(Primitive::Block),
            Class => Self::Primitive(Primitive::Class),
            Sel => Self::Primitive(Primitive::Sel),
            Unknown => Self::Primitive(Primitive::Unknown),
            BitField(b, t) => Self::BitField(*b, t.as_deref()),
            Pointer(t) => Self::Indirection(IndirectionKind::Pointer, t),
            Atomic(t) => Self::Indirection(IndirectionKind::Atomic, t),
            Array(len, item) => Self::Array(*len, item),
            Struct(name, fields) => {
                if !verify_name(name) {
                    panic!("Struct name was not a valid identifier");
                }
                Self::Container(ContainerKind::Struct, name, fields)
            }
            Union(name, members) => {
                if !verify_name(name) {
                    panic!("Union name was not a valid identifier");
                }
                Self::Container(ContainerKind::Union, name, members)
            }
            None => Self::NoneInvalid,
        }
    }
}

impl EncodingType for Encoding {
    fn helper(&self) -> Helper<'_, Self> {
        Helper::new(self)
    }
}

impl EncodingType for EncodingBox {
    fn helper(&self) -> Helper<'_, Self> {
        Helper::from_box(self)
    }
}
