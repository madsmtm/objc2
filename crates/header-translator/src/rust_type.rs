use std::collections::VecDeque;
use std::fmt::Debug;
use std::{fmt, fmt::Display, iter, mem, str::FromStr, sync::LazyLock};

use clang::{CallingConvention, Entity, EntityKind, Nullability, Type, TypeKind};
use proc_macro2::{TokenStream, TokenTree};
use regex::Regex;

use crate::config::{ItemGeneric, PointerBounds, PointerLifetime, StmtData, TypeOverride};
use crate::context::Context;
use crate::display_helper::FormatterFn;
use crate::id::{ItemIdentifier, ItemTree};
use crate::name_translation::cf_no_ref;
use crate::protocol::ProtocolRef;
use crate::stmt::{anonymous_record_name, bridged_to, parse_class_generics, GenericWithBound};
use crate::stmt::{parse_superclasses, superclasses_required_items};
use crate::thread_safety::ThreadSafety;
use crate::unexposed_attr::UnexposedAttr;

/// Helper for parsing various attributes.
///
/// This is pretty ugly, but required because libClang doesn't expose
/// lifetime information.
#[derive(Debug)]
struct AttributeParser<'a> {
    _original: &'a str,
    to_check: Vec<&'a str>,
}

impl<'a> AttributeParser<'a> {
    fn new(original: &'a str, inner: &str) -> Self {
        assert!(original.len() >= inner.len(), "{original:?} >= {inner:?}");
        let _original = original;

        // Split into parts at non-identifier boundaries.
        static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[A-Za-z0-9_]+|.").unwrap());
        let original = RE
            .find_iter(original)
            .map(|m| m.as_str().trim())
            .filter(|s| !s.is_empty());
        let mut inner = RE
            .find_iter(inner)
            .map(|m| m.as_str().trim())
            .filter(|s| !s.is_empty())
            .peekable();

        // And grab the parts from `original` that are not in `inner`.
        let mut to_check = Vec::new();
        for part in original {
            if let Some(inner_part) = inner.peek() {
                if part == *inner_part {
                    inner.next();
                    continue;
                }
                to_check.push(part);
            } else {
                to_check.push(part);
            }
        }

        Self {
            _original,
            to_check,
        }
    }

    fn parse(&mut self, needle: &str) -> usize {
        let mut found = 0;
        self.to_check.retain(|elem| {
            if *elem == needle {
                found += 1;
                false
            } else {
                true
            }
        });
        found
    }

    fn parse_sequence(&mut self, needle: &[&str]) -> usize {
        let mut found = 0;
        for (i, window) in self.to_check.clone().windows(needle.len()).enumerate() {
            if window == needle {
                found += 1;
                for _ in 0..needle.len() {
                    self.to_check.remove(i);
                }
            }
        }
        found
    }

    fn is_volatile(&mut self) -> bool {
        self.parse("volatile") > 0
    }

    fn lifetime(&mut self) -> Lifetime {
        if self.parse("__unsafe_unretained") > 0 {
            Lifetime::Unretained
        } else if self.parse("__strong") > 0 {
            Lifetime::Strong
        } else if self.parse("__weak") > 0 {
            Lifetime::Weak
        } else if self.parse("__autoreleasing") > 0 {
            Lifetime::Autoreleasing
        } else {
            Lifetime::Unspecified
        }
    }

    fn sending(&mut self) -> bool {
        // __attribute__((swift_attr("sending")))
        self.parse_sequence(&[
            "__attribute__",
            "(",
            "(",
            "swift_attr",
            "(",
            "\"",
            "sending",
            "\"",
            ")",
            ")",
            ")",
        ]) > 0
    }

    fn check(mut self) {
        // Ignore nullabililty, libClang exposes these sufficiently.
        self.parse("_Nullable");
        self.parse("_Nonnull");
        self.parse("_Null_unspecified");
        self.parse("_Nullable_result");

        // Ignore `const`, libClang exposes this sufficiently.
        self.parse("const");

        // We completely ignore `__kindof` in Rust as it is done in Swift, since
        // it only exists to allow legacy Objective-C code to continue compiling.
        //
        // See <https://lapcatsoftware.com/articles/kindof.html>
        self.parse("__kindof");

        if !self.to_check.is_empty() {
            error!(?self, "could not extract all attributes");
        }
    }
}

/// ObjCLifetime
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Lifetime {
    Unspecified,
    /// OCL_ExplicitNone
    Unretained,
    /// OCL_Strong
    Strong,
    /// OCL_Weak
    Weak,
    /// OCL_Autoreleasing
    Autoreleasing,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum MethodArgumentQualifier {
    In,
    Inout,
    Out,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SafetyProperty {
    /// The type is unsafe in the selected position.
    Unsafe { reasons: Vec<String> },
    /// The safety of using the type in this position is unknown.
    Unknown { reasons: Vec<String> },
    /// The type is always safe in this position (and methods/functions using
    /// this is thus eligible for being automatically marked safe).
    Safe,
}

impl SafetyProperty {
    pub fn new_unsafe(reason: impl Into<String>) -> Self {
        Self::Unsafe {
            reasons: vec![reason.into()],
        }
    }

    pub fn new_unknown(reason: impl Into<String>) -> Self {
        Self::Unknown {
            reasons: vec![reason.into()],
        }
    }

    /// Merge two safety properties.
    ///
    /// When there are multiple reasons for something being unsafe, these
    /// are merged as well.
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (
                Self::Unsafe { reasons: reasons1 },
                Self::Unsafe { reasons: reasons2 } | Self::Unknown { reasons: reasons2 },
            ) => Self::Unsafe {
                reasons: reasons1.into_iter().chain(reasons2).collect(),
            },
            (Self::Unknown { reasons: reasons1 }, Self::Unsafe { reasons: reasons2 }) => {
                Self::Unsafe {
                    reasons: reasons1.into_iter().chain(reasons2).collect(),
                }
            }
            (Self::Unsafe { reasons }, Self::Safe) => Self::Unsafe { reasons },
            (Self::Safe, Self::Unsafe { reasons }) => Self::Unsafe { reasons },

            (Self::Unknown { reasons: reasons1 }, Self::Unknown { reasons: reasons2 }) => {
                Self::Unknown {
                    reasons: reasons1.into_iter().chain(reasons2).collect(),
                }
            }
            (Self::Unknown { reasons }, Self::Safe) => Self::Unknown { reasons },
            (Self::Safe, Self::Unknown { reasons }) => Self::Unknown { reasons },

            (Self::Safe, Self::Safe) => Self::Safe,
        }
    }

    pub fn is_unsafe(&self) -> bool {
        matches!(self, SafetyProperty::Unsafe { .. })
    }

    pub fn is_safe(&self) -> bool {
        matches!(self, SafetyProperty::Safe)
    }

    fn ignore(self) -> Self {
        match self {
            Self::Unsafe { .. } => Self::Unsafe { reasons: vec![] },
            Self::Unknown { .. } => Self::Unknown { reasons: vec![] },
            Self::Safe => Self::Safe,
        }
    }

    fn context(self, context: impl Display) -> Self {
        match self {
            Self::Unsafe { mut reasons } => {
                // TODO: Is this how we wanna do context?
                for reason in &mut reasons {
                    *reason = format!("{context} {reason}");
                }
                Self::Unsafe { reasons }
            }
            Self::Unknown { mut reasons } => {
                // TODO: Is this how we wanna do context?
                for reason in &mut reasons {
                    *reason = format!("{context} {reason}");
                }
                Self::Unknown { reasons }
            }
            Self::Safe => Self::Safe,
        }
    }

    fn preface(self, preface: impl Display) -> Self {
        match self {
            Self::Unsafe { mut reasons } => {
                for reason in &mut reasons {
                    *reason = format!("{preface} {reason}");
                }
                Self::Unsafe { reasons }
            }
            Self::Unknown { mut reasons } => {
                for reason in &mut reasons {
                    *reason = format!("{preface} {reason}");
                }
                Self::Unknown { reasons }
            }
            Self::Safe => Self::Safe,
        }
    }

    /// This comment is incomplete, it cannot know about extra requirements
    /// that a method may have.
    ///
    /// A concern could be that adding the "# Safety" comments might lead
    /// users to think that that is all they need to uphold, whereas in
    /// reality there might be further requirements stated elsewhere in the
    /// documentation. This is _probably_ fine, having a safety comment is
    /// better than not having it.
    ///
    /// TODO: Maybe search for "undefined behaviour" in the docstring, and use
    /// that as a marker too?
    pub fn to_safety_comment(&self) -> Option<String> {
        match self {
            Self::Unsafe { reasons } | Self::Unknown { reasons } => Some(if reasons.len() == 1 {
                format!("{}.", reasons[0])
            } else {
                format!("- {}.", reasons.join(".\n- "))
            }),
            Self::Safe => None,
        }
    }
}

/// The safety properties of a type.
///
/// These depend on which position the type is used in.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct TypeSafety {
    /// The type's safety properties when passed into foreign code.
    pub in_argument: SafetyProperty,
    /// The type's safety properties when given by foreign code.
    pub in_return: SafetyProperty,
}

impl TypeSafety {
    const SAFE: Self = Self {
        in_argument: SafetyProperty::Safe,
        in_return: SafetyProperty::Safe,
    };

    fn always_unsafe(reason: impl Into<String> + Clone) -> Self {
        Self {
            in_argument: SafetyProperty::new_unsafe(reason.clone()),
            in_return: SafetyProperty::new_unsafe(reason),
        }
    }

    fn unknown_in_argument(reason: impl Into<String>) -> Self {
        Self {
            in_argument: SafetyProperty::new_unknown(reason),
            in_return: SafetyProperty::Safe,
        }
    }

    fn unsafe_in_argument(reason: impl Into<String>) -> Self {
        Self {
            in_argument: SafetyProperty::new_unsafe(reason),
            in_return: SafetyProperty::Safe,
        }
    }

    fn unsafe_in_return(reason: impl Into<String>) -> Self {
        Self {
            in_argument: SafetyProperty::Safe,
            in_return: SafetyProperty::new_unsafe(reason),
        }
    }

    fn merge(self, other: Self) -> Self {
        Self {
            in_argument: self.in_argument.merge(other.in_argument),
            in_return: self.in_return.merge(other.in_return),
        }
    }

    fn ignore_in_argument(self) -> Self {
        Self {
            in_argument: self.in_argument.ignore(),
            in_return: self.in_return,
        }
    }

    fn context(self, context: impl Display + Clone) -> Self {
        Self {
            in_argument: self.in_argument.context(context.clone()),
            in_return: self.in_return.context(context),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Primitive {
    /// `()` or `c_void`
    Void,
    C99Bool,
    Char,
    SChar,
    UChar,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    LongLong,
    ULongLong,
    Float,
    Double,
    /// Not yet supported by `rustc`
    /// <https://github.com/rust-lang/rust/issues/116909>
    F16,
    F32,
    F64,
    /// Not yet supported by `rustc`
    /// <https://github.com/rust-lang/rust/issues/116909>
    F128,
    // Integer types
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    ISize,
    USize,
    PtrDiff,
    VaList,
    // Objective-C
    ObjcBool,
    NSInteger,
    NSUInteger,
    Imp,
}

impl Primitive {
    fn required_items(&self) -> impl Iterator<Item = ItemTree> {
        match self {
            Self::ObjcBool => Some(ItemTree::objc("Bool")),
            Self::NSInteger => Some(ItemTree::objc("NSInteger")),
            Self::NSUInteger => Some(ItemTree::objc("NSUInteger")),
            Self::Imp => Some(ItemTree::objc("Imp")),
            Self::VaList => Some(ItemTree::core_ffi("VaList")),
            _ => {
                let s = self.as_str();
                if s.starts_with("c_") {
                    Some(ItemTree::core_ffi(s))
                } else if *self == Self::Void {
                    Some(ItemTree::core_ffi("c_void"))
                } else {
                    None
                }
            }
        }
        .into_iter()
    }

    const fn as_str(&self) -> &'static str {
        match self {
            // Primitives
            Self::Void => "()",
            Self::C99Bool => "bool",
            Self::Char => "c_char",
            Self::SChar => "c_schar",
            Self::UChar => "c_uchar",
            Self::Short => "c_short",
            Self::UShort => "c_ushort",
            Self::Int => "c_int",
            Self::UInt => "c_uint",
            Self::Long => "c_long",
            Self::ULong => "c_ulong",
            Self::LongLong => "c_longlong",
            Self::ULongLong => "c_ulonglong",
            Self::Float => "c_float",
            Self::Double => "c_double",
            Self::F16 => "f16",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::F128 => "f128",
            Self::I8 => "i8",
            Self::U8 => "u8",
            Self::I16 => "i16",
            Self::U16 => "u16",
            Self::I32 => "i32",
            Self::U32 => "u32",
            Self::I64 => "i64",
            Self::U64 => "u64",
            // TODO: Use core::ffi::c_ssize_t
            Self::ISize => "isize",
            // TODO: Use core::ffi::c_size_t
            Self::USize => "usize",
            Self::VaList => "VaList",
            // TODO: Use core::ffi::c_ptr_diff_t
            Self::PtrDiff => "isize",
            Self::ObjcBool => "Bool",
            Self::NSInteger => "NSInteger",
            Self::NSUInteger => "NSUInteger",
            // Assume nullable for now
            Self::Imp => "Option<Imp>",
        }
    }

    fn is_signed(&self) -> Option<bool> {
        Some(match self {
            Self::Char => return None, // Target-specific
            Self::SChar | Self::Short | Self::Int | Self::Long | Self::LongLong => true,
            Self::UChar | Self::UShort | Self::UInt | Self::ULong | Self::ULongLong => false,
            Self::Float | Self::Double | Self::F16 | Self::F32 | Self::F64 | Self::F128 => true, // Unsure
            Self::I8 | Self::I16 | Self::I32 | Self::I64 | Self::ISize => true,
            Self::U8 | Self::U16 | Self::U32 | Self::U64 | Self::USize => false,
            Self::PtrDiff => true,
            Self::NSInteger => true,
            Self::NSUInteger => false,
            _ => return None,
        })
    }

    fn safety(&self) -> TypeSafety {
        match self {
            // FIXME(madsmtm): Remove Imp from primitive?
            Self::Imp => TypeSafety::always_unsafe("must be a valid IMP"),
            Self::VaList => TypeSafety::always_unsafe("must be valid"),
            _ => TypeSafety::SAFE,
        }
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

fn get_class_data(
    entity_ref: &Entity<'_>,
    context: &Context<'_>,
) -> (
    ItemIdentifier,
    Vec<GenericWithBound>,
    ThreadSafety,
    Vec<ItemIdentifier>,
) {
    // @class produces a ObjCInterfaceDecl if we didn't load the actual
    // declaration, but we don't actually want that, since it'll point to the
    // wrong place.
    let entity = entity_ref
        .get_location()
        .expect("class location")
        .get_entity()
        .expect("class entity");

    let mut id = ItemIdentifier::new(&entity, context);

    let data = context
        .library(&id)
        .class_data
        .get(&id.name)
        .unwrap_or(StmtData::empty());

    let declaration_generics = parse_class_generics(&entity, context, data);

    match entity.get_kind() {
        EntityKind::ObjCInterfaceDecl => {
            let thread_safety = ThreadSafety::from_decl(&entity, context);
            let superclasses = parse_superclasses(&entity, context)
                .into_iter()
                .map(|(id, _, _)| id)
                .collect();

            (id, declaration_generics, thread_safety, superclasses)
        }
        EntityKind::ObjCClassRef => {
            let thread_safety = ThreadSafety::from_ref(&entity, context);
            (id, declaration_generics, thread_safety, vec![])
        }
        EntityKind::MacroExpansion => {
            id.name = entity_ref.get_name().unwrap_or_else(|| {
                error!(?entity_ref, ?entity, "macro ref did not have name");
                id.name
            });
            // We cannot get thread safety from macro expansions
            let thread_safety = ThreadSafety::dummy();
            // Similarly, we cannot get for required items
            let superclasses = vec![];
            (id, declaration_generics, thread_safety, superclasses)
        }
        _ => {
            error!(?entity, "was not a class");
            (id, declaration_generics, ThreadSafety::dummy(), vec![])
        }
    }
}

fn parse_protocol(entity: Entity<'_>, context: &Context<'_>) -> (ProtocolRef, ThreadSafety) {
    let mut entity = entity.get_definition().unwrap_or(entity);
    // @protocol produces a ObjCProtocolDecl if we didn't
    // load the actual declaration, but we don't actually
    // want that, since it'll point to the wrong place.
    let source_entity = entity
        .get_location()
        .expect("itemref location")
        .get_entity()
        .expect("itemref entity");

    // Workaround for OS_OBJECT_DECL.
    if source_entity.get_kind() != EntityKind::MacroExpansion {
        entity = source_entity;
    }

    let id = ItemIdentifier::new(&entity, context);

    match entity.get_kind() {
        EntityKind::ObjCProtocolDecl => {
            let protocol = ProtocolRef::from_entity(&entity, context);
            let thread_safety = ThreadSafety::from_decl(&entity, context);
            (protocol, thread_safety)
        }
        EntityKind::ObjCProtocolRef => {
            let protocol = ProtocolRef::from_entity(&entity, context);
            let thread_safety = ThreadSafety::from_ref(&entity, context);
            (protocol, thread_safety)
        }
        _ => {
            error!(?entity, "was not a protocol");
            let protocol = ProtocolRef {
                id,
                super_protocols: vec![],
            };
            (protocol, ThreadSafety::dummy())
        }
    }
}

/// Pad generics with `AnyObject` until it is of the given size.
fn pad_generics<'a>(
    generics: &'a [PointeeTy],
    pad_with: &'a PointeeTy,
    len: usize,
) -> impl Iterator<Item = &'a PointeeTy> {
    let missing = len.checked_sub(generics.len()).unwrap_or_else(|| {
        error!(?generics, ?len, "had too many generics");
        0
    });

    generics.iter().chain(iter::repeat_n(pad_with, missing))
}

/// Only allow `*SENDABLE` attributes, not `*NON_SENDABLE`.
fn only_positive_sendable(sendable: Option<bool>) -> bool {
    match sendable {
        Some(true) => true,
        Some(false) => {
            error!("unexpected non-sendable marker");
            false
        }
        None => false,
    }
}

/// Types that are only valid behind pointers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PointeeTy {
    Class {
        id: ItemIdentifier,
        thread_safety: ThreadSafety,
        superclasses: Vec<ItemIdentifier>,
        generics: Vec<PointeeTy>,
        declaration_generics: Vec<GenericWithBound>,
        protocols: Vec<(ProtocolRef, ThreadSafety)>,
        /// Whether there's a *SENDABLE requirement on the type.
        sendable: bool,
    },
    GenericParam {
        /// Whether the generic parameter is guaranteed to be an object type.
        ///
        /// `CFArray` etc. sets this to `false`.
        object_like: bool,
        name: String,
    },
    AnyObject {
        protocols: Vec<(ProtocolRef, ThreadSafety)>,
        /// Whether there's a *SENDABLE requirement on the type.
        sendable: bool,
    },
    AnyProtocol,
    AnyClass {
        protocols: Vec<(ProtocolRef, ThreadSafety)>,
    },
    Self_,
    Fn {
        is_variadic: bool,
        no_escape: bool,
        arguments: Vec<Ty>,
        result_type: Box<Ty>,
    },
    Block {
        sendable: Option<bool>,
        no_escape: bool,
        arguments: Vec<Ty>,
        result_type: Box<Ty>,
    },
    CFTypeDef {
        id: ItemIdentifier,
        generics: Vec<PointeeTy>,
        num_declaration_generics: usize,
        /// The superclass of this CF type. Must point to another `CFTypeDef`.
        to: Option<Box<PointeeTy>>,
    },
    CFOpaque,
    DispatchTypeDef {
        id: ItemIdentifier,
    },
    NetworkTypeDef {
        id: ItemIdentifier,
    },
    TypeDef {
        id: ItemIdentifier,
        to: Box<PointeeTy>,
    },
}

impl PointeeTy {
    /// Recurse into typedefs.
    fn through_typedef(&self) -> &Self {
        match self {
            Self::TypeDef { to, .. } => to.through_typedef(),
            _ => self,
        }
    }

    pub(crate) fn parse_generic_bound(ty: Type<'_>, context: &Context<'_>) -> Self {
        let ty = Ty::parse(ty, false, context);
        match ty {
            Ty::Pointee(pointee) => pointee,
            Ty::Pointer {
                nullability: Nullability::Unspecified,
                read: _,
                written: _,
                lifetime: Lifetime::Unspecified,
                bounds: PointerBounds::Single,
                pointee,
            } => match *pointee {
                Ty::Pointee(pointee) => pointee,
                ty => {
                    error!(?ty, "invalid generic bound pointer");
                    PointeeTy::GenericParam {
                        object_like: true,
                        name: "Unknown".to_string(),
                    }
                }
            },
            ty => {
                error!(?ty, "invalid generic bound");
                PointeeTy::GenericParam {
                    object_like: true,
                    name: "Unknown".to_string(),
                }
            }
        }
    }

    pub(crate) fn is_plain_anyobject(&self) -> bool {
        matches!(self, Self::AnyObject { protocols, sendable: false } if protocols.is_empty())
    }

