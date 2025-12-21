//! Parse the data in `0.txt` / `1.txt` / `3.txt`.
//!
//! This is a custom binary format (that is mostly ASCII and very simple).
use crate::Kind;

/// A single entry in `*.txt`.
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
pub struct TxtItem<'data> {
    pub uuid: &'data str,
    pub name: &'data str,
    pub kind: Kind,
}

impl<'data> TxtItem<'data> {
    /// Parse the data in one of the `*.txt` files.
    pub fn iter(data: &'data str) -> impl Iterator<Item = Self> {
        // The data is line-separated.
        data.lines().map(|line| {
            // Each part is NUL terminated.
            let (name, rest) = line.split_once('\0').expect("must have name");
            let (kind, rest) = rest.split_once('\0').expect("must have name");
            let (id, rest) = rest.split_once('\0').expect("must have name");
            assert_eq!(rest, "", "must not have any more data");

            let kind: u8 = kind.parse().expect("kind must be integer");
            let kind = match kind {
                1 => Kind::Module,
                2 => Kind::Overview,
                5 => Kind::Namespace,
                7 => Kind::GlobalVariable,
                8 => Kind::Typedef,
                9 => Kind::AssociatedType,
                10 => Kind::Function,
                11 => Kind::SwiftOp,
                12 => Kind::Macro,
                13 => Kind::Union,
                14 => Kind::Enum,
                15 => Kind::EnumCase,
                16 => Kind::Struct,
                17 => Kind::Class,
                18 => Kind::Protocol,
                19 => Kind::InitMethod,
                20 => Kind::Method,
                21 => Kind::Property,
                22 => Kind::Node,
                24 => Kind::TypeMethod,
                25 => Kind::TypeProperty,
                28 => Kind::Article,
                29 => Kind::SampleCode,
                32 => Kind::PropertyListKey,
                30 => Kind::Endpoint,
                31 => Kind::Object,
                34 => Kind::ConceptsTutorial,
                35 => Kind::Root,
                36 => Kind::Tutorial,
                _ => panic!("unknown kind {kind} in {id} ({name})"),
            };

            Self {
                uuid: id,
                name,
                kind,
            }
        })
    }
}
