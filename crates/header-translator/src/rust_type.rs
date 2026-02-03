use std::fmt::Display;
use std::str::FromStr;
use std::sync::LazyLock;
use std::{fmt, iter, mem};

use clang::{CallingConvention, Entity, EntityKind, Nullability, Type, TypeKind};
use proc_macro2::{TokenStream, TokenTree};

use crate::config::{ItemGeneric, PointerBounds, TypeOverride};
use crate::context::Context;
use crate::display_helper::FormatterFn;
use crate::id::{ItemIdentifier, ItemTree};
use crate::name_translation::cf_no_ref;
use crate::protocol::ProtocolRef;
use crate::stmt::{anonymous_record_name, bridged_to, parse_class_generics, GenericWithBound};
use crate::stmt::{parse_superclasses, superclasses_required_items};
use crate::thread_safety::ThreadSafety;
use crate::unexposed_attr::UnexposedAttr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ParsePosition {
    Suffix,
    Prefix,
}

impl ParsePosition {
    fn strip<'a>(self, s: &'a str, needle: &str) -> Option<&'a str> {
        match self {
            Self::Suffix => s.strip_suffix(needle),
            Self::Prefix => s.strip_prefix(needle),
        }
    }
}

/// Helper for parsing various attributes.
///
/// This is _very_ ugly, but required because libclang doesn't expose
/// lifetime information.
#[derive(Debug)]
struct AttributeParser<'a, 'b> {
    _original_name: &'a str,
    name: &'a str,
    expected_name: &'b str,
}

impl<'a, 'b> AttributeParser<'a, 'b> {
    fn new(name: &'a str, expected_name: &'b str) -> Self {
        Self {
            _original_name: name,
            name: name.trim(),
            expected_name: expected_name.trim(),
        }
    }

    fn map(&mut self, f: impl Fn(&str) -> &str) {
        self.name = f(self.name);
        self.expected_name = f(self.expected_name);
    }

    fn set_constant_array(&mut self) {
        self.map(|s| {
            let (s, _) = s.split_once('[').expect("array to contain [");
            s.trim()
        });
    }

    /// Parse an incomplete array like:
    /// `id<MTLFunctionHandle>  _Nullable const  _Nonnull __unsafe_unretained[]`
    /// By removing the ending `[]`.
    fn set_incomplete_array(&mut self) {
        self.map(|s| s.strip_suffix("[]").expect("array to end with []").trim());
    }

    /// Parse a function pointer like:
    /// `void (^ _Nonnull __strong)(...)`
    /// By extracting the inner data to:
    /// `^ _Nonnull __strong`
    fn set_fn_ptr(&mut self) {
        self.map(|s| {
            let (_, s) = s.split_once('(').expect("fn to have begin parenthesis");
            let (s, _) = s.split_once(')').expect("fn to have end parenthesis");
            s.trim()
        });
    }

    fn set_inner_pointer(&mut self) {
        if let Some(rest) = self.name.strip_suffix('*') {
            self.name = rest.trim();
        } else {
            error!(?self, "expected pointer to have star");
        }
    }
}

impl AttributeParser<'_, '_> {
    fn strip(&mut self, needle: &str, position: ParsePosition) -> bool {
        if let Some(rest) = position.strip(self.name, needle) {
            // If the string is present in the name
            if position.strip(self.expected_name, needle).is_some() {
                let rest = rest.trim();
                // If it can be stripped from both `name` and `expected_name`,
                // it might appear twice in `name`.
                //
                // This is done to support:
                // "const char * _Nonnull  _Nonnull[]".
                if position.strip(rest, needle).is_some() {
                    self.name = rest;
                    return true;
                }
            } else {
                // And _not_ in the expected name, then we should strip it so that they match.
                self.name = rest.trim();
                return true;
            }
        }

        false
    }

    fn lifetime(&mut self, position: ParsePosition) -> Lifetime {
        if self.strip("__unsafe_unretained", position) {
            Lifetime::Unretained
        } else if self.strip("__strong", position) {
            Lifetime::Strong
        } else if self.strip("__weak", position) {
            Lifetime::Weak
        } else if self.strip("__autoreleasing", position) {
            Lifetime::Autoreleasing
        } else {
            Lifetime::Unspecified
        }
    }

    /// We completely ignore `__kindof` in Rust as it is done in Swift, since
    /// it only exists to allow legacy Objective-C code to continue compiling.
    ///
    /// See <https://lapcatsoftware.com/articles/kindof.html>
    fn is_kindof(&mut self, position: ParsePosition) -> bool {
        self.strip("__kindof", position)
    }

    fn is_const(&mut self, position: ParsePosition) -> bool {
        self.strip("const", position)
    }

    fn is_volatile(&mut self, position: ParsePosition) -> bool {
        self.strip("volatile", position)
    }

    fn nullability(&mut self, position: ParsePosition) -> Option<Nullability> {
        if self.strip("_Nullable", position) {
            Some(Nullability::Nullable)
        } else if self.strip("_Nonnull", position) {
            Some(Nullability::NonNull)
        } else if self.strip("_Null_unspecified", position) {
            Some(Nullability::Unspecified)
        } else {
            None
        }
    }

    fn nullable_result(&mut self, position: ParsePosition) -> bool {
        self.strip("_Nullable_result", position)
    }

    fn sending(&mut self, position: ParsePosition) -> bool {
        self.strip("__attribute__((swift_attr(\"sending\")))", position)
    }
}

