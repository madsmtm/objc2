use std::str::FromStr;
use std::sync::LazyLock;
use std::{fmt, iter, mem};

use clang::{CallingConvention, Entity, EntityKind, Nullability, Type, TypeKind};
use proc_macro2::{TokenStream, TokenTree};

use crate::context::Context;
use crate::display_helper::FormatterFn;
use crate::id::{ItemIdentifier, ItemTree};
use crate::name_translation::cf_no_ref;
use crate::protocol::ProtocolRef;
use crate::stmt::{anonymous_record_name, bridged_to};
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

/// Types that are only valid behind pointers.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PointeeTy {
    Class {
        id: ItemIdentifier,
        thread_safety: ThreadSafety,
        superclasses: Vec<ItemIdentifier>,
        generics: Vec<Ty>,
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
    },
    DispatchTypeDef {
        id: ItemIdentifier,
    },
    TypeDef {
        id: ItemIdentifier,
        to: Box<PointeeTy>,
    },
    CStr,
}

impl PointeeTy {
    fn required_items(&self) -> impl Iterator<Item = ItemTree> {
        match self {
            Self::Class {
                id,
                thread_safety: _,
                superclasses,
                generics,
                protocols,
            } => {
                let superclasses = superclasses_required_items(superclasses.iter().cloned());
                let protocols = protocols
                    .iter()
                    .flat_map(|(protocol, _)| protocol.required_items());
                iter::once(ItemTree::new(id.clone(), superclasses))
                    .chain(protocols)
                    .chain(generics.iter().flat_map(|generic| generic.required_items()))
                    .collect()
            }
            Self::GenericParam { .. } => vec![],
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
            Self::CFTypeDef { id } | Self::DispatchTypeDef { id } => {
                vec![ItemTree::from_id(id.clone())]
            }
            Self::TypeDef { id, to } => {
                vec![ItemTree::new(id.clone(), to.required_items())]
            }
            Self::CStr => vec![ItemTree::core_ffi("CStr")],
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
            Self::CFTypeDef { .. } | Self::DispatchTypeDef { .. } => false,
            Self::TypeDef { to, .. } => to.requires_mainthreadmarker(self_requires),
            Self::CStr => false,
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

    pub(crate) fn implementable(&self) -> Option<ItemTree> {
        match self {
            Self::CFTypeDef { id } | Self::Class { id, .. } | Self::DispatchTypeDef { id } => {
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
        match self {
            // Recurse into typedefs
            Self::TypeDef { to, .. } => to.is_static_object(),
            Self::AnyClass { .. } => true,
            _ => false,
        }
    }

    fn is_objc_type(&self) -> bool {
        matches!(
            self,
            Self::Class { .. }
                | Self::GenericParam { .. }
                | Self::AnyObject { .. }
                | Self::AnyProtocol
                | Self::AnyClass { .. }
                | Self::Self_
                | Self::TypeDef { .. }
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
        is_const: bool,
        lifetime: Lifetime,
        pointee: Box<Self>,
    },
    TypeDef {
        id: ItemIdentifier,
        to: Box<Self>,
    },
    IncompleteArray {
        nullability: Nullability,
        is_const: bool,
        pointee: Box<Self>,
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
        fields: Vec<Ty>,
        /// Whether the struct's declaration has a bridge attribute.
        is_bridged: bool,
    },
    Union {
        id: ItemIdentifier,
        /// FIXME: This does not work for recursive structs.
        fields: Vec<Ty>,
    },
}

impl Ty {
    fn parse(attributed_ty: Type<'_>, mut lifetime: Lifetime, context: &Context<'_>) -> Self {
        let mut ty = attributed_ty;
        let _span = debug_span!("ty", ?ty, ?lifetime).entered();

        let mut attributed_name = attributed_ty.get_display_name();
        let mut name = ty.get_display_name();
        let mut unexposed_nullability = None;
        let mut no_escape = false;

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
                Some(
                    UnexposedAttr::NonIsolated
                    | UnexposedAttr::UIActor
                    | UnexposedAttr::Sendable
                    | UnexposedAttr::NonSendable,
                ) => {
                    // Ignored for now; these are usually also emitted on the method/property,
                    // which is where they will be useful in any case.
                }
                Some(UnexposedAttr::ReturnsRetained) => {
                    lifetime = Lifetime::Strong;
                }
                Some(UnexposedAttr::ReturnsNotRetained) => {
                    lifetime = Lifetime::Autoreleasing;
                }
                Some(UnexposedAttr::NoEscape) => {
                    // TODO: Use this on Pointer and BlockPointer
                    no_escape = true;
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
                    vec![Self::Pointee(PointeeTy::Self_)]
                } else {
                    ty.get_fields()
                        .expect("struct fields")
                        .into_iter()
                        .map(|field| {
                            Self::parse(
                                field.get_type().expect("struct field type"),
                                Lifetime::Unspecified,
                                context,
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
                    is_const,
                    lifetime,
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
                    is_const: true,
                    lifetime,
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
                    .map(|param| Self::parse(param, Lifetime::Unspecified, context))
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
                        if id.name != name {
                            error!(?name, "ObjCObject -> ObjCInterface invalid name");
                        }

                        if !generics.is_empty() && !protocols.is_empty() {
                            panic!("got object with both protocols and generics: {name:?}, {protocols:?}, {generics:?}");
                        }

                        if generics.is_empty() && protocols.is_empty() {
                            panic!("got object with empty protocols and generics: {name:?}");
                        }

                        PointeeTy::Class {
                            id,
                            thread_safety,
                            superclasses,
                            generics,
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

                let is_const = ty.is_const_qualified() || pointee.is_const_qualified();
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };

                let pointee = Self::parse(pointee, Lifetime::Unspecified, context);
                Self::Pointer {
                    nullability,
                    is_const,
                    lifetime,
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
                match Self::parse(ty, Lifetime::Unspecified, context) {
                    Self::Pointee(PointeeTy::Fn {
                        is_variadic: false,
                        no_escape,
                        arguments,
                        result_type,
                    }) => Self::Pointer {
                        nullability,
                        is_const,
                        lifetime,
                        pointee: Box::new(Self::Pointee(PointeeTy::Block {
                            sendable: None,
                            no_escape,
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
                    is_const,
                    lifetime,
                    pointee: Box::new(Self::parse(ty, lifetime, context)),
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
                            is_const,
                            lifetime,
                            pointee: Box::new(Self::Pointee(PointeeTy::Self_)),
                        }
                    }

                    // Emit `dispatch_object_t` as a raw pointer (at least for now,
                    // since we currently treat `DispatchObject` as a trait).
                    "dispatch_object_t" => {
                        return Self::Pointer {
                            nullability,
                            is_const,
                            lifetime,
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
                            is_const,
                            lifetime,
                            pointee,
                        };
                    }

                    _ => {}
                }

                if let EntityKind::TemplateTypeParameter = declaration.get_kind() {
                    return Self::Pointer {
                        nullability,
                        is_const,
                        lifetime,
                        pointee: Box::new(Self::Pointee(PointeeTy::GenericParam {
                            name: typedef_name,
                        })),
                    };
                }

                let mut inner = Self::parse(inner, Lifetime::Unspecified, context);

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
                    is_const: inner_is_const,
                    lifetime: inner_lifetime,
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
                        *inner_is_const = is_const;
                    }
                    if lifetime != Lifetime::Unspecified {
                        *inner_lifetime = lifetime;
                    }

                    if pointee
                        .is_direct_cf_type(&id.name, bridged_to(&declaration, context).is_some())
                    {
                        // A bit annoying that we replace the typedef name
                        // here, as that's also what determines whether the
                        // type is a CF type or not... But that's how it is
                        // currently.
                        let id = context.replace_typedef_name(id, true);
                        *pointee = Box::new(Self::Pointee(PointeeTy::CFTypeDef { id }));
                        return inner;
                    } else if pointee.is_object_like() {
                        if let Self::Pointee(pointee_ty) = &mut **pointee {
                            let id = context.replace_typedef_name(id, pointee_ty.is_cf_type());
                            // Replace with a dummy type (will be re-replaced
                            // on the line below).
                            let to = Box::new(mem::replace(pointee_ty, PointeeTy::Self_));
                            *pointee = Box::new(Self::Pointee(PointeeTy::TypeDef { id, to }));
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

                Self::TypeDef {
                    id,
                    to: Box::new(inner),
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
                    .map(|ty| Self::parse(ty, Lifetime::Unspecified, context))
                    .collect();

                let result_type = ty.get_result_type().expect("fn type to have result type");
                let result_type = Self::parse(result_type, Lifetime::Unspecified, context);

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

                let pointee = Self::parse(ty, lifetime, context);
                Self::IncompleteArray {
                    nullability,
                    is_const,
                    pointee: Box::new(pointee),
                }
            }
            TypeKind::ConstantArray => {
                let mut parser = AttributeParser::new(&attributed_name, &name);
                parser.set_constant_array();
                let _is_const = get_is_const(parser.is_const(ParsePosition::Suffix));
                let _nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };

                let element = ty.get_element_type().expect("array to have element type");
                let element_type = Self::parse(element, lifetime, context);
                let num_elements = ty
                    .get_size()
                    .expect("constant array to have element length");
                Self::Array {
                    element_type: Box::new(element_type),
                    num_elements,
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
                pointee,
                nullability,
                ..
            }
            | Self::IncompleteArray {
                pointee,
                nullability,
                ..
            } => pointee
                .required_items()
                .chain((*nullability == Nullability::NonNull).then(ItemTree::core_ptr_nonnull))
                .collect(),
            Self::TypeDef { id, to, .. } => vec![ItemTree::new(id.clone(), to.required_items())],
            Self::Array { element_type, .. } => element_type.required_items().collect(),
            Self::Enum { id, ty } => {
                vec![ItemTree::new(id.clone(), ty.required_items())]
            }
            Self::Struct { id, fields, .. } | Self::Union { id, fields, .. } => {
                let fields = fields.iter().flat_map(|field| field.required_items());
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
            Self::IncompleteArray { pointee, .. } => {
                pointee.requires_mainthreadmarker(self_requires)
            }
            Self::TypeDef { to, .. } => to.requires_mainthreadmarker(self_requires),
            Self::Array { element_type, .. } => {
                element_type.requires_mainthreadmarker(self_requires)
            }
            Self::Enum { ty, .. } => ty.requires_mainthreadmarker(self_requires),
            Self::Struct { fields, .. } | Self::Union { fields, .. } => fields
                .iter()
                .any(|field| field.requires_mainthreadmarker(self_requires)),
        }
    }

    /// Whether this type can provide a MainThreadMarker.
    pub(crate) fn provides_mainthreadmarker(&self, self_provides: bool) -> bool {
        // Important: We mostly visit the top-level types, to not include
        // optional things like `Option<&NSView>` or `&NSArray<NSView>`.
        match self {
            Self::Pointee(pointee_ty) => pointee_ty.provides_mainthreadmarker(self_provides),
            Self::Pointer {
                // Only visit non-null pointers
                nullability: Nullability::NonNull,
                pointee,
                ..
            } => pointee.provides_mainthreadmarker(self_provides),
            Self::TypeDef { to, .. } => to.provides_mainthreadmarker(self_provides),
            Self::Enum { ty, .. } => ty.provides_mainthreadmarker(self_provides),
            Self::Struct { fields, .. } => fields
                .iter()
                .any(|field| field.provides_mainthreadmarker(self_provides)),
            Self::Union { fields, .. } => fields
                .iter()
                .all(|field| field.provides_mainthreadmarker(self_provides)),
            _ => false,
        }
    }

    /// Return the `ItemTree` for the nearest implement-able type, if any.
    pub(crate) fn implementable(&self) -> Option<ItemTree> {
        match self {
            Self::Primitive(_) | Self::Simd { .. } | Self::Sel { .. } => None,
            Self::Pointee(pointee) => pointee.implementable(),
            Self::Pointer { pointee, .. }
            | Self::IncompleteArray { pointee, .. }
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
                let fields = fields.iter().flat_map(|field| field.required_items());
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
        match self {
            // Recurse into typedefs
            Self::TypeDef { to, .. } => to.is_static_object(),
            Self::Pointee(pointee_ty) => pointee_ty.is_static_object(),
            _ => false,
        }
    }

    fn is_object_like(&self) -> bool {
        self.is_objc_type() || self.is_cf_type() || self.is_dispatch_type()
    }

    /// Determine whether the inner type of a `Pointer` is object-like.
    fn is_objc_type(&self) -> bool {
        match self {
            // Recurse into typedefs
            Self::TypeDef { to, .. } => to.is_objc_type(),
            Self::Pointee(pointee_ty) => pointee_ty.is_objc_type(),
            _ => false,
        }
    }

    /// Determine whether the pointee inside a `Pointer` is a CF-like type.
    fn is_cf_type(&self) -> bool {
        match self {
            // Recurse into typedefs
            Self::TypeDef { to, .. } => to.is_cf_type(),
            Self::Pointee(pointee_ty) => pointee_ty.is_cf_type(),
            _ => false,
        }
    }

    /// Determine whether the pointee inside a `Pointer` is a Dispatch-like type.
    fn is_dispatch_type(&self) -> bool {
        match self {
            // Recurse into typedefs
            Self::TypeDef { to, .. } => to.is_dispatch_type(),
            Self::Pointee(pointee_ty) => pointee_ty.is_dispatch_type(),
            _ => false,
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
            if let Ty::Pointee(PointeeTy::CFTypeDef { id }) = &**pointee {
                if id.name == "CFAllocator" {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn is_object_like_ptr(&self) -> bool {
        if let Self::Pointer { pointee, .. } = self {
            pointee.is_objc_type()
        } else {
            false
        }
    }

    pub(crate) fn is_cf_type_id(&self) -> bool {
        matches!(self, Self::TypeDef { id, .. } if id.name == "CFTypeID")
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
            Self::Struct { fields, .. } => fields.iter().any(|field| field.contains_union()),
            _ => false,
        }
    }

    pub(crate) fn directly_contains_fn_ptr(&self) -> bool {
        match self {
            Self::Pointer { pointee, .. }
                if matches!(&**pointee, Ty::Pointee(PointeeTy::Fn { .. })) =>
            {
                true
            }
            Self::TypeDef { to, .. } => to.directly_contains_fn_ptr(),
            _ => false,
        }
    }

    pub(crate) fn is_objc_bool(&self) -> bool {
        match self {
            Self::Primitive(Primitive::ObjcBool) => true,
            Self::TypeDef { to, .. } => to.is_objc_bool(),
            _ => false,
        }
    }

    fn plain(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| {
            match self {
                Self::Primitive(prim) => write!(f, "{prim}"),
                Self::Pointee(_) => {
                    error!(?self, "must be behind pointer");
                    write!(f, "{}", self.behind_pointer())
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
                    is_const,
                    // Ignore
                    lifetime: _,
                    pointee,
                } => match &**pointee {
                    Self::Pointee(PointeeTy::Fn {
                        is_variadic,
                        no_escape: _,
                        arguments,
                        result_type,
                    }) => {
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
                        if *nullability == Nullability::NonNull {
                            write!(f, "NonNull<{}>", pointee.behind_pointer())
                        } else if *is_const {
                            write!(f, "*const {}", pointee.behind_pointer())
                        } else {
                            write!(f, "*mut {}", pointee.behind_pointer())
                        }
                    }
                },
                Self::TypeDef { id, .. } => {
                    write!(f, "{}", id.path())
                }
                Self::IncompleteArray {
                    nullability,
                    is_const,
                    pointee,
                } => {
                    if *nullability == Nullability::NonNull {
                        write!(f, "NonNull<{}>", pointee.behind_pointer())
                    } else if *is_const {
                        write!(f, "*const {}", pointee.behind_pointer())
                    } else {
                        write!(f, "*mut {}", pointee.behind_pointer())
                    }
                }
                Self::Array {
                    element_type,
                    num_elements,
                } => write!(
                    f,
                    "ArrayUnknownABI<[{}; {num_elements}]>",
                    element_type.plain()
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
            }
        })
    }

    fn behind_pointer(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Pointee(pointee) => match pointee {
                PointeeTy::Class {
                    id,
                    thread_safety: _,
                    superclasses: _,
                    generics,
                    protocols: _,
                } => {
                    write!(f, "{}", id.path())?;
                    if !generics.is_empty() {
                        write!(f, "<")?;
                        for generic in generics {
                            if let Self::Pointer { pointee, .. } = generic {
                                write!(f, "{},", pointee.behind_pointer())?;
                            } else {
                                error!(?self, ?generic, "unknown generic");
                                write!(f, "{},", generic.behind_pointer())?;
                            }
                        }
                        write!(f, ">")?;
                    }
                    Ok(())
                }
                PointeeTy::GenericParam { name } => write!(f, "{name}"),
                PointeeTy::AnyObject { protocols } => match &**protocols {
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
                PointeeTy::AnyProtocol => write!(f, "AnyProtocol"),
                PointeeTy::AnyClass { protocols } => match &**protocols {
                    [] => write!(f, "AnyClass"),
                    // TODO: Handle this better
                    _ => write!(f, "AnyClass"),
                },
                PointeeTy::Self_ => write!(f, "Self"),
                // TODO: Handle this better.
                PointeeTy::Fn { .. } => {
                    write!(f, "core::ffi::c_void /* TODO: Should be a function. */")
                }
                PointeeTy::Block {
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
                PointeeTy::CFTypeDef { id } => write!(f, "{}", id.path()),
                PointeeTy::DispatchTypeDef { id } => write!(f, "{}", id.path()),
                PointeeTy::TypeDef { id, .. } => write!(f, "{}", id.path()),
                PointeeTy::CStr => write!(f, "CStr"),
            },
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
            Self::Primitive(Primitive::C99Bool) => {
                warn!("C99's bool as Objective-C method return is ill supported");
                write!(f, " -> bool")
            }
            Self::Primitive(Primitive::ObjcBool) => write!(f, " -> bool"),
            _ => write!(f, " -> {}", self.plain()),
        })
    }

    pub(crate) fn method_return_with_error(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| {
            match self {
                Self::Pointer {
                    nullability: Nullability::Nullable,
                    lifetime: Lifetime::Unspecified,
                    pointee,
                    ..
                } if pointee.is_static_object() => {
                    // NULL -> error
                    write!(
                        f,
                        " -> Result<&'static {}, Retained<{}>>",
                        pointee.behind_pointer(),
                        ItemIdentifier::nserror().path(),
                    )
                }
                Self::Pointer {
                    nullability: Nullability::Nullable,
                    lifetime: Lifetime::Unspecified,
                    pointee,
                    ..
                } if pointee.is_object_like() => {
                    // NULL -> error
                    write!(
                        f,
                        " -> Result<Retained<{}>, Retained<{}>>",
                        pointee.behind_pointer(),
                        ItemIdentifier::nserror().path(),
                    )
                }
                Self::Primitive(Primitive::ObjcBool) => {
                    // NO -> error
                    write!(
                        f,
                        " -> Result<(), Retained<{}>>",
                        ItemIdentifier::nserror().path()
                    )
                }
                _ => {
                    error!("unknown error result type {self:?}");
                    write!(f, "{}", self.method_return())
                }
            }
        })
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
            Self::Pointer { pointee, .. } if pointee.is_cf_type() => {
                items.push(ItemTree::cf("CFRetained"));
                items.push(ItemTree::core_ptr_nonnull());
            }
            Self::Pointer { pointee, .. }
                if pointee.is_objc_type() && !pointee.is_static_object() =>
            {
                items.push(ItemTree::objc("Retained"));
            }
            Self::Pointer { pointee, .. } if pointee.is_dispatch_type() => {
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
                is_const,
                pointee,
                ..
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
                    if *nullability == Nullability::NonNull {
                        write!(f, "-> Option<NonNull<{}>>", pointee.behind_pointer())
                    } else if *is_const {
                        write!(f, " -> *const {}", pointee.behind_pointer())
                    } else {
                        write!(f, " -> *mut {}", pointee.behind_pointer())
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
                lifetime,
                pointee,
                ..
            } => {
                match lifetime {
                    Lifetime::Autoreleasing if !returns_retained => {}
                    Lifetime::Strong if returns_retained => {}
                    Lifetime::Unspecified => {}
                    _ => error!(?lifetime, returns_retained, "invalid lifetime"),
                }

                if pointee.is_static_object() {
                    if *nullability == Nullability::NonNull {
                        let res = format!(" -> &'static {}", pointee.behind_pointer());
                        Some((res, start, ";\nret.expect(\"function was marked as returning non-null, but actually returned NULL\")"))
                    } else {
                        // No conversion necessary
                        None
                    }
                } else if pointee.is_cf_type() {
                    let res = if *nullability == Nullability::NonNull {
                        format!(" -> CFRetained<{}>", pointee.behind_pointer())
                    } else {
                        format!(" -> Option<CFRetained<{}>>", pointee.behind_pointer())
                    };
                    Some((res, start, end_cf(*nullability)))
                } else if pointee.is_dispatch_type() {
                    let res = if *nullability == Nullability::NonNull {
                        format!(" -> DispatchRetained<{}>", pointee.behind_pointer())
                    } else {
                        format!(" -> Option<DispatchRetained<{}>>", pointee.behind_pointer())
                    };
                    Some((res, start, end_dispatch(*nullability)))
                } else if pointee.is_objc_type() && !pointee.is_static_object() {
                    let res = if *nullability == Nullability::NonNull {
                        format!(" -> Retained<{}>", pointee.behind_pointer())
                    } else {
                        format!(" -> Option<Retained<{}>>", pointee.behind_pointer())
                    };
                    Some((res, start, end_objc(*nullability)))
                } else {
                    if *nullability == Nullability::NonNull {
                        let res = format!(" -> NonNull<{}>", pointee.behind_pointer());
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
                // `const` is irrelevant in statics since they're always
                // constant.
                is_const: _,
                lifetime: Lifetime::Strong | Lifetime::Unspecified,
                pointee,
            } if pointee.is_object_like() || **pointee == Ty::Pointee(PointeeTy::CStr) => {
                if *nullability == Nullability::NonNull {
                    write!(f, "&'static {}", pointee.behind_pointer())
                } else {
                    write!(f, "Option<&'static {}>", pointee.behind_pointer())
                }
            }
            _ => write!(f, "{}", self.behind_pointer()),
        })
    }

    pub(crate) fn const_(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Pointer {
                nullability,
                // `const` is irrelevant in constants since they're always
                // constant.
                is_const: _,
                lifetime: Lifetime::Strong | Lifetime::Unspecified,
                pointee,
            } if pointee.is_object_like() || **pointee == Ty::Pointee(PointeeTy::CStr) => {
                if *nullability == Nullability::NonNull {
                    write!(f, "&{}", pointee.behind_pointer())
                } else {
                    write!(f, "Option<&{}>", pointee.behind_pointer())
                }
            }
            _ => write!(f, "{}", self.plain()),
        })
    }

    pub(crate) fn typedef(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Pointer {
                nullability: _,
                is_const: _,
                lifetime: _,
                pointee,
            } if pointee.is_object_like() => {
                write!(f, "{}", pointee.behind_pointer())
            }
            Self::IncompleteArray { .. } => {
                error!("incomplete array in typedef");
                write!(f, "{}", self.behind_pointer())
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
                is_const: _,
                lifetime,
                pointee,
            } if pointee.is_object_like()
                || matches!(
                    **pointee,
                    Self::Pointee(PointeeTy::AnyClass { .. } | PointeeTy::Block { .. })
                ) =>
            {
                if *lifetime == Lifetime::Autoreleasing {
                    error!(?self, "autoreleasing in fn argument");
                }
                if *nullability == Nullability::NonNull {
                    write!(f, "&{}", pointee.behind_pointer())
                } else {
                    write!(f, "Option<&{}>", pointee.behind_pointer())
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
            // TODO: Support out / autoreleasing pointers?
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
                is_const: false,
                lifetime: Lifetime::Unspecified,
                pointee,
            } => match &**pointee {
                Self::Pointer {
                    nullability: inner_nullability,
                    // Don't care about the const-ness of the id.
                    is_const: _,
                    lifetime: Lifetime::Autoreleasing,
                    pointee,
                } => {
                    let tokens = if *inner_nullability == Nullability::NonNull {
                        format!("Retained<{}>", pointee.behind_pointer())
                    } else {
                        format!("Option<Retained<{}>>", pointee.behind_pointer())
                    };
                    if *nullability == Nullability::NonNull {
                        write!(f, "&mut {tokens}")
                    } else {
                        write!(f, "Option<&mut {tokens}>")
                    }
                }
                _ => write!(f, "{}", self.fn_argument()),
            },
            _ => write!(f, "{}", self.fn_argument()),
        })
    }

    pub(crate) fn method_argument_encoding_type(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Primitive(Primitive::C99Bool) => write!(f, "Bool"),
            _ => write!(f, "{}", self.plain()),
        })
    }

    pub(crate) fn record(&self) -> impl fmt::Display + '_ {
        FormatterFn(move |f| match self {
            Self::Array {
                element_type,
                num_elements,
            } => write!(f, "[{}; {num_elements}]", element_type.plain()),
            _ => write!(f, "{}", self.plain()),
        })
    }

    fn fn_contains_bool(&self) -> bool {
        match self {
            Self::Pointer { pointee, .. } => {
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
            }
            Self::TypeDef { to, .. } => to.fn_contains_bool(),
            _ => false,
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
        let mut ty = Self::parse(ty, Lifetime::Unspecified, context);

        match &mut ty {
            Self::Pointer { pointee, .. } => match &mut **pointee {
                Self::Pointee(PointeeTy::Block {
                    sendable,
                    no_escape,
                    ..
                }) => {
                    *sendable = arg_sendable;
                    *no_escape = arg_no_escape;
                    arg_sendable = None;
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
        let mut ty = Self::parse(ty, Lifetime::Unspecified, context);

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
        Self::parse(ty, Lifetime::Unspecified, context)
    }

    pub(crate) fn pointer_to_opaque_struct_or_void(
        &self,
        typedef_name: &str,
        typedef_is_bridged: bool,
    ) -> Option<(bool, Option<&str>)> {
        if let Self::Pointer {
            pointee,
            is_const: _, // const-ness doesn't matter when defining the type
            nullability,
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

    pub(crate) fn parse_property(
        ty: Type<'_>,
        // Ignored; see `parse_property_return`
        _is_copy: bool,
        _sendable: Option<bool>,
        context: &Context<'_>,
    ) -> Self {
        Self::parse(ty, Lifetime::Unspecified, context)
    }

    pub(crate) fn parse_property_return(
        ty: Type<'_>,
        is_copy: bool,
        _sendable: Option<bool>,
        context: &Context<'_>,
    ) -> Self {
        let mut ty = Self::parse(ty, Lifetime::Unspecified, context);

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

    pub(crate) fn parse_record_field(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse(ty, Lifetime::Unspecified, context)
    }

    pub fn is_signed(&self) -> Option<bool> {
        match self {
            Self::Primitive(prim) => prim.is_signed(),
            Self::Simd { ty, .. } => ty.is_signed(),
            Self::Enum { ty, .. } => ty.is_signed(),
            Self::TypeDef { to, .. } => to.is_signed(),
            _ => None,
        }
    }

    pub(crate) fn parse_enum(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse(ty, Lifetime::Unspecified, context)
    }

    pub(crate) fn is_simple_uint(&self) -> bool {
        matches!(self, Self::Primitive(Primitive::UInt))
    }

    pub(crate) fn parse_static(ty: Type<'_>, context: &Context<'_>) -> Self {
        Self::parse(ty, Lifetime::Unspecified, context)
    }

    pub(crate) fn argument_is_error_out(&self) -> bool {
        if let Self::Pointer {
            // We always pass a place to write the error information,
            // so doesn't matter whether it's optional or not.
            nullability: Nullability::Nullable | Nullability::NonNull,
            is_const,
            lifetime: Lifetime::Unspecified,
            pointee,
        } = self
        {
            if let Self::Pointer {
                nullability: inner_nullability,
                is_const: inner_is_const,
                lifetime,
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
                    assert!(!is_const, "expected error not const {self:?}");
                    assert_eq!(
                        *inner_nullability,
                        Nullability::Nullable,
                        "invalid inner error nullability {self:?}"
                    );
                    assert!(!inner_is_const, "expected inner error not const {self:?}");

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
        match self {
            Self::Enum { .. } => true,
            Self::TypeDef { to, .. } => to.is_enum_through_typedef(),
            _ => false,
        }
    }

    pub(crate) fn is_floating_through_typedef(&self) -> bool {
        match self {
            Self::Primitive(
                Primitive::F32 | Primitive::F64 | Primitive::Float | Primitive::Double,
            ) => true,
            Self::TypeDef { to, .. } => to.is_floating_through_typedef(),
            _ => false,
        }
    }

    /// SIMD is not yet possible in FFI, see:
    /// <https://github.com/rust-lang/rust/issues/63068>
    pub(crate) fn needs_simd(&self) -> bool {
        match self {
            Self::Simd { .. } => true,
            Self::Pointer { pointee, .. } | Self::IncompleteArray { pointee, .. } => {
                pointee.needs_simd()
            }
            Self::TypeDef { to, .. } => to.needs_simd(),
            Self::Array { element_type, .. } => element_type.needs_simd(),
            Self::Struct { fields, .. } | Self::Union { fields, .. } => {
                fields.iter().any(|field| field.needs_simd())
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
                    PointeeTy::CFTypeDef { id } => id,
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
            is_const: true,
            lifetime: Lifetime::Unspecified,
            pointee: Box::new(Self::Pointee(PointeeTy::CFTypeDef {
                id: ItemIdentifier::cf_string(),
            })),
        }
    }

    pub(crate) fn const_ns_string_ref() -> Self {
        Self::Pointer {
            nullability: Nullability::NonNull,
            is_const: true,
            lifetime: Lifetime::Unspecified,
            pointee: Box::new(Self::Pointee(PointeeTy::Class {
                id: ItemIdentifier::ns_string(),
                thread_safety: ThreadSafety::dummy(),
                superclasses: vec![],
                generics: vec![],
                protocols: vec![],
            })),
        }
    }

    pub(crate) fn const_cf_uuid_ref() -> Self {
        Self::Pointer {
            nullability: Nullability::NonNull,
            is_const: true,
            lifetime: Lifetime::Unspecified,
            pointee: Box::new(Self::Pointee(PointeeTy::CFTypeDef {
                id: ItemIdentifier::cf_uuid(),
            })),
        }
    }

    pub(crate) fn const_cstr_ref() -> Self {
        Self::Pointer {
            nullability: Nullability::NonNull,
            is_const: true,
            lifetime: Lifetime::Unspecified,
            pointee: Box::new(Self::Pointee(PointeeTy::CStr)),
        }
    }
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
        if let Ok(attr) = UnexposedAttr::from_name(&ident, || {
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
            iter.next();
            attr
        } else {
            None
        }
    } else {
        None
    };
    (TokenStream::from_iter(iter).to_string(), attr)
}

#[cfg(test)]
mod tests {
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
                fields: vec![Ty::Primitive(Primitive::Char)],
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
}
