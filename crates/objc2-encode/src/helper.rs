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

    const fn bitfield(self) -> Self {
        // This is a bit irrelevant, since bitfields can only contain integral
        // types
        self
    }

    const fn atomic(self) -> Self {
        // Move all the way down
        Self::Bottom
    }

    const fn pointer(self) -> Self {
        // Move one step down
        match self {
            Self::Top => Self::Within,
            Self::Bottom | Self::Within => Self::Bottom,
        }
    }

    const fn array(self) -> Self {
        // TODO: Is this correct?
        self
    }

    const fn container(self) -> Self {
        match self {
            // Move top one step down
            Self::Top | Self::Within => Self::Within,
            Self::Bottom => Self::Bottom,
        }
    }

    const fn include_container_fields(self) -> bool {
        match self {
            Self::Top | Self::Within => true,
            Self::Bottom => false,
        }
    }
}

pub(crate) fn compare_encodings<E1: EncodingType, E2: EncodingType>(
    enc1: &E1,
    level1: NestingLevel,
    enc2: &E2,
    level2: NestingLevel,
    include_all: bool,
) -> bool {
    use Helper::*;
    // Note: Ideally `Block` and sequence of `Object, Unknown` in struct
    // should compare equivalent, but we don't bother since in practice a
    // plain `Unknown` will never appear.

    let level1 = if include_all {
        NestingLevel::new()
    } else {
        level1
    };
    let level2 = if include_all {
        NestingLevel::new()
    } else {
        level2
    };

    // TODO: Are level1 and level2 ever be different?

    match (enc1.helper(level1), enc2.helper(level2)) {
        (Primitive(p1), Primitive(p2)) => p1 == p2,
        (
            BitField(size1, Some((offset1, type1)), level1),
            BitField(size2, Some((offset2, type2)), level2),
        ) => {
            size1 == size2
                && offset1 == offset2
                && compare_encodings(type1, level1, type2, level2, include_all)
        }
        (BitField(size1, None, _level1), BitField(size2, None, _level2)) => size1 == size2,
        // The type-encoding of a bitfield is always either available, or it
        // is not (depends on platform); so if it was available in one, but
        // not the other, we should compare the encodings unequal.
        (BitField(_, _, _), BitField(_, _, _)) => false,
        (Indirection(kind1, t1, level1), Indirection(kind2, t2, level2)) => {
            kind1 == kind2 && compare_encodings(t1, level1, t2, level2, include_all)
        }
        (Array(len1, item1, level1), Array(len2, item2, level2)) => {
            len1 == len2 && compare_encodings(item1, level1, item2, level2, include_all)
        }
        (Container(kind1, name1, items1, level1), Container(kind2, name2, items2, level2)) => {
            kind1 == kind2
                && name1 == name2
                && match (items1, items2) {
                    (None, None) => true,
                    (Some(items1), Some(items2)) => {
                        if items1.len() != items2.len() {
                            return false;
                        }
                        for (item1, item2) in items1.iter().zip(items2.iter()) {
                            if !compare_encodings(item1, level1, item2, level2, include_all) {
                                return false;
                            }
                        }
                        true
                    }
                    // A bit unsure about this one, but the "safe" default
                    // here is to say that a container with items does not
                    // compare equal to another container without items.
                    //
                    // Note that this may be confusing, since a `Pointer` to
                    // the two containers might suddenly start comparing
                    // equal, but
                    _ => false,
                }
        }
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
    fn helper(&self, level: NestingLevel) -> Helper<'_, Self>;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) enum Helper<'a, E = Encoding> {
    Primitive(Primitive),
    BitField(u8, Option<&'a (u64, E)>, NestingLevel),
    Indirection(IndirectionKind, &'a E, NestingLevel),
    Array(u64, &'a E, NestingLevel),
    Container(ContainerKind, &'a str, Option<&'a [E]>, NestingLevel),
}

impl<E: EncodingType> fmt::Display for Helper<'_, E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Primitive(primitive) => write!(f, "{}", primitive.to_str()),
            Self::BitField(size, None, _level) => {
                write!(f, "b{size}")
            }
            Self::BitField(size, Some((offset, t)), level) => {
                write!(f, "b{offset}{}{size}", t.helper(*level))
            }
            Self::Indirection(kind, t, level) => {
                write!(f, "{}{}", kind.prefix(), t.helper(*level))
            }
            Self::Array(len, item, level) => {
                write!(f, "[{}{}]", len, item.helper(*level))
            }
            Self::Container(kind, name, items, level) => {
                write!(f, "{}", kind.start())?;
                write!(f, "{name}")?;
                if let Some(items) = items {
                    write!(f, "=")?;
                    for item in *items {
                        write!(f, "{}", item.helper(*level))?;
                    }
                }
                write!(f, "{}", kind.end())
            }
        }
    }
}

impl Helper<'_> {
    pub(crate) const fn new(encoding: &Encoding, level: NestingLevel) -> Self {
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
            BitField(b, t) => Self::BitField(*b, *t, level.bitfield()),
            Pointer(t) => Self::Indirection(IndirectionKind::Pointer, t, level.pointer()),
            Atomic(t) => Self::Indirection(IndirectionKind::Atomic, t, level.atomic()),
            Array(len, item) => Self::Array(*len, item, level.array()),
            Struct(name, fields) => {
                if !verify_name(name) {
                    panic!("Struct name was not a valid identifier");
                }
                let fields = if level.include_container_fields() {
                    Some(*fields)
                } else {
                    None
                };
                Self::Container(ContainerKind::Struct, name, fields, level.container())
            }
            Union(name, members) => {
                if !verify_name(name) {
                    panic!("Union name was not a valid identifier");
                }
                let members = if level.include_container_fields() {
                    Some(*members)
                } else {
                    None
                };
                Self::Container(ContainerKind::Union, name, members, level.container())
            }
        }
    }
}

impl<'a> Helper<'a, EncodingBox> {
    pub(crate) fn from_box(encoding: &'a EncodingBox, level: NestingLevel) -> Self {
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
            BitField(b, t) => Self::BitField(*b, t.as_deref(), level.bitfield()),
            Pointer(t) => Self::Indirection(IndirectionKind::Pointer, t, level.pointer()),
            Atomic(t) => Self::Indirection(IndirectionKind::Atomic, t, level.atomic()),
            Array(len, item) => Self::Array(*len, item, level.array()),
            Struct(name, fields) => {
                if !verify_name(name) {
                    panic!("Struct name was not a valid identifier");
                }
                let fields = if level.include_container_fields() {
                    fields.as_deref()
                } else {
                    None
                };
                Self::Container(ContainerKind::Struct, name, fields, level.container())
            }
            Union(name, members) => {
                if !verify_name(name) {
                    panic!("Union name was not a valid identifier");
                }
                let members = if level.include_container_fields() {
                    members.as_deref()
                } else {
                    None
                };
                Self::Container(ContainerKind::Union, name, members, level.container())
            }
        }
    }
}

impl EncodingType for Encoding {
    fn helper(&self, level: NestingLevel) -> Helper<'_, Self> {
        Helper::new(self, level)
    }
}

impl EncodingType for EncodingBox {
    fn helper(&self, level: NestingLevel) -> Helper<'_, Self> {
        Helper::from_box(self, level)
    }
}