impl Drop for AttributeParser<'_, '_> {
    fn drop(&mut self) {
        if !std::thread::panicking() && self.name != self.expected_name {
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

impl Lifetime {
    fn update(&mut self, new: Self) {
        match (*self, new) {
            (_, Self::Unspecified) => {
                // No lifetime attribute parsed
            }
            (Self::Unspecified, _) => {
                *self = new;
            }
            // Temporary
            (Self::Strong, Self::Strong) => {}
            (old, new) => error!(?old, ?new, "invalid lifetime update"),
        }
    }
}

// TODO: refactor this
fn update_nullability(nullability: &mut Nullability, new: Option<Nullability>) {
    match (*nullability, new) {
        (_, None) => {
            // No nullability attribute parsed
        }
        (Nullability::Unspecified, Some(new)) => {
            *nullability = new;
        }
        (old, new) => error!(?old, ?new, "invalid nullability update"),
    }
}

fn check_nullability(ty: &Type<'_>, new: Option<Nullability>) -> Nullability {
    let on_ty = ty.get_nullability();
    if new != on_ty {
        error!(?ty, ?on_ty, ?new, "failed parsing nullability");
    }
    new.unwrap_or(Nullability::Unspecified)
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
            Self::Void => "c_void",
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
) -> (ItemIdentifier, ThreadSafety, Vec<ItemIdentifier>) {
    // @class produces a ObjCInterfaceDecl if we didn't load the actual
    // declaration, but we don't actually want that, since it'll point to the
    // wrong place.
    let entity = entity_ref
        .get_location()
        .expect("class location")
        .get_entity()
        .expect("class entity");

    let mut id = ItemIdentifier::new(&entity, context);

    match entity.get_kind() {
        EntityKind::ObjCInterfaceDecl => {
            let thread_safety = ThreadSafety::from_decl(&entity, context);
            let superclasses = parse_superclasses(&entity, context)
                .into_iter()
                .map(|(id, _, _)| id)
                .collect();

            (id, thread_safety, superclasses)
        }
        EntityKind::ObjCClassRef => {
            let thread_safety = ThreadSafety::from_ref(&entity, context);
            (id, thread_safety, vec![])
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
            (id, thread_safety, superclasses)
        }
        _ => {
            error!(?entity, "was not a class");
            (id, ThreadSafety::dummy(), vec![])
        }
    }
}

fn parse_protocol(entity: Entity<'_>, context: &Context<'_>) -> (ProtocolRef, ThreadSafety) {
    let entity = entity.get_definition().unwrap_or(entity);
    // @protocol produces a ObjCProtocolDecl if we didn't
    // load the actual declaration, but we don't actually
    // want that, since it'll point to the wrong place.
    let entity = entity
        .get_location()
        .expect("itemref location")
        .get_entity()
        .expect("itemref entity");

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
    },
    GenericParam {
        name: String,
    },
    AnyObject {
        protocols: Vec<(ProtocolRef, ThreadSafety)>,
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
    },
    CFOpaque,
    DispatchTypeDef {
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
        let ty = Ty::parse(ty, Lifetime::Unspecified, false, context);
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
                        name: "Unknown".to_string(),
                    }
                }
            },
            ty => {
                error!(?ty, "invalid generic bound");
                PointeeTy::GenericParam {
                    name: "Unknown".to_string(),
                }
            }
        }
    }

    pub(crate) fn is_plain_anyobject(&self) -> bool {
        matches!(self, Self::AnyObject { protocols } if protocols.is_empty())
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
            Self::GenericParam { name: _ } => vec![],
            Self::AnyObject { protocols } => {
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
            Self::CFTypeDef { id, generics, .. } => iter::once(ItemTree::from_id(id.clone()))
                .chain(generics.iter().flat_map(|generic| generic.required_items()))
                .collect(),
            Self::DispatchTypeDef { id } => {
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
            Self::AnyObject { protocols } => protocols
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
            Self::TypeDef { to, .. } => to.requires_mainthreadmarker(self_requires),
        }
    }

    fn provides_mainthreadmarker(&self, self_provides: bool) -> bool {
        // Important: We mostly visit the top-level types, to not include
        // optional things like `Option<&NSView>` or `&NSArray<NSView>`.
        match self {
            Self::Class { thread_safety, .. } => thread_safety.inferred_mainthreadonly(),
            Self::AnyObject { protocols } => {
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

    fn behind_pointer(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Class {
                id,
                thread_safety: _,
                superclasses: _,
                generics,
                declaration_generics: _,
                protocols: _,
            } => {
                write!(f, "{}", id.path())?;
                if !generics.is_empty() {
                    write!(f, "<")?;
                    for generic in generics {
                        write!(f, "{},", generic.behind_pointer())?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
            Self::GenericParam { name } => write!(f, "{name}"),
            Self::AnyObject { protocols } => match &**protocols {
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
                    write!(f, "{}, ", arg.plain())?;
                }
                write!(f, ")")?;
                write!(f, "{}", result_type.fn_type_return())?;
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
                        write!(f, "{},", generic.behind_pointer())?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
            Self::CFOpaque => write!(f, "Opaque"),
            Self::DispatchTypeDef { id } => write!(f, "{}", id.path()),
            Self::TypeDef { id, .. } => write!(f, "{}", id.path()),
        })
    }

    pub(crate) fn add_protocol(&mut self, entity: Entity<'_>, context: &Context<'_>) {
        let Self::AnyObject { protocols } = self else {
            error!(?self, "invalid type to add protocols to");
            return;
        };

        protocols.push(parse_protocol(entity, context));
    }

    pub(crate) fn generic_bound(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Class { .. } => {
                // HACK: Use `AsRef<Bound>`.
                //
                // This is not perfect, since `AsRef` can be implemented for
                // other reasons than subclassing. We really need a general
                // `SubclassOf` trait, but this is at least better than
                // nothing.
                write!(f, "AsRef<{}>", self.behind_pointer())
            }
            Self::AnyObject { protocols } => match &**protocols {
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
                write!(f, "/* {} */", pointee.behind_pointer())
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
                generics,
                declaration_generics,
                protocols,
                ..
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

                safety
            }
            Self::AnyObject { protocols } => match &**protocols {
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
            },
            // Generic parameters are safe.
            // TODO: NSDictionary's KeyType perhaps isn't really?
            Self::GenericParam { .. } => TypeSafety::SAFE,
            // Self types have all generics specified on the impl, so they are
            // basically `Class { generics: vec![GenericParam...] }`.
            Self::Self_ => TypeSafety::SAFE,
            Self::CFTypeDef {
                id,
                generics,
                num_declaration_generics,
            } => {
                let mut safety = if matches!(&*id.name, "CFType" | "CFTypeRef") {
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
            Self::CFTypeDef { id, .. } | Self::Class { id, .. } | Self::DispatchTypeDef { id } => {
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
                | Self::GenericParam { .. }
                | Self::AnyObject { .. }
                | Self::AnyProtocol
                | Self::AnyClass { .. }
                | Self::Self_
        )
    }

    fn is_cf_type(&self) -> bool {
        match self {
            // Recurse into typedefs
            Self::TypeDef { to, .. } => to.is_cf_type(),
            Self::CFTypeDef { .. } => true,
            _ => false,
        }
    }

    fn is_dispatch_type(&self) -> bool {
        match self {
            // Recurse into typedefs
            Self::TypeDef { to, .. } => to.is_dispatch_type(),
            Self::DispatchTypeDef { .. } => true,
            _ => false,
        }
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
                },
                Self::Class {
                    id: other_id,
                    thread_safety: _,
                    superclasses: _,
                    generics: other_generics,
                    declaration_generics: other_declaration_generics,
                    protocols: other_protocols,
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
                    const ANYOBJECT: PointeeTy = PointeeTy::AnyObject { protocols: vec![] };
                    pad_generics(generics, &ANYOBJECT, len)
                        .zip(pad_generics(other_generics, &ANYOBJECT, len))
                        .all(|(this, other)| this.is_subtype_of(other))
                } else {
                    generics == other_generics
                };

                generics_are_subtypes
                    && protocols_is_subtype_of(protocols, other_protocols)
                    && id == other_id
                    || superclasses.contains(other_id)
            }

            // All classes are subtypes of a plain `AnyObject`.
            //
            // TODO: Handle classes with `protocols` set.
            (Self::Class { .. }, Self::AnyObject { protocols }) if protocols.is_empty() => true,

            // Protocol objects are subtypes of protocol objects with fewer
            // requirements.
            (
                Self::AnyObject { protocols },
                Self::AnyObject {
                    protocols: other_protocols,
                },
            ) => protocols_is_subtype_of(protocols, other_protocols),

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
}

impl Ty {
    fn parse(
        attributed_ty: Type<'_>,
        mut lifetime: Lifetime,
        array_decays_to_pointer: bool,
        context: &Context<'_>,
    ) -> Self {
        let mut ty = attributed_ty;
        let _span = debug_span!("ty", ?ty, ?lifetime).entered();

        let mut attributed_name = attributed_ty.get_display_name();
        let mut name = ty.get_display_name();
        let mut unexposed_nullability = None;
        let mut no_escape = false;
        let mut sendable = None;

        while let TypeKind::Unexposed | TypeKind::Attributed = ty.get_kind() {
            if let TypeKind::Attributed = ty.get_kind() {
                ty = ty
                    .get_modified_type()
                    .expect("attributed type to have modified type");
                name = ty.get_display_name();
                continue;
            }

            if let Some(nullability) = ty.get_nullability() {
                if unexposed_nullability.is_some() {
                    error!("unexposed nullability already set");
                }
                unexposed_nullability = Some(nullability);
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
                Some(UnexposedAttr::ReturnsRetained) => {
                    lifetime = Lifetime::Strong;
                }
                Some(UnexposedAttr::ReturnsNotRetained) => {
                    lifetime = Lifetime::Autoreleasing;
                }
                Some(UnexposedAttr::NoEscape) => {
                    no_escape = true;
                }
                Some(UnexposedAttr::FullyUnavailable) => {
                    // Irrelevant on types.
                }
                Some(attr) => error!(?attr, "unknown attribute on type"),
                None => {}
            }

            attributed_name = new_attributed_name;
            name = new_name;

            if let Some(modified) = ty.get_modified_type() {
                ty = modified;
            } else {
                error!("expected unexposed / attributed type to have modified type");
                ty = ty.get_canonical_type();
                name = ty.get_display_name();
                break;
            }
        }

        let _span = debug_span!("ty after unexposed/attributed", ?ty).entered();

        let elaborated_ty = ty;

        if let Some(true) = ty.is_elaborated() {
            ty = ty.get_elaborated_type().expect("elaborated");
        }

        let _span = debug_span!("ty after elaborated", ?ty).entered();

        let get_is_const = |new: bool| {
            if new {
                if !attributed_ty.is_const_qualified() || ty.is_const_qualified() {
                    warn!("unnecessarily stripped const");
                }
                true
            } else {
                if attributed_ty.is_const_qualified() {
                    warn!("type was const but that could not be stripped");
                }
                // Some type kinds have `const` directly on them, instead of
                // storing it inside `Attributed`.
                //
                // TODO: Remove the need for this.
                ty.is_const_qualified()
            }
        };

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
                                    Lifetime::Unspecified,
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
                        Lifetime::Unspecified,
                        false,
                        context,
                    )),
                }
            }
            TypeKind::ObjCId => {
                let mut parser = AttributeParser::new(&attributed_name, "id");

                lifetime.update(parser.lifetime(ParsePosition::Prefix));

                let is_const = get_is_const(parser.is_const(ParsePosition::Suffix));
                lifetime.update(parser.lifetime(ParsePosition::Suffix));

                // TODO: Use _Nullable_result
                let _nullable_result = parser.nullable_result(ParsePosition::Suffix);

                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };

                Self::Pointer {
                    nullability,
                    read: true,
                    written: !is_const,
                    lifetime,
                    bounds: PointerBounds::Single,
                    pointee: Box::new(Self::Pointee(PointeeTy::AnyObject { protocols: vec![] })),
                }
            }
            TypeKind::ObjCClass => {
                let mut parser = AttributeParser::new(&attributed_name, &name);
                let lifetime = parser.lifetime(ParsePosition::Suffix);
                let nullability = unexposed_nullability
                    .or(parser.nullability(ParsePosition::Suffix))
                    .or(ty.get_nullability())
                    .unwrap_or(Nullability::Unspecified);

                Self::Pointer {
                    nullability,
                    read: true,
                    written: false,
                    lifetime,
                    bounds: PointerBounds::Single,
                    pointee: Box::new(Self::Pointee(PointeeTy::AnyClass { protocols: vec![] })),
                }
            }
            TypeKind::ObjCSel => {
                let mut parser = AttributeParser::new(&attributed_name, &name);
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };
                Self::Sel { nullability }
            }
            TypeKind::ObjCInterface => {
                let declaration = ty.get_declaration().expect("ObjCInterface declaration");
                let declaration_generics = parse_class_generics(&declaration, context);

                if !ty.get_objc_type_arguments().is_empty() {
                    panic!("generics not empty: {ty:?}");
                }
                if !ty.get_objc_protocol_declarations().is_empty() {
                    panic!("protocols not empty: {ty:?}");
                }

                if name == "Protocol" {
                    Self::Pointee(PointeeTy::AnyProtocol)
                } else {
                    let (id, thread_safety, superclasses) = get_class_data(&declaration, context);
                    if id.name != name.strip_prefix("const ").unwrap_or(&name) {
                        error!(?name, "invalid interface name");
                    }
                    Self::Pointee(PointeeTy::Class {
                        id,
                        thread_safety,
                        superclasses,
                        protocols: vec![],
                        generics: vec![],
                        declaration_generics,
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
                    .map(|param| Self::parse(param, Lifetime::Unspecified, false, context))
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

                        PointeeTy::AnyObject { protocols }
                    }
                    TypeKind::ObjCInterface => {
                        let declaration = base_ty
                            .get_declaration()
                            .expect("ObjCObject -> ObjCInterface declaration");
                        let (id, thread_safety, superclasses) =
                            get_class_data(&declaration, context);
                        let declaration_generics = parse_class_generics(&declaration, context);
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
                let mut parser = AttributeParser::new(&attributed_name, &name);
                let pointee = ty.get_pointee_type().expect("pointer to have pointee");
                if let TypeKind::FunctionPrototype | TypeKind::FunctionNoPrototype =
                    pointee.get_kind()
                {
                    parser.set_fn_ptr();
                }

                // TODO: Use this
                // (Swift's @Sendable = Send + Sync, Swift's sending = Send).
                let _sending = parser.sending(ParsePosition::Suffix);

                let is_const = ty.is_const_qualified() || pointee.is_const_qualified();
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };

                // Only top-level arrays decay to pointers.
                let pointee = Self::parse(pointee, Lifetime::Unspecified, false, context);
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
                    lifetime,
                    bounds,
                    pointee: Box::new(pointee),
                }
            }
            TypeKind::BlockPointer => {
                let mut parser = AttributeParser::new(&attributed_name, &name);
                parser.set_fn_ptr();

                let is_const = get_is_const(parser.is_const(ParsePosition::Suffix));
                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                let nullability = parser.nullability(ParsePosition::Suffix);
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, nullability)
                };

                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                match Self::parse(ty, Lifetime::Unspecified, false, context) {
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
                let mut parser = AttributeParser::new(&attributed_name, &name);
                let is_kindof = parser.is_kindof(ParsePosition::Prefix);

                let is_const = parser.is_const(ParsePosition::Suffix) || ty.is_const_qualified();
                lifetime.update(parser.lifetime(ParsePosition::Suffix));

                // TODO: Use _Nullable_result
                let _nullable_result = parser.nullable_result(ParsePosition::Suffix);

                let mut nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };

                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                drop(parser);

                let pointer_name = ty.get_display_name();
                let pointee = ty.get_pointee_type().expect("pointer type to have pointee");

                let mut ty = pointee;
                while let TypeKind::Attributed = ty.get_kind() {
                    ty = ty
                        .get_modified_type()
                        .expect("attributed type to have modified type");
                }
                let attributed_name = pointee.get_display_name();
                let name = ty.get_display_name();

                let mut parser = AttributeParser::new(&attributed_name, &name);

                let mut _is_kindof = is_kindof || parser.is_kindof(ParsePosition::Prefix);

                let pointee_is_const = parser.is_const(ParsePosition::Suffix);
                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                let new = parser.nullability(ParsePosition::Suffix);
                if new != pointee.get_nullability() {
                    error!("failed parsing nullability");
                }
                update_nullability(&mut nullability, new);
                lifetime.update(parser.lifetime(ParsePosition::Suffix));

                if !is_const && pointee_is_const {
                    warn!(?ty, "pointee was const while ObjCObjectPointer was not");
                }
                drop(parser);

                let pointee_name = ty.get_display_name();
                let mut parser = AttributeParser::new(&pointer_name, &pointee_name);

                _is_kindof = parser.is_kindof(ParsePosition::Prefix);
                lifetime.update(parser.lifetime(ParsePosition::Prefix));
                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                // Ignore const for now
                _ = parser.is_const(ParsePosition::Suffix);
                if !matches!(
                    pointee.get_objc_object_base_type().map(|ty| ty.get_kind()),
                    Some(TypeKind::ObjCId | TypeKind::ObjCClass)
                ) {
                    parser.set_inner_pointer();
                }
                drop(parser);

                // TODO: Maybe do something with the information in the elaborated type?
                if let Some(true) = ty.is_elaborated() {
                    ty = ty.get_elaborated_type().expect("elaborated");
                }

                Self::Pointer {
                    nullability,
                    read: true,
                    written: !is_const,
                    lifetime,
                    bounds: PointerBounds::Single,
                    pointee: Box::new(Self::parse(ty, lifetime, false, context)),
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
                let mut _is_kindof = parser.is_kindof(ParsePosition::Prefix);
                let mut _is_volatile = parser.is_volatile(ParsePosition::Prefix);
                let is_const1 = parser.is_const(ParsePosition::Prefix);
                lifetime.update(parser.lifetime(ParsePosition::Prefix));

                let is_const2 = parser.is_const(ParsePosition::Suffix);
                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                let nullability = unexposed_nullability
                    .or(parser.nullability(ParsePosition::Suffix))
                    .unwrap_or(Nullability::Unspecified);
                drop(parser);

                let is_const = if is_const1 || is_const2 {
                    if !attributed_ty.is_const_qualified()
                        && !elaborated_ty.is_const_qualified()
                        && !ty.is_const_qualified()
                    {
                        warn!(
                            ?attributed_ty,
                            ?elaborated_ty,
                            ?ty,
                            ?typedef_name,
                            ?is_const1,
                            ?is_const2,
                            attr = ?attributed_ty.is_const_qualified(),
                            elaborated = ?elaborated_ty.is_const_qualified(),
                            ty = ?ty.is_const_qualified(),
                            "typedef unnecessarily stripped const",
                        );
                    }
                    true
                } else {
                    if ty.is_const_qualified() {
                        warn!("typedef was const but that could not be stripped");
                    }
                    false
                };

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
                            name: typedef_name,
                        })),
                    };
                }

                let mut inner = Self::parse(inner, Lifetime::Unspecified, false, context);

                let id = ItemIdentifier::new(&declaration, context);

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

                    if pointee
                        .is_direct_cf_type(&id.name, bridged_to(&declaration, context).is_some())
                    {
                        let declaration_generics = context
                            .library(&id)
                            .typedef_data
                            .get(&id.name)
                            .map(|data| data.generics.clone())
                            .unwrap_or_default();

                        // A bit annoying that we replace the typedef name
                        // here, as that's also what determines whether the
                        // type is a CF type or not... But that's how it is
                        // currently.
                        let id = context.replace_typedef_name(id, true);
                        **pointee = Self::Pointee(PointeeTy::CFTypeDef {
                            id,
                            generics: vec![],
                            num_declaration_generics: declaration_generics.len(),
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
                            error!(
                                ?pointee,
                                "is_object_like/is_cf_type/is_os_type but not Pointee"
                            );
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

                if array_decays_to_pointer && matches!(inner.through_typedef(), Self::Array { .. })
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
                    .map(|ty| Self::parse(ty, Lifetime::Unspecified, true, context))
                    .collect();

                let result_type = ty.get_result_type().expect("fn type to have result type");
                let result_type = Self::parse(result_type, Lifetime::Unspecified, false, context);

                Self::Pointee(PointeeTy::Fn {
                    is_variadic: ty.get_kind() == TypeKind::FunctionPrototype && ty.is_variadic(),
                    no_escape,
                    arguments,
                    result_type: Box::new(result_type),
                })
            }
            TypeKind::IncompleteArray => {
                let mut parser = AttributeParser::new(&attributed_name, &name);
                parser.set_incomplete_array();

                let is_const = get_is_const(parser.is_const(ParsePosition::Suffix));
                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };

                let ty = ty
                    .get_element_type()
                    .expect("incomplete array to have element type");

                let pointee = Self::parse(ty, Lifetime::Unspecified, false, context);
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
                let mut parser = AttributeParser::new(&attributed_name, &name);
                parser.set_constant_array();
                let is_const = get_is_const(parser.is_const(ParsePosition::Suffix));
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };

                // Only top-level arrays decay to pointers. E.g.
                // int[4][2] -> int(*)[2]
                let element = ty.get_element_type().expect("array to have element type");
                let element_type = Box::new(Self::parse(element, lifetime, false, context));

                let num_elements = ty
                    .get_size()
                    .expect("constant array to have element length");

                if array_decays_to_pointer {
                    Self::Pointer {
                        nullability,
                        read: true,
                        written: !is_const,
                        lifetime,
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
                    name: "Unknown".to_string(),
                })
            }
        }
    }

    /// Recurse into typedefs (at the topmost layer).
    fn through_typedef(&self) -> &Self {
        match self {
            Self::TypeDef { to, .. } => to.through_typedef(),
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
                let reason = if *nullability == Nullability::Nullable {
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
            Self::TypeDef { to, .. } => to.safety(),
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
        if let Self::Pointer {
            nullability,
            read: _,
            written: true,
            lifetime: Lifetime::Unspecified,
            bounds: PointerBounds::Unspecified, // TODO: Is this correct?
            pointee,
        } = self
        {
            if let Self::Pointer {
                nullability: inner_nullability,
                read: _,
                written: _,
                lifetime: Lifetime::Autoreleasing,
                bounds: PointerBounds::Single,
                pointee,
            } = &**pointee
            {
                debug_assert!(self.fmt_out_pointer().is_some());

                let safety = pointee.safety();

                // Out pointers are both inputs and outputs, and thus they
                // have requirements in both directions.
                //
                // TODO: Actual usage is often for outputs only, so we might
                // be able to relax this? Maybe just allow
                // `SafetyProperty::Unknown` in arguments?
                let mut safety = safety.in_argument.merge(safety.in_return);

                if *nullability == Nullability::Unspecified
                    || *inner_nullability == Nullability::Unspecified
                {
                    safety = safety.merge(SafetyProperty::new_unknown("might not allow `None`"));
                }

                return safety.preface(format!("`{arg_name}`"));
            }
        }
        debug_assert!(self.fmt_out_pointer().is_none());

        self.safety_in_fn_argument(arg_name)
    }

    pub(crate) fn safety_in_fn_argument(&self, arg_name: &str) -> SafetyProperty {
        self.safety_in_fn()
            .in_argument
            .preface(format!("`{arg_name}`"))
    }

    pub(crate) fn safety_in_fn_return(&self) -> SafetyProperty {
        self.safety_in_fn().in_return.preface("The returned")
    }

    #[allow(dead_code)]
    fn is_primitive(&self) -> bool {
        matches!(
            self.through_typedef(),
            Self::Primitive(_) | Self::Simd { .. } | Self::Enum { .. }
        )
    }

    pub(crate) fn is_primitive_or_record(&self) -> bool {
        matches!(
            self.through_typedef(),
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
        }
    }

    /// AnyClass is safe to return as `&'static T`, since the runtime will it
    /// alive forever (and it has infinite retain count).
    ///
    /// AnyProtocol is not, though, since there's a single global object that
    /// the runtime is keeping track of, so forgetting to `release` those
    /// would leak resources.
    fn is_static_object(&self) -> bool {
        if let Self::Pointee(pointee_ty) = self.through_typedef() {
            pointee_ty.is_static_object()
        } else {
            false
        }
    }

    fn is_object_like(&self) -> bool {
        self.is_objc_type() || self.is_cf_type() || self.is_dispatch_type()
    }

    /// Determine whether the inner type of a `Pointer` is object-like.
    fn is_objc_type(&self) -> bool {
        if let Self::Pointee(pointee_ty) = self.through_typedef() {
            pointee_ty.is_objc_type()
        } else {
            false
        }
    }

    /// Determine whether the pointee inside a `Pointer` is a CF-like type.
    fn is_cf_type(&self) -> bool {
        if let Self::Pointee(pointee_ty) = self.through_typedef() {
            pointee_ty.is_cf_type()
        } else {
            false
        }
    }

    /// Determine whether the pointee inside a `Pointer` is a Dispatch-like type.
    fn is_dispatch_type(&self) -> bool {
        if let Self::Pointee(pointee_ty) = self.through_typedef() {
            pointee_ty.is_dispatch_type()
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
            _ => false,
        }
    }

    pub(crate) fn is_cf_type_ptr(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self {
            pointee.is_cf_type()
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_cf_allocator(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self {
            if let Ty::Pointee(PointeeTy::CFTypeDef { id, .. }) = &**pointee {
                if id.name == "CFAllocator" {
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
        if let Self::Pointer { pointee, .. } = self.through_typedef() {
            if let Self::Pointee(pointee) = pointee.through_typedef() {
                matches!(pointee.through_typedef(), PointeeTy::Class { id, .. } | PointeeTy::CFTypeDef { id, .. } if id.name.contains("Mutable"))
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_pointee_cstr(&self) -> bool {
        // Only check `char`; `unsigned char` or `signed char` are not
        // converted to `CStr` automatically.
        matches!(self.through_typedef(), Self::Primitive(Primitive::Char))
    }

    pub(crate) fn contains_union(&self) -> bool {
        match self {
            Self::Union { .. } => true,
            Self::TypeDef { id, .. }
                if matches!(&*id.name, "MPSPackedFloat3" | "MTLPackedFloat3") =>
            {
                // These are custom-defined to not contain the internal union.
                false
            }
            Self::TypeDef { to, .. } => to.contains_union(),
            Self::Struct { fields, .. } => fields.iter().any(|(_, field)| field.contains_union()),
            _ => false,
        }
    }

    pub(crate) fn directly_contains_fn_ptr(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self.through_typedef() {
            matches!(&**pointee, Ty::Pointee(PointeeTy::Fn { .. }))
        } else {
            false
        }
    }

    pub(crate) fn is_objc_bool(&self) -> bool {
        matches!(self.through_typedef(), Self::Primitive(Primitive::ObjcBool))
    }

    pub(crate) fn has_zero_default(&self) -> bool {
        match self.through_typedef() {
            Self::Primitive(Primitive::Void | Primitive::VaList) => false,
            Self::Primitive(_) => true,
            Self::Simd { .. } => true,
            Self::Sel {
                nullability: Nullability::Nullable,
            } => true,
            Self::Pointer {
                // Conservative, we could add `Nullability::Unspecified` here
                // too, but not emitting a `Default` impl for those makes it
                // less of a breaking change if we change the fields to be
                // `NonNull` in the future.
                nullability: Nullability::Nullable,
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
            _ => false,
        }
    }

    fn plain(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| {
            match self {
                Self::Primitive(prim) => write!(f, "{prim}"),
                Self::Pointee(pointee) => {
                    error!(?self, "must be behind pointer");
                    write!(f, "{}", pointee.behind_pointer())
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
                            write!(f, "{},", arg.plain())?;
                        }
                        if *is_variadic {
                            write!(f, "...")?;
                        }
                        write!(f, ")")?;
                        write!(f, "{}", result_type.fn_type_return())?;
                        if *nullability != Nullability::NonNull {
                            write!(f, ">")?;
                        }
                        Ok(())
                    }
                    pointee => {
                        let pointee = maybemaybeuninit(*read, pointee.behind_pointer());
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
                } => write!(f, "[{}; {num_elements}]", element_type.plain()),
                Self::Struct { id, .. } => {
                    write!(f, "{}", id.path())
                }
                Self::Union { id, .. } => {
                    write!(f, "{}", id.path())
                }
                Self::Enum { id, .. } => {
                    write!(f, "{}", id.path())
                }
            }
        })
    }

    fn behind_pointer(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Pointee(pointee) => write!(f, "{}", pointee.behind_pointer()),
            _ => write!(f, "{}", self.plain()),
        })
    }

    pub(crate) fn method_return(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            // Don't output anything here.
            Self::Primitive(Primitive::Void) => Ok(()),
            Self::Pointer {
                nullability,
                pointee,
                bounds: PointerBounds::Single,
                ..
            } if pointee.is_static_object() => {
                if *nullability == Nullability::NonNull {
                    // TODO: Add runtime nullability check here.
                    write!(f, " -> &'static {}", pointee.behind_pointer())
                } else {
                    write!(f, " -> Option<&'static {}>", pointee.behind_pointer())
                }
            }
            Self::Pointer {
                nullability,
                lifetime: _, // TODO: Use this somehow?
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if pointee.is_object_like() && !pointee.is_static_object() => {
                // NOTE: We return CF types as `Retained` for now, since we
                // don't have support for the CF wrapper in msg_send! yet.
                if *nullability == Nullability::NonNull {
                    write!(f, " -> Retained<{}>", pointee.behind_pointer())
                } else {
                    write!(f, " -> Option<Retained<{}>>", pointee.behind_pointer())
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
                    write!(f, " -> &CStr")
                } else {
                    write!(f, " -> Option<&CStr>")
                }
            }
            Self::Primitive(Primitive::C99Bool) => {
                warn!("C99's bool as Objective-C method return is ill supported");
                write!(f, " -> bool")
            }
            Self::Primitive(Primitive::ObjcBool) => write!(f, " -> bool"),
            _ => write!(f, " -> {}", self.plain()),
        })
    }

    pub fn method_return_inner_pointer(&self) -> impl fmt::Display + '_ {
        // TODO(breaking): Return " -> self.plain()" here instead.
        self.method_return()
    }

    pub(crate) fn method_return_with_error(&self) -> Option<impl fmt::Display + '_> {
        let display_closure: Box<dyn Fn(&mut fmt::Formatter<'_>) -> _> = match self {
            Self::Pointer {
                nullability: Nullability::Nullable,
                lifetime: Lifetime::Unspecified,
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if pointee.is_static_object() => {
                // NULL -> error
                Box::new(move |f| {
                    write!(
                        f,
                        " -> Result<&'static {}, Retained<{}>>",
                        pointee.behind_pointer(),
                        ItemIdentifier::nserror().path(),
                    )
                })
            }
            Self::Pointer {
                nullability: Nullability::Nullable,
                lifetime: Lifetime::Unspecified,
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if pointee.is_object_like() => {
                // NULL -> error
                Box::new(move |f| {
                    write!(
                        f,
                        " -> Result<Retained<{}>, Retained<{}>>",
                        pointee.behind_pointer(),
                        ItemIdentifier::nserror().path(),
                    )
                })
            }
            Self::Primitive(Primitive::C99Bool) | Self::Primitive(Primitive::ObjcBool) => {
                if *self == Self::Primitive(Primitive::C99Bool) {
                    warn!("C99's bool as Objective-C method return is ill supported");
                }
                // NO -> error
                Box::new(move |f| {
                    write!(
                        f,
                        " -> Result<(), Retained<{}>>",
                        ItemIdentifier::nserror().path()
                    )
                })
            }
            _ => {
                return None;
            }
        };
        Some(FormatterFn(display_closure))
    }

    pub(crate) fn method_return_encoding_type(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Primitive(Primitive::Void) => write!(f, "()"),
            Self::Primitive(Primitive::C99Bool) => write!(f, "Bool"),
            Self::Pointer { pointee, .. } if **pointee == Self::Pointee(PointeeTy::Self_) => {
                write!(f, "*mut This")
            }
            _ => write!(f, "{}", self.plain()),
        })
    }

    fn fn_type_return(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            // Don't output anything here.
            Self::Primitive(Primitive::Void) => Ok(()),
            Self::Pointer {
                nullability,
                pointee,
                bounds: PointerBounds::Single,
                ..
            } if pointee.is_static_object() => {
                if *nullability == Nullability::NonNull {
                    // TODO: Add runtime nullability check here (can we even
                    // do that?).
                    write!(f, " -> &'static {}", pointee.behind_pointer())
                } else {
                    write!(f, " -> Option<&'static {}>", pointee.behind_pointer())
                }
            }
            _ => write!(f, " -> {}", self.plain()),
        })
    }

    pub(crate) fn fn_return_required_items(&self) -> impl Iterator<Item = ItemTree> {
        let mut items: Vec<_> = self.required_items().collect();
        match self {
            Self::Pointer {
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if pointee.is_cf_type() => {
                items.push(ItemTree::cf("CFRetained"));
                items.push(ItemTree::core_ptr_nonnull());
            }
            Self::Pointer {
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if pointee.is_objc_type() && !pointee.is_static_object() => {
                items.push(ItemTree::objc("Retained"));
            }
            Self::Pointer {
                bounds: PointerBounds::Single,
                pointee,
                ..
            } if pointee.is_dispatch_type() => {
                items.push(ItemTree::dispatch("DispatchRetained"));
            }
            _ => {}
        }
        items.into_iter()
    }

    pub(crate) fn fn_return(
        &self,
        returns_retained: bool,
    ) -> (
        impl fmt::Display + '_,
        Option<(
            impl fmt::Display + '_,
            impl fmt::Display + '_,
            impl fmt::Display + '_,
        )>,
    ) {
        let start = "let ret = ";
        // SAFETY: The function is marked with the correct retain semantics,
        // otherwise it'd be invalid to use from Obj-C with ARC and Swift too.
        let end_cf = |nullability| {
            match (nullability, returns_retained) {
                // TODO: Avoid NULL check, and let CFRetain do that instead?
                (Nullability::NonNull, true) => ";\nlet ret = ret.expect(\"function was marked as returning non-null, but actually returned NULL\");\nunsafe { CFRetained::from_raw(ret) }",
                (Nullability::NonNull, false) => ";\nlet ret = ret.expect(\"function was marked as returning non-null, but actually returned NULL\");\nunsafe { CFRetained::retain(ret) }",
                // CFRetain aborts on NULL pointers, so there's not really a more
                // efficient way to do this (except if we were to use e.g.
                // `CGColorRetain`/`CVOpenGLBufferRetain`/..., but that's a huge
                // hassle).
                (_, true) => ";\nret.map(|ret| unsafe { CFRetained::from_raw(ret) })",
                (_, false) => ";\nret.map(|ret| unsafe { CFRetained::retain(ret) })",
            }
        };
        let end_dispatch = |nullability| {
            match (nullability, returns_retained) {
                (Nullability::NonNull, true) => ";\nlet ret = ret.expect(\"function was marked as returning non-null, but actually returned NULL\");\nunsafe { DispatchRetained::from_raw(ret) }",
                (Nullability::NonNull, false) => ";\nlet ret = ret.expect(\"function was marked as returning non-null, but actually returned NULL\");\nunsafe { DispatchRetained::retain(ret) }",
                (_, true) => ";\nret.map(|ret| unsafe { DispatchRetained::from_raw(ret) })",
                (_, false) => ";\nret.map(|ret| unsafe { DispatchRetained::retain(ret) })",
            }
        };
        let end_objc = |nullability| {
            match (nullability, returns_retained) {
                (Nullability::NonNull, true) => {
                    ";\nunsafe { Retained::from_raw(ret) }.expect(\"function was marked as returning non-null, but actually returned NULL\")"
                }
                (Nullability::NonNull, false) => {
                    ";\nunsafe { Retained::retain_autoreleased(ret) }.expect(\"function was marked as returning non-null, but actually returned NULL\")"
                }
                (_, true) => ";\nunsafe { Retained::from_raw(ret) }",
                (_, false) => ";\nunsafe { Retained::retain_autoreleased(ret) }",
            }
        };

        let ret = FormatterFn(move |f| match self {
            // Don't output anything here.
            Self::Primitive(Primitive::Void) => Ok(()),
            Self::Pointer {
                nullability,
                read,
                written,
                lifetime: _,
                bounds: _,
                pointee,
            } => {
                // Ignore nullability, always emit a nullable pointer. We will
                // unwrap it later in `fn_return_converter`.
                //
                // This is required because nullability attributes in Clang
                // are a hint, and not an ABI stable promise.
                if pointee.is_static_object() {
                    write!(f, "-> Option<&'static {}>", pointee.behind_pointer())
                } else if pointee.is_cf_type() || pointee.is_dispatch_type() {
                    write!(f, "-> Option<NonNull<{}>>", pointee.behind_pointer())
                } else if pointee.is_objc_type() {
                    write!(f, "-> *mut {}", pointee.behind_pointer())
                } else {
                    let pointee = maybemaybeuninit(*read, pointee.behind_pointer());
                    if *nullability == Nullability::NonNull {
                        write!(f, "-> Option<NonNull<{pointee}>>",)
                    } else if *written {
                        write!(f, " -> *mut {pointee}")
                    } else {
                        write!(f, " -> *const {pointee}",)
                    }
                }
            }
            _ => write!(f, " -> {}", self.plain()),
        });
        let converter = match self {
            _ if self.is_objc_bool() => Some((" -> bool".to_string(), "", ".as_bool()")),
            Self::TypeDef { id, .. } if matches!(&*id.name, "Boolean" | "boolean_t") => {
                Some((" -> bool".to_string(), start, ";\nret != 0"))
            }
            Self::Pointer {
                nullability,
                read,
                written,
                lifetime,
                bounds,
                pointee,
            } => {
                match lifetime {
                    Lifetime::Autoreleasing if !returns_retained => {}
                    Lifetime::Strong if returns_retained => {}
                    Lifetime::Unspecified => {}
                    _ => error!(?lifetime, returns_retained, "invalid lifetime"),
                }

                if pointee.is_static_object() && *bounds == PointerBounds::Single {
                    if *nullability == Nullability::NonNull {
                        let res = format!(" -> &'static {}", pointee.behind_pointer());
                        Some((res, start, ";\nret.expect(\"function was marked as returning non-null, but actually returned NULL\")"))
                    } else {
                        // No conversion necessary
                        None
                    }
                } else if pointee.is_cf_type() && *bounds == PointerBounds::Single {
                    let res = if *nullability == Nullability::NonNull {
                        format!(" -> CFRetained<{}>", pointee.behind_pointer())
                    } else {
                        format!(" -> Option<CFRetained<{}>>", pointee.behind_pointer())
                    };
                    Some((res, start, end_cf(*nullability)))
                } else if pointee.is_dispatch_type() && *bounds == PointerBounds::Single {
                    let res = if *nullability == Nullability::NonNull {
                        format!(" -> DispatchRetained<{}>", pointee.behind_pointer())
                    } else {
                        format!(" -> Option<DispatchRetained<{}>>", pointee.behind_pointer())
                    };
                    Some((res, start, end_dispatch(*nullability)))
                } else if pointee.is_objc_type()
                    && !pointee.is_static_object()
                    && *bounds == PointerBounds::Single
                {
                    let res = if *nullability == Nullability::NonNull {
                        format!(" -> Retained<{}>", pointee.behind_pointer())
                    } else {
                        format!(" -> Option<Retained<{}>>", pointee.behind_pointer())
                    };
                    Some((res, start, end_objc(*nullability)))
                } else if pointee.is_pointee_cstr()
                    && *bounds == PointerBounds::NullTerminated
                    && !returns_retained
                {
                    // TODO: Return `Box<CStr, std::alloc::System>` when `returns_retained`.

                    // TODO: Use `'static` here when specified.
                    if *nullability == Nullability::NonNull {
                        Some((" -> &CStr".to_string(), start, ";\nlet ret = ret.expect(\"function was marked as returning non-null, but actually returned NULL\");\nunsafe { CStr::from_ptr(ret.as_ptr()) }"))
                    } else {
                        Some((
                            " -> Option<&CStr>".to_string(),
                            start,
                            if *written {
                                ";\nNonNull::new(ret).map(|ret| unsafe { CStr::from_ptr(ret.as_ptr()) })"
                            } else {
                                ";\nNonNull::new(ret.cast_mut()).map(|ret| unsafe { CStr::from_ptr(ret.as_ptr()) })"
                            },
                        ))
                    }
                } else {
                    let pointee = maybemaybeuninit(*read, pointee.behind_pointer());
                    if *nullability == Nullability::NonNull {
                        let res = format!(" -> NonNull<{pointee}>");
                        Some((res, start, ";\nret.expect(\"function was marked as returning non-null, but actually returned NULL\")"))
                    } else {
                        None
                    }
                }
            }
            _ => None,
        };
        (ret, converter)
    }

    pub(crate) fn var(&self) -> impl fmt::Display + '_ {
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
                let pointee = maybemaybeuninit(*read, pointee.behind_pointer());
                if *nullability == Nullability::NonNull {
                    write!(f, "&'static {pointee}")
                } else {
                    write!(f, "Option<&'static {pointee}>")
                }
            }
            _ => write!(f, "{}", self.behind_pointer()),
        })
    }

    pub(crate) fn const_(&self) -> impl fmt::Display + '_ {
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
                let pointee = maybemaybeuninit(*read, pointee.behind_pointer());
                if *nullability == Nullability::NonNull {
                    write!(f, "&{pointee}")
                } else {
                    write!(f, "Option<&{pointee}>")
                }
            }
            Self::Pointer {
                nullability,
                read: _,
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
            _ => write!(f, "{}", self.plain()),
        })
    }

    pub(crate) fn typedef(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Pointer {
                nullability: _,
                read: _,
                written: _,
                lifetime: _,
                bounds: _,
                pointee,
            } if pointee.is_object_like() => {
                write!(f, "{}", pointee.behind_pointer())
            }
            // We mark `typedefs` as-if behind a pointer, as even though
            // typedefs are _usually_ to a pointer of the type (handled
            // above), we sometimes have typedefs to the inner type as well.
            //
            // Examples:
            // typedef NSDictionary<NSString *, MPSGraphExecutable *> MPSGraphCallableMap;
            // typedef void NSUncaughtExceptionHandler(NSException *exception);
            _ => write!(f, "{}", self.behind_pointer()),
        })
    }

    pub(crate) fn fn_argument(&self) -> impl fmt::Display + '_ {
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

                let inner = maybemaybeuninit(*read, pointee.behind_pointer());
                // We don't care if `PointeeTy` pointers may be written to,
                // since those use interior mutability anyhow.
                let mut_ = if !written || matches!(pointee.through_typedef(), Self::Pointee(_)) {
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
            _ => write!(f, "{}", self.plain()),
        })
    }

    pub(crate) fn fn_argument_converter(
        &self,
    ) -> Option<(
        impl fmt::Display + '_,
        impl fmt::Display + '_,
        impl fmt::Display + '_,
    )> {
        match self {
            _ if self.is_objc_bool() => Some(("bool", "Bool::new(", ")")),
            Self::TypeDef { id, .. } if matches!(&*id.name, "Boolean" | "boolean_t") => {
                Some(("bool", "", " as _"))
            }
            Self::Pointer {
                nullability,
                read: _,
                // Map both `const` and non-`const` to `&CStr`.
                written,
                lifetime: Lifetime::Unspecified, // TODO
                bounds: PointerBounds::NullTerminated,
                pointee,
            } if pointee.is_pointee_cstr() => {
                if *nullability == Nullability::NonNull {
                    Some(("&CStr", "NonNull::new(", ".as_ptr().cast_mut()).unwrap()"))
                } else {
                    Some((
                        "Option<&CStr>",
                        "",
                        if *written {
                            ".map(|ptr| ptr.as_ptr().cast_mut()).unwrap_or_else(core::ptr::null_mut)"
                        } else {
                            ".map(|ptr| ptr.as_ptr()).unwrap_or_else(core::ptr::null)"
                        },
                    ))
                }
            }
            // TODO: Support out / autoreleasing pointers?
            _ => None,
        }
    }

    fn fmt_out_pointer(&self) -> Option<impl fmt::Display + '_> {
        match self {
            Self::Pointer {
                nullability,
                read,
                written: true,
                lifetime: Lifetime::Unspecified,
                bounds: PointerBounds::Unspecified, // TODO: Is this correct?
                pointee,
            } => match &**pointee {
                Self::Pointer {
                    nullability: inner_nullability,
                    // Don't care about the const-ness of the id.
                    read: _,
                    written: _,
                    lifetime: Lifetime::Autoreleasing,
                    bounds: PointerBounds::Single,
                    pointee,
                } => Some(FormatterFn(move |f| {
                    if !read {
                        // We don't (yet) support `&mut MaybeUninit<Retained<T>>`.
                        error!("out pointers must currently be readable");
                    }
                    let inner = FormatterFn(|f| {
                        if *inner_nullability == Nullability::NonNull {
                            write!(f, "Retained<{}>", pointee.behind_pointer())
                        } else {
                            write!(f, "Option<Retained<{}>>", pointee.behind_pointer())
                        }
                    });
                    if *nullability == Nullability::NonNull {
                        write!(f, "&mut {inner}")
                    } else {
                        write!(f, "Option<&mut {inner}>")
                    }
                })),
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
            _ => {
                if let Some(fmt) = self.fmt_out_pointer() {
                    write!(f, "{fmt}")
                } else {
                    write!(f, "{}", self.fn_argument())
                }
            }
        })
    }

    pub(crate) fn method_argument_encoding_type(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Primitive(Primitive::C99Bool) => write!(f, "Bool"),
            _ => write!(f, "{}", self.plain()),
        })
    }

    pub(crate) fn record(&self) -> impl fmt::Display + '_ {
        self.plain()
    }

    fn fn_contains_bool(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self.through_typedef() {
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
        FormatterFn(move |f| write!(f, "{}", self.plain()))
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

    pub(crate) const VOID_RESULT: Self = Self::Primitive(Primitive::Void);

    pub(crate) fn parse_method_argument(
        ty: Type<'_>,
        _qualifier: Option<MethodArgumentQualifier>,
        mut arg_sendable: Option<bool>,
        mut arg_no_escape: bool,
        context: &Context<'_>,
    ) -> Self {
        let mut ty = Self::parse(ty, Lifetime::Unspecified, true, context);

        match &mut ty {
            Self::Pointer { pointee, .. } => match &mut **pointee {
                Self::Pointee(PointeeTy::Block {
                    sendable,
                    no_escape,
                    ..
                }) => {
                    if let Some(arg_sendable) = arg_sendable.take() {
                        *sendable = Some(arg_sendable);
                    }
                    *no_escape = arg_no_escape;
                    arg_no_escape = false;
                }
                Self::Pointee(PointeeTy::Fn { no_escape, .. }) => {
                    *no_escape = arg_no_escape;
                    arg_no_escape = false;
                }
                _ => {}
            },
            // Ignore typedefs for now
            Self::TypeDef { .. } => {
                arg_sendable = None;
                arg_no_escape = false;
            }
            _ => {}
        }

        if arg_sendable.is_some() {
            warn!(?ty, "did not consume sendable in argument");
        }

        if arg_no_escape {
            warn!(?ty, "did not consume no_escape in argument");
        }

        // TODO: Is the qualifier useful for anything?

        ty
    }

    pub(crate) fn parse_method_return(
        ty: Type<'_>,
        default_nonnull: bool,
        context: &Context<'_>,
    ) -> Self {
        let mut ty = Self::parse(ty, Lifetime::Unspecified, false, context);

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

    pub(crate) fn parse_function_argument(
        ty: Type<'_>,
        attr: Option<UnexposedAttr>,
        context: &Context<'_>,
    ) -> Self {
        match attr {
            Some(UnexposedAttr::NoEscape) => {
                // TODO: Use this if mapping `fn + context ptr` to closure.
            }
            Some(UnexposedAttr::ReturnsRetained | UnexposedAttr::ReturnsNotRetained) => {
                // TODO: Massage this into a lifetime
            }
            Some(attr) => {
                error!(?attr, "unknown attribute in function argument");
            }
            None => {}
        }
        Self::parse_method_argument(ty, None, None, false, context)
    }

    pub(crate) fn parse_function_return(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse_method_return(ty, false, context)
    }

    pub(crate) fn parse_typedef(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse(ty, Lifetime::Unspecified, false, context)
    }

    pub(crate) fn pointer_to_opaque_struct_or_void(
        &self,
        typedef_name: &str,
        typedef_is_bridged: bool,
    ) -> Option<(bool, Option<&str>)> {
        if let Self::Pointer {
            pointee,
            read: _,
            written: _, // const-ness doesn't matter when defining the type
            nullability,
            bounds: _,
            lifetime,
        } = self
        {
            let is_cf = pointee.is_direct_cf_type(typedef_name, typedef_is_bridged);
            if let Self::Struct { id, fields, .. } = &**pointee {
                if fields.is_empty() {
                    // Extra checks to ensure we don't loose information
                    if *nullability != Nullability::Unspecified {
                        error!(?id, ?nullability, "opaque pointer had nullability");
                    }
                    if *lifetime != Lifetime::Unspecified {
                        error!(?id, ?lifetime, "opaque pointer had lifetime");
                    }

                    return Some((is_cf, Some(&id.name)));
                }
            }
            if let Self::Primitive(Primitive::Void) = &**pointee {
                return Some((is_cf, None));
            }
        }
        None
    }

    pub(crate) fn parse_property_getter(
        ty: Type<'_>,
        is_copy: bool,
        _sendable: Option<bool>,
        context: &Context<'_>,
    ) -> Self {
        let mut ty = Self::parse(ty, Lifetime::Unspecified, false, context);

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
        let mut ty = Self::parse(ty, Lifetime::Unspecified, true, context);

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
        Self::parse(ty, Lifetime::Unspecified, false, context)
    }

    pub fn is_signed(&self) -> Option<bool> {
        match self.through_typedef() {
            Self::Primitive(prim) => prim.is_signed(),
            Self::Simd { ty, .. } => ty.is_signed(),
            Self::Enum { ty, .. } => ty.is_signed(),
            _ => None,
        }
    }

    pub(crate) fn parse_enum(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse(ty, Lifetime::Unspecified, false, context)
    }

    pub(crate) fn is_simple_uint(&self) -> bool {
        matches!(self, Self::Primitive(Primitive::UInt))
    }

    pub(crate) fn parse_static(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse(ty, Lifetime::Unspecified, false, context)
    }

    pub(crate) fn argument_is_error_out(&self) -> bool {
        if let Self::Pointer {
            // We always pass a place to write the error information,
            // so doesn't matter whether it's optional or not.
            // TODO(breaking): Allow Unspecified here too.
            nullability: Nullability::NonNull | Nullability::Nullable,
            read: _,
            written,
            lifetime: Lifetime::Unspecified,
            bounds: PointerBounds::Unspecified, // TODO.
            pointee,
        } = self
        {
            if let Self::Pointer {
                nullability: inner_nullability,
                read: inner_read,
                written: inner_written,
                lifetime,
                bounds: PointerBounds::Single,
                pointee,
            } = &**pointee
            {
                if let Self::Pointee(PointeeTy::Class {
                    id,
                    generics,
                    protocols,
                    ..
                }) = &**pointee
                {
                    if !id.is_nserror() {
                        return false;
                    }
                    assert!(*written, "expected error written {self:?}");
                    assert_ne!(
                        *inner_nullability,
                        Nullability::NonNull,
                        "invalid inner error nullability {self:?}"
                    );
                    assert!(*inner_read, "expected inner error read {self:?}");
                    assert!(*inner_written, "expected inner error written {self:?}");

                    assert_eq!(generics, &[], "invalid error generics {self:?}");
                    assert_eq!(protocols, &[], "invalid error protocols {self:?}");
                    assert_eq!(
                        *lifetime,
                        Lifetime::Autoreleasing,
                        "invalid error lifetime {self:?}"
                    );
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn is_retainable(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self {
            pointee.is_object_like() && !pointee.is_static_object()
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

    pub(crate) fn is_enum_through_typedef(&self) -> bool {
        matches!(self.through_typedef(), Self::Enum { .. })
    }

    pub(crate) fn is_floating_through_typedef(&self) -> bool {
        matches!(
            self.through_typedef(),
            Self::Primitive(Primitive::F32 | Primitive::F64 | Primitive::Float | Primitive::Double)
        )
    }

    /// SIMD is not yet possible in FFI, see:
    /// <https://github.com/rust-lang/rust/issues/63068>
    pub(crate) fn needs_simd(&self) -> bool {
        match self.through_typedef() {
            Self::Simd { .. } => true,
            Self::Pointer { pointee, .. } => pointee.needs_simd(),
            Self::Array { element_type, .. } => element_type.needs_simd(),
            Self::Struct { fields, .. } | Self::Union { fields, .. } => {
                fields.iter().any(|(_, field)| field.needs_simd())
            }
            Self::Pointee(
                PointeeTy::Fn {
                    result_type,
                    arguments,
                    ..
                }
                | PointeeTy::Block {
                    result_type,
                    arguments,
                    ..
                },
            ) => result_type.needs_simd() || arguments.iter().any(|arg| arg.needs_simd()),
            _ => false,
        }
    }

    pub(crate) fn try_fix_related_result_type(&mut self) {
        if let Self::Pointer { pointee, .. } = self {
            if let Self::Pointee(PointeeTy::AnyObject { protocols }) = &**pointee {
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
                if fn_name.contains(type_name) && !matches!(type_name, "ODSession" | "CFAllocator")
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
        match (self.through_typedef(), other.through_typedef()) {
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
        match self.through_typedef() {
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

#[cfg(test)]
mod tests {
    use core::slice;

    use super::*;

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
        };
        let search_field_delegate = PointeeTy::AnyObject {
            protocols: vec![(search_field_delegate, ThreadSafety::dummy())],
        };
        let nsobject_protocol = PointeeTy::AnyObject {
            protocols: vec![(nsobject_protocol, ThreadSafety::dummy())],
        };
        let anyobject = PointeeTy::AnyObject { protocols: vec![] };

        // Protocol objects are subtypes one way, but not the other.
        assert!(search_field_delegate.is_subtype_of(&text_field_delegate));
        assert!(!text_field_delegate.is_subtype_of(&search_field_delegate));

        // Everything is a subtype of AnyObject.
        assert!(simple_class("NSObject", &[]).is_subtype_of(&anyobject));
        assert!(search_field_delegate.is_subtype_of(&anyobject));
        assert!(nsobject_protocol.is_subtype_of(&anyobject));
    }
}