    pub(crate) fn required_items(&self) -> impl Iterator<Item = ItemTree> {
        match self {
            Self::Class {
                id,
                thread_safety: _,
                superclasses,
                generics,
                declaration_generics,
                protocols,
                sendable: _,
            } => {
                let superclasses = superclasses_required_items(superclasses.iter().cloned());
                let protocols = protocols
                    .iter()
                    .flat_map(|(protocol, _)| protocol.required_items());
                iter::once(ItemTree::new(id.clone(), superclasses))
                    .chain(protocols)
                    .chain(generics.iter().flat_map(|generic| generic.required_items()))
                    .chain(
                        declaration_generics
                            .iter()
                            .flat_map(|(_, bound)| bound.as_ref())
                            .flat_map(|bound| bound.required_items()),
                    )
                    .collect()
            }
            Self::GenericParam { .. } => vec![],
            Self::AnyObject { protocols, .. } => {
                let protocols = protocols
                    .iter()
                    .flat_map(|(protocol, _)| protocol.required_items());
                iter::once(ItemTree::objc("AnyObject"))
                    .chain(protocols)
                    .collect()
            }
            Self::AnyProtocol => vec![ItemTree::objc("AnyProtocol")],
            Self::AnyClass { protocols } => {
                let protocols = protocols
                    .iter()
                    .flat_map(|(protocol, _)| protocol.required_items());
                iter::once(ItemTree::objc("AnyClass"))
                    .chain(protocols)
                    .collect()
            }
            // Methods are always emitted on an `impl`, which means that
            // `Self` is always available there, and don't required additional
            // imports, cfgs or other such things.
            Self::Self_ => vec![],
            Self::Fn {
                is_variadic: _,
                no_escape: _,
                arguments,
                result_type,
            } => result_type
                .required_items()
                .chain(arguments.iter().flat_map(|arg| arg.required_items()))
                .collect(),
            Self::Block {
                sendable: _,
                no_escape: _,
                arguments,
                result_type,
            } => iter::once(ItemTree::block())
                .chain(result_type.required_items())
                .chain(arguments.iter().flat_map(|arg| arg.required_items()))
                .collect(),
            Self::CFTypeDef {
                id, generics, to, ..
            } => iter::once(ItemTree::new(
                id.clone(),
                to.iter().flat_map(|to| to.required_items()),
            ))
            .chain(generics.iter().flat_map(|generic| generic.required_items()))
            .collect(),
            Self::DispatchTypeDef { id } => {
                vec![ItemTree::from_id(id.clone())]
            }
            Self::NetworkTypeDef { id } => {
                vec![ItemTree::from_id(id.clone())]
            }
            Self::CFOpaque => vec![],
            Self::TypeDef { id, to } => {
                vec![ItemTree::new(id.clone(), to.required_items())]
            }
        }
        .into_iter()
    }

    fn requires_mainthreadmarker(&self, self_requires: bool) -> bool {
        match self {
            Self::Class {
                id: _,
                thread_safety,
                superclasses: _,
                generics,
                declaration_generics: _,
                protocols,
                sendable: _,
            } => {
                thread_safety.inferred_mainthreadonly()
                    || generics
                        .iter()
                        .any(|generic| generic.requires_mainthreadmarker(self_requires))
                    || protocols
                        .iter()
                        .any(|(_, thread_safety)| thread_safety.inferred_mainthreadonly())
            }
            Self::GenericParam { .. } => false,
            Self::AnyObject { protocols, .. } => protocols
                .iter()
                .any(|(_, thread_safety)| thread_safety.inferred_mainthreadonly()),
            Self::AnyProtocol => false,
            Self::AnyClass { protocols } => protocols
                .iter()
                .any(|(_, thread_safety)| thread_safety.inferred_mainthreadonly()),
            Self::Self_ => self_requires,
            Self::Fn {
                is_variadic: _,
                no_escape: _,
                arguments,
                result_type,
            }
            | Self::Block {
                sendable: _,
                no_escape: _,
                arguments,
                result_type,
            } => {
                // We're overly cautious here, might be able to relax this if
                // the block is sendable.
                arguments
                    .iter()
                    .any(|arg| arg.requires_mainthreadmarker(self_requires))
                    || result_type.requires_mainthreadmarker(self_requires)
            }
            Self::CFTypeDef { generics, .. } => generics
                .iter()
                .any(|generic| generic.requires_mainthreadmarker(self_requires)),
            Self::CFOpaque => false,
            Self::DispatchTypeDef { .. } => false,
            Self::NetworkTypeDef { .. } => false,
            Self::TypeDef { to, .. } => to.requires_mainthreadmarker(self_requires),
        }
    }

    fn provides_mainthreadmarker(&self, self_provides: bool) -> bool {
        // Important: We mostly visit the top-level types, to not include
        // optional things like `Option<&NSView>` or `&NSArray<NSView>`.
        match self {
            Self::Class { thread_safety, .. } => thread_safety.inferred_mainthreadonly(),
            Self::AnyObject { protocols, .. } => {
                match &**protocols {
                    [] => false,
                    [(_, thread_safety)] => thread_safety.inferred_mainthreadonly(),
                    // TODO: Handle this better
                    _ => false,
                }
            }
            Self::Self_ => self_provides,
            Self::TypeDef { to, .. } => to.provides_mainthreadmarker(self_provides),
            _ => false,
        }
    }

    fn behind_pointer(&self, allow_generic_param: bool) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Class {
                id,
                thread_safety: _,
                superclasses: _,
                generics,
                declaration_generics: _,
                protocols: _,
                sendable: _,
            } => {
                write!(f, "{}", id.path())?;
                if !generics.is_empty() {
                    write!(f, "<")?;
                    for generic in generics {
                        if !allow_generic_param && matches!(generic, Self::GenericParam { .. }) {
                            continue;
                        }
                        write!(f, "{},", generic.behind_pointer(allow_generic_param))?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
            Self::GenericParam { name, .. } => {
                if allow_generic_param {
                    write!(f, "{name}")
                } else {
                    write!(f, "c_void")
                }
            }
            Self::AnyObject {
                protocols,
                // TODO: Emit as `Object<dyn Any + Send + Sync>`?
                sendable: _,
            } => match &**protocols {
                [] => write!(f, "AnyObject"),
                [(protocol, _)] => write!(f, "ProtocolObject<dyn {}>", protocol.id.path()),
                // TODO: Handle this better
                [(first, _), rest @ ..] => {
                    write!(f, "AnyObject /* {}", first.id.path())?;
                    for (protocol, _) in rest {
                        write!(f, "+ {}", protocol.id.path())?;
                    }
                    write!(f, " */")?;
                    Ok(())
                }
            },
            Self::AnyProtocol => write!(f, "AnyProtocol"),
            Self::AnyClass { protocols } => match &**protocols {
                [] => write!(f, "AnyClass"),
                // TODO: Handle this better
                _ => write!(f, "AnyClass"),
            },
            Self::Self_ => write!(f, "Self"),
            // TODO: Handle this better.
            Self::Fn { .. } => {
                write!(f, "core::ffi::c_void /* TODO: Should be a function. */")
            }
            Self::Block {
                sendable: _,
                no_escape,
                arguments,
                result_type,
            } => {
                write!(f, "block2::DynBlock<dyn Fn(")?;
                for arg in arguments {
                    write!(f, "{}, ", arg.plain(allow_generic_param))?;
                }
                write!(f, ")")?;
                write!(
                    f,
                    "{}",
                    result_type.prefix_return(result_type.fn_type_return())
                )?;
                if *no_escape {
                    write!(f, " + '_")?;
                } else {
                    // `dyn Fn()` in function parameters implies `+ 'static`,
                    // so no need to specify that.
                    //
                    // write!(f, " + 'static")?;
                }
                write!(f, ">")
            }
            Self::CFTypeDef { id, generics, .. } => {
                write!(f, "{}", id.path())?;
                if !generics.is_empty() {
                    write!(f, "<")?;
                    for generic in generics {
                        if !allow_generic_param && matches!(generic, Self::GenericParam { .. }) {
                            continue;
                        }
                        write!(f, "{},", generic.behind_pointer(allow_generic_param))?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
            Self::CFOpaque => write!(f, "Opaque"),
            Self::DispatchTypeDef { id } => write!(f, "{}", id.path()),
            Self::NetworkTypeDef { id } => write!(f, "{}", id.path()),
            Self::TypeDef { id, .. } => write!(f, "{}", id.path()),
        })
    }

    pub(crate) fn add_protocol(&mut self, entity: Entity<'_>, context: &Context<'_>) {
        let Self::AnyObject { protocols, .. } = self else {
            error!(?self, "invalid type to add protocols to");
            return;
        };

        protocols.push(parse_protocol(entity, context));
    }

    pub(crate) fn generic_bound(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Class {
                sendable: false, // TODO
                ..
            } => {
                // HACK: Use `AsRef<Bound>`.
                //
                // This is not perfect, since `AsRef` can be implemented for
                // other reasons than subclassing. We really need a general
                // `SubclassOf` trait, but this is at least better than
                // nothing.
                write!(f, "AsRef<{}>", self.behind_pointer(true))
            }
            Self::AnyObject {
                protocols,
                sendable: false, // TODO
            } => match &**protocols {
                // Should be avoided by `is_plain_anyobject` above
                [] => write!(f, "InvalidAnyObjectAsBound"),
                [(first, _), rest @ ..] => {
                    write!(f, "{}", first.id.path())?;
                    for (protocol, _) in rest {
                        write!(f, " + {}", protocol.id.path())?;
                    }
                    Ok(())
                }
            },
            pointee => {
                error!(?pointee, "unhandled generic bound");
                write!(f, "/* {} */", pointee.behind_pointer(true))
            }
        })
    }

    fn safety(&self) -> TypeSafety {
        match self {
            // TODO: Unsure of the safety of `NSCoder`s.
            Self::Class { id, .. } if id.name == "NSCoder" => {
                TypeSafety::unknown_in_argument("possibly has further requirements")
            }
            // TODO: Unsure of the thread-safety of run loops and dispatch queues.
            // https://github.com/madsmtm/objc2/issues/696
            Self::Class { id, .. } if id.name == "NSRunLoop" => {
                TypeSafety::unknown_in_argument("possibly has additional threading requirements")
            }
            Self::CFTypeDef { id, .. } if id.name == "CFRunLoop" => {
                TypeSafety::unknown_in_argument("possibly has additional threading requirements")
            }
            Self::DispatchTypeDef { id, .. } if id.name == "DispatchQueue" => {
                TypeSafety::unknown_in_argument("possibly has additional threading requirements")
            }
            // Unsure if operation queues are thread-safe? Do blocks added to
            // it have to be sendable?
            Self::Class { id, .. } if id.name == "NSOperationQueue" => {
                TypeSafety::unknown_in_argument("possibly has additional threading requirements")
            }
            // `objc2` ensures that objects are initialized before being
            // allowed as references.
            Self::Class {
                id,
                thread_safety: _,
                superclasses: _,
                generics,
                declaration_generics,
                protocols,
                sendable,
            } => {
                // Check that the inner generics are safe to use.
                // TODO: NSMutableArray and variance?
                let mut safety = generics.iter().fold(TypeSafety::SAFE, |safety, generic| {
                    safety.merge(generic.safety().context("generic"))
                });

                if generics.len() != declaration_generics.len() {
                    let mut gave_more_specific_message = false;
                    for (_, bound) in declaration_generics {
                        if let Some(bound) = bound {
                            // Unclear if these bounds are correct.
                            let reason = format!("should be bound by `{}`", bound.generic_bound());
                            safety = safety
                                .merge(TypeSafety::unknown_in_argument(reason))
                                .context("generic");
                            gave_more_specific_message = true;
                        }
                    }

                    // If all generics aren't specified, the remaining are
                    // AnyObject, so apply the restrictions from that as well.
                    if !gave_more_specific_message {
                        safety = safety
                            .merge(TypeSafety::unknown_in_argument(
                                "should be of the correct type",
                            ))
                            .context("generic");
                    }
                }

                // We don't uphold protocol type safety properly yet, since we
                // have no way of specifying ad-hoc protocol requirements.
                if !protocols.is_empty() {
                    safety = safety.merge(TypeSafety::unsafe_in_argument(format!(
                        "must implement {}",
                        separate_with_comma_and(protocols.iter().map(|(p, _)| &p.id.name))
                    )));
                } else if id.name == "NSObject" {
                    // `NSObject` has similar safety to `AnyObject`.
                    safety = TypeSafety::unknown_in_argument("should be of the correct type");
                } else if id.name == "NSEnumerator" {
                    safety = TypeSafety::unsafe_in_return(
                        "enumerator's underlying collection should not be mutated while in use",
                    );
                }

                if id.name.contains("Mutable") {
                    // When returning a mutable collection, treat it as an
                    // argument as well, since
                    //
                    // An example of this is `-[NSThread threadDictionary]`,
                    // which, apart from being super thread-unsafe, also
                    // doesn't check the values that you insert into it.
                    //
                    // (they are invariant).
                    safety = TypeSafety {
                        in_argument: safety.in_argument.clone(),
                        in_return: safety.in_return.merge(safety.in_argument),
                    };
                }

                if *sendable {
                    safety = safety.merge(TypeSafety::unsafe_in_argument("must be thread-safe"));
                }

                safety
            }
            Self::AnyObject {
                protocols,
                sendable,
            } => {
                let mut safety = match &**protocols {
                    // Make `AnyObject` conservatively disallowed as argument,
                    // see https://github.com/madsmtm/objc2/issues/562.
                    //
                    // Returning it is fine though, since if you actually
                    // want to do anything with the type, you have to downcast it,
                    // and `objc2` checks that at runtime.
                    [] => TypeSafety::unknown_in_argument("should be of the correct type"),
                    // Certain protocols are not descriptive enough, and often
                    // essentially amount to `AnyObject`.
                    [(protocol, _)]
                        if matches!(
                            &*protocol.id.name,
                            "NSObjectProtocol"
                                | "NSCoding"
                                | "NSSecureCoding"
                                | "NSCopying"
                                | "NSMutableCopying"
                                | "NSFastEnumeration"
                        ) =>
                    {
                        TypeSafety::unknown_in_argument("should be of the correct type")
                    }
                    // Passing `MTLFunction` is spiritually similar to passing an
                    // `unsafe` function pointer; we can't know without inspecting
                    // the function (or it's documentation) whether it has special
                    // safety requirements. Example:
                    //
                    // ```metal
                    // constant float data[5] = { 1.0, 2.0, 3.0, 4.0, 5.0 };
                    //
                    // // Safety: Must not be called with an index < 5.
                    // kernel void add_static(
                    //     device const float* input,
                    //     device float* result,
                    //     uint index [[thread_position_in_grid]]
                    // ) {
                    //     if (5 <= index) {
                    //         // For illustration purposes.
                    //         __builtin_unreachable();
                    //     }
                    //     result[index] = input[index] + data[index];
                    // }
                    // ```
                    [(protocol, _)]
                        if protocol.is_subprotocol_of("MTLFunction")
                            || protocol.is_subprotocol_of("MTLFunctionHandle") =>
                    {
                        TypeSafety::unknown_in_argument("must be safe to call").merge(
                            TypeSafety::unknown_in_argument(
                                "must have the correct argument and return types",
                            ),
                        )
                    }
                    // Access to the contents of a resource has to be manually
                    // synchronized using things like `didModifyRange:` (CPU side)
                    // or `synchronizeResource:`, `useResource:usage:` and
                    // `MTLFence` (GPU side).
                    [(protocol, _)] if protocol.is_subprotocol_of("MTLResource") => {
                        let safety = TypeSafety::unknown_in_argument("may need to be synchronized");

                        // Additionally, resources in a command buffer must be
                        // kept alive by the application for as long as they're
                        // used. If this is not done, it is possible to encounter
                        // use-after-frees with:
                        // - `MTLCommandBufferDescriptor::setRetainedReferences(false)`.
                        // - `MTLCommandQueue::commandBufferWithUnretainedReferences()`.
                        // - All `MTL4CommandBuffer`s.
                        let safety = safety.merge(TypeSafety::unknown_in_argument(
                            "may be unretained, you must ensure it is kept alive while in use",
                        ));

                        // TODO: Should we also document the requirement for
                        // resources to be properly bound? What exactly are the
                        // requirements though, and when does Metal automatically
                        // bind resources?

                        // `MTLBuffer` is effectively a `Box<[u8]>` stored on the
                        // GPU (and depending on the storage mode, optionally also
                        // on the CPU). Type-safety of the contents is left
                        // completely up to the user.
                        if protocol.id.name == "MTLBuffer" {
                            safety.merge(TypeSafety::unknown_in_argument(
                                "contents should be of the correct type",
                            ))
                        } else {
                            safety
                        }
                    }
                    // Other `ProtocolObject<dyn MyProtocol>`s are treated as
                    // proper types. (An example here is delegate protocols).
                    [_] => TypeSafety::SAFE,
                    // FIXME: Only a single protocol is properly supported,
                    // multiple protocol restrictions are currently `AnyObject`.
                    _ => TypeSafety::unknown_in_argument("should be of the correct type"),
                };

                if *sendable {
                    safety = safety.merge(TypeSafety::unsafe_in_argument("must be thread-safe"));
                }

                safety
            }
            // Object generic parameters are safe.
            // TODO: NSDictionary's KeyType perhaps isn't really?
            Self::GenericParam {
                object_like: true, ..
            } => TypeSafety::SAFE,
            Self::GenericParam {
                object_like: false, ..
            } => PointeeTy::CFOpaque.safety(),
            // Self types have all generics specified on the impl, so they are
            // basically `Class { generics: vec![GenericParam...] }`.
            Self::Self_ => TypeSafety::SAFE,
            Self::CFTypeDef {
                id,
                generics,
                num_declaration_generics,
                to: _,
            } => {
                let mut safety = if id.is_cftype() {
                    // `CFType`, like `AnyObject`, is not known to be safe.
                    TypeSafety::unknown_in_argument("should be of the correct type")
                } else {
                    // `&CFString` and similar are safe, but types like
                    // `&CFArray` are not safe, since their generic can be
                    // anything (including `usize`, i.e. they don't even have
                    // to be objects).
                    let padded =
                        pad_generics(generics, &PointeeTy::CFOpaque, *num_declaration_generics);
                    padded.fold(TypeSafety::SAFE, |safety, generic| {
                        safety.merge(generic.safety().context("generic"))
                    })
                };

                if id.name.contains("Mutable") {
                    // Same as in `Self::Class` above.
                    safety = TypeSafety {
                        in_argument: safety.in_argument.clone(),
                        in_return: safety.in_return.merge(safety.in_argument),
                    };
                }

                safety
            }
            Self::CFOpaque => TypeSafety::unsafe_in_argument("must be of the correct type"),
            // Dispatch objects have strong type-safety, and are thus
            // safe in both positions.
            Self::DispatchTypeDef { .. } => TypeSafety::SAFE,
            // Same for Network types.
            Self::NetworkTypeDef { .. } => TypeSafety::SAFE,
            // Taking `&AnyClass` can be perilous if the method tries to
            // assume it can e.g. create new instances of the class. Some uses
            // are safe though, such as `NSStringFromClass`.
            Self::AnyClass { protocols } => {
                if protocols.is_empty() {
                    TypeSafety::unknown_in_argument("probably has further requirements")
                } else {
                    TypeSafety::unsafe_in_argument(format!(
                        "must implement {}",
                        separate_with_comma_and(protocols.iter().map(|(p, _)| &p.id.name))
                    ))
                }
            }
            // Same with `&AnyProtocol`.
            Self::AnyProtocol => {
                TypeSafety::unknown_in_argument("possibly has further requirements")
            }
            // Sendable blocks are not yet propagate in the API, and so are
            // not safe: https://github.com/madsmtm/objc2/issues/572
            Self::Block {
                sendable: Some(true),
                ..
            } => TypeSafety::always_unsafe("block must be sendable"),
            Self::Block {
                sendable: _,
                arguments,
                result_type,
                no_escape: _, // Doesn't have an effect on this
            } => {
                let argument_safety =
                    arguments
                        .iter()
                        .enumerate()
                        .fold(TypeSafety::SAFE, |safety, (i, arg)| {
                            // We don't currently handle lifetimes in blocks, so
                            // let's be conservative here for now.
                            safety.merge(arg.safety().context(if i == 0 && arguments.len() == 1 {
                                "block's argument".to_string()
                            } else {
                                format!("block's argument {}", i + 1)
                            }))
                        });

                // Currently conservative, since blocks doesn't handle memory
                // management, and thus currently return raw pointers.
                let result_ty_safety = result_type.safety().context("block's return");

                TypeSafety {
                    // Blocks in arguments have a sort of flipped view; they
                    // require that the result type is safe as an argument,
                    // since the user provides this from the block. And they
                    // require that the arguments are as safe as if they were
                    // returned, since they are passed to into the block.
                    in_argument: result_ty_safety
                        .in_argument
                        .merge(argument_safety.in_return),
                    // Returning blocks is the opposite; the returned block's
                    // arguments must be safe to call from user code, and the
                    // result type must be safe to use afterwards.
                    in_return: argument_safety
                        .in_argument
                        .merge(result_ty_safety.in_return),
                }
            }
            Self::Fn { .. } => {
                // Function pointers are emitted as `unsafe fn`, so they are
                // never safe in argument position.
                //
                // That fact cuts both ways though: when being returned, they
                // are safe, since the user must uphold their safety
                // guarantees if they want to call the function.
                TypeSafety::unsafe_in_argument("must be implemented correctly")
            }
            Self::TypeDef { to, .. } => to.safety(),
        }
    }

    pub(crate) fn implementable(&self) -> Option<ItemTree> {
        match self {
            Self::CFTypeDef { id, to, .. } => Some(ItemTree::new(
                id.clone(),
                to.iter().flat_map(|to| to.implementable()),
            )),
            Self::Class { id, .. } | Self::DispatchTypeDef { id } | Self::NetworkTypeDef { id } => {
                Some(ItemTree::from_id(id.clone()))
            }
            // We shouldn't encounter this here, since `Self` is only on
            // Objective-C methods, but if we do, it's very unclear how we
            // should translate it.
            Self::Self_ => Some(ItemTree::from_id(ItemIdentifier::builtin("__Self__"))),
            Self::TypeDef { id, to } => to
                .implementable()
                .filter(|implementor| implementor.id() == id),
            _ => None,
        }
    }

    fn is_static_object(&self) -> bool {
        matches!(self.through_typedef(), Self::AnyClass { .. })
    }

    fn is_objc_type(&self) -> bool {
        matches!(
            self.through_typedef(),
            Self::Class { .. }
                | Self::GenericParam {
                    object_like: true,
                    ..
                }
                | Self::AnyObject { .. }
                | Self::AnyProtocol
                | Self::AnyClass { .. }
                | Self::Self_
        )
    }

    fn is_cf_type(&self) -> bool {
        matches!(self.through_typedef(), Self::CFTypeDef { .. })
    }

    fn is_dispatch_type(&self) -> bool {
        matches!(self.through_typedef(), Self::DispatchTypeDef { .. })
    }

    fn is_network_type(&self) -> bool {
        matches!(self.through_typedef(), Self::NetworkTypeDef { .. })
    }

    fn is_block_type(&self) -> bool {
        matches!(self.through_typedef(), Self::Block { .. })
    }

    fn is_subtype_of(&self, other: &Self) -> bool {
        /// Ensure that the other protocols are upheld by our protocols.
        fn protocols_is_subtype_of(
            this: &[(ProtocolRef, ThreadSafety)],
            other: &[(ProtocolRef, ThreadSafety)],
        ) -> bool {
            other.iter().all(|(other_protocol, _)| {
                this.iter()
                    .any(|(p, _)| p.is_subprotocol_of(&other_protocol.id.name))
            })
        }

        // Again, typedefness doesn't matter for subtyping.
        match (self.through_typedef(), other.through_typedef()) {
            // Classes are subtypes if they are subclasses.
            (
                Self::Class {
                    id,
                    thread_safety: _,
                    superclasses,
                    generics,
                    declaration_generics,
                    protocols,
                    sendable,
                },
                Self::Class {
                    id: other_id,
                    thread_safety: _,
                    superclasses: _,
                    generics: other_generics,
                    declaration_generics: other_declaration_generics,
                    protocols: other_protocols,
                    sendable: other_sendable,
                },
            ) => {
                // Inexhaustive list of types with __covariant generics.
                let generics_are_subtypes = if matches!(
                    &*id.name,
                    "NSArray" | "NSDictionary" | "NSSet" | "NSOrderedSet" | "NSFetchRequest"
                ) {
                    let len = generics
                        .len()
                        .max(other_generics.len())
                        .max(declaration_generics.len())
                        .max(other_declaration_generics.len());
                    const ANYOBJECT: PointeeTy = PointeeTy::AnyObject {
                        protocols: vec![],
                        sendable: false,
                    };
                    pad_generics(generics, &ANYOBJECT, len)
                        .zip(pad_generics(other_generics, &ANYOBJECT, len))
                        .all(|(this, other)| this.is_subtype_of(other))
                } else {
                    generics == other_generics
                };

                generics_are_subtypes
                    && protocols_is_subtype_of(protocols, other_protocols)
                    && (id == other_id || superclasses.contains(other_id))
                    && *sendable == *other_sendable
            }

            // All classes are subtypes of a plain `AnyObject`.
            //
            // TODO: Handle classes with `protocols` set.
            (
                Self::Class {
                    sendable: false, ..
                },
                Self::AnyObject {
                    protocols,
                    sendable: false,
                },
            ) if protocols.is_empty() => true,

            // Protocol objects are subtypes of protocol objects with fewer
            // requirements.
            (
                Self::AnyObject {
                    protocols,
                    sendable,
                },
                Self::AnyObject {
                    protocols: other_protocols,
                    sendable: other_sendable,
                },
            ) => {
                protocols_is_subtype_of(protocols, other_protocols) && *sendable == *other_sendable
            }

            // TODO: Track the type of `Self` instead!
            (Self::Self_, _) | (_, Self::Self_) => true,

            // Everything else is invariant for now.
            (this, other) => this == other,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Primitive(Primitive),
    Pointee(PointeeTy),
    /// Only handle vectors specially, matrixes are "just" structs on top.
    Simd {
        ty: Primitive,
        size: u8,
    },
    Sel {
        nullability: Nullability,
    },
    Pointer {
        nullability: Nullability,
        /// Whether this pointer may be read through.
        read: bool,
        /// Whether this pointer may be written to.
        written: bool,
        lifetime: Lifetime,
        bounds: PointerBounds,
        pointee: Box<Self>,
    },
    TypeDef {
        id: ItemIdentifier,
        to: Box<Self>,
    },
    Array {
        element_type: Box<Self>,
        num_elements: usize,
    },
    Enum {
        id: ItemIdentifier,
        ty: Box<Self>,
        // No need to store variants here, they don't matter for what the type
        // itself can do.
    },
    Struct {
        id: ItemIdentifier,
        /// FIXME: This does not work for recursive structs.
        fields: Vec<(String, Ty)>,
        /// Whether the struct's declaration has a bridge attribute.
        is_bridged: bool,
    },
    Union {
        id: ItemIdentifier,
        /// FIXME: This does not work for recursive structs.
        fields: Vec<(String, Ty)>,
    },
    /// `Result<T, E>`
    Result {
        /// If this is `c_void`, the result type is `Result<(), E>`
        ty: Box<Self>,
        err: Box<Self>,
        /// The inner / original return type. Useful for e.g. knowing if we're
        /// converting `Boolean` or `Bool`.
        original_ty: Box<Self>,
    },
}

impl Ty {
    fn parse(
        attributed_ty: Type<'_>,
        array_decays_to_pointer: bool,
        context: &Context<'_>,
    ) -> Self {
        let mut ty = attributed_ty;
        let _span = debug_span!("Ty::parse", ?ty).entered();
        let mut spans = VecDeque::new();

        let mut attributed_name = attributed_ty.get_display_name();
        let mut name = ty.get_display_name();
        let mut no_escape = false;
        let mut sendable = None;

        let mut is_const = false;
        let mut attributed_or_unexposed_nullability = None;

        // Unexposed and attributed types may come in any order, and we want
        // to strip both.
        while let TypeKind::Unexposed | TypeKind::Attributed = ty.get_kind() {
            // Grab const-ness and nullability from attributed/unexposed type.
            is_const |= ty.is_const_qualified();
            if let Some(new) = ty.get_nullability() {
                // Always update, it's the innermost attribute that counts.
                // (At least it seems to be, I only know of one place where
                // this happens: `VNFaceLandmarkRegion2D::normalizedPoints`).
                attributed_or_unexposed_nullability = Some(new);
            }

            if let TypeKind::Attributed = ty.get_kind() {
                ty = ty.get_modified_type().expect("attributed modified");
                spans.push_back(debug_span!("attributed", ?ty).entered());
                name = ty.get_display_name();
                continue;
            }

            let (new_attributed_name, attributed_attr) = parse_unexposed_tokens(&attributed_name);
            // Also parse the expected name to ensure that the formatting that
            // TokenStream does is the same on both.
            let (new_name, attr) = parse_unexposed_tokens(&name);
            if attributed_attr != attr {
                error!(
                    ?attributed_attr,
                    ?attr,
                    "attributed attr was not equal to attr",
                );
            }

            match attr {
                Some(UnexposedAttr::Sendable) => {
                    sendable = Some(true);
                }
                Some(UnexposedAttr::NonSendable) => {
                    sendable = Some(false);
                }
                Some(UnexposedAttr::NonIsolated | UnexposedAttr::UIActor) => {
                    // Ignored for now.
                }
                Some(UnexposedAttr::NoEscape) => {
                    no_escape = true;
                }
                Some(UnexposedAttr::FullyUnavailable) => {
                    // Irrelevant on types.
                }
                Some(UnexposedAttr::ReturnsRetained | UnexposedAttr::ReturnsNotRetained) => {
                    // A `CF_RETURNS_RETAINED` or similar that is known by
                    // Clang to be part of the type is also converted properly
                    // into `__strong`. So we don't need to do anything here.
                }
                Some(attr) => error!(?attr, "unknown attribute on type"),
                None => {}
            }

            attributed_name = new_attributed_name;
            name = new_name;

            ty = ty.get_modified_type().expect("unexposed modified");
            spans.push_back(debug_span!("unexposed", ?ty).entered());
        }

        let is_const = is_const || ty.is_const_qualified();

        let nullability = attributed_or_unexposed_nullability
            .or_else(|| ty.get_nullability())
            .unwrap_or(Nullability::Unspecified);

        // We generally don't care about whether a type is elaborated or not.
        // (Elaborated means the `struct` or `enum` qualifier in a type such
        // as `struct Foo*`).
        if let Some(true) = ty.is_elaborated() {
            ty = ty.get_elaborated_type().expect("elaborated");
            spans.push_back(debug_span!("elaborated", ?ty).entered());
        }

        if sendable.is_some()
            && !matches!(
                ty.get_kind(),
                TypeKind::BlockPointer
                    | TypeKind::ObjCObjectPointer
                    | TypeKind::ObjCId
                    | TypeKind::ObjCObject
                    | TypeKind::ObjCInterface
                    | TypeKind::Typedef
            )
        {
            error!(?ty, sendable, "unused sendable marker on type");
        }

        match ty.get_kind() {
            TypeKind::Void => Self::Primitive(Primitive::Void),
            TypeKind::Bool => Self::Primitive(Primitive::C99Bool),
            TypeKind::CharS | TypeKind::CharU => Self::Primitive(Primitive::Char),
            TypeKind::SChar => Self::Primitive(Primitive::SChar),
            TypeKind::UChar => Self::Primitive(Primitive::UChar),
            TypeKind::Short => Self::Primitive(Primitive::Short),
            TypeKind::UShort => Self::Primitive(Primitive::UShort),
            TypeKind::Int => Self::Primitive(Primitive::Int),
            TypeKind::UInt => Self::Primitive(Primitive::UInt),
            TypeKind::Long => Self::Primitive(Primitive::Long),
            TypeKind::ULong => Self::Primitive(Primitive::ULong),
            TypeKind::LongLong => Self::Primitive(Primitive::LongLong),
            TypeKind::ULongLong => Self::Primitive(Primitive::ULongLong),
            TypeKind::Float => Self::Primitive(Primitive::Float),
            TypeKind::Double => Self::Primitive(Primitive::Double),
            TypeKind::LongDouble => {
                // https://github.com/rust-lang/rust/issues/116909
                error!("long double is not yet supported in Rust");
                Self::Pointee(PointeeTy::GenericParam {
                    object_like: false,
                    name: "UnknownLongDouble".to_string(),
                })
            }
            TypeKind::Record => {
                let declaration = ty.get_declaration().expect("record declaration");
                let id = ItemIdentifier::new_optional(&declaration, context);

                // Override for self-referential / recursive types
                let fields = if matches!(
                    id.name.as_deref(),
                    Some(
                        "QElem"
                            | "TMTask"
                            | "SleepQRec"
                            | "MIDISysexSendRequest"
                            | "MIDISysexSendRequestUMP"
                            | "MIDIDriverInterface"
                            | "cssm_list_element"
                            | "malloc_zone_t"
                            | "_malloc_zone_t"
                            | "GDevice"
                            | "TERec"
                            | "NMRec"
                    )
                ) {
                    // Fake fields, we'll have to define it ourselves
                    vec![("foo".to_string(), Self::Pointee(PointeeTy::Self_))]
                } else {
                    ty.get_fields()
                        .expect("struct fields")
                        .into_iter()
                        .map(|field| {
                            let field_name = field.get_name().unwrap();
                            (
                                field_name,
                                Self::parse(
                                    field.get_type().expect("struct field type"),
                                    false,
                                    context,
                                ),
                            )
                        })
                        .collect()
                };

                match declaration.get_kind() {
                    EntityKind::StructDecl => Self::Struct {
                        id: id.map_name(|name| {
                            name.or_else(|| anonymous_record_name(&declaration, context))
                                .unwrap_or_else(|| "UnknownStruct".into())
                        }),
                        fields,
                        is_bridged: bridged_to(&declaration, context).is_some(),
                    },
                    EntityKind::UnionDecl => Self::Union {
                        id: id.map_name(|name| {
                            name.or_else(|| anonymous_record_name(&declaration, context))
                                .unwrap_or_else(|| "UnknownUnion".into())
                        }),
                        fields,
                    },
                    _ => {
                        error!(?declaration, "unknown record type decl");
                        Self::Pointee(PointeeTy::GenericParam {
                            object_like: false,
                            name: "UnknownRecord".into(),
                        })
                    }
                }
            }
            TypeKind::Enum => {
                let declaration = ty.get_declaration().expect("enum declaration");
                Self::Enum {
                    id: ItemIdentifier::new(&declaration, context),
                    ty: Box::new(Ty::parse(
                        declaration
                            .get_enum_underlying_type()
                            .expect("enum underlying type"),
                        false,
                        context,
                    )),
                }
            }
            TypeKind::ObjCId => {
                // The `ObjCId` itself may also contain attributes.
                let mut parser = AttributeParser::new(&attributed_name, "id");
                let lifetime = parser.lifetime();
                parser.check();

                Self::Pointer {
                    nullability,
                    read: true,
                    written: !is_const,
                    lifetime,
                    bounds: PointerBounds::Single,
                    pointee: Box::new(Self::Pointee(PointeeTy::AnyObject {
                        protocols: vec![],
                        sendable: only_positive_sendable(sendable),
                    })),
                }
            }
            TypeKind::ObjCClass => {
                // The `ObjCClass` itself may also contain attributes.
                let mut parser = AttributeParser::new(&attributed_name, "Class");
                let lifetime = parser.lifetime();
                parser.check();

                Self::Pointer {
                    nullability,
                    read: true,
                    written: false,
                    lifetime,
                    bounds: PointerBounds::Single,
                    pointee: Box::new(Self::Pointee(PointeeTy::AnyClass { protocols: vec![] })),
                }
            }
            TypeKind::ObjCSel => Self::Sel { nullability },
            TypeKind::ObjCInterface => {
                let declaration = ty.get_declaration().expect("ObjCInterface declaration");

                if !ty.get_objc_type_arguments().is_empty() {
                    panic!("generics not empty: {ty:?}");
                }
                if !ty.get_objc_protocol_declarations().is_empty() {
                    panic!("protocols not empty: {ty:?}");
                }

                if name == "Protocol" {
                    Self::Pointee(PointeeTy::AnyProtocol)
                } else {
                    let (id, declaration_generics, thread_safety, superclasses) =
                        get_class_data(&declaration, context);
                    if id.name != name.strip_prefix("const ").unwrap_or(&name) {
                        warn!(?name, "invalid interface name");
                    }
                    Self::Pointee(PointeeTy::Class {
                        id,
                        thread_safety,
                        superclasses,
                        protocols: vec![],
                        generics: vec![],
                        declaration_generics,
                        sendable: only_positive_sendable(sendable),
                    })
                }
            }
            TypeKind::ObjCObject => {
                let base_ty = ty
                    .get_objc_object_base_type()
                    .expect("object to have base type");
                let name = base_ty.get_display_name();

                let generics: Vec<_> = ty
                    .get_objc_type_arguments()
                    .into_iter()
                    .map(|param| Self::parse(param, false, context))
                    .map(|generic| match generic {
                        Self::Pointer { pointee, .. } => {
                            pointee.into_pointee().unwrap_or_else(|| {
                                error!(?name, "unknown generic in class");
                                PointeeTy::Self_
                            })
                        }
                        generic => {
                            error!(?name, ?generic, "unknown generic in class");
                            PointeeTy::Self_
                        }
                    })
                    .collect();

                let protocols: Vec<_> = ty
                    .get_objc_protocol_declarations()
                    .into_iter()
                    .map(|entity| parse_protocol(entity, context))
                    .collect();

                Self::Pointee(match base_ty.get_kind() {
                    TypeKind::ObjCId => {
                        assert_eq!(name, "id");

                        if !generics.is_empty() {
                            panic!("generics not empty: {ty:?}, {generics:?}");
                        }

                        PointeeTy::AnyObject {
                            protocols,
                            sendable: only_positive_sendable(sendable),
                        }
                    }
                    TypeKind::ObjCInterface => {
                        let declaration = base_ty
                            .get_declaration()
                            .expect("ObjCObject -> ObjCInterface declaration");
                        let (id, declaration_generics, thread_safety, superclasses) =
                            get_class_data(&declaration, context);
                        if id.name != name {
                            error!(?name, "ObjCObject -> ObjCInterface invalid name");
                        }

                        if !generics.is_empty() && !protocols.is_empty() {
                            panic!("got object with both protocols and generics: {name:?}, {protocols:?}, {generics:?}");
                        }
                        if generics.is_empty() && protocols.is_empty() {
                            panic!("got object with empty protocols and generics: {name:?}");
                        }
                        if declaration_generics.len() < generics.len() {
                            panic!("got class with more generics than declaration: {name:?}");
                        }

                        PointeeTy::Class {
                            id,
                            thread_safety,
                            superclasses,
                            generics,
                            declaration_generics,
                            protocols,
                            sendable: only_positive_sendable(sendable),
                        }
                    }
                    TypeKind::ObjCClass => {
                        assert!(generics.is_empty(), "ObjCClass with generics");

                        PointeeTy::AnyClass { protocols }
                    }
                    kind => panic!("unknown ObjCObject kind {ty:?}, {kind:?}"),
                })
            }
            TypeKind::Pointer => {
                let pointee = ty.get_pointee_type().expect("pointer to have pointee");

                let pointee_name = pointee.get_display_name();
                let mut parser = AttributeParser::new(&attributed_name, &pointee_name);
                // TODO: Use this
                // (Swift's @Sendable = Send + Sync, Swift's sending = move).
                let _sending = parser.sending();
                parser.parse("*");
                if matches!(
                    pointee.get_kind(),
                    TypeKind::FunctionPrototype | TypeKind::FunctionNoPrototype
                ) {
                    parser.parse("(");
                    parser.parse(")");
                }
                parser.check();

                let is_const = is_const || pointee.is_const_qualified();

                // Only top-level arrays decay to pointers.
                let pointee = Self::parse(pointee, false, context);
                let bounds = if matches!(pointee, Ty::Pointee(_)) {
                    // Object-like pointers default to being a pointer to a
                    // single object.
                    PointerBounds::Single
                } else {
                    PointerBounds::Unspecified
                };
                Self::Pointer {
                    nullability,
                    // Assuming the pointer may be read from is the safe default.
                    read: true,
                    // Assuming the pointer may be written to is a safe default,
                    // but we can probably relax this if the pointer is `const`.
                    written: !is_const, // Heuristic
                    lifetime: Lifetime::Unspecified,
                    bounds,
                    pointee: Box::new(pointee),
                }
            }
            TypeKind::BlockPointer => {
                let pointee = ty.get_pointee_type().expect("pointer type to have pointee");

                // `BlockPointer` itself can contain lifetime, so let's parse
                // the pointee.
                let pointee_name = pointee.get_display_name();
                let mut parser = AttributeParser::new(&attributed_name, &pointee_name);
                let lifetime = parser.lifetime();
                parser.parse("^");
                parser.parse("(");
                parser.parse(")");
                parser.check();

                match Self::parse(pointee, false, context) {
                    Self::Pointee(PointeeTy::Fn {
                        is_variadic: false,
                        no_escape: fn_no_escape,
                        arguments,
                        result_type,
                    }) => Self::Pointer {
                        nullability,
                        read: true,
                        // TODO: What does `const` mean on block pointers?
                        written: !is_const,
                        lifetime,
                        bounds: PointerBounds::Single,
                        pointee: Box::new(Self::Pointee(PointeeTy::Block {
                            sendable,
                            no_escape: fn_no_escape || no_escape,
                            arguments,
                            result_type,
                        })),
                    },
                    pointee => panic!("unexpected pointee in block: {pointee:?}"),
                }
            }
            TypeKind::ObjCObjectPointer => {
                let pointee = ty.get_pointee_type().expect("pointer type to have pointee");

                // The pointer part of ObjCObjectPointer contains lifetime
                // information too, so let's also extract that.
                let pointee_name = pointee.get_display_name();
                let mut parser = AttributeParser::new(&attributed_name, &pointee_name);
                let lifetime = parser.lifetime();
                parser.parse("*");
                parser.parse("UI_APPEARANCE_SELECTOR");
                parser.parse("ITLIB_AVAILABLE");
                parser.parse("COREMOTION_EXPORT");
                parser.check();

                let mut pointee = Self::parse(pointee, false, context);

                // Apply sendability attribute to inner.
                if only_positive_sendable(sendable) {
                    if let Self::Pointee(PointeeTy::Class { sendable, .. }) = &mut pointee {
                        *sendable = true;
                    } else {
                        error!(?ty, "unused sendable marker on object pointer");
                    }
                }

                Self::Pointer {
                    nullability,
                    read: true,
                    written: !is_const,
                    lifetime,
                    bounds: PointerBounds::Single,
                    pointee: Box::new(pointee),
                }
            }
            TypeKind::Typedef => {
                let typedef_name = ty.get_typedef_name().expect("typedef has name");
                let declaration = ty.get_declaration().expect("typedef declaration");
                assert_eq!(
                    typedef_name,
                    declaration.get_name().expect("typedef declaration name")
                );
                let inner = declaration
                    .get_typedef_underlying_type()
                    .expect("typedef underlying type");
                let _span = debug_span!("typedef", ?typedef_name, ?declaration, ?inner).entered();

                let mut parser = AttributeParser::new(&attributed_name, &typedef_name);
                let _is_volatile = parser.is_volatile();
                let lifetime = parser.lifetime();
                parser.check();

                match &*typedef_name {
                    "BOOL" => return Self::Primitive(Primitive::ObjcBool),
                    "IMP" => return Self::Primitive(Primitive::Imp),

                    "int8_t" => return Self::Primitive(Primitive::I8),
                    "__int8_t" => return Self::Primitive(Primitive::I8),
                    "uint8_t" => return Self::Primitive(Primitive::U8),
                    "__uint8_t" => return Self::Primitive(Primitive::U8),
                    "int16_t" => return Self::Primitive(Primitive::I16),
                    "__int16_t" => return Self::Primitive(Primitive::I16),
                    "uint16_t" => return Self::Primitive(Primitive::U16),
                    "__uint16_t" => return Self::Primitive(Primitive::U16),
                    "int32_t" => return Self::Primitive(Primitive::I32),
                    "__int32_t" => return Self::Primitive(Primitive::I32),
                    "uint32_t" => return Self::Primitive(Primitive::U32),
                    "__uint32_t" => return Self::Primitive(Primitive::U32),
                    "int64_t" => return Self::Primitive(Primitive::I64),
                    "__int64_t" => return Self::Primitive(Primitive::I64),
                    "uint64_t" => return Self::Primitive(Primitive::U64),
                    "__uint64_t" => return Self::Primitive(Primitive::U64),
                    "ssize_t" => return Self::Primitive(Primitive::ISize),
                    "size_t" => return Self::Primitive(Primitive::USize),
                    "ptrdiff_t" => return Self::Primitive(Primitive::PtrDiff),
                    // https://github.com/rust-lang/rust/issues/65473
                    "intptr_t" => return Self::Primitive(Primitive::ISize),
                    "uintptr_t" => return Self::Primitive(Primitive::USize),

                    // include/sys/_types/_XXX.h
                    "u_char" => return Self::Primitive(Primitive::UChar),
                    "u_short" => return Self::Primitive(Primitive::UShort),
                    "u_int" => return Self::Primitive(Primitive::UInt),
                    "u_int8_t" => return Self::Primitive(Primitive::U8),
                    "u_int16_t" => return Self::Primitive(Primitive::U16),
                    "u_int32_t" => return Self::Primitive(Primitive::U32),
                    "u_int64_t" => return Self::Primitive(Primitive::U64),

                    // math.h
                    "float_t" => return Self::Primitive(Primitive::Float),
                    "double_t" => return Self::Primitive(Primitive::Double),

                    // Varargs, still unsupported by Rust.
                    "__builtin_va_list" => return Self::Primitive(Primitive::VaList),

                    // MacTypes.h
                    "UInt8" => return Self::Primitive(Primitive::U8),
                    "UInt16" => return Self::Primitive(Primitive::U16),
                    "UInt32" => return Self::Primitive(Primitive::U32),
                    "UInt64" => return Self::Primitive(Primitive::U64),
                    "SInt8" => return Self::Primitive(Primitive::I8),
                    "SInt16" => return Self::Primitive(Primitive::I16),
                    "SInt32" => return Self::Primitive(Primitive::I32),
                    "SInt64" => return Self::Primitive(Primitive::I64),
                    "Float32" => return Self::Primitive(Primitive::F32),
                    "Float64" => return Self::Primitive(Primitive::F64),

                    simd if simd.starts_with("simd_")
                        | simd.starts_with("vector_")
                        | simd.starts_with("matrix_") =>
                    {
                        let rest = simd
                            .strip_prefix("simd_")
                            .or_else(|| simd.strip_prefix("vector_"))
                            .or_else(|| simd.strip_prefix("matrix_"))
                            .unwrap();

                        if let Some(idx) = rest.find(|c: char| c.is_numeric()) {
                            let (ty_name, rest) = rest.split_at(idx);
                            // SIMD types are explicitly documented with their
                            // width in bytes (so it's safe for us to).
                            let ty = match ty_name {
                                "char" => Primitive::I8,
                                "uchar" => Primitive::U8,
                                "short" => Primitive::I16,
                                "ushort" => Primitive::U16,
                                "half" => Primitive::F16,
                                "int" => Primitive::I32,
                                "uint" => Primitive::U32,
                                "float" => Primitive::F32,
                                // long on 64-bit, long long otherwise
                                "long" => Primitive::ISize,
                                "ulong" => Primitive::USize,
                                "double" => Primitive::F64,
                                _ => {
                                    error!(typedef_name, ty_name, "unknown simd type");
                                    Primitive::Void
                                }
                            };
                            if !rest.contains('x') {
                                match rest.parse::<u8>() {
                                    Ok(size) => {
                                        return Self::Simd { ty, size };
                                    }
                                    Err(err) => {
                                        error!(typedef_name, ?err, "could not parse simd size");
                                    }
                                }
                            } else {
                                // Ignore if contains `x`, this is a simd
                                // matrix (which is just a struct).
                            }
                        } else if matches!(rest, "quath" | "quatf" | "quatd") {
                            // Ignore, a typedef to a normal struct.
                        } else {
                            error!(typedef_name, "could not parse simd type");
                        }
                    }

                    // Workaround for this otherwise requiring libc.
                    "dispatch_qos_class_t" => {
                        return Self::TypeDef {
                            id: ItemIdentifier::new(&declaration, context),
                            to: Box::new(Self::Primitive(Primitive::Int)),
                        }
                    }

                    // HACK: Make IOBluetooth work without depending on IOKit.
                    "IOReturn" => {
                        return Self::TypeDef {
                            id: ItemIdentifier::builtin("IOReturn"),
                            to: Box::new(Self::Primitive(Primitive::Int)),
                        }
                    }
                    "IOItemCount" => {
                        return Self::TypeDef {
                            id: ItemIdentifier::builtin("IOItemCount"),
                            to: Box::new(Self::Primitive(Primitive::U32)),
                        }
                    }

                    // HACK: Prevent OSLog from requiring dependency on os
                    "os_activity_id_t" | "os_signpost_id_t" => {
                        return Self::TypeDef {
                            id: ItemIdentifier::builtin(typedef_name),
                            to: Box::new(Self::Primitive(Primitive::I64)),
                        }
                    }

                    "NSInteger" => return Self::Primitive(Primitive::NSInteger),
                    "NSUInteger" => return Self::Primitive(Primitive::NSUInteger),

                    "instancetype" => {
                        return Self::Pointer {
                            nullability,
                            read: true,
                            written: !is_const,
                            lifetime,
                            bounds: PointerBounds::Single,
                            pointee: Box::new(Self::Pointee(PointeeTy::Self_)),
                        }
                    }

                    // Emit `dispatch_object_t` as a raw pointer (at least for now,
                    // since we currently treat `DispatchObject` as a trait).
                    "dispatch_object_t" => {
                        return Self::Pointer {
                            nullability,
                            read: true,
                            written: !is_const,
                            lifetime,
                            bounds: PointerBounds::Unsafe,
                            pointee: Box::new(Self::TypeDef {
                                id: ItemIdentifier::builtin("dispatch_object_s"),
                                to: Box::new(Self::Primitive(Primitive::PtrDiff)),
                            }),
                        };
                    }

                    // Handle other Dispatch Objective-C objects.
                    name if name.starts_with("dispatch_")
                        && inner.get_kind() == TypeKind::ObjCObjectPointer =>
                    {
                        let id = ItemIdentifier::new(&declaration, context);
                        let id = context.replace_typedef_name(id, false);
                        let pointee = Box::new(Self::Pointee(PointeeTy::DispatchTypeDef { id }));
                        return Self::Pointer {
                            nullability,
                            read: true,
                            written: !is_const,
                            lifetime,
                            bounds: PointerBounds::Single,
                            pointee,
                        };
                    }

                    // Handle other Network Objective-C objects.
                    name if name.starts_with("nw_")
                        && inner.get_kind() == TypeKind::ObjCObjectPointer =>
                    {
                        let id = ItemIdentifier::new(&declaration, context);
                        let id = context.replace_typedef_name(id, false);
                        let pointee = Box::new(Self::Pointee(PointeeTy::NetworkTypeDef { id }));
                        return Self::Pointer {
                            nullability,
                            read: true,
                            written: !is_const,
                            lifetime,
                            bounds: PointerBounds::Single,
                            pointee,
                        };
                    }

                    _ => {}
                }

                if let EntityKind::TemplateTypeParameter = declaration.get_kind() {
                    return Self::Pointer {
                        nullability,
                        read: true,
                        written: !is_const,
                        lifetime,
                        bounds: PointerBounds::Single,
                        pointee: Box::new(Self::Pointee(PointeeTy::GenericParam {
                            object_like: true,
                            name: typedef_name,
                        })),
                    };
                }

                let mut inner = Self::parse(inner, false, context);

                let id = ItemIdentifier::new(&declaration, context);
                let data = context.library(&id).get(&declaration);

                if let Some(sendable) = sendable {
                    if data.sendable != Some(sendable) {
                        error!(?ty, "mismatch between sendable attributes on typedef");
                    }
                }

                // "Push" the typedef into an inner object pointer.
                //
                // When we encounter a typedef declaration like this:
                //     typedef NSString* NSAbc;
                //
                // We emit it as:
                //     type NSAbc = NSString;
                //     struct NSAbc(NSString);
                //
                // Instead of:
                //     type NSAbc = *const NSString;
                //
                // Because that means we can use ordinary Retained<NSAbc>
                // elsewhere.
                //
                // Doing this here is important to enable things like:
                //
                //     typedef __autoreleasing MTLArgument *__nullable MTLAutoreleasedArgument;
                //     - (id <MTLArgumentEncoder>)newArgumentEncoderWithBufferIndex:(NSUInteger)bufferIndex
                //                                                       reflection:(MTLAutoreleasedArgument * __nullable)reflection;
                //
                // Which needs us to "see" the `__autoreleasing` on
                // `MTLAutoreleasedArgument` all the way to the `reflection`
                // parameter.
                if let Self::Pointer {
                    nullability: inner_nullability,
                    read: _,
                    written: inner_writes,
                    lifetime: inner_lifetime,
                    bounds: inner_bounds,
                    pointee,
                } = &mut inner
                {
                    // The outermost attribute, i.e. the one on the typedef,
                    // is the one that matters.
                    //
                    // We intentionally mutate the Pointer's data, such that
                    // this works regardless of what we do further down below.
                    if nullability != Nullability::Unspecified {
                        *inner_nullability = nullability;
                    }
                    if is_const {
                        *inner_writes = !is_const;
                    }
                    if lifetime != Lifetime::Unspecified {
                        *inner_lifetime = lifetime;
                    }
                    *inner_bounds = PointerBounds::Single;

                    if id.is_cfallocator() {
                        // CFAllocatorRef is safely nullable, the default
                        // allocator is a typedef to NULL.
                        if *inner_nullability == Nullability::Unspecified {
                            *inner_nullability = Nullability::Nullable;
                        }
                    }

                    // Propagate sendability from typedef.
                    if let Some(sendable_override) = data.sendable {
                        if let Self::Pointee(PointeeTy::Block { sendable, .. }) = &mut **pointee {
                            *sendable = Some(sendable_override);
                        } else {
                            error!(?id, "tried to set sendable on non-block typedef");
                        }
                    }

                    if pointee
                        .is_direct_cf_type(&id.name, bridged_to(&declaration, context).is_some())
                    {
                        let declaration_generics = data.generics.clone().unwrap_or_default();
                        let to = if let Self::Pointee(pointee @ PointeeTy::CFTypeDef { .. }) =
                            &**pointee
                        {
                            Some(Box::new(pointee.clone()))
                        } else {
                            None
                        };

                        // A bit annoying that we replace the typedef name
                        // here, as that's also what determines whether the
                        // type is a CF type or not... But that's how it is
                        // currently.
                        let id = context.replace_typedef_name(id, true);
                        **pointee = Self::Pointee(PointeeTy::CFTypeDef {
                            id,
                            generics: vec![],
                            num_declaration_generics: declaration_generics.len(),
                            to,
                        });
                        return inner;
                    } else if pointee.is_object_like() {
                        if let Self::Pointee(pointee_ty) = &mut **pointee {
                            let id = context.replace_typedef_name(id, pointee_ty.is_cf_type());
                            // Replace with a dummy type (will be re-replaced
                            // on the line below).
                            let to = Box::new(mem::replace(pointee_ty, PointeeTy::Self_));
                            **pointee = Self::Pointee(PointeeTy::TypeDef { id, to });
                            return inner;
                        } else {
                            error!(?pointee, "is_object_like but not Pointee");
                        }
                    }
                } else {
                    // Ignore properties that are set here, we can't use the
                    // information for anything.
                    //
                    // Example:
                    // const CFTimeInterval AVCoreAnimationBeginTimeAtZero;
                    // typedef NSArray<NSNumber*> MPSShape;
                }

                if array_decays_to_pointer && matches!(inner.through_wrapper(), Self::Array { .. })
                {
                    Self::Pointer {
                        nullability,
                        read: true,
                        written: !is_const,
                        lifetime,
                        bounds: PointerBounds::Single,
                        pointee: Box::new(Self::TypeDef {
                            id,
                            to: Box::new(inner),
                        }),
                    }
                } else {
                    Self::TypeDef {
                        id,
                        to: Box::new(inner),
                    }
                }
            }
            // Assume that functions without a prototype simply have 0 arguments.
            TypeKind::FunctionPrototype | TypeKind::FunctionNoPrototype => {
                let call_conv = ty.get_calling_convention().expect("fn calling convention");
                assert_eq!(
                    call_conv,
                    CallingConvention::Cdecl,
                    "fn calling convention is C"
                );

                let arguments = ty
                    .get_argument_types()
                    .expect("fn type to have argument types")
                    .into_iter()
                    .map(|ty| Self::parse(ty, true, context))
                    .collect();

                let result_type = ty.get_result_type().expect("fn type to have result type");
                let result_type = Self::parse(result_type, false, context);

                Self::Pointee(PointeeTy::Fn {
                    is_variadic: ty.get_kind() == TypeKind::FunctionPrototype && ty.is_variadic(),
                    no_escape,
                    arguments,
                    result_type: Box::new(result_type),
                })
            }
            TypeKind::IncompleteArray => {
                let mut parser = AttributeParser::new(&attributed_name, &name);
                let lifetime = parser.lifetime();
                parser.check();

                let ty = ty
                    .get_element_type()
                    .expect("incomplete array to have element type");

                let pointee = Self::parse(ty, false, context);
                Self::Pointer {
                    nullability,
                    read: true,
                    written: !is_const,
                    lifetime,
                    bounds: PointerBounds::CountedBy("Unknown".into()),
                    pointee: Box::new(pointee),
                }
            }
            TypeKind::ConstantArray => {
                // Only top-level arrays decay to pointers. E.g.
                // int[4][2] -> int(*)[2]
                let element = ty.get_element_type().expect("array to have element type");
                let element_type = Box::new(Self::parse(element, false, context));

                let num_elements = ty
                    .get_size()
                    .expect("constant array to have element length");

                if array_decays_to_pointer {
                    Self::Pointer {
                        nullability,
                        read: true,
                        written: !is_const,
                        lifetime: Lifetime::Unspecified,
                        // The ABI of arrays is such that `&[T; N]` -> `*const T`.
                        //
                        // So in that sense, since the array already contains
                        // bounds information in its type, this is a "single"
                        // object.
                        //
                        // See also:
                        // <https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=5a75ef9f07259901c4a220bb09001f44>
                        //
                        // Note that C does not support returning arrays
                        // directly, they must be either wrapped in a struct,
                        // or given as a parameter. So we don't have to handle
                        // that.
                        bounds: PointerBounds::Single,
                        pointee: Box::new(Self::Array {
                            element_type,
                            num_elements,
                        }),
                    }
                } else {
                    Self::Array {
                        element_type,
                        num_elements,
                    }
                }
            }
            _ => {
                error!(?ty, "unknown type kind");
                Self::Pointee(PointeeTy::GenericParam {
                    object_like: false,
                    name: "Unknown".to_string(),
                })
            }
        }
    }

    /// Recurse into typedefs and `Result<(), E>` (at the topmost layer).
    fn through_wrapper(&self) -> &Self {
        match self {
            Self::TypeDef { to, .. } => to.through_wrapper(),
            Self::Result { ty, err, .. } if **ty == Self::VOID => err.through_wrapper(),
            _ => self,
        }
    }

    pub(crate) fn required_items(&self) -> impl Iterator<Item = ItemTree> {
        let items: Vec<ItemTree> = match self {
            Self::Primitive(prim) => prim.required_items().collect(),
            Self::Pointee(pointee_ty) => pointee_ty.required_items().collect(),
            Self::Simd { ty, .. } => ty
                .required_items()
                .chain(iter::once(ItemTree::core_simd_simd()))
                .collect(),
            Self::Sel { .. } => vec![ItemTree::objc("Sel")],
            Self::Pointer {
                nullability,
                read,
                written: _,  // `&mut` doesn't require any imports.
                lifetime: _, // `'static` doesn't require any imports.
                bounds,
                pointee,
            } => pointee
                .required_items()
                .chain((*nullability == Nullability::NonNull).then(ItemTree::core_ptr_nonnull))
                .chain((!read).then(ItemTree::core_mem_maybeuninit))
                .chain(
                    (*bounds == PointerBounds::NullTerminated && pointee.is_pointee_cstr())
                        .then(|| ItemTree::core_ffi("CStr")),
                )
                .collect(),
            Self::TypeDef { id, to, .. } => vec![ItemTree::new(id.clone(), to.required_items())],
            Self::Array { element_type, .. } => element_type.required_items().collect(),
            Self::Enum { id, ty } => {
                vec![ItemTree::new(id.clone(), ty.required_items())]
            }
            Self::Struct { id, fields, .. } | Self::Union { id, fields, .. } => {
                let fields = fields.iter().flat_map(|(_, field)| field.required_items());
                vec![ItemTree::new(id.clone(), fields)]
            }
            Self::Result {
                ty,
                original_ty,
                err,
                ..
            } => ty
                .required_items()
                .chain(original_ty.required_items())
                .chain(err.required_items())
                .collect(),
        };
        items.into_iter()
    }

    /// Whether this type requires MainThreadMarker to construct.
    pub(crate) fn requires_mainthreadmarker(&self, self_requires: bool) -> bool {
        match self {
            Self::Primitive(_) => false,
            Self::Pointee(pointee_ty) => pointee_ty.requires_mainthreadmarker(self_requires),
            Self::Simd { .. } => false,
            Self::Sel { .. } => false,
            Self::Pointer { pointee, .. } => pointee.requires_mainthreadmarker(self_requires),
            Self::TypeDef { to, .. } => to.requires_mainthreadmarker(self_requires),
            Self::Array { element_type, .. } => {
                element_type.requires_mainthreadmarker(self_requires)
            }
            Self::Enum { ty, .. } => ty.requires_mainthreadmarker(self_requires),
            Self::Struct { fields, .. } | Self::Union { fields, .. } => fields
                .iter()
                .any(|(_, field)| field.requires_mainthreadmarker(self_requires)),
            Self::Result { ty, err, .. } => {
                ty.requires_mainthreadmarker(self_requires)
                    || err.requires_mainthreadmarker(self_requires)
            }
        }
    }

    /// Whether this type can provide a MainThreadMarker.
    pub(crate) fn provides_mainthreadmarker(&self, self_provides: bool) -> bool {
        // Important: We mostly visit the top-level types, to not include
        // optional things like `Option<&NSView>` or `&NSArray<NSView>`.
        match self {
            Self::Pointee(pointee_ty) => pointee_ty.provides_mainthreadmarker(self_provides),
            Self::Pointer {
                // Only visit non-null single pointers.
                nullability: Nullability::NonNull,
                bounds: PointerBounds::Single,
                pointee,
                ..
            } => pointee.provides_mainthreadmarker(self_provides),
            Self::TypeDef { to, .. } => to.provides_mainthreadmarker(self_provides),
            Self::Enum { ty, .. } => ty.provides_mainthreadmarker(self_provides),
            Self::Struct { fields, .. } => fields
                .iter()
                .any(|(_, field)| field.provides_mainthreadmarker(self_provides)),
            Self::Union { fields, .. } => fields
                .iter()
                .all(|(_, field)| field.provides_mainthreadmarker(self_provides)),
            Self::Result { ty, err, .. } => {
                ty.provides_mainthreadmarker(self_provides)
                    && err.provides_mainthreadmarker(self_provides)
            }
            _ => false,
        }
    }

    /// The (conservative) safety properties of the type.
    fn safety(&self) -> TypeSafety {
        match self {
            Self::Primitive(prim) => prim.safety(),
            Self::Pointee(pointee) => pointee.safety(),
            // Most SIMD functions are unsafe, but the type itself must be
            // initialized and is by perfectly safe.
            Self::Simd { .. } => TypeSafety::SAFE,
            // Rarely safe, selectors can point to anything, and it's hard to
            // specify threading requirements.
            Self::Sel { .. } => TypeSafety::unsafe_in_argument("must be a valid selector"),
            // Function pointers have validity requirements, and are therefore
            // always safe. Note though that we still defer to the actual
            // function pointee to figure out if it's safe in that particular
            // situation.
            Self::Pointer {
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if matches!(**pointee, Ty::Pointee(PointeeTy::Fn { .. })) => pointee.safety(),
            // By default, all other pointers aren't safe in arguments, though
            // they are generally safe to return (at least if the pointee is).
            Self::Pointer {
                nullability,
                pointee,
                ..
            } => {
                let reason = if matches!(
                    *nullability,
                    Nullability::Nullable | Nullability::NullableResult
                ) {
                    "must be a valid pointer or null"
                } else {
                    "must be a valid pointer"
                };
                TypeSafety::unsafe_in_argument(reason).merge(pointee.safety().ignore_in_argument())
            }
            // Safe as long as the inner element type is.
            Self::Array { element_type, .. } => element_type.safety().context("array element"),
            // Enums are safe in both positions.
            //
            // Note that enums don't strictly prevent passing invalid
            // enumeration values, but this is fine, C code (and by extension
            // Objective-C code) is written with that in mind.
            Self::Enum { .. } => TypeSafety::SAFE,
            // Structs inherit the safety of all their fields.
            Self::Struct { fields, .. } => {
                fields
                    .iter()
                    .fold(TypeSafety::SAFE, |safety, (field_name, field)| {
                        let mut field_safety = field.safety();
                        if field_name == "version" {
                            // Setting the right version in e.g.
                            // `CFRunLoopObserverContext` is important.
                            field_safety = field_safety
                                .merge(TypeSafety::unsafe_in_argument("must be set correctly"));
                        }
                        safety.merge(field_safety.context(format!("struct field `{field_name}`")))
                    })
            }
            // Conservative.
            Self::Union { .. } => TypeSafety::always_unsafe("must be correctly initialized"),
            Self::TypeDef { id, .. } if id.name == "CFStringEncoding" => {
                TypeSafety::unknown_in_argument("should be set correctly")
            }
            Self::TypeDef { to, .. } => to.safety(),
            Self::Result { ty, err, .. } => ty.safety().merge(err.safety()),
        }
    }

    fn safety_in_fn(&self) -> TypeSafety {
        match self {
            // At the top-level of functions/methods, pointers to object-like
            // things and block/fn pointers are generally emitted as `&$Ty`
            // and returned as `Retained<$Ty>` (possibly in an `Option`).
            //
            // This is safe if the pointee is.
            Self::Pointer {
                bounds: PointerBounds::Single,
                nullability,
                pointee,
                ..
            } => {
                let mut safety = pointee.safety();
                if *nullability == Nullability::Unspecified {
                    // Mark as unknown in argument, to avoid situations where
                    // an API does not actually support taking a nullable
                    // value (i.e. cases where we might be emitting the wrong
                    // binding).
                    //
                    // See also https://github.com/madsmtm/objc2/issues/695.
                    safety =
                        safety.merge(TypeSafety::unknown_in_argument("might not allow `None`"));
                }
                safety
            }
            // `&CStr`.
            Self::Pointer {
                bounds: PointerBounds::NullTerminated,
                nullability,
                pointee,
                ..
            } if pointee.is_pointee_cstr() => {
                let mut safety = pointee.safety();
                if *nullability == Nullability::Unspecified {
                    safety =
                        safety.merge(TypeSafety::unknown_in_argument("might not allow `None`"));
                }
                // TODO: Allow setting correct lifetime bounds (currently we
                // assume that the pointer won't escape in argument position).
                safety = safety.merge(TypeSafety::unsafe_in_return("must bound the lifetime"));
                safety
            }
            _ => self.safety(),
        }
    }

    pub(crate) fn safety_in_method_argument(&self, arg_name: &str) -> SafetyProperty {
        // Out pointers
        if let Some((nullability, pointee, inner_nullability, Lifetime::Autoreleasing)) =
            self.out_pointer_data()
        {
            let safety = pointee.safety();

            // Out pointers are both inputs and outputs, and thus they
            // have requirements in both directions.
            //
            // TODO: Actual usage is often for outputs only, so we might
            // be able to relax this? Maybe just allow
            // `SafetyProperty::Unknown` in arguments?
            let mut safety = safety.in_argument.merge(safety.in_return);

            // TODO: Remove the `inner_nullability` check, it's probably not
            // necessary? Basically all out pointers allow a value of `None`.
            if nullability == Nullability::Unspecified
                || inner_nullability == Nullability::Unspecified
            {
                safety = safety.merge(SafetyProperty::new_unknown("might not allow `None`"));
            }

            safety.preface(format!("`{arg_name}`"))
        } else {
            self.safety_in_fn()
                .in_argument
                .preface(format!("`{arg_name}`"))
        }
    }

    pub(crate) fn safety_in_fn_argument(&self, arg_name: &str) -> SafetyProperty {
        // Out pointers, see `fn_argument_converter`.
        if let Some((
            nullability,
            pointee,
            Nullability::Unspecified | Nullability::Nullable | Nullability::NullableResult,
            Lifetime::Autoreleasing | Lifetime::Strong,
        )) = self.out_pointer_data()
        {
            // Contrary to methods, functions only allow `None` as input, so
            // there we don't need to ensure that the input is correct.
            let mut safety = pointee.safety().in_return;

            if nullability == Nullability::Unspecified {
                safety = safety.merge(SafetyProperty::new_unknown("might not allow `None`"));
            }

            safety.preface(format!("`{arg_name}`"))
        } else {
            self.safety_in_fn()
                .in_argument
                .preface(format!("`{arg_name}`"))
        }
    }

    pub(crate) fn safety_in_fn_return(&self) -> SafetyProperty {
        self.safety_in_fn().in_return.preface("The returned")
    }

    pub(crate) fn is_primitive_or_record(&self) -> bool {
        matches!(
            self.through_wrapper(),
            Self::Primitive(_)
                | Self::Simd { .. }
                | Self::Enum { .. }
                | Self::Struct { .. }
                | Self::Union { .. }
        )
    }

    /// Return the `ItemTree` for the nearest implement-able type, if any.
    pub(crate) fn implementable(&self) -> Option<ItemTree> {
        match self {
            Self::Primitive(_) | Self::Simd { .. } | Self::Sel { .. } => None,
            Self::Pointee(pointee) => pointee.implementable(),
            Self::Pointer { pointee, .. }
            | Self::Array {
                element_type: pointee,
                ..
            } => pointee.implementable(),
            // TypeDefs aren't implement-able, even if their underlying type
            // is, since the type might come from another crate.
            //
            // So only if the typedef is "fake" / points to a type with the
            // exact same name do we want to mark it implementable.
            Self::TypeDef { id, to } => to
                .implementable()
                .filter(|implementor| implementor.id() == id),
            Self::Enum { id, ty } => Some(ItemTree::new(id.clone(), ty.required_items())),
            Self::Struct { id, fields, .. } | Self::Union { id, fields } => {
                let fields = fields.iter().flat_map(|(_, field)| field.required_items());
                Some(ItemTree::new(id.clone(), fields))
            }
            Self::Result { ty, .. } => ty.implementable(),
        }
    }

    /// AnyClass is safe to return as `&'static T`, since the runtime will it
    /// alive forever (and it has infinite retain count).
    ///
    /// AnyProtocol is not, though, since there's a single global object that
    /// the runtime is keeping track of, so forgetting to `release` those
    /// would leak resources.
    fn is_static_object(&self) -> bool {
        if let Self::Pointee(pointee_ty) = self.through_wrapper() {
            pointee_ty.is_static_object()
        } else {
            false
        }
    }

    fn is_object_like(&self) -> bool {
        self.is_objc_type()
            || self.is_cf_type()
            || self.is_dispatch_type()
            || self.is_network_type()
            || self.is_block_type()
    }

    /// Determine whether the inner type of a `Pointer` is object-like.
    fn is_objc_type(&self) -> bool {
        if let Self::Pointee(pointee_ty) = self.through_wrapper() {
            pointee_ty.is_objc_type()
        } else {
            false
        }
    }

    /// Determine whether the pointee inside a `Pointer` is a CF-like type.
    fn is_cf_type(&self) -> bool {
        if let Self::Pointee(pointee_ty) = self.through_wrapper() {
            pointee_ty.is_cf_type()
        } else {
            false
        }
    }

    /// Determine whether the pointee inside a `Pointer` is a Dispatch-like type.
    fn is_dispatch_type(&self) -> bool {
        if let Self::Pointee(pointee_ty) = self.through_wrapper() {
            pointee_ty.is_dispatch_type()
        } else {
            false
        }
    }

    /// Determine whether the pointee inside a `Pointer` is a Network-like type.
    fn is_network_type(&self) -> bool {
        if let Self::Pointee(pointee_ty) = self.through_wrapper() {
            pointee_ty.is_network_type()
        } else {
            false
        }
    }

    /// Determine whether the inner type of a `Pointer` is a block.
    fn is_block_type(&self) -> bool {
        if let Self::Pointee(pointee_ty) = self.through_wrapper() {
            pointee_ty.is_block_type()
        } else {
            false
        }
    }

    /// Determine whether the pointee inside a `Pointer` is the inner
    /// struct/void type required for a type to be considered a CF type.
    ///
    /// Similar to what's done in Swift's implementation:
    /// <https://github.com/swiftlang/swift/blob/swift-6.0.3-RELEASE/lib/ClangImporter/CFTypeInfo.cpp#L53>
    fn is_direct_cf_type(&self, typedef_name: &str, typedef_is_bridged: bool) -> bool {
        // Pre-defined list of known CF types.
        // Taken from the Swift project (i.e. this is also what they do).
        static KNOWN_CF_TYPES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
            let database = include_str!("CFDatabase.def");
            let mut res = vec![];
            for item in database.split("\nCF_TYPE(").skip(1) {
                let (typename, _) = item.split_once(")").unwrap();
                res.push(typename);
            }
            res
        });

        // TODO: Figure out when to do the isCFObjectRef check that Clang does:
        // <https://github.com/llvm/llvm-project/blob/llvmorg-19.1.6/clang/lib/Analysis/CocoaConventions.cpp#L57>
        match self {
            // Typedefs to structs are CF types if bridged, or in
            // pre-defined list.
            Self::Struct { is_bridged, .. } => {
                *is_bridged || KNOWN_CF_TYPES.contains(&typedef_name)
            }
            // Typedefs to void* are CF types if the typedef is
            // bridged, or in pre-defined list.
            Self::Primitive(Primitive::Void) => {
                typedef_is_bridged || KNOWN_CF_TYPES.contains(&typedef_name)
            }
            // Typedefs to other CF types are themselves CF types.
            // Example: `CFPropertyList` is a typedef to CFType, but we want
            // it to be it's own type. Similar for `SecTransformRef`.
            Self::Pointee(PointeeTy::CFTypeDef { .. }) => {
                // We don't want these two types to be newtypes quite yet though.
                if matches!(
                    typedef_name,
                    "CMClockOrTimebaseRef" | "SecTransformStringOrAttributeRef"
                ) {
                    return false;
                }
                // Also unsure if we should do this (at least yet), see:
                // https://github.com/madsmtm/objc2/issues/735
                if typedef_name == "CFPropertyListRef" {
                    return false;
                }
                // For these, we kinda wanna make them superclasses of the
                // thing they represent? Unsure yet.
                if matches!(typedef_name, "CMBufferRef" | "VTSessionRef") {
                    return false;
                }
                typedef_is_bridged || KNOWN_CF_TYPES.contains(&typedef_name)
            }
            _ => false,
        }
    }

    pub(crate) fn is_cf_type_typedef(&self, typedef_name: &str, typedef_is_bridged: bool) -> bool {
        if let Self::Pointer { pointee, .. } = self {
            pointee.is_direct_cf_type(typedef_name, typedef_is_bridged)
        } else {
            false
        }
    }

    pub(crate) fn is_cf_type_ptr(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self {
            pointee.is_cf_type()
        } else {
            false
        }
    }

    pub(crate) fn is_cf_allocator(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self {
            if let Ty::Pointee(PointeeTy::CFTypeDef { id, .. }) = &**pointee {
                if id.is_cfallocator() {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn is_object_like_ptr(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self {
            pointee.is_object_like()
        } else {
            false
        }
    }

    pub(crate) fn is_cf_type_id(&self) -> bool {
        matches!(self, Self::TypeDef { id, .. } if id.name == "CFTypeID")
    }

    pub(crate) fn is_class_with_mutable_in_name(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self.through_wrapper() {
            if let Self::Pointee(pointee) = pointee.through_wrapper() {
                matches!(pointee.through_typedef(), PointeeTy::Class { id, .. } | PointeeTy::CFTypeDef { id, .. } if id.name.contains("Mutable"))
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Whether the type, if behind a pointer, is allowed to be converted
    /// to/from `CStr`.
    fn is_pointee_cstr(&self) -> bool {
        // Only check `char`; `unsigned char` or `signed char` are not
        // converted to `CStr` automatically.
        match self.through_wrapper() {
            Self::Primitive(Primitive::Char) => true,
            // `[c_char; N]` arrays work similar to `*c_char` in that we want
            // to map them to `&CStr` when possible.
            Self::Array { element_type, .. } => {
                matches!(**element_type, Self::Primitive(Primitive::Char))
            }
            _ => false,
        }
    }

    pub(crate) fn contains_union(&self) -> bool {
        match self {
            Self::Union { .. } => true,
            Self::TypeDef { id, .. }
                if matches!(
                    &*id.name,
                    "MPSPackedFloat3" | "MTLPackedFloat3" | "WideChar"
                ) =>
            {
                // These are custom-defined to not contain the internal union.
                false
            }
            Self::TypeDef { to, .. } => to.contains_union(),
            Self::Struct { fields, .. } => fields.iter().any(|(_, field)| field.contains_union()),
            Self::Array { element_type, .. } => element_type.contains_union(),
            _ => false,
        }
    }

    pub(crate) fn directly_contains_fn_ptr(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self.through_wrapper() {
            matches!(&**pointee, Ty::Pointee(PointeeTy::Fn { .. }))
        } else {
            false
        }
    }

    pub(crate) fn is_objc_bool(&self) -> bool {
        matches!(self.through_wrapper(), Self::Primitive(Primitive::ObjcBool))
    }

    fn has_zero_niche(&self) -> bool {
        matches!(
            self,
            Self::Pointer {
                nullability: Nullability::NonNull,
                ..
            } | Self::Sel {
                nullability: Nullability::NonNull,
            }
        )
    }

    pub(crate) fn has_zero_default(&self) -> bool {
        match self {
            Self::Primitive(Primitive::Void | Primitive::VaList) => false,
            Self::Primitive(_) => true,
            Self::Simd { .. } => true,
            Self::Sel {
                nullability: Nullability::Nullable | Nullability::NullableResult,
            } => true,
            Self::Pointer {
                // Conservative, we could add `Nullability::Unspecified` here
                // too, but not emitting a `Default` impl for those makes it
                // less of a breaking change if we change the fields to be
                // `NonNull` in the future.
                nullability: Nullability::Nullable | Nullability::NullableResult,
                pointee,
                ..
                // TODO: Return `true` here always once MSRV is 1.88, that's
                // when pointer types started implementing `Default` (which
                // allows us to derive the `Default`).
                //
                // Alternatively, we could implement `Default` manually on
                // these, but that's a bit of a hassle, so we won't bother for
                // now.
            } => matches!(&**pointee, Ty::Pointee(PointeeTy::Fn { .. })),
            // Only arrays up to size 32 implement Default.
            Self::Array {
                element_type,
                num_elements,
            } if *num_elements <= 32 => element_type.has_zero_default(),
            // Almost all enums have a default, and the type-system will catch
            // errors here, so let's keep the check simple.
            Self::Enum { id, ty } if id.name != "MIDINotificationMessageID" => {
                ty.has_zero_default()
            }
            // NOTE: `MTLResourceID` is not `Default` for now, since we still
            // need to figure out if creating it from invalid IDs is safe.
            Self::Struct { id, .. } if id.name == "MTLResourceID" => false,
            // HACK: MTLPackedFloat3 is redefined as a simple struct, which
            // implements `Default`.
            Self::Struct { id, .. } if matches!(&*id.name, "_MTLPackedFloat3" | "_MPSPackedFloat3") => true,
            Self::Struct {
                fields,
                is_bridged: false,
                ..
            } => fields.iter().all(|(_, field)| field.has_zero_default()),
            Self::TypeDef { to, .. } => to.has_zero_default(),
            Self::Result { ty, err, .. } if **ty == Self::VOID => {
                err.has_zero_niche()
            },
            _ => false,
        }
    }

    fn plain(&self, allow_generic_param: bool) -> impl fmt::Display + '_ {
        FormatterFn(move |f| {
            match self {
                Self::Primitive(prim) => write!(f, "{prim}"),
                Self::Pointee(pointee) => {
                    error!(?self, "must be behind pointer");
                    write!(f, "{}", pointee.behind_pointer(allow_generic_param))
                }
                Self::Simd { ty, size } => write!(f, "Simd<{ty}, {size}>"),
                Self::Sel { nullability } => {
                    if *nullability == Nullability::NonNull {
                        write!(f, "Sel")
                    } else {
                        write!(f, "Option<Sel>")
                    }
                }
                Self::Pointer {
                    nullability,
                    read,
                    written,
                    // Ignore
                    lifetime: _,
                    bounds,
                    pointee,
                } => match &**pointee {
                    Self::Pointee(PointeeTy::Fn {
                        is_variadic,
                        no_escape: _,
                        arguments,
                        result_type,
                    }) if *bounds == PointerBounds::Single => {
                        if *nullability != Nullability::NonNull {
                            write!(f, "Option<")?;
                        }
                        // Allow pointers that the user provides to unwind.
                        //
                        // This is not _necessarily_ safe, though in practice
                        // it will be for all of Apple's frameworks.
                        write!(f, "unsafe extern \"C-unwind\" fn(")?;
                        for arg in arguments {
                            write!(f, "{},", arg.plain(allow_generic_param))?;
                        }
                        if *is_variadic {
                            write!(f, "...")?;
                        }
                        write!(f, ")")?;
                        write!(
                            f,
                            "{}",
                            result_type.prefix_return(result_type.fn_type_return())
                        )?;
                        if *nullability != Nullability::NonNull {
                            write!(f, ">")?;
                        }
                        Ok(())
                    }
                    pointee => {
                        let pointee =
                            maybemaybeuninit(*read, pointee.behind_pointer(allow_generic_param));
                        if *nullability == Nullability::NonNull {
                            write!(f, "NonNull<{pointee}>")
                        } else if *written {
                            write!(f, "*mut {pointee}")
                        } else {
                            write!(f, "*const {pointee}")
                        }
                    }
                },
                Self::TypeDef { id, .. } => {
                    write!(f, "{}", id.path())
                }
                Self::Array {
                    element_type,
                    num_elements,
                } => write!(
                    f,
                    "[{}; {num_elements}]",
                    element_type.plain(allow_generic_param)
                ),
                Self::Struct { id, .. } => {
                    write!(f, "{}", id.path())
                }
                Self::Union { id, .. } => {
                    write!(f, "{}", id.path())
                }
                Self::Enum { id, .. } => {
                    write!(f, "{}", id.path())
                }
                Self::Result { ty, err, .. } => {
                    write!(
                        f,
                        "Result<{}, {}>",
                        ty.plain(allow_generic_param),
                        err.plain(allow_generic_param)
                    )
                }
            }
        })
    }

    fn behind_pointer(&self, allow_generic_param: bool) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Primitive(Primitive::Void) => write!(f, "c_void"),
            Self::Pointee(pointee) => write!(f, "{}", pointee.behind_pointer(allow_generic_param)),
            _ => write!(f, "{}", self.plain(allow_generic_param)),
        })
    }

    /// Prefix the inner value with ` -> ` if needed.
    ///
    /// If the type is `c_void`, we don't print anything at all.
    pub(crate) fn prefix_return<'a>(
        &'a self,
        inner: impl fmt::Display + 'a,
    ) -> impl fmt::Display + 'a {
        FormatterFn(move |f| match self {
            // Don't output anything here.
            Self::Primitive(Primitive::Void) => Ok(()),
            _ => write!(f, " -> {inner}"),
        })
    }

    pub(crate) fn method_return(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Pointer {
                nullability,
                pointee,
                bounds: PointerBounds::Single,
                ..
            } if pointee.is_static_object() => {
                if *nullability == Nullability::NonNull {
                    // TODO: Add runtime nullability check here.
                    write!(f, "&'static {}", pointee.behind_pointer(true))
                } else {
                    write!(f, "Option<&'static {}>", pointee.behind_pointer(true))
                }
            }
            Self::Pointer {
                nullability: _,
                lifetime: _, // TODO
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if pointee.is_block_type() => {
                // TODO: Emit `RcBlock` or similar.
                write!(f, "{}", self.plain(true))
            }
            Self::Pointer {
                nullability,
                lifetime: _, // TODO: Use this somehow?
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if pointee.is_object_like() => {
                // NOTE: We return CF types as `Retained` for now, since we
                // don't have support for the CF wrapper in msg_send! yet.
                if *nullability == Nullability::NonNull {
                    write!(f, "Retained<{}>", pointee.behind_pointer(true))
                } else {
                    write!(f, "Option<Retained<{}>>", pointee.behind_pointer(true))
                }
            }
            Self::Pointer {
                nullability,
                lifetime: _, // TODO
                bounds: PointerBounds::NullTerminated,
                pointee,
                ..
            } if pointee.is_pointee_cstr() => {
                if *nullability == Nullability::NonNull {
                    write!(f, "&CStr")
                } else {
                    write!(f, "Option<&CStr>")
                }
            }
            Self::Primitive(Primitive::C99Bool) => {
                warn!("C99's bool as Objective-C method return is ill supported");
                write!(f, "bool")
            }
            Self::Primitive(Primitive::ObjcBool) => write!(f, "bool"),
            Self::Result { ty, err, .. } => {
                // Using `Result<Option<X>, Y>` will be detected by `objc2`,
                // so we don't bother with checking validity here.
                write!(f, "Result<{}, {}>", ty.method_return(), err.method_return())
            }
            _ => write!(f, "{}", self.plain(true)),
        })
    }

    pub fn method_return_inner_pointer(&self) -> impl fmt::Display + '_ {
        // TODO(breaking): Return "self.plain()" here instead.
        self.method_return()
    }

    /// Attempt to convert the type to a `Result<T, Retained<NSError>>`.
    pub(crate) fn convert_to_result(&self, err: Ty) -> Option<Self> {
        // We allow return values with unspecified nullability, because that's
        // what Swift seems to do as well. Note that non-null return values
        // cannot be mapped as `Result<T, E>`, because the return value `T`
        // would always be present.
        match self {
            Self::Pointer {
                nullability:
                    Nullability::Nullable | Nullability::NullableResult | Nullability::Unspecified,
                read,
                written,
                lifetime,
                bounds,
                pointee,
                // TODO: Remove this check?
            } if pointee.is_object_like() && !pointee.is_block_type() => {
                Some(Self::Result {
                    ty: Box::new(Self::Pointer {
                        // NULL -> error
                        nullability: Nullability::NonNull,
                        read: *read,
                        written: *written,
                        lifetime: *lifetime,
                        bounds: bounds.clone(),
                        pointee: pointee.clone(),
                    }),
                    err: Box::new(err),
                    original_ty: Box::new(self.clone()),
                })
            }
            Self::Primitive(Primitive::C99Bool) | Self::Primitive(Primitive::ObjcBool) => {
                if *self == Self::Primitive(Primitive::C99Bool) {
                    warn!("C99's bool as Objective-C method return is ill supported");
                }
                // NO -> error
                Some(Self::Result {
                    ty: Box::new(Self::Primitive(Primitive::Void)),
                    err: Box::new(err),
                    original_ty: Box::new(self.clone()),
                })
            }
            _ => None,
        }
    }

    pub(crate) fn method_return_encoding_type(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Primitive(Primitive::C99Bool) => write!(f, "Bool"),
            Self::Pointer { pointee, .. } if **pointee == Self::Pointee(PointeeTy::Self_) => {
                write!(f, "*mut This")
            }
            _ => write!(f, "{}", self.plain(true)),
        })
    }

    fn fn_type_return(&self) -> impl fmt::Display + '_ {
        let allow_generic_param = true; // Might not be entirely correct?
        FormatterFn(move |f| match self {
            Self::Pointer {
                nullability,
                pointee,
                bounds: PointerBounds::Single,
                ..
            } if pointee.is_static_object() => {
                if *nullability == Nullability::NonNull {
                    // TODO: Add runtime nullability check here (can we even
                    // do that?).
                    write!(
                        f,
                        "&'static {}",
                        pointee.behind_pointer(allow_generic_param)
                    )
                } else {
                    write!(
                        f,
                        "Option<&'static {}>",
                        pointee.behind_pointer(allow_generic_param)
                    )
                }
            }
            _ => write!(f, "{}", self.plain(allow_generic_param)),
        })
    }

    fn retain_wrapper(&self) -> Option<RetainWrapper> {
        if self.is_objc_type() && !self.is_static_object() {
            Some(RetainWrapper::Retained)
        } else if self.is_cf_type() {
            Some(RetainWrapper::CFRetained)
        } else if self.is_dispatch_type() {
            Some(RetainWrapper::DispatchRetained)
        } else if self.is_network_type() {
            Some(RetainWrapper::NWRetained)
        } else {
            None
        }
    }

    pub(crate) fn is_retained_return(&self) -> bool {
        matches!(
            self,
            Self::Pointer {
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if pointee.retain_wrapper().is_some()
        )
    }

    pub(crate) fn fn_return(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Pointer {
                nullability,
                read,
                written,
                lifetime: _,
                bounds,
                pointee,
            } => {
                // Ignore nullability, always emit a nullable pointer. We will
                // unwrap it later in `fn_return_converter`.
                //
                // This is required because nullability attributes in Clang
                // are a hint, and not an ABI stable promise.
                if *bounds == PointerBounds::Single {
                    if pointee.is_static_object() {
                        return write!(f, "Option<&'static {}>", pointee.behind_pointer(false));
                    } else if let Some(wrapper) = pointee.retain_wrapper() {
                        if wrapper == RetainWrapper::Retained {
                            return write!(f, "*mut {}", pointee.behind_pointer(false));
                        } else {
                            return write!(f, "Option<NonNull<{}>>", pointee.behind_pointer(false));
                        }
                    }
                }

                let pointee = maybemaybeuninit(*read, pointee.behind_pointer(false));
                if *nullability == Nullability::NonNull {
                    write!(f, "Option<NonNull<{pointee}>>",)
                } else if *written {
                    write!(f, "*mut {pointee}")
                } else {
                    write!(f, "*const {pointee}",)
                }
            }
            _ => write!(f, "{}", self.plain(false)),
        })
    }

    pub(crate) fn fn_return_converted(&self, returns_retained: bool) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            _ if self.is_objc_bool() => write!(f, "bool"),
            Self::TypeDef { id, .. } if matches!(&*id.name, "Boolean" | "boolean_t") => {
                write!(f, "bool")
            }
            Self::Pointer {
                // Null-check only necessary here
                nullability: Nullability::NonNull,
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if pointee.is_static_object() => {
                write!(f, "&'static {}", pointee.behind_pointer(true))
            }
            Self::Pointer {
                nullability,
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if let Some(wrapper) = pointee.retain_wrapper() => {
                if *nullability == Nullability::NonNull {
                    write!(f, "{}<{}>", wrapper.name(), pointee.behind_pointer(true))
                } else {
                    write!(
                        f,
                        "Option<{}<{}>>",
                        wrapper.name(),
                        pointee.behind_pointer(true)
                    )
                }
            }
            Self::Pointer {
                nullability,
                bounds: PointerBounds::NullTerminated,
                pointee,
                ..
            } if pointee.is_pointee_cstr() && !returns_retained => {
                // TODO: Return `Box<CStr, std::alloc::System>` when `returns_retained`.

                // TODO: Use `'static` here when specified.
                if *nullability == Nullability::NonNull {
                    write!(f, "&CStr")
                } else {
                    write!(f, "Option<&CStr>")
                }
            }
            Self::Pointer { pointee, .. }
                if pointee.is_generic_param()
                    || matches!(&**pointee, Self::Pointer { pointee, .. } if pointee.is_generic_param()) =>
            {
                write!(f, "{}", self.plain(true))
            }
            Self::Pointer {
                nullability: Nullability::NonNull,
                read,
                pointee,
                ..
            } => {
                let pointee = maybemaybeuninit(*read, pointee.behind_pointer(true));
                write!(f, "NonNull<{pointee}>")
            }
            _ => write!(f, "{}", self.fn_return()),
        })
    }

    pub(crate) fn fn_return_required_items(&self) -> impl Iterator<Item = ItemTree> {
        let mut items: Vec<_> = self.required_items().collect();
        match self {
            Self::Pointer {
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if let Some(wrapper) = pointee.retain_wrapper() => {
                items.push(wrapper.item());
                if pointee.is_cf_type() {
                    items.push(ItemTree::core_ptr_nonnull());
                }
            }
            _ => {}
        }
        items.into_iter()
    }

    pub(crate) fn fn_return_converter<'s: 'r, 'a: 'r, 'r>(
        &'s self,
        returns_retained: bool,
        fn_call: impl fmt::Display + 'a,
    ) -> impl fmt::Display + 'r {
        FormatterFn(move |f| {
            match self {
                _ if self.is_objc_bool() => write!(f, "{fn_call}.as_bool()"),
                Self::TypeDef { id, .. } if matches!(&*id.name, "Boolean" | "boolean_t") => {
                    write!(f, "let ret = {fn_call};\nret != 0")
                }
                Self::Pointer {
                    // Null-check only necessary here
                    nullability: Nullability::NonNull,
                    bounds: PointerBounds::Single,
                    pointee,
                    ..
                } if pointee.is_static_object() => {
                    write!(f, "let ret = {fn_call};")?;
                    write!(f, "ret.expect(\"function was marked as returning non-null, but actually returned NULL\")")?;
                    Ok(())
                }
                Self::Pointer {
                    nullability,
                    lifetime,
                    bounds: PointerBounds::Single,
                    pointee,
                    ..
                } if let Some(wrapper) = pointee.retain_wrapper() => {
                    match lifetime {
                        Lifetime::Autoreleasing if !returns_retained => {}
                        Lifetime::Strong if returns_retained => {}
                        Lifetime::Unspecified => {}
                        _ => error!(?lifetime, returns_retained, "invalid lifetime"),
                    }

                    let needs_cast = matches!(
                        &**pointee,
                        Self::Pointee(PointeeTy::CFTypeDef { generics, .. })
                            if generics.iter().any(|generic| matches!(generic, PointeeTy::GenericParam { .. })
                        )
                    );

                    write!(
                        f,
                        "{}",
                        wrapper.fn_return(*nullability, returns_retained, needs_cast, &fn_call)
                    )?;

                    Ok(())
                }
                Self::Pointer {
                    nullability,
                    written,
                    bounds: PointerBounds::NullTerminated,
                    pointee,
                    ..
                } if pointee.is_pointee_cstr() && !returns_retained => {
                    writeln!(f, "let ret = {fn_call};")?;

                    if *nullability == Nullability::NonNull {
                        writeln!(f, "let ret = ret.expect(\"function was marked as returning non-null, but actually returned NULL\");")?;
                        writeln!(f, "unsafe {{ CStr::from_ptr(ret.as_ptr()) }}")?;
                    } else {
                        if *written {
                            writeln!(f, "NonNull::new(ret).map(|ret| unsafe {{ CStr::from_ptr(ret.as_ptr()) }})")?;
                        } else {
                            writeln!(f, "NonNull::new(ret.cast_mut()).map(|ret| unsafe {{ CStr::from_ptr(ret.as_ptr()) }})")?;
                        }
                    }
                    Ok(())
                }
                Self::Pointer { pointee, .. }
                    if pointee.is_generic_param()
                        || matches!(&**pointee, Self::Pointer { pointee, .. } if pointee.is_generic_param()) =>
                {
                    write!(f, "{fn_call}.cast()")
                }
                Self::Pointer {
                    nullability: Nullability::NonNull,
                    ..
                } => {
                    writeln!(f, "let ret = {fn_call};")?;
                    write!(f, "ret.expect(\"function was marked as returning non-null, but actually returned NULL\")")?;
                    Ok(())
                }
                _ => write!(f, "{fn_call}"),
            }
        })
    }

    pub(crate) fn var(&self) -> impl fmt::Display + '_ {
        // Might not be correct, but probably doesn't matter.
        let allow_generic_param = true;
        FormatterFn(move |f| match self {
            Self::Pointer {
                nullability,
                read,
                // `const` is irrelevant in statics since they're always
                // constant.
                written: _,
                lifetime: Lifetime::Strong | Lifetime::Unspecified,
                bounds: PointerBounds::Single,
                pointee,
            } if pointee.is_object_like() => {
                let pointee = maybemaybeuninit(*read, pointee.behind_pointer(allow_generic_param));
                match *nullability {
                    Nullability::NonNull => write!(f, "&'static {pointee}"),
                    // NOTE: The sound option here would be to emit these as
                    // `Option<&T>`, since we don't know the nullability.
                    //
                    // In practice though, it's much very common that statics
                    // don't have a nullability applied (especially in CF),
                    // and we want to emit these as non-null too whenever we
                    // can.
                    //
                    // We test that all non-null statics are actually non-null
                    // in `test-frameworks` with `check_static_nonnull`, so
                    // there shouldn't be any danger from doing this
                    //
                    // (I guess the value of a static changes from one OS
                    // version to another to become nullable, but that'd be an
                    // observable change, so I strongly doubt it will happen).
                    //
                    // Swift seems to do a similar thing, e.g.
                    // `NSExtensionJavaScriptPreprocessingResultsKey` is
                    // mapped as `String`, not `String!`.
                    Nullability::Unspecified => write!(f, "&'static {pointee}"),
                    Nullability::Nullable | Nullability::NullableResult => {
                        write!(f, "Option<&'static {pointee}>")
                    }
                }
            }
            _ => write!(f, "{}", self.behind_pointer(allow_generic_param)),
        })
    }

    pub(crate) fn const_(&self) -> impl fmt::Display + '_ {
        // Might not be correct, but probably doesn't matter.
        let allow_generic_param = true;
        FormatterFn(move |f| match self {
            Self::Pointer {
                nullability,
                read,
                // `const` is irrelevant in constants since they're always
                // constant.
                written: _,
                lifetime: Lifetime::Strong | Lifetime::Unspecified,
                bounds: PointerBounds::Single,
                pointee,
            } if pointee.is_object_like() => {
                let pointee = maybemaybeuninit(*read, pointee.behind_pointer(allow_generic_param));
                if *nullability == Nullability::NonNull {
                    write!(f, "&{pointee}")
                } else {
                    write!(f, "Option<&{pointee}>")
                }
            }
            Self::Pointer {
                nullability,
                read: _,
                // `const` is irrelevant in constants since they're always
                // constant.
                written: _,
                lifetime: Lifetime::Unspecified, // TODO
                bounds: PointerBounds::NullTerminated,
                pointee,
            } if pointee.is_pointee_cstr() => {
                if *nullability == Nullability::NonNull {
                    write!(f, "&CStr")
                } else {
                    write!(f, "Option<&CStr>")
                }
            }
            _ => write!(f, "{}", self.plain(allow_generic_param)),
        })
    }

    pub(crate) fn typedef(&self) -> impl fmt::Display + '_ {
        let allow_generic_param = true; // Might not be correct?
        FormatterFn(move |f| match self {
            Self::Pointer {
                nullability: _,
                read: _,
                written: _,
                lifetime: _,
                bounds: _,
                pointee,
            } if pointee.is_object_like() => {
                write!(f, "{}", pointee.behind_pointer(allow_generic_param))
            }
            // We mark `typedefs` as-if behind a pointer, as even though
            // typedefs are _usually_ to a pointer of the type (handled
            // above), we sometimes have typedefs to the inner type as well.
            //
            // Examples:
            // typedef NSDictionary<NSString *, MPSGraphExecutable *> MPSGraphCallableMap;
            // typedef void NSUncaughtExceptionHandler(NSException *exception);
            _ => write!(f, "{}", self.behind_pointer(allow_generic_param)),
        })
    }

    fn fn_argument(&self, allow_generic_param: bool) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Pointer {
                nullability,
                read,
                written,
                lifetime,
                bounds: PointerBounds::Single,
                pointee,
            } if !matches!(**pointee, Self::Pointee(PointeeTy::Fn { .. })) => {
                if *lifetime == Lifetime::Autoreleasing {
                    error!(?self, "autoreleasing in fn argument");
                }

                let inner = maybemaybeuninit(*read, pointee.behind_pointer(allow_generic_param));
                // We don't care if `PointeeTy` pointers may be written to,
                // since those use interior mutability anyhow.
                let mut_ = if !written || matches!(pointee.through_wrapper(), Self::Pointee(_)) {
                    ""
                } else {
                    "mut "
                };
                if *nullability == Nullability::NonNull {
                    write!(f, "&{mut_}{inner}")
                } else {
                    write!(f, "Option<&{mut_}{inner}>")
                }
            }
            _ => write!(f, "{}", self.plain(allow_generic_param)),
        })
    }

    pub(crate) fn fn_argument_unconverted(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            _ if let Some((
                nullability,
                pointee,
                Nullability::Unspecified | Nullability::Nullable | Nullability::NullableResult,
                Lifetime::Autoreleasing | Lifetime::Strong,
            )) = self.out_pointer_data() =>
            {
                // Emitting signatures with incomplete semantics like this for
                // FFI functions is a bit of a cardinal sin, but it'll be
                // fiiine, everything here is behind a pointer, so there's no
                // risk of the compiler asserting anything.
                let name = pointee.retain_wrapper().unwrap().name();
                let behind_pointer = pointee.behind_pointer(true);
                if nullability == Nullability::NonNull {
                    write!(f, "&mut Option<{name}<{behind_pointer}>>")
                } else {
                    write!(f, "Option<&mut Option<{name}<{behind_pointer}>>>")
                }
            }
            _ => write!(f, "{}", self.fn_argument(false)),
        })
    }

    pub(crate) fn fn_argument_converted(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            _ if self.is_objc_bool() => write!(f, "bool"),
            Self::TypeDef { id, .. } if matches!(&*id.name, "Boolean" | "boolean_t") => {
                write!(f, "bool")
            }
            _ if let Some((
                nullability,
                pointee,
                Nullability::Unspecified | Nullability::Nullable | Nullability::NullableResult,
                Lifetime::Autoreleasing | Lifetime::Strong,
            )) = self.out_pointer_data() =>
            {
                let name = pointee.retain_wrapper().unwrap().name();
                let behind_pointer = pointee.behind_pointer(true);
                if nullability == Nullability::NonNull {
                    write!(f, "&mut Option<{name}<{behind_pointer}>>")
                } else {
                    write!(f, "Option<&mut Option<{name}<{behind_pointer}>>>")
                }
            }
            Self::Pointer {
                nullability,
                read: _,
                written: false,
                lifetime: Lifetime::Unspecified, // TODO
                bounds: PointerBounds::NullTerminated,
                pointee,
            } if pointee.is_pointee_cstr() => {
                if *nullability == Nullability::NonNull {
                    write!(f, "&CStr")
                } else {
                    write!(f, "Option<&CStr>")
                }
            }
            _ => write!(f, "{}", self.fn_argument(true)),
        })
    }

    pub(crate) fn fn_argument_required_items(&self) -> impl Iterator<Item = ItemTree> {
        let mut items: Vec<_> = self.required_items().collect();
        match self {
            _ if let Some((
                _,
                pointee,
                Nullability::Unspecified | Nullability::Nullable | Nullability::NullableResult,
                Lifetime::Autoreleasing | Lifetime::Strong,
            )) = self.out_pointer_data() =>
            {
                let wrapper = pointee.retain_wrapper().unwrap();
                items.push(wrapper.item());
            }
            _ => {}
        }
        items.into_iter()
    }

    pub(crate) fn fn_argument_converter<'s: 'r, 'a: 'r, 'r>(
        &'s self,
        arg: &'a str,
        arg_to: &'a str,
    ) -> impl fmt::Display + 'r {
        FormatterFn(move |f| match self {
            _ if self.is_objc_bool() => writeln!(f, "let {arg_to} = Bool::new({arg});"),
            Self::TypeDef { id, .. } if matches!(&*id.name, "Boolean" | "boolean_t") => {
                writeln!(f, "let {arg_to} = {arg} as _;")
            }
            _ if let Some((
                nullability,
                pointee,
                // The inner type has to be nullable, see below.
                Nullability::Unspecified | Nullability::Nullable | Nullability::NullableResult,
                lifetime @ (Lifetime::Autoreleasing | Lifetime::Strong),
            )) = self.out_pointer_data() =>
            {
                assert_eq!(arg, arg_to);
                let wrapper = pointee.retain_wrapper().unwrap();
                let name = wrapper.name();

                // Both `CF_RETURNS_RETAINED` and `CF_RETURNS_NOT_RETAINED`
                // work similarly: If the given pointer is nullable they check
                // it, and then they write the return value to the pointer.
                //
                // I.e. basically `if (ptr) *ptr = CreateOrGetValue();`.
                //
                // When the value is not retained (autoreleased), we need to
                // retain it ourselves after the function returns (if a value
                // was set).
                //
                // Unfortunately, callees don't check if there's a value
                // already in `ptr` and release that, which is counter to how
                // `&mut Option<CFRetained<T>>` works (and I don't know of a
                // better way to describe it)?
                //
                // This means that we can only support input values that are
                // `None`, and is why we require the inner type is nullable.

                // Assert that the given inner value is `NULL`; it is invalid
                // usage of the API to pass anything else, because that value
                // will get leaked, see above.
                let assert_is_null = |f: &mut fmt::Formatter<'_>| {
                    writeln!(f, "assert!({arg}.is_none(), \"parameter `{arg}` must point to `None` on entry\");")
                };

                // Retain at the end of the function.
                //
                // "Autoreleasing" is a bit of a misnomer here, it's
                // not actually autorelease semantics, it's rather "getter",
                // where the lifetime is tied to the object we're getting from
                // (which we don't know in general, thus we must retain).
                if lifetime == Lifetime::Autoreleasing {
                    let retain_on_drop = format!(
                        "Retain{}OnDrop",
                        heck::ToUpperCamelCase::to_upper_camel_case(arg)
                    );

                    // We retain inside `Drop` to support functions that
                    // unwind after setting the out pointer.
                    let t = pointee.behind_pointer(true);
                    writeln!(
                        f,
                        "struct {retain_on_drop}<'a>(&'a mut Option<{name}<{t}>>);"
                    )?;
                    writeln!(f, "impl Drop for {retain_on_drop}<'_> {{")?;
                    writeln!(f, "    #[inline]")?;
                    writeln!(f, "    fn drop(&mut self) {{")?;
                    // Read the pointer and retain it if it was set. Don't
                    // release it; this will be done in the user's frame.
                    //
                    // Note: We rely on `Option<T>: Clone` here. It would be a
                    // bit more efficient for Objective-C types to use
                    // `Retained::retain`, but it's miniscule, so we won't
                    // bother.
                    writeln!(f, "        let _ = core::mem::ManuallyDrop::<Option<_>>::new(self.0.clone());")?;
                    writeln!(f, "    }}")?;
                    writeln!(f, "}}")?;

                    if nullability == Nullability::NonNull {
                        assert_is_null(f)?;

                        // Get and reborrow.
                        writeln!(f, "let {arg} = &mut *{retain_on_drop}({arg}).0;")?;

                        // Retains at end of function.
                    } else {
                        writeln!(f, "let mut {arg} = if let Some({arg}) = {arg} {{")?;
                        assert_is_null(f)?;
                        writeln!(f, "    Some({retain_on_drop}({arg}))")?;
                        writeln!(f, "}} else {{")?;
                        writeln!(f, "    None")?;
                        writeln!(f, "}};")?;

                        // Reborrow.
                        writeln!(f, "let {arg} = {arg}.as_mut().map(|arg| &mut *arg.0);")?;

                        // Retains at end of function after checking drop flag.
                    }
                } else {
                    if nullability == Nullability::NonNull {
                        assert_is_null(f)?;
                    } else {
                        writeln!(f, "if let Some({arg}) = {arg}.as_ref() {{")?;
                        assert_is_null(f)?;
                        writeln!(f, "}};")?;
                    }

                    // We don't need to do anything else for functions with
                    // `CF_RETURNS_RETAINED`; we're given ownership over the
                    // returned value, which is the same as we'd expect with
                    // `&mut Option<CFRetained<T>>`.
                }

                Ok(())
            }
            Self::Pointer {
                nullability,
                read: _,
                // Only map `const char*` to `&CStr`
                written: false,
                lifetime: Lifetime::Unspecified, // TODO
                bounds: PointerBounds::NullTerminated,
                pointee,
            } if pointee.is_pointee_cstr() => {
                // We could emit a length check here for `char[N]`, to ensure
                // that the user doesn't pass a string that is too long.
                //
                // But in practice, at least for the places that this is
                // relevant (namely IOKit), it doesn't matter, the length is
                // gotten via. `strlen`, and passing a string that is too long
                // (seemingly) doesn't change the behaviour.

                writeln!(f, "let {arg_to} = ")?;
                if *nullability == Nullability::NonNull {
                    writeln!(f, "NonNull::new({arg}.as_ptr().cast_mut()).unwrap()")?;
                } else {
                    writeln!(
                        f,
                        "{arg}.map(|ptr| ptr.as_ptr()).unwrap_or_else(core::ptr::null)"
                    )?;
                }
                if !matches!(pointee.through_wrapper(), Self::Primitive(Primitive::Char)) {
                    writeln!(f, ".cast()")?;
                }
                writeln!(f, ";")?;

                Ok(())
            }
            // HACK to support CFArray<T>.
            Self::Pointer {
                nullability,
                pointee,
                bounds,
                ..
            } => {
                if matches!(&**pointee, Self::Pointee(PointeeTy::GenericParam { .. }))
                    || matches!(&**pointee, Self::Pointer { pointee, .. }
                        if matches!(&**pointee, Self::Pointee(PointeeTy::GenericParam { .. }))
                    )
                {
                    if *nullability == Nullability::NonNull {
                        if *bounds == PointerBounds::Single {
                            writeln!(
                                f,
                                "let {arg_to} = unsafe {{ core::mem::transmute({arg}) }};"
                            )
                        } else {
                            writeln!(f, "let {arg_to} = {arg}.map(|x| x.cast());")
                        }
                    } else {
                        if *bounds == PointerBounds::Single {
                            writeln!(f, "let {arg_to} = {arg}.map(|x| unsafe {{ core::mem::transmute(x) }});")
                        } else {
                            writeln!(f, "let {arg_to} = {arg}.cast();")
                        }
                    }
                } else if let Self::Pointee(PointeeTy::CFTypeDef { generics, .. }) = &**pointee {
                    if generics
                        .iter()
                        .any(|generic| matches!(generic, PointeeTy::GenericParam { .. }))
                    {
                        if *nullability == Nullability::NonNull {
                            writeln!(f, "let {arg_to} = {arg}.as_opaque();")
                        } else {
                            writeln!(f, "let {arg_to} = {arg}.map(|obj| obj.as_opaque());")
                        }
                    } else {
                        Ok(())
                    }
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        })
    }

    fn out_pointer_data(&self) -> Option<(Nullability, &Ty, Nullability, Lifetime)> {
        match self {
            Self::Pointer {
                nullability,
                read,
                written: true,
                lifetime: Lifetime::Unspecified,
                // TODO: Is this correct?
                bounds: PointerBounds::Unspecified | PointerBounds::Single,
                pointee,
            } => match &**pointee {
                Self::Pointer {
                    nullability: inner_nullability,
                    // Don't care about the const-ness of the inner type.
                    read: _,
                    written: _,
                    lifetime,
                    bounds: PointerBounds::Single,
                    pointee,
                } if pointee.retain_wrapper().is_some() => {
                    if !read {
                        // We don't (yet) support `&mut MaybeUninit<Retained<T>>`.
                        error!("out pointers must currently be readable");
                    }
                    Some((*nullability, pointee, *inner_nullability, *lifetime))
                }
                _ => None,
            },
            _ => None,
        }
    }

    pub(crate) fn method_argument(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Primitive(Primitive::C99Bool) => {
                warn!("C99's bool as Objective-C method argument is ill supported");
                write!(f, "bool")
            }
            Self::Primitive(Primitive::ObjcBool) => {
                write!(f, "bool")
            }
            Self::Pointer {
                nullability,
                read: _,
                written: false,
                lifetime: Lifetime::Unspecified, // TODO
                bounds: PointerBounds::NullTerminated,
                pointee,
            } if pointee.is_pointee_cstr() => {
                if *nullability == Nullability::NonNull {
                    write!(f, "&CStr")
                } else {
                    write!(f, "Option<&CStr>")
                }
            }
            _ if let Some((nullability, pointee, inner_nullability, Lifetime::Autoreleasing)) =
                self.out_pointer_data() =>
            {
                let inner = FormatterFn(|f| {
                    if inner_nullability == Nullability::NonNull {
                        write!(f, "Retained<{}>", pointee.behind_pointer(true))
                    } else {
                        write!(f, "Option<Retained<{}>>", pointee.behind_pointer(true))
                    }
                });
                if nullability == Nullability::NonNull {
                    write!(f, "&mut {inner}")
                } else {
                    write!(f, "Option<&mut {inner}>")
                }
            }
            _ => write!(f, "{}", self.fn_argument(true)),
        })
    }

    /// Is this a function out parameter to a non-error (CFError/NSError) type?
    pub(crate) fn is_fn_out_param_nonerror(&self) -> bool {
        if let Some((
            _,
            pointee,
            Nullability::Unspecified | Nullability::Nullable | Nullability::NullableResult,
            Lifetime::Autoreleasing | Lifetime::Strong,
        )) = self.out_pointer_data()
        {
            match pointee.through_wrapper() {
                Ty::Pointee(PointeeTy::CFTypeDef { id, .. }) if id.is_cferror() => false,
                Ty::Pointee(PointeeTy::Class { id, .. }) if id.is_nserror() => false,
                _ => true,
            }
        } else {
            false
        }
    }

    /// Apply various heuristics to out parameter retained-ness.
    ///
    /// These aren't perfect, but they should be good enough to get started.
    pub(crate) fn set_default_retained_out_param(&mut self, follows_create_rule: bool) {
        if let Self::Pointer { pointee, .. } = self {
            if let Self::Pointer {
                lifetime, pointee, ..
            } = &mut **pointee
            {
                if *lifetime == Lifetime::Unspecified {
                    match pointee.through_wrapper() {
                        // Error params usually follow the create rule (they
                        // aren't attached to anything, so they can't really
                        // be returned in any other way).
                        Ty::Pointee(PointeeTy::CFTypeDef { id, .. }) if id.is_cferror() => {
                            *lifetime = Lifetime::Strong;
                        }
                        Ty::Pointee(PointeeTy::Class { id, .. }) if id.is_nserror() => {
                            // Unsure
                        }
                        _ if pointee.retain_wrapper().is_some() && follows_create_rule => {
                            *lifetime = Lifetime::Strong;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub(crate) fn method_argument_encoding_type(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Primitive(Primitive::C99Bool) => write!(f, "Bool"),
            _ => write!(f, "{}", self.plain(true)),
        })
    }

    pub(crate) fn record(&self) -> impl fmt::Display + '_ {
        self.plain(true)
    }

    fn fn_contains_bool(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self.through_wrapper() {
            if let Self::Pointee(PointeeTy::Fn {
                arguments,
                result_type,
                ..
            }) = &**pointee
            {
                if arguments
                    .iter()
                    .any(|arg| matches!(arg, Self::Primitive(Primitive::C99Bool)))
                {
                    return true;
                }
                if matches!(**result_type, Self::Primitive(Primitive::C99Bool)) {
                    return true;
                }
            }
            false
        } else {
            false
        }
    }

    pub(crate) fn record_encoding(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Primitive(Primitive::C99Bool) => write!(f, "Encoding::Bool"),
            Self::Primitive(Primitive::Long) => write!(f, "Encoding::C_LONG"),
            Self::Primitive(Primitive::ULong) => write!(f, "Encoding::C_ULONG"),
            // TODO: Make all function pointers be encode, regardless of arguments
            _ if self.fn_contains_bool() => {
                write!(f, "Encoding::Pointer(&Encoding::Unknown)")
            }
            _ => write!(f, "<{}>::ENCODING", self.record()),
        })
    }

    pub(crate) fn enum_(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| write!(f, "{}", self.plain(true)))
    }

    pub(crate) fn enum_encoding(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Primitive(Primitive::C99Bool) => write!(f, "Encoding::Bool"),
            Self::Primitive(Primitive::Long) => write!(f, "Encoding::C_LONG"),
            Self::Primitive(Primitive::ULong) => write!(f, "Encoding::C_ULONG"),
            _ => write!(f, "{}::ENCODING", self.enum_()),
        })
    }

    // FIXME: See https://github.com/rust-lang/rfcs/pull/3659
    pub(crate) fn closed_enum_repr(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Primitive(Primitive::NSInteger) => write!(f, "#[repr(isize)] // NSInteger"),
            Self::Primitive(Primitive::NSUInteger) => write!(f, "#[repr(usize)] // NSUInteger"),
            Self::Primitive(Primitive::U32) => write!(f, "#[repr(u32)]"),
            Self::Primitive(Primitive::Int) => write!(f, "#[repr(i32)] // c_int"),
            _ => panic!("invalid closed enum repr: {self:?}"),
        })
    }

    pub(crate) const VOID: Self = Self::Primitive(Primitive::Void);

    pub(crate) fn parse_function_argument(
        ty: Type<'_>,
        _qualifier: Option<MethodArgumentQualifier>,
        mut param_sendable: Option<bool>,
        mut param_no_escape: bool,
        mut param_out_pointer_retained: Option<bool>,
        context: &Context<'_>,
    ) -> Self {
        let mut ty = Self::parse(ty, true, context);

        match &mut ty {
            Self::Pointer { pointee, .. } => match &mut **pointee {
                Self::Pointer { lifetime, .. } => {
                    // Sometimes, if the `CF_RETURNS_RETAINED` is put in the
                    // wrong location, Clang doesn't know (or at least doesn't
                    // act like it knows) where to add the lifetime specifier,
                    // instead the attribute ends up on the `ParmDecl`. So we
                    // have to pass it through here.
                    //
                    // E.g. `CF_RETURNS_RETAINED CFStringRef * foo` doesn't
                    // get a `__strong` modifier automatically by Clang, while
                    // `CFStringRef * CF_RETURNS_RETAINED foo` does.
                    //
                    // (This is probably a bug in libClang).
                    if let Some(out_pointer_retained) = param_out_pointer_retained.take() {
                        *lifetime = if out_pointer_retained {
                            Lifetime::Strong
                        } else {
                            Lifetime::Autoreleasing
                        };
                    }
                }
                Self::Pointee(PointeeTy::Block {
                    sendable,
                    no_escape,
                    ..
                }) => {
                    if let Some(param_sendable) = param_sendable.take() {
                        *sendable = Some(param_sendable);
                    }
                    *no_escape = param_no_escape;
                    param_no_escape = false;
                }
                Self::Pointee(
                    PointeeTy::Class { sendable, .. } | PointeeTy::AnyObject { sendable, .. },
                ) => {
                    if only_positive_sendable(param_sendable.take()) {
                        *sendable = true;
                    }
                }
                Self::Pointee(PointeeTy::Fn { no_escape, .. }) => {
                    *no_escape = param_no_escape;
                    param_no_escape = false;
                }
                // Ignore `param_no_escape` on typedefs for now.
                Self::Pointee(PointeeTy::TypeDef { .. }) => {
                    param_no_escape = false;
                }
                // Hacky fix for `out_pointer_retained` in `SSLGetConnection`,
                // we would want to assign this to the `SSLConnectionRef`, but
                // that can't work yet because `SSLConnectionRef` is a `void*`
                // typedef.
                Self::TypeDef { id, .. } if id.name == "SSLConnectionRef" => {
                    param_out_pointer_retained = None;
                }
                _ => {}
            },
            // Ignore `param_no_escape` on typedefs for now.
            Self::TypeDef { .. } => {
                param_no_escape = false;
            }
            _ => {}
        }

        if param_sendable.is_some() {
            // Important for soundness.
            error!(?ty, "did not consume sendable in argument");
        }

        if param_no_escape {
            warn!(?ty, "did not consume no_escape in argument");
        }

        if param_out_pointer_retained.is_some() {
            // Important for soundness.
            error!(?ty, "did not consume out_pointer_retained in argument");
        }

        // TODO: Is the qualifier useful for anything?

        ty
    }

    pub(crate) fn parse_method_return(
        ty: Type<'_>,
        default_nonnull: bool,
        context: &Context<'_>,
    ) -> Self {
        let mut ty = Self::parse(ty, false, context);

        // As in `parse_property_return`, the nullability is not guaranteed by
        // the method, and can also fail in OOM situations, but that is
        // handled by `#[unsafe(method(...))]`
        if default_nonnull {
            match &mut ty {
                Self::Pointer { nullability, .. } => {
                    if *nullability == Nullability::Unspecified {
                        *nullability = Nullability::NonNull;
                    }
                }
                _ => warn!(?ty, "`default_nonnull` which is not an object"),
            }
        }

        ty
    }

    pub(crate) fn parse_function_return(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse_method_return(ty, false, context)
    }

    pub(crate) fn parse_typedef(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse(ty, false, context)
    }

    pub(crate) fn parse_property_getter(
        ty: Type<'_>,
        is_copy: bool,
        _sendable: Option<bool>,
        context: &Context<'_>,
    ) -> Self {
        let mut ty = Self::parse(ty, false, context);

        // `@property(copy)` is expected to always return a nonnull instance
        // (e.g. for strings it returns the empty string, while
        // `NSEnergyFormatter::numberFormatter` creates a new number object).
        //
        // So if the nullability is not specified by the type, we set it to
        // `nonnull` to get the desired return type.
        //
        // Note that we still keep the setter nullable, since the user may
        // want to rely on the "set `None` gets a default value"-behaviour.
        //
        // Note that none of this is strictly guaranteed by the method, and it
        // can also fail in OOM situations, so we must still perform an unwrap
        // to be sure (Swift also uses forced unwrapping here).
        //
        // This unwrap is done by `#[unsafe(method(...))]` when we specify the
        // return type as `Retained`.
        if is_copy {
            match &mut ty {
                Self::Pointer { nullability, .. } => {
                    if *nullability == Nullability::Unspecified {
                        *nullability = Nullability::NonNull;
                    }
                }
                Self::TypeDef { .. } => {
                    // Typedefs to e.g. blocks.
                    // TODO: Move these into Pointer?
                }
                _ => warn!(?ty, "property(copy) which is not an object"),
            }
        }

        ty
    }

    pub(crate) fn parse_property_setter(
        ty: Type<'_>,
        is_copy: bool,
        _sendable: Option<bool>,
        context: &Context<'_>,
    ) -> Self {
        let mut ty = Self::parse(ty, true, context);

        // See `parse_property_getter` above.
        if is_copy {
            match &mut ty {
                Self::Pointer { nullability, .. } => {
                    if *nullability == Nullability::Unspecified {
                        *nullability = Nullability::Nullable;
                    }
                }
                Self::TypeDef { .. } => {}
                _ => warn!(?ty, "property(copy) which is not an object"),
            }
        }

        ty
    }

    pub(crate) fn parse_record_field(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse(ty, false, context)
    }

    pub fn is_signed(&self) -> Option<bool> {
        match self.through_wrapper() {
            Self::Primitive(prim) => prim.is_signed(),
            Self::Simd { ty, .. } => ty.is_signed(),
            Self::Enum { ty, .. } => ty.is_signed(),
            _ => None,
        }
    }

    pub(crate) fn parse_enum(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse(ty, false, context)
    }

    pub(crate) fn is_simple_uint(&self) -> bool {
        matches!(self, Self::Primitive(Primitive::UInt))
    }

    pub(crate) fn parse_static(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse(ty, false, context)
    }

    pub(crate) fn argument_as_error_out(&self) -> Option<Ty> {
        if let Some((
            // We always pass a place to write the error information,
            // so doesn't matter whether it's optional or not.
            _,
            pointee,
            Nullability::Nullable | Nullability::NullableResult | Nullability::Unspecified,
            lifetime @ (Lifetime::Autoreleasing | Lifetime::Strong),
        )) = self.out_pointer_data()
        {
            let is_error = match pointee {
                // Error params usually follow the create rule (they
                // aren't attached to anything, so they can't really
                // be returned in any other way).
                Ty::Pointee(PointeeTy::CFTypeDef { id, .. }) if id.is_cferror() => true,
                Ty::Pointee(PointeeTy::Class { id, .. }) if id.is_nserror() => {
                    assert_eq!(
                        lifetime,
                        Lifetime::Autoreleasing,
                        "invalid error lifetime {self:?}"
                    );
                    true
                }
                _ => false,
            };
            if is_error {
                return Some(Ty::Pointer {
                    nullability: Nullability::NonNull,
                    read: true,
                    written: true,
                    lifetime,
                    bounds: PointerBounds::Single,
                    pointee: Box::new(pointee.clone()),
                });
            }
        }
        None
    }

    pub(crate) fn is_retainable(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self {
            // Unsure if static items and blocks should be considered retainable?
            pointee.is_object_like() && !pointee.is_static_object() && !pointee.is_block_type()
        } else {
            false
        }
    }

    pub(crate) fn is_instancetype(&self) -> bool {
        matches!(self, Self::Pointer { pointee, .. } if **pointee == Self::Pointee(PointeeTy::Self_))
    }

    pub(crate) fn is_enum(&self, s: &str) -> bool {
        matches!(self, Self::Enum { id, .. } if id.name == s)
    }

    pub(crate) fn is_record(&self, s: &str) -> bool {
        matches!(self, Self::Struct { id, .. } | Self::Union { id, .. } if id.name == s)
    }

    fn is_generic_param(&self) -> bool {
        matches!(self, Self::Pointee(PointeeTy::GenericParam { .. }))
    }

    pub(crate) fn is_enum_through_typedef(&self) -> bool {
        matches!(self.through_wrapper(), Self::Enum { .. })
    }

    pub(crate) fn is_floating_through_typedef(&self) -> bool {
        matches!(
            self.through_wrapper(),
            Self::Primitive(Primitive::F32 | Primitive::F64 | Primitive::Float | Primitive::Double)
        )
    }

    pub(crate) fn is_block_returning_void(&self) -> bool {
        matches!(
            self.through_wrapper(),
            Self::Pointer { pointee, .. } if matches!(
                pointee.through_wrapper(),
                Self::Pointee(pointee) if matches!(
                    pointee.through_typedef(),
                    PointeeTy::Block { result_type, .. } if **result_type == Ty::VOID,
                ),
            ),
        )
    }

    pub(crate) fn default_block_to_sendable(&mut self) {
        match &mut *self {
            Self::Pointer { pointee, .. } => match &mut **pointee {
                Self::Pointee(pointee) => match pointee {
                    PointeeTy::Block { sendable, .. } => {
                        if let Some(_sendable) = sendable {
                            // Keep explicit value
                        } else {
                            // Default to sendable.
                            *sendable = Some(true);
                        }
                    }
                    PointeeTy::TypeDef { id, to } => {
                        if matches!(&**to, PointeeTy::Block { sendable: None, .. }) {
                            error!(
                                "making blocks sendable doesn't work through typedefs, manually mark the block as sendable!\ntypedef.{}.sendable = true",
                                id.name,
                            );
                        }
                    }
                    _ => {}
                },
                Self::TypeDef { .. } => unimplemented!("block pointer pointee typedef"),
                _ => {}
            },
            Self::TypeDef { .. } => unimplemented!("block pointer typedef"),
            _ => {}
        }
    }

    /// SIMD is not yet possible in FFI, see:
    /// <https://github.com/rust-lang/rust/issues/63068>
    pub(crate) fn needs_simd(&self) -> bool {
        match self.through_wrapper() {
            Self::Simd { .. } => true,
            Self::Pointer { pointee, .. } => pointee.needs_simd(),
            Self::Array { element_type, .. } => element_type.needs_simd(),
            Self::Struct { fields, .. } | Self::Union { fields, .. } => {
                fields.iter().any(|(_, field)| field.needs_simd())
            }
            Self::Pointee(pointee) => match pointee.through_typedef() {
                PointeeTy::Fn {
                    result_type,
                    arguments,
                    ..
                }
                | PointeeTy::Block {
                    result_type,
                    arguments,
                    ..
                } => result_type.needs_simd() || arguments.iter().any(|arg| arg.needs_simd()),
                _ => false,
            },
            _ => false,
        }
    }

    pub(crate) fn try_fix_related_result_type(&mut self) {
        if let Self::Pointer { pointee, .. } = self {
            if let Self::Pointee(PointeeTy::AnyObject {
                protocols,
                sendable: false,
            }) = &**pointee
            {
                if !protocols.is_empty() {
                    warn!(?pointee, "related result type with protocols");
                    return;
                }

                **pointee = Self::Pointee(PointeeTy::Self_);
            } else {
                // Only fix if the type is `id`
            }
        } else {
            panic!("tried to fix related result type on non-id type")
        }
    }

    pub(crate) fn fix_fn_first_argument_cf_nullability(&mut self, fn_name: &str) {
        if let Self::Pointer {
            pointee,
            nullability: nullability @ Nullability::Unspecified,
            ..
        } = self
        {
            if let Self::Pointee(pointee_ty) = &**pointee {
                let id = match pointee_ty {
                    PointeeTy::TypeDef { id, to } if to.is_cf_type() => id,
                    PointeeTy::CFTypeDef { id, .. } => id,
                    _ => return,
                };
                let type_name = cf_no_ref(&id.name);
                // We don't ever want to mark these as non-NULL, as they have NULL
                // statics (`kCFAllocatorDefault` and `kODSessionDefault`).
                if fn_name.contains(type_name)
                    && !matches!(type_name, "ODSession" | "ODSessionRef" | "CFAllocator")
                {
                    // Is likely a getter, so let's mark it as non-null (CF will
                    // usually crash if given an unexpected NULL pointer, but
                    // we're not entirely sure it will always do so).
                    *nullability = Nullability::NonNull;
                }
            }
        }
    }

    pub(crate) fn is_self_ty_legal(&self, for_id: &ItemIdentifier) -> bool {
        match self {
            Self::Pointer {
                // Only non-NULL pointers are valid `self` types.
                // (at least until we get arbitrary self types).
                nullability: Nullability::NonNull,
                bounds: PointerBounds::Single,
                pointee,
                ..
            } => match &**pointee {
                Self::Pointee(pointee) => pointee
                    .implementable()
                    .is_some_and(|implementor| implementor.id() == for_id),
                // NOTE: Do not allow structs etc., that requires arbitrary self types.
                // NOTE: Do not recurse pointers here, that's invalid for
                // e.g. `SecKeychainCopyDefault`.
                _ => false,
            },
            Self::Enum { id, .. } | Self::Struct { id, .. } | Self::Union { id, .. } => {
                id == for_id
            }
            // Perhaps overly restrictive, but that's necessary to not
            // accidentally allow typedefs to `*mut T`.
            Self::TypeDef { id, .. } => id == for_id,
            _ => false,
        }
    }

    pub(crate) fn const_cf_string_ref() -> Self {
        Self::Pointer {
            nullability: Nullability::NonNull,
            read: true,
            written: false,
            lifetime: Lifetime::Unspecified,
            bounds: PointerBounds::Single,
            pointee: Box::new(Self::Pointee(PointeeTy::CFTypeDef {
                id: ItemIdentifier::cf_string(),
                generics: vec![],
                num_declaration_generics: 0,
                to: None,
            })),
        }
    }

    pub(crate) fn const_ns_string_ref() -> Self {
        Self::Pointer {
            nullability: Nullability::NonNull,
            read: true,
            written: false,
            lifetime: Lifetime::Unspecified,
            bounds: PointerBounds::Single,
            pointee: Box::new(Self::Pointee(PointeeTy::Class {
                id: ItemIdentifier::ns_string(),
                thread_safety: ThreadSafety::dummy(),
                superclasses: vec![],
                generics: vec![],
                declaration_generics: vec![],
                protocols: vec![],
                sendable: false,
            })),
        }
    }

    pub(crate) fn const_cf_uuid_ref() -> Self {
        Self::Pointer {
            nullability: Nullability::NonNull,
            read: true,
            written: false,
            lifetime: Lifetime::Unspecified,
            bounds: PointerBounds::Single,
            pointee: Box::new(Self::Pointee(PointeeTy::CFTypeDef {
                id: ItemIdentifier::cf_uuid(),
                generics: vec![],
                num_declaration_generics: 0,
                to: None,
            })),
        }
    }

    pub(crate) fn const_cstr_ref() -> Self {
        Self::Pointer {
            nullability: Nullability::NonNull,
            read: true,
            written: false,
            lifetime: Lifetime::Unspecified,
            bounds: PointerBounds::NullTerminated,
            pointee: Box::new(Self::Primitive(Primitive::Char)),
        }
    }

    pub(crate) fn change_nullability(&mut self, new: Nullability) {
        match self {
            Ty::Pointer { nullability, .. } | Ty::Sel { nullability, .. } => {
                if *nullability == new {
                    warn!(?nullability, ?new, "nullability already set");
                }
                *nullability = new;
            }
            ty => error!(?ty, "unexpected type for nullability attribute"),
        }
    }

    fn change_generics(&mut self, new: &[ItemGeneric]) {
        fn to_cf(generic: &ItemGeneric) -> PointeeTy {
            PointeeTy::CFTypeDef {
                id: generic.id.clone(),
                generics: generic.generics.iter().map(to_cf).collect(),
                // TODO: How would we get this information correctly?
                num_declaration_generics: generic.generics.len(),
                // TODO: How would we get this information correctly?
                to: None,
            }
        }

        match self {
            // Recurse through pointers, e.g. it's unambiguous to change the
            // generic of `**CFArray` to `**CFArray<CFString>`.
            Ty::Pointer { pointee, .. } => pointee.change_generics(new),
            Ty::Pointee(pointee) => match pointee {
                PointeeTy::CFTypeDef { generics, .. } => {
                    *generics = new.iter().map(to_cf).collect();
                }
                ty => error!(?ty, "unsupported type for generics attribute"),
            },
            ty => error!(?ty, "unexpected type for generics attribute"),
        }
    }

    pub(crate) fn apply_override(&mut self, override_: &TypeOverride) {
        if let Some(nullability) = override_.nullability {
            self.change_nullability(nullability.into());
        }
        if let Some(generics) = &override_.generics {
            self.change_generics(generics);
        }
        if override_.bounds != PointerBounds::Unspecified {
            match &mut *self {
                Ty::Pointer { bounds, .. } => {
                    if *bounds == override_.bounds {
                        warn!(?bounds, new = ?override_.bounds, "bounds already set");
                    }
                    *bounds = override_.bounds.clone();
                }
                ty => error!(?ty, "unexpected type for bounds attribute"),
            }
        }
        match &override_.lifetime {
            PointerLifetime::Unspecified => {}
            new_lifetime @ (PointerLifetime::OutPointerUnsafe
            | PointerLifetime::OutPointerRetained
            | PointerLifetime::OutPointerNotRetained) => match &mut *self {
                Ty::Pointer { pointee, .. } => match &mut **pointee {
                    Ty::Pointer { lifetime, .. } => {
                        let new_lifetime = match *new_lifetime {
                            PointerLifetime::OutPointerUnsafe => Lifetime::Unretained,
                            PointerLifetime::OutPointerRetained => Lifetime::Strong,
                            PointerLifetime::OutPointerNotRetained => Lifetime::Autoreleasing,
                            _ => unreachable!(),
                        };
                        if *lifetime == new_lifetime {
                            warn!(?lifetime, new = ?override_.lifetime, "lifetime already set");
                        }
                        *lifetime = new_lifetime;
                    }
                    ty => error!(?ty, "unexpected inner type for lifetime attribute"),
                },
                ty => error!(?ty, "unexpected type for lifetime attribute"),
            },
        }
        if let Some(new) = override_.read {
            match &mut *self {
                Ty::Pointer { read, .. } => {
                    if *read == new {
                        warn!(read, new, "read-ness already set");
                    }
                    *read = new;
                }
                ty => error!(?ty, "unexpected type for read attribute"),
            }
        }
        if let Some(new) = override_.written {
            match &mut *self {
                Ty::Pointer { written, .. } => {
                    if *written == new {
                        warn!(written, new, "write-ness already set");
                    }
                    *written = new;
                }
                ty => error!(?ty, "unexpected type for written attribute"),
            }
        }
    }

    /// Replaces `void*` with `name*`.
    ///
    /// Returns `true` if a replacement happened.
    pub(crate) fn try_replace_void_with_generic(&mut self, name: &str) {
        if let Self::Pointer { pointee, .. } = self {
            match &mut **pointee {
                this @ Self::Primitive(Primitive::Void) => {
                    *this = Self::Pointee(PointeeTy::GenericParam {
                        object_like: false,
                        name: name.to_string(),
                    });
                }
                pointee => pointee.try_replace_void_with_generic(name),
            }
        }
    }

    pub(crate) fn add_default_generics_when_matching(
        &mut self,
        matching: &ItemIdentifier,
        new_generics: impl Iterator<Item = String>,
    ) {
        match self {
            Self::Pointer { pointee, .. } => {
                pointee.add_default_generics_when_matching(matching, new_generics)
            }
            Self::Array { element_type, .. } => {
                element_type.add_default_generics_when_matching(matching, new_generics)
            }
            Self::Pointee(pointee) => match pointee {
                PointeeTy::CFTypeDef { id, generics, .. }
                    if id == matching && generics.is_empty() =>
                {
                    *generics = new_generics
                        .map(|name| PointeeTy::GenericParam {
                            object_like: false,
                            name,
                        })
                        .collect();
                }
                _ => {}
            },
            _ => {}
        }
    }

    /// Whether the type is valid.
    ///
    /// Examples:
    /// - `&NSString` is a subtype of `&NSObject`.
    /// - `&NSString` is a subtype of `Option<&NSString>`.
    /// - `&NSArray<NString>` is a subtype of `&NSArray<NSObject>`
    ///   (arrays arecovariant).
    /// - `&NSMutableArray<NString>` is _NOT_ a subtype of
    ///   `&NSMutableArray<NSObject>` (mutable arrays are invariant).
    /// - `&ProtocolObject<dyn NSApplicationDelegate>` is a subtype of
    ///   `&ProtocolObject<dyn NSObjectProtocol>`.
    pub(crate) fn is_subtype_of(&self, other: &Self) -> bool {
        /// Non-null pointers are subtypes of nullable pointers.
        fn nullability_is_subtype(this: &Nullability, other: &Nullability) -> bool {
            match (this, other) {
                (Nullability::NonNull, _) => true,
                (this, other) => this == other,
            }
        }

        // Typedefs doesn't matter for subtyping (in that sense, they are covariant).
        match (self.through_wrapper(), other.through_wrapper()) {
            (
                Self::Pointer {
                    nullability,
                    read,
                    written,
                    lifetime,
                    bounds,
                    pointee,
                },
                Self::Pointer {
                    nullability: other_nullability,
                    read: other_read,
                    written: other_written,
                    lifetime: other_lifetime,
                    bounds: other_bounds,
                    pointee: other_pointee,
                },
            ) => {
                #[allow(clippy::match_single_binding)]
                let read_is_subtype = match (read, other_read) {
                    // (true, false) => true,
                    (this, other) => this == other,
                };

                #[allow(clippy::match_single_binding)]
                let written_is_subtype = match (written, other_written) {
                    // (true, false) => true,
                    (this, other) => this == other,
                };

                let lifetime_is_subtype = match (lifetime, other_lifetime) {
                    (Lifetime::Unspecified, _) => true,
                    (this, other) => this == other,
                };

                // TODO.
                let bounds_is_subtype = match (bounds, other_bounds) {
                    (PointerBounds::Unspecified, _) => true,
                    (this, other) => this == other,
                };

                nullability_is_subtype(nullability, other_nullability)
                    && read_is_subtype
                    && written_is_subtype
                    && lifetime_is_subtype
                    && bounds_is_subtype
                    && pointee.is_subtype_of(other_pointee)
            }
            (
                Self::Sel { nullability },
                Self::Sel {
                    nullability: other_nullability,
                },
            ) => nullability_is_subtype(nullability, other_nullability),
            // Arrays are covariant.
            (
                Self::Array {
                    element_type,
                    num_elements,
                },
                Self::Array {
                    element_type: other_element_type,
                    num_elements: other_num_elements,
                },
            ) => {
                num_elements == other_num_elements && element_type.is_subtype_of(other_element_type)
            }
            // Forward to PointeeTy::is_subtype_of
            (Self::Pointee(this), Self::Pointee(other)) => this.is_subtype_of(other),
            // Everything else is invariant (for now at least).
            (this, other) => this == other,
        }
    }

    /// Whether the type could in theory affect the bounds of the receiver.
    ///
    /// This is meant to catch `NSInteger`, `NSRange`, `MTL4BufferRange`, `MTLGPUAddress` and
    /// similar constructs.
    pub(crate) fn can_affect_bounds(&self) -> bool {
        match self.through_wrapper() {
            Self::Pointer { pointee, .. } => pointee.can_affect_bounds(),
            Self::Array { element_type, .. } => element_type.can_affect_bounds(),
            Self::Primitive(prim) | Self::Simd { ty: prim, .. } => matches!(
                prim,
                // 32-bit and 64-bit integers.
                Primitive::I32
                    | Primitive::I64
                    | Primitive::Int
                    | Primitive::Long
                    | Primitive::ISize
                    | Primitive::NSInteger
                    | Primitive::U32
                    | Primitive::U64
                    | Primitive::UInt
                    | Primitive::ULong
                    | Primitive::USize
                    | Primitive::NSUInteger
                    | Primitive::PtrDiff
            ),
            Self::Struct { fields, .. } | Self::Union { fields, .. } => {
                fields.iter().any(|(_, field)| field.can_affect_bounds())
            }
            // Enumerations are intentionally not bounds-affecting (e.g. not
            // `MTLIndexType`).
            Self::Pointee(_) | Self::Enum { .. } | Self::Sel { .. } => false,
            Self::TypeDef { .. } => unreachable!("using through_typedef"),
            Self::Result {
                ty,
                err,
                original_ty,
            } => {
                ty.can_affect_bounds() || err.can_affect_bounds() || original_ty.can_affect_bounds()
            }
        }
    }

    fn into_pointee(self) -> Option<PointeeTy> {
        match self {
            Self::Pointee(pointee) => Some(pointee),
            Self::TypeDef { id, to } => to.into_pointee().map(|to| PointeeTy::TypeDef {
                id,
                to: Box::new(to),
            }),
            _ => None,
        }
    }
}

fn maybemaybeuninit(read: bool, inner: impl Display) -> impl Display {
    FormatterFn(move |f| {
        if read {
            write!(f, "{inner}")
        } else {
            write!(f, "MaybeUninit<{inner}>")
        }
    })
}

/// Strip macros from unexposed types.
///
/// These appear in newer clang versions.
/// - NS_SWIFT_NAME
/// - NS_SWIFT_UNAVAILABLE
/// - NS_REFINED_FOR_SWIFT
/// - ...
fn parse_unexposed_tokens(s: &str) -> (String, Option<UnexposedAttr>) {
    let tokens = TokenStream::from_str(s).expect("parse attributed name");
    let mut iter = tokens.into_iter().peekable();
    let attr = if let Some(TokenTree::Ident(ident)) = iter.peek() {
        let ident = ident.to_string();
        let mut has_advanced = false;
        if let Ok(attr) = UnexposedAttr::from_name(&ident, || {
            iter.next();
            has_advanced = true;
            if let Some(TokenTree::Group(_)) = iter.peek() {
                let Some(TokenTree::Group(group)) = iter.next() else {
                    unreachable!();
                };
                group.stream()
            } else {
                // The associated data on the macro is removed since Xcode 16.3.
                trace!(?ident, "expected group in macro");
                TokenStream::new()
            }
        }) {
            if !has_advanced {
                iter.next();
            }
            attr
        } else {
            None
        }
    } else {
        None
    };
    (TokenStream::from_iter(iter).to_string(), attr)
}

fn separate_with_comma_and<T: Display>(items: impl IntoIterator<Item = T> + Clone) -> impl Display {
    FormatterFn(move |f| {
        let mut iter = items.clone().into_iter().peekable();

        if let Some(item) = iter.next() {
            write!(f, "{item}")?;
        }

        while let Some(item) = iter.next() {
            if iter.peek().is_some() {
                write!(f, ", {item}")?;
            } else {
                write!(f, " and {item}")?;
            }
        }

        Ok(())
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RetainWrapper {
    Retained,
    CFRetained,
    DispatchRetained,
    NWRetained,
}

impl RetainWrapper {
    fn name(self) -> &'static str {
        match self {
            Self::Retained => "Retained",
            Self::CFRetained => "CFRetained",
            Self::DispatchRetained => "DispatchRetained",
            Self::NWRetained => "NWRetained",
        }
    }

    fn item(self) -> ItemTree {
        match self {
            Self::Retained => ItemTree::objc("Retained"),
            Self::CFRetained => ItemTree::cf("CFRetained"),
            Self::DispatchRetained => ItemTree::dispatch("DispatchRetained"),
            Self::NWRetained => ItemTree::network("NWRetained"),
        }
    }

    fn fn_return<'a>(
        self,
        nullability: Nullability,
        returns_retained: bool,
        needs_cast: bool,
        fn_call: impl Display + 'a,
    ) -> impl Display + 'a {
        FormatterFn(move |f| {
            writeln!(f, "let ret = {fn_call};")?;

            let cast = if needs_cast { ".cast()" } else { "" };

            let expect = ".expect(\"function was marked as returning non-null, but actually returned NULL\")";

            // SAFETY: The function is marked with the correct retain
            // semantics, otherwise it'd be invalid to use from Swift and
            // Obj-C with ARC too.
            if self == Self::Retained {
                if returns_retained {
                    write!(f, "unsafe {{ Retained::from_raw(ret{cast}) }}")?;
                } else {
                    write!(f, "unsafe {{ Retained::retain_autoreleased(ret{cast}) }}")?;
                }
                if nullability == Nullability::NonNull {
                    write!(f, "{expect}")?;
                }
            } else {
                if nullability == Nullability::NonNull {
                    // TODO: Avoid NULL check, and let CFRetain do that instead?
                    writeln!(f, "let ret = ret{expect};")?;
                } else {
                    write!(f, "ret.map(|ret| ")?;
                }
                // CFRetain aborts on NULL pointers, so there's not really a more
                // efficient way to do this (except if we were to use e.g.
                // `CGColorRetain`/`CVOpenGLBufferRetain`/..., but that's a huge
                // hassle).
                if returns_retained {
                    write!(f, "unsafe {{ {}::from_raw(ret{cast}) }}", self.name())?;
                } else {
                    write!(f, "unsafe {{ {}::retain(ret{cast}) }}", self.name())?;
                }
                if nullability != Nullability::NonNull {
                    write!(f, ")")?;
                }
            }

            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use core::slice;

    use super::*;

    #[test]
    fn parse_attributes() {
        let mut parser = AttributeParser::new("void (^ _Nonnull __strong)(void)", "void (^)(void)");
        assert_eq!(parser.to_check, ["_Nonnull", "__strong"]);
        parser.lifetime();
        parser.parse("_Nonnull");
        assert!(parser.to_check.is_empty());

        let mut parser =
            AttributeParser::new("NSArray<Foo> * _Nullable _Nullable", "NSArray<Foo> *");
        assert_eq!(parser.to_check, ["_Nullable", "_Nullable"]);
        assert_eq!(parser.parse("_Nullable"), 2);
        assert!(parser.to_check.is_empty());

        let mut parser = AttributeParser::new(
            "id<MTLFunctionHandle>  _Nullable const  _Nonnull __unsafe_unretained[]",
            "id<MTLFunctionHandle>  _Nullable const",
        );
        assert_eq!(
            parser.to_check,
            ["_Nonnull", "__unsafe_unretained", "[", "]"]
        );
        parser.parse("]");
        parser.parse("[");
        parser.lifetime();
        parser.parse("_Nonnull");
        assert!(parser.to_check.is_empty());

        let mut parser =
            AttributeParser::new("NSArray<Foo> * foo __attribute__((foo))", "NSArray<Foo> *");
        assert_eq!(
            parser.to_check,
            ["foo", "__attribute__", "(", "(", "foo", ")", ")"]
        );
        assert_eq!(
            parser.parse_sequence(&["__attribute__", "(", "(", "foo", ")", ")"]),
            1
        );
        assert_eq!(parser.parse_sequence(&["foo"]), 1);
        assert!(parser.to_check.is_empty());
    }

    #[test]
    fn test_parse_unexposed_tokens() {
        fn check(inp: &str, expected: &str) {
            let (actual, attr) = parse_unexposed_tokens(inp);
            assert_eq!(actual, expected);
            assert_eq!(attr, None);
        }

        check("NS_RETURNS_INNER_POINTER const char *", "const char *");
        check(
            "API_UNAVAILABLE(macos) NSString *const __strong",
            "NSString * const __strong",
        );
        check(
            "API_UNAVAILABLE NSString *const __strong",
            "NSString * const __strong",
        );
        check("NS_REFINED_FOR_SWIFT NSNumber *", "NSNumber *");
        check(
            "API_AVAILABLE(macos(10.9)) const NSProgressUserInfoKey __strong",
            "const NSProgressUserInfoKey __strong",
        );
        check(
            "NS_SWIFT_NAME(replacementIndex) const NSAttributedStringKey __strong",
            "const NSAttributedStringKey __strong",
        );
        check(
            "API_DEPRECATED(\"\", macos(10.0, 10.5)) NSString *const __strong",
            "NSString * const __strong",
        );
        check(
            "API_DEPRECATED_WITH_REPLACEMENT(\"@\\\"com.adobe.encapsulated-postscript\\\"\", macos(10.0,10.14)) NSPasteboardType __strong",
            "NSPasteboardType __strong",
        );

        let (actual, attr) = parse_unexposed_tokens("NS_SWIFT_NONISOLATED NSTextAttachment *");
        assert_eq!(actual, "NSTextAttachment *");
        assert_eq!(attr, Some(UnexposedAttr::NonIsolated));

        let (actual, attr) = parse_unexposed_tokens("NS_SWIFT_UI_ACTOR SEL");
        assert_eq!(actual, "SEL");
        assert_eq!(attr, Some(UnexposedAttr::UIActor));
    }

    #[test]
    fn test_required_items_tree() {
        let ty = Ty::TypeDef {
            id: ItemIdentifier::dummy(0),
            to: Box::new(Ty::Struct {
                id: ItemIdentifier::dummy(1),
                fields: vec![("foo".to_string(), Ty::Primitive(Primitive::Char))],
                is_bridged: false,
            }),
        };
        let required_items = [ItemTree::new(
            ItemIdentifier::dummy(0),
            [ItemTree::new(
                ItemIdentifier::dummy(1),
                Primitive::Char.required_items(),
            )],
        )];

        assert_eq!(ty.required_items().collect::<Vec<_>>(), required_items);
    }

    #[test]
    fn subtyping() {
        let nonnull = Ty::Pointer {
            nullability: Nullability::NonNull,
            read: true,
            written: true,
            lifetime: Lifetime::Unspecified,
            bounds: PointerBounds::Unspecified,
            pointee: Box::new(Ty::Primitive(Primitive::Void)),
        };
        let nullable = Ty::Pointer {
            nullability: Nullability::Nullable,
            read: true,
            written: true,
            lifetime: Lifetime::Unspecified,
            bounds: PointerBounds::Unspecified,
            pointee: Box::new(Ty::Primitive(Primitive::Void)),
        };

        // NonNull pointers are subtypes of nullable pointers.
        assert!(nonnull.is_subtype_of(&nullable));
        // The reverse isn't true.
        assert!(!nullable.is_subtype_of(&nonnull));

        fn simple_class(name: &str, superclasses: &[&str]) -> PointeeTy {
            PointeeTy::Class {
                id: ItemIdentifier::builtin(name),
                thread_safety: ThreadSafety::dummy(),
                superclasses: superclasses
                    .iter()
                    .map(|name| ItemIdentifier::builtin(*name))
                    .collect(),
                generics: vec![],
                declaration_generics: vec![],
                protocols: vec![],
                sendable: false,
            }
        }

        // `NSString` is a subtype of `NSObject`.
        assert!(
            simple_class("NSString", &["NSObject"]).is_subtype_of(&simple_class("NSObject", &[]))
        );
        // Same with `NSMutableString` and `NSString`.
        assert!(simple_class("NSMutableString", &["NSString", "NSObject"])
            .is_subtype_of(&simple_class("NSString", &["NSObject"])));
        // Not the other direction.
        assert!(
            !simple_class("NSObject", &[]).is_subtype_of(&simple_class("NSString", &["NSObject"]))
        );

        // It is also not enough to simply have a common ancestor.
        assert!(!simple_class("NSString", &["NSObject"])
            .is_subtype_of(&simple_class("NSValue", &["NSObject"])));

        fn generic(name: &str, generic: PointeeTy) -> PointeeTy {
            PointeeTy::Class {
                id: ItemIdentifier::builtin(name),
                thread_safety: ThreadSafety::dummy(),
                superclasses: vec![],
                generics: vec![generic],
                declaration_generics: vec![("ObjectType".to_string(), None)],
                protocols: vec![],
                sendable: false,
            }
        }

        // NSArray is covariant, so this should pass.
        assert!(generic("NSArray", simple_class("NSString", &["NSObject"]))
            .is_subtype_of(&generic("NSArray", simple_class("NSObject", &[]))));
        // Though not in the opposite direction.
        assert!(!generic("NSArray", simple_class("NSObject", &[]))
            .is_subtype_of(&generic("NSArray", simple_class("NSString", &[]))));

        // NSMutableArray is invariant, so this shouldn't pass.
        assert!(
            !generic("NSMutableArray", simple_class("NSString", &["NSObject"]))
                .is_subtype_of(&generic("NSMutableArray", simple_class("NSObject", &[])))
        );

        fn protocol(name: &str, super_protocols: &[ProtocolRef]) -> ProtocolRef {
            ProtocolRef {
                id: ItemIdentifier::builtin(name),
                super_protocols: super_protocols.to_vec(),
            }
        }

        let nsobject_protocol = protocol("NSObjectProtocol", &[]);

        let text_field_delegate = protocol(
            "NSTextFieldDelegate",
            &[protocol(
                "NSControlTextEditingDelegate",
                slice::from_ref(&nsobject_protocol),
            )],
        );

        let search_field_delegate = protocol(
            "NSSearchFieldDelegate",
            slice::from_ref(&text_field_delegate),
        );

        let text_field_delegate = PointeeTy::AnyObject {
            protocols: vec![(text_field_delegate, ThreadSafety::dummy())],
            sendable: false,
        };
        let search_field_delegate = PointeeTy::AnyObject {
            protocols: vec![(search_field_delegate, ThreadSafety::dummy())],
            sendable: false,
        };
        let nsobject_protocol = PointeeTy::AnyObject {
            protocols: vec![(nsobject_protocol, ThreadSafety::dummy())],
            sendable: false,
        };
        let anyobject = PointeeTy::AnyObject {
            protocols: vec![],
            sendable: false,
        };

        // Protocol objects are subtypes one way, but not the other.
        assert!(search_field_delegate.is_subtype_of(&text_field_delegate));
        assert!(!text_field_delegate.is_subtype_of(&search_field_delegate));

        // Everything is a subtype of AnyObject.
        assert!(simple_class("NSObject", &[]).is_subtype_of(&anyobject));
        assert!(search_field_delegate.is_subtype_of(&anyobject));
        assert!(nsobject_protocol.is_subtype_of(&anyobject));
    }
}
