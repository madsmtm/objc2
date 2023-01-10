use std::fmt;
use std::str::FromStr;

use clang::{CallingConvention, EntityKind, Nullability, Type, TypeKind};
use proc_macro2::{TokenStream, TokenTree};
use serde::Deserialize;

use crate::context::Context;
use crate::id::ItemIdentifier;
use crate::method::MemoryManagement;

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
                // it might just appear twice in `name`.
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

    fn is_kindof(&mut self, position: ParsePosition) -> bool {
        self.strip("__kindof", position)
    }

    fn is_const(&mut self, position: ParsePosition) -> bool {
        self.strip("const", position)
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
}

impl Drop for AttributeParser<'_, '_> {
    fn drop(&mut self) {
        if !std::thread::panicking() && self.name != self.expected_name {
            error!(?self, "could not extract all attributes");
        }
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(from = "bool")]
pub enum Ownership {
    Owned,
    Shared,
}

impl From<bool> for Ownership {
    fn from(b: bool) -> Self {
        if b {
            Self::Owned
        } else {
            Self::Shared
        }
    }
}

impl Default for Ownership {
    fn default() -> Self {
        Ownership::Shared
    }
}

impl fmt::Display for Ownership {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Owned => write!(f, "Owned"),
            Self::Shared => write!(f, "Shared"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum TypeParams {
    Empty,
    // TODO: Ensure in the type-system that these are never empty
    Generics(Vec<IdType>),
    Protocols(Vec<ItemIdentifier>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum IdType {
    Class {
        id: ItemIdentifier,
        params: TypeParams,
        ownership: Option<Ownership>,
    },
    TypeDef {
        id: ItemIdentifier,
    },
    GenericParam {
        name: String,
    },
    AnyObject {
        protocols: Vec<ItemIdentifier>,
    },
    AnyProtocol,
    AnyClass {
        protocols: Vec<ItemIdentifier>,
    },
    Allocated,
    Self_ {
        ownership: Option<Ownership>,
    },
}

impl IdType {
    #[allow(dead_code)]
    fn _id(&self) -> Option<&ItemIdentifier> {
        match self {
            Self::Class { id, .. } => Some(id),
            Self::AnyObject { protocols } => match &**protocols {
                [id] if id.name == "NSCopying" || id.name == "NSMutableCopying" => None,
                [id] => Some(id),
                _ => None,
            },
            Self::TypeDef { id, .. } => Some(id),
            _ => None,
        }
    }

    fn ownership(&self) -> impl fmt::Display + '_ {
        struct IdTypeOwnership<'a>(&'a IdType);

        impl fmt::Display for IdTypeOwnership<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    IdType::Class {
                        ownership: Some(ownership),
                        ..
                    }
                    | IdType::Self_ {
                        ownership: Some(ownership),
                    } => write!(f, "{ownership}"),
                    IdType::GenericParam { name } => write!(f, "{name}Ownership"),
                    _ => write!(f, "{}", Ownership::Shared),
                }
            }
        }

        IdTypeOwnership(self)
    }

    fn parse_objc_pointer(
        ty: Type<'_>,
        pointer_name: &str,
        lifetime: &mut Lifetime,
        is_kindof: &mut bool,
        context: &Context<'_>,
    ) -> Self {
        let generics: Vec<_> = ty
            .get_objc_type_arguments()
            .into_iter()
            .map(
                |param| match Inner::parse(param, Lifetime::Unspecified, context) {
                    Inner::Id {
                        ty,
                        is_const: false,
                        lifetime: Lifetime::Unspecified,
                        nullability: Nullability::Unspecified,
                    } => ty,
                    Inner::Class {
                        nullability: Nullability::Unspecified,
                    } => Self::AnyClass { protocols: vec![] },
                    param => {
                        panic!("invalid generic parameter {param:?} in {ty:?}")
                    }
                },
            )
            .collect();

        let protocols: Vec<_> = ty
            .get_objc_protocol_declarations()
            .into_iter()
            .map(|entity| ItemIdentifier::new(&entity, context))
            .collect();

        match ty.get_kind() {
            TypeKind::ObjCInterface => {
                if !generics.is_empty() {
                    panic!("generics not empty: {ty:?}, {generics:?}");
                }
                if !protocols.is_empty() {
                    panic!("protocols not empty: {ty:?}, {protocols:?}");
                }
                let name = ty.get_display_name();

                let mut parser = AttributeParser::new(pointer_name, &name);

                *is_kindof = parser.is_kindof(ParsePosition::Prefix);
                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                // Ignore const for now
                _ = parser.is_const(ParsePosition::Suffix);
                parser.set_inner_pointer();
                drop(parser);

                if name == "Protocol" {
                    Self::AnyProtocol
                } else {
                    let declaration = ty.get_declaration().expect("ObjCInterface declaration");
                    let id = ItemIdentifier::new(&declaration, context);
                    assert_eq!(id.name, name);
                    Self::Class {
                        id,
                        params: TypeParams::Empty,
                        ownership: None,
                    }
                }
            }
            TypeKind::ObjCObject => {
                let pointee_name = ty.get_display_name();
                let base_ty = ty
                    .get_objc_object_base_type()
                    .expect("object to have base type");
                let name = base_ty.get_display_name();

                match base_ty.get_kind() {
                    TypeKind::ObjCId => {
                        assert_eq!(name, "id");

                        let mut parser = AttributeParser::new(pointer_name, &pointee_name);
                        lifetime.update(parser.lifetime(ParsePosition::Prefix));

                        if !generics.is_empty() {
                            panic!("generics not empty: {ty:?}, {generics:?}");
                        }

                        Self::AnyObject { protocols }
                    }
                    TypeKind::ObjCInterface => {
                        let declaration = base_ty
                            .get_declaration()
                            .expect("ObjCObject -> ObjCInterface declaration");
                        let id = ItemIdentifier::new(&declaration, context);
                        assert_eq!(id.name, name);

                        if !generics.is_empty() && !protocols.is_empty() {
                            panic!("got object with both protocols and generics: {name:?}, {protocols:?}, {generics:?}");
                        }

                        if generics.is_empty() && protocols.is_empty() {
                            panic!("got object with empty protocols and generics: {name:?}");
                        }

                        let mut parser = AttributeParser::new(pointer_name, &pointee_name);
                        lifetime.update(parser.lifetime(ParsePosition::Prefix));
                        parser.set_inner_pointer();

                        Self::Class {
                            id,
                            params: if protocols.is_empty() {
                                TypeParams::Generics(generics)
                            } else {
                                TypeParams::Protocols(protocols)
                            },
                            ownership: None,
                        }
                    }
                    TypeKind::ObjCClass => {
                        assert!(generics.is_empty(), "ObjCClass with generics");

                        Self::AnyClass { protocols }
                    }
                    kind => panic!("unknown ObjCObject kind {ty:?}, {kind:?}"),
                }
            }
            _ => panic!("pointee was neither objcinterface nor objcobject: {ty:?}"),
        }
    }

    fn visit_required_types(&self, f: &mut impl FnMut(&ItemIdentifier)) {
        // TODO
        // if let Some(id) = self.id() {
        //     f(&id);
        // }

        if let Self::Class { id, params, .. } = self {
            f(&id);
            if let TypeParams::Generics(generics) = params {
                for generic in generics {
                    generic.visit_required_types(f);
                }
            }
        }
    }
}

impl fmt::Display for IdType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Class { id, params, .. } => {
                write!(f, "{}", id.path())?;
                if let TypeParams::Generics(generics) = params {
                    write!(f, "<")?;
                    for generic in generics {
                        write!(f, "{generic},")?;
                    }
                    write!(f, ">")?;
                }
                Ok(())
            }
            Self::AnyObject { protocols } => match &**protocols {
                [] => write!(f, "Object"),
                [id] if id.name == "NSCopying" || id.name == "NSMutableCopying" => {
                    write!(f, "Object")
                }
                [id] => write!(f, "{}", id.path()),
                // TODO: Handle this better
                _ => write!(f, "TodoProtocols"),
            },
            Self::TypeDef { id, .. } => write!(f, "{}", id.path()),
            Self::GenericParam { name } => write!(f, "{name}"),
            Self::AnyProtocol => write!(f, "Protocol"),
            // TODO: Handle this better
            Self::AnyClass { .. } => write!(f, "TodoClass"),
            Self::Allocated => write!(f, "Allocated"),
            Self::Self_ { .. } => write!(f, "Self"),
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Inner {
    // Primitives
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
    F32,
    F64,
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

    // Objective-C
    Id {
        ty: IdType,
        is_const: bool,
        lifetime: Lifetime,
        nullability: Nullability,
    },
    Class {
        nullability: Nullability,
    },
    Sel {
        nullability: Nullability,
    },
    ObjcBool,

    // Others
    Pointer {
        nullability: Nullability,
        is_const: bool,
        pointee: Box<Inner>,
    },
    IncompleteArray {
        nullability: Nullability,
        is_const: bool,
        pointee: Box<Inner>,
    },
    Array {
        element_type: Box<Inner>,
        num_elements: usize,
    },
    Enum {
        id: ItemIdentifier,
    },
    Struct {
        id: ItemIdentifier,
    },
    Fn {
        is_variadic: bool,
        arguments: Vec<Inner>,
        result_type: Box<Inner>,
    },
    Block {
        arguments: Vec<Inner>,
        result_type: Box<Inner>,
    },

    TypeDef {
        id: ItemIdentifier,
    },
}

impl Inner {
    fn parse(ty: Type<'_>, mut lifetime: Lifetime, context: &Context<'_>) -> Self {
        let _span = debug_span!("ty", ?ty, ?lifetime).entered();

        let attributed_ty = ty;
        let mut ty = attributed_ty;
        while let TypeKind::Attributed = ty.get_kind() {
            ty = ty
                .get_modified_type()
                .expect("attributed type to have modified type");
        }

        let _span = debug_span!("ty2", ?ty).entered();

        let mut attributed_name = attributed_ty.get_display_name();
        let mut name = ty.get_display_name();

        let unexposed_nullability = if let TypeKind::Unexposed = ty.get_kind() {
            let nullability = ty.get_nullability();
            attributed_name = parse_unexposed_tokens(&attributed_name);
            // Also parse the expected name to ensure that the formatting that
            // TokenStream does is the same on both.
            name = parse_unexposed_tokens(&name);
            ty = ty
                .get_modified_type()
                .expect("attributed type to have modified type");
            nullability
        } else {
            None
        };

        let _span = debug_span!("ty3", ?ty).entered();

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

        use TypeKind::*;
        match ty.get_kind() {
            Void => Self::Void,
            Bool => Self::C99Bool,
            CharS | CharU => Self::Char,
            SChar => Self::SChar,
            UChar => Self::UChar,
            Short => Self::Short,
            UShort => Self::UShort,
            Int => Self::Int,
            UInt => Self::UInt,
            Long => Self::Long,
            ULong => Self::ULong,
            LongLong => Self::LongLong,
            ULongLong => Self::ULongLong,
            Float => Self::Float,
            Double => Self::Double,
            ObjCId => {
                let mut parser = AttributeParser::new(&attributed_name, "id");

                lifetime.update(parser.lifetime(ParsePosition::Prefix));

                let is_const = get_is_const(parser.is_const(ParsePosition::Suffix));
                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };

                Self::Id {
                    ty: IdType::AnyObject { protocols: vec![] },
                    is_const,
                    lifetime,
                    nullability,
                }
            }
            ObjCClass => {
                let mut parser = AttributeParser::new(&attributed_name, &name);
                let _lifetime = parser.lifetime(ParsePosition::Suffix);
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };
                Self::Class { nullability }
            }
            ObjCSel => {
                let mut parser = AttributeParser::new(&attributed_name, &name);
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };
                Self::Sel { nullability }
            }
            Pointer => {
                let mut parser = AttributeParser::new(&attributed_name, &name);
                let pointee = ty.get_pointee_type().expect("pointer to have pointee");
                if let TypeKind::FunctionPrototype = pointee.get_kind() {
                    parser.set_fn_ptr();
                }

                let is_const = get_is_const(parser.is_const(ParsePosition::Suffix));
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };

                let pointee = Self::parse(pointee, Lifetime::Unspecified, context);
                Self::Pointer {
                    nullability,
                    is_const,
                    pointee: Box::new(pointee),
                }
            }
            BlockPointer => {
                let mut parser = AttributeParser::new(&attributed_name, &name);
                parser.set_fn_ptr();

                let is_const = get_is_const(parser.is_const(ParsePosition::Suffix));
                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };

                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                match Self::parse(ty, Lifetime::Unspecified, context) {
                    Self::Fn {
                        is_variadic: false,
                        arguments,
                        result_type,
                    } => Self::Pointer {
                        nullability,
                        is_const,
                        pointee: Box::new(Self::Block {
                            arguments,
                            result_type,
                        }),
                    },
                    pointee => panic!("unexpected pointee in block: {pointee:?}"),
                }
            }
            ObjCObjectPointer => {
                let mut parser = AttributeParser::new(&attributed_name, &name);
                let is_kindof = parser.is_kindof(ParsePosition::Prefix);

                let is_const = get_is_const(parser.is_const(ParsePosition::Suffix));
                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                let mut nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };

                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                drop(parser);

                let pointer_name = ty.get_display_name();
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");

                let attributed_ty = ty;
                let mut ty = attributed_ty;
                while let TypeKind::Attributed = ty.get_kind() {
                    ty = ty
                        .get_modified_type()
                        .expect("attributed type to have modified type");
                }
                let attributed_name = attributed_ty.get_display_name();
                let name = ty.get_display_name();

                let mut parser = AttributeParser::new(&attributed_name, &name);

                let mut is_kindof = is_kindof || parser.is_kindof(ParsePosition::Prefix);

                let pointee_is_const = parser.is_const(ParsePosition::Suffix);
                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                let new = parser.nullability(ParsePosition::Suffix);
                if new != attributed_ty.get_nullability() {
                    error!("failed parsing nullability");
                }
                update_nullability(&mut nullability, new);
                lifetime.update(parser.lifetime(ParsePosition::Suffix));

                if !is_const && pointee_is_const {
                    warn!(?ty, "pointee was const while ObjCObjectPointer was not");
                }
                drop(parser);

                Self::Id {
                    ty: IdType::parse_objc_pointer(
                        ty,
                        &pointer_name,
                        &mut lifetime,
                        &mut is_kindof,
                        context,
                    ),
                    is_const,
                    lifetime,
                    nullability,
                }
            }
            Typedef => {
                let typedef_name = ty.get_typedef_name().expect("typedef has name");

                let mut parser = AttributeParser::new(&attributed_name, &typedef_name);
                let mut is_kindof = parser.is_kindof(ParsePosition::Prefix);
                let is_const1 = parser.is_const(ParsePosition::Prefix);
                lifetime.update(parser.lifetime(ParsePosition::Prefix));

                let is_const2 = parser.is_const(ParsePosition::Suffix);
                lifetime.update(parser.lifetime(ParsePosition::Suffix));
                let nullability = if let Some(nullability) = unexposed_nullability {
                    nullability
                } else {
                    check_nullability(&attributed_ty, parser.nullability(ParsePosition::Suffix))
                };
                drop(parser);

                let is_const = if is_const1 || is_const2 {
                    if !attributed_ty.is_const_qualified() && !ty.is_const_qualified() {
                        warn!(?attributed_ty, ?ty, ?typedef_name, ?is_const1, ?is_const2, attr = ?attributed_ty.is_const_qualified(), ty = ?ty.is_const_qualified(), "unnecessarily stripped const");
                    }
                    true
                } else {
                    if ty.is_const_qualified() {
                        warn!("type was const but that could not be stripped");
                    }
                    false
                };

                match &*typedef_name {
                    "BOOL" => Self::ObjcBool,

                    "int8_t" => Self::I8,
                    "uint8_t" => Self::U8,
                    "int16_t" => Self::I16,
                    "uint16_t" => Self::U16,
                    "int32_t" => Self::I32,
                    "uint32_t" => Self::U32,
                    "int64_t" => Self::I64,
                    "uint64_t" => Self::U64,
                    "ssize_t" => Self::ISize,
                    "size_t" => Self::USize,

                    // MacTypes.h
                    "UInt8" => Self::U8,
                    "UInt16" => Self::U16,
                    "UInt32" => Self::U32,
                    "UInt64" => Self::U64,
                    "SInt8" => Self::I8,
                    "SInt16" => Self::I16,
                    "SInt32" => Self::I32,
                    "SInt64" => Self::I64,
                    "Float32" => Self::F32,
                    "Float64" => Self::F64,
                    "Float80" => panic!("can't handle 80 bit MacOS float"),
                    "Float96" => panic!("can't handle 96 bit 68881 float"),

                    "instancetype" => Self::Id {
                        ty: IdType::Self_ { ownership: None },
                        is_const,
                        lifetime,
                        nullability,
                    },
                    _ => {
                        let canonical = ty.get_canonical_type();
                        let declaration = ty.get_declaration();
                        let _span = debug_span!("typedef", ?typedef_name, ?canonical, ?declaration)
                            .entered();
                        match canonical.get_kind() {
                            ObjCObjectPointer => {
                                let pointee = canonical
                                    .get_pointee_type()
                                    .expect("pointer type to have pointee");
                                let _span = debug_span!("ObjCObjectPointer", ?pointee).entered();
                                let declaration =
                                    declaration.expect("typedef ObjCObjectPointer declaration");

                                assert!(
                                    pointee.get_objc_type_arguments().is_empty(),
                                    "typedef generics not empty"
                                );

                                let ty = if let EntityKind::TemplateTypeParameter =
                                    declaration.get_kind()
                                {
                                    IdType::GenericParam { name: typedef_name }
                                } else {
                                    // TODO: Refactor this
                                    let _ = IdType::parse_objc_pointer(
                                        pointee,
                                        &canonical.get_display_name(),
                                        &mut lifetime,
                                        &mut is_kindof,
                                        context,
                                    );

                                    IdType::TypeDef {
                                        id: ItemIdentifier::new(&declaration, context),
                                    }
                                };

                                Self::Id {
                                    ty,
                                    is_const,
                                    lifetime,
                                    nullability,
                                }
                            }
                            _ => {
                                let declaration = declaration.expect("typedef declaration");
                                Self::TypeDef {
                                    id: ItemIdentifier::with_name(
                                        typedef_name,
                                        &declaration,
                                        context,
                                    ),
                                }
                            }
                        }
                    }
                }
            }
            Elaborated => {
                let ty = ty.get_elaborated_type().expect("elaborated");
                match ty.get_kind() {
                    TypeKind::Record => {
                        let declaration = ty.get_declaration().expect("record declaration");
                        let name = ty
                            .get_display_name()
                            .trim_start_matches("struct ")
                            .to_string();
                        Self::Struct {
                            id: ItemIdentifier::with_name(name, &declaration, context),
                        }
                    }
                    TypeKind::Enum => {
                        let declaration = ty.get_declaration().expect("enum declaration");
                        let name = ty
                            .get_display_name()
                            .trim_start_matches("enum ")
                            .to_string();
                        Self::Enum {
                            id: ItemIdentifier::with_name(name, &declaration, context),
                        }
                    }
                    _ => panic!("unknown elaborated type {ty:?}"),
                }
            }
            FunctionPrototype => {
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
                    .map(|ty| Inner::parse(ty, Lifetime::Unspecified, context))
                    .collect();

                let result_type = ty.get_result_type().expect("fn type to have result type");
                let result_type = Inner::parse(result_type, Lifetime::Unspecified, context);

                Self::Fn {
                    is_variadic: ty.is_variadic(),
                    arguments,
                    result_type: Box::new(result_type),
                }
            }
            IncompleteArray => {
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
            ConstantArray => {
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
            _ => panic!("unsupported type: {ty:?}"),
        }
    }

    fn visit_lifetime(&self, mut f: impl FnMut(Lifetime)) {
        match self {
            Self::Id { lifetime, .. } => f(*lifetime),
            Self::Pointer { pointee, .. } => pointee.visit_lifetime(f),
            Self::IncompleteArray { pointee, .. } => pointee.visit_lifetime(f),
            Self::Array { element_type, .. } => element_type.visit_lifetime(f),
            _ => {}
        }
    }

    fn visit_required_types(&self, f: &mut impl FnMut(&ItemIdentifier)) {
        match self {
            // Objective-C
            Self::Id { ty, .. } => {
                // f("objc2");
                ty.visit_required_types(f);
            }
            Self::Class { .. } | Self::Sel { .. } | Self::ObjcBool => {
                // f("objc2");
            }

            // Others
            Self::Pointer { pointee, .. } | Self::IncompleteArray { pointee, .. } => {
                pointee.visit_required_types(f);
            }
            Self::Array { element_type, .. } => {
                element_type.visit_required_types(f);
            }
            // TODO
            // Enum { id } | Struct { id } | TypeDef { id } => {
            //
            // }
            Self::Fn {
                arguments,
                result_type,
                ..
            }
            | Self::Block {
                arguments,
                result_type,
            } => {
                // TODO if block
                // f("block2");
                for arg in arguments {
                    arg.visit_required_types(f);
                }
                result_type.visit_required_types(f);
            }
            _ => {}
        }
    }
}

/// This is sound to output in (almost, c_void is not a valid return type) any
/// context. `Ty` is then used to change these types into something nicer when
/// required.
impl fmt::Display for Inner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Inner::*;
        match self {
            // Primitives
            Void => write!(f, "c_void"),
            C99Bool => write!(f, "bool"),
            Char => write!(f, "c_char"),
            SChar => write!(f, "c_schar"),
            UChar => write!(f, "c_uchar"),
            Short => write!(f, "c_short"),
            UShort => write!(f, "c_ushort"),
            Int => write!(f, "c_int"),
            UInt => write!(f, "c_uint"),
            Long => write!(f, "c_long"),
            ULong => write!(f, "c_ulong"),
            LongLong => write!(f, "c_longlong"),
            ULongLong => write!(f, "c_ulonglong"),
            Float => write!(f, "c_float"),
            Double => write!(f, "c_double"),
            F32 => write!(f, "f32"),
            F64 => write!(f, "f64"),
            I8 => write!(f, "i8"),
            U8 => write!(f, "u8"),
            I16 => write!(f, "i16"),
            U16 => write!(f, "u16"),
            I32 => write!(f, "i32"),
            U32 => write!(f, "u32"),
            I64 => write!(f, "i64"),
            U64 => write!(f, "u64"),
            // TODO: Use core::ffi::c_ssize_t
            ISize => write!(f, "isize"),
            // TODO: Use core::ffi::c_size_t
            USize => write!(f, "usize"),

            // Objective-C
            Id {
                ty,
                is_const,
                // Ignore
                lifetime: _,
                nullability,
            } => {
                if *nullability == Nullability::NonNull {
                    write!(f, "NonNull<{ty}>")
                } else if *is_const {
                    write!(f, "*const {ty}")
                } else {
                    write!(f, "*mut {ty}")
                }
            }
            Class { nullability } => {
                if *nullability == Nullability::NonNull {
                    write!(f, "NonNull<Class>")
                } else {
                    write!(f, "*const Class")
                }
            }
            Sel { nullability } => {
                if *nullability == Nullability::NonNull {
                    write!(f, "Sel")
                } else {
                    write!(f, "Option<Sel>")
                }
            }
            ObjcBool => write!(f, "Bool"),

            // Others
            Pointer {
                nullability,
                is_const,
                pointee,
            } => match &**pointee {
                Self::Fn {
                    is_variadic,
                    arguments,
                    result_type,
                } => {
                    if *nullability != Nullability::NonNull {
                        write!(f, "Option<")?;
                    }
                    write!(f, "unsafe extern \"C\" fn(")?;
                    for arg in arguments {
                        write!(f, "{arg},")?;
                    }
                    if *is_variadic {
                        write!(f, "...")?;
                    }
                    write!(f, ")")?;
                    match &**result_type {
                        Self::Void => {
                            // Don't output anything
                        }
                        ty => write!(f, " -> {ty}")?,
                    }
                    if *nullability != Nullability::NonNull {
                        write!(f, ">")?;
                    }
                    Ok(())
                }
                pointee => {
                    if *nullability == Nullability::NonNull {
                        write!(f, "NonNull<{pointee}>")
                    } else if *is_const {
                        write!(f, "*const {pointee}")
                    } else {
                        write!(f, "*mut {pointee}")
                    }
                }
            },
            IncompleteArray {
                nullability,
                is_const,
                pointee,
            } => {
                if *nullability == Nullability::NonNull {
                    write!(f, "NonNull<{pointee}>")
                } else if *is_const {
                    write!(f, "*const {pointee}")
                } else {
                    write!(f, "*mut {pointee}")
                }
            }
            Array {
                element_type,
                num_elements,
            } => write!(f, "ArrayUnknownABI<[{element_type}; {num_elements}]>"),
            Enum { id } | Struct { id } | TypeDef { id } => write!(f, "{}", id.name),
            Self::Fn { .. } => write!(f, "TodoFunction"),
            Self::Block {
                arguments,
                result_type,
            } => {
                write!(f, "Block<(")?;
                for arg in arguments {
                    write!(f, "{arg}, ")?;
                }
                write!(f, "), ")?;
                match &**result_type {
                    Self::Void => write!(f, "()")?,
                    ty => write!(f, "{ty}")?,
                }
                write!(f, ">")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TyKind {
    MethodReturn { with_error: bool },
    FnReturn,
    Static,
    Typedef,
    MethodArgument { is_consumed: bool },
    FnArgument,
    Struct,
    Enum,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ty {
    ty: Inner,
    kind: TyKind,
}

impl Ty {
    pub const VOID_RESULT: Self = Self {
        ty: Inner::Void,
        kind: TyKind::MethodReturn { with_error: false },
    };

    pub fn parse_method_argument(ty: Type<'_>, is_consumed: bool, context: &Context<'_>) -> Self {
        let ty = Inner::parse(ty, Lifetime::Unspecified, context);

        match &ty {
            Inner::Pointer { pointee, .. } => pointee.visit_lifetime(|lifetime| {
                if lifetime != Lifetime::Autoreleasing {
                    error!(?ty, "unexpected lifetime in pointer argument");
                }
            }),
            Inner::IncompleteArray { pointee, .. } => pointee.visit_lifetime(|lifetime| {
                if lifetime != Lifetime::Unretained {
                    error!(?ty, "unexpected lifetime in incomplete array argument");
                }
            }),
            _ => ty.visit_lifetime(|lifetime| {
                if lifetime != Lifetime::Strong {
                    error!(?ty, "unexpected lifetime in argument");
                }
            }),
        }

        Self {
            ty,
            kind: TyKind::MethodArgument { is_consumed },
        }
    }

    pub fn parse_method_return(ty: Type<'_>, context: &Context<'_>) -> Self {
        let ty = Inner::parse(ty, Lifetime::Unspecified, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                error!(?ty, "unexpected lifetime in return");
            }
        });

        Self {
            ty,
            kind: TyKind::MethodReturn { with_error: false },
        }
    }

    pub fn parse_function_argument(ty: Type<'_>, context: &Context<'_>) -> Self {
        let mut this = Self::parse_method_argument(ty, false, context);
        this.kind = TyKind::FnArgument;
        this
    }

    pub fn parse_function_return(ty: Type<'_>, context: &Context<'_>) -> Self {
        let mut this = Self::parse_method_return(ty, context);
        this.kind = TyKind::FnReturn;
        this
    }

    pub fn parse_typedef(ty: Type<'_>, typedef_name: &str, context: &Context<'_>) -> Option<Self> {
        let mut ty = Inner::parse(ty, Lifetime::Unspecified, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                error!(?ty, "unexpected lifetime in typedef");
            }
        });

        match &mut ty {
            // Handled by Stmt::EnumDecl
            Inner::Enum { .. } => None,
            // Handled above and in Stmt::StructDecl
            Inner::Struct { id } if id.name == typedef_name => None,
            Inner::Struct { id } if id.name != typedef_name => {
                warn!(?id, "invalid struct in typedef");
                None
            }
            // Opaque structs
            Inner::Pointer { pointee, .. } if matches!(&**pointee, Inner::Struct { .. }) => {
                **pointee = Inner::Void;
                Some(Self {
                    ty,
                    kind: TyKind::Typedef,
                })
            }
            Inner::IncompleteArray { .. } => {
                unimplemented!("incomplete array in struct")
            }
            _ => Some(Self {
                ty,
                kind: TyKind::Typedef,
            }),
        }
    }

    pub fn parse_property(
        ty: Type<'_>,
        // Ignored; see `parse_property_return`
        _is_copy: bool,
        context: &Context<'_>,
    ) -> Self {
        let ty = Inner::parse(ty, Lifetime::Unspecified, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                error!(?ty, "unexpected lifetime in property");
            }
        });

        Self {
            ty,
            kind: TyKind::MethodArgument { is_consumed: false },
        }
    }

    pub fn parse_property_return(ty: Type<'_>, is_copy: bool, context: &Context<'_>) -> Self {
        let mut ty = Inner::parse(ty, Lifetime::Unspecified, context);

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
        // This unwrap is done by `#[method_id(...)]` when we specify the
        // return type as `Id`.
        if is_copy {
            match &mut ty {
                Inner::Id { nullability, .. } => {
                    if *nullability == Nullability::Unspecified {
                        *nullability = Nullability::NonNull;
                    }
                }
                Inner::Pointer {
                    nullability: Nullability::Nullable | Nullability::NonNull,
                    ..
                } => {
                    // Ignore pointers
                }
                _ => warn!(?ty, "property(copy) which is not an object"),
            }
        }

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                error!(?ty, "unexpected lifetime in property");
            }
        });

        Self {
            ty,
            kind: TyKind::MethodReturn { with_error: false },
        }
    }

    pub fn parse_struct_field(ty: Type<'_>, context: &Context<'_>) -> Self {
        let ty = Inner::parse(ty, Lifetime::Unspecified, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unretained {
                error!(?ty, "unexpected lifetime in struct field");
            }
        });

        Self {
            ty,
            kind: TyKind::Struct,
        }
    }

    pub fn parse_enum(ty: Type<'_>, context: &Context<'_>) -> Self {
        let ty = Inner::parse(ty, Lifetime::Unspecified, context);

        ty.visit_lifetime(|_lifetime| {
            error!(?ty, "unexpected lifetime in enum");
        });

        Self {
            ty,
            kind: TyKind::Enum,
        }
    }

    pub fn parse_static(ty: Type<'_>, context: &Context<'_>) -> Self {
        let ty = Inner::parse(ty, Lifetime::Unspecified, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Strong && lifetime != Lifetime::Unspecified {
                error!(?ty, "unexpected lifetime in var");
            }
        });

        Self {
            ty,
            kind: TyKind::Static,
        }
    }

    pub(crate) fn set_ownership(&mut self, mut get_ownership: impl FnMut(&str) -> Ownership) {
        assert!(matches!(self.kind, TyKind::MethodReturn { .. }));
        if let Inner::Id { ty, .. } = &mut self.ty {
            match ty {
                IdType::Class { id, ownership, .. } => {
                    *ownership = Some(get_ownership(&id.name));
                }
                IdType::Self_ { ownership } => {
                    *ownership = Some(get_ownership("Self"));
                }
                _ => {}
            }
        }
    }

    pub fn visit_required_types(&self, f: &mut impl FnMut(&ItemIdentifier)) {
        match &self.kind {
            TyKind::MethodReturn { with_error: true } => f(&ItemIdentifier::nserror()),
            _ => {}
        }

        self.ty.visit_required_types(f);
    }
}

impl Ty {
    pub fn argument_is_error_out(&self) -> bool {
        if let Inner::Pointer {
            nullability,
            is_const,
            pointee,
        } = &self.ty
        {
            if let Inner::Id {
                ty:
                    IdType::Class {
                        id,
                        params: TypeParams::Empty,
                        ownership: None,
                    },
                is_const: id_is_const,
                lifetime,
                nullability: id_nullability,
            } = &**pointee
            {
                if !id.is_nserror() {
                    return false;
                }
                assert_eq!(
                    *nullability,
                    Nullability::Nullable,
                    "invalid error nullability {self:?}"
                );
                assert!(!is_const, "expected error not const {self:?}");

                assert_eq!(
                    *id_nullability,
                    Nullability::Nullable,
                    "invalid inner error nullability {self:?}"
                );
                assert!(!id_is_const, "expected inner error not const {self:?}");
                assert_eq!(
                    *lifetime,
                    Lifetime::Autoreleasing,
                    "invalid error lifetime {self:?}"
                );
                return true;
            }
        }
        false
    }

    pub fn is_id(&self) -> bool {
        matches!(self.ty, Inner::Id { .. })
    }

    pub fn set_is_alloc(&mut self) {
        match &mut self.ty {
            Inner::Id {
                ty: ty @ IdType::Self_ { ownership: None },
                lifetime: Lifetime::Unspecified,
                is_const: false,
                nullability: _,
            } => {
                *ty = IdType::Allocated;
            }
            _ => error!(?self, "invalid alloc return type"),
        }
    }

    pub fn set_is_error(&mut self) {
        if let TyKind::MethodReturn { with_error } = &mut self.kind {
            *with_error = true;
        } else {
            panic!("invalid set_is_error usage");
        }
    }

    pub fn is_error(&self) -> bool {
        if let TyKind::MethodReturn { with_error } = &self.kind {
            *with_error
        } else {
            panic!("invalid set_is_error usage");
        }
    }

    pub fn is_instancetype(&self) -> bool {
        matches!(
            &self.ty,
            Inner::Id {
                ty: IdType::Self_ { .. },
                ..
            }
        )
    }

    pub fn is_typedef_to(&self, s: &str) -> bool {
        matches!(&self.ty, Inner::TypeDef { id } if id.name == s)
    }

    /// Related result types
    /// <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#related-result-types>
    pub fn fix_related_result_type(&mut self, is_class: bool, selector: &str) {
        if let Inner::Id {
            ty: ty @ IdType::AnyObject { .. },
            ..
        } = &mut self.ty
        {
            let is_related = if is_class {
                MemoryManagement::is_new(selector) || MemoryManagement::is_alloc(selector)
            } else {
                MemoryManagement::is_init(selector) || selector == "self"
            };

            if is_related {
                if let IdType::AnyObject { protocols } = &ty {
                    if !protocols.is_empty() {
                        warn!(?ty, "related result type with protocols");
                        return;
                    }
                }

                *ty = IdType::Self_ { ownership: None };
            }
        }
    }

    pub fn is_nsstring(&self) -> bool {
        if let Inner::Id {
            ty: IdType::Class { id, .. },
            ..
        } = &self.ty
        {
            id.name == "NSString"
        } else {
            false
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TyKind::MethodReturn { with_error: false } => {
                if let Inner::Void = &self.ty {
                    // Don't output anything
                    return Ok(());
                }

                write!(f, " -> ")?;

                match &self.ty {
                    Inner::Id {
                        ty,
                        // Ignore
                        is_const: _,
                        // Ignore
                        lifetime: _,
                        nullability,
                    } => {
                        if *nullability == Nullability::NonNull {
                            write!(f, "Id<{ty}, {}>", ty.ownership())
                        } else {
                            write!(f, "Option<Id<{ty}, {}>>", ty.ownership())
                        }
                    }
                    Inner::Class { nullability } => {
                        if *nullability == Nullability::NonNull {
                            write!(f, "&'static Class")
                        } else {
                            write!(f, "Option<&'static Class>")
                        }
                    }
                    Inner::C99Bool => {
                        panic!("C99's bool as Objective-C method return is unsupported")
                    }
                    Inner::ObjcBool => write!(f, "bool"),
                    ty => write!(f, "{ty}"),
                }
            }
            TyKind::MethodReturn { with_error: true } => match &self.ty {
                Inner::Id {
                    ty,
                    lifetime: Lifetime::Unspecified,
                    is_const: false,
                    nullability: Nullability::Nullable,
                } => {
                    // NULL -> error
                    write!(
                        f,
                        " -> Result<Id<{ty}, {}>, Id<{}, Shared>>",
                        ty.ownership(),
                        ItemIdentifier::nserror().path(),
                    )
                }
                Inner::ObjcBool => {
                    // NO -> error
                    write!(
                        f,
                        " -> Result<(), Id<{}, Shared>>",
                        ItemIdentifier::nserror().path()
                    )
                }
                _ => panic!("unknown error result type {self:?}"),
            },
            TyKind::Static => match &self.ty {
                Inner::Id {
                    ty,
                    // `const` is irrelevant in statics since they're always
                    // constant.
                    is_const: _,
                    lifetime: Lifetime::Strong | Lifetime::Unspecified,
                    nullability,
                } => {
                    if *nullability == Nullability::NonNull {
                        write!(f, "&'static {ty}")
                    } else {
                        write!(f, "Option<&'static {ty}>")
                    }
                }
                ty @ Inner::Id { .. } => panic!("invalid static {ty:?}"),
                ty => write!(f, "{ty}"),
            },
            TyKind::Typedef => match &self.ty {
                // When we encounter a typedef declaration like this:
                //     typedef NSString* NSAbc;
                //
                // We parse it as one of:
                //     type NSAbc = NSString;
                //     struct NSAbc(NSString);
                //
                // Instead of:
                //     type NSAbc = *const NSString;
                //
                // Because that means we can use ordinary Id<NSAbc> elsewhere.
                Inner::Id {
                    ty:
                        ty @ IdType::Class {
                            id,
                            params: TypeParams::Empty,
                            ownership: None,
                        },
                    is_const: _,
                    lifetime: _,
                    nullability: Nullability::Nullable | Nullability::Unspecified,
                } if id.is_nsstring() => {
                    write!(f, "{ty}")
                }
                Inner::Id {
                    ty: ty @ IdType::AnyObject { .. },
                    ..
                } => write!(f, "{ty}"),
                ty @ Inner::Id { .. } => {
                    panic!("typedef declaration was not NSString: {ty:?}");
                }
                ty => write!(f, "{ty}"),
            },
            TyKind::MethodArgument { is_consumed: _ } | TyKind::FnArgument => match &self.ty {
                Inner::Id {
                    ty,
                    is_const: false,
                    lifetime: Lifetime::Unspecified | Lifetime::Strong,
                    nullability,
                } => {
                    if *nullability == Nullability::NonNull {
                        write!(f, "&{ty}")
                    } else {
                        write!(f, "Option<&{ty}>")
                    }
                }
                Inner::Class { nullability } => {
                    if *nullability == Nullability::NonNull {
                        write!(f, "&Class")
                    } else {
                        write!(f, "Option<&Class>")
                    }
                }
                Inner::C99Bool if self.kind == TyKind::MethodArgument { is_consumed: false } => {
                    panic!("C99's bool as Objective-C method argument is unsupported")
                }
                Inner::ObjcBool if self.kind == TyKind::MethodArgument { is_consumed: false } => {
                    write!(f, "bool")
                }
                ty @ Inner::Pointer {
                    nullability,
                    is_const: false,
                    pointee,
                } => match &**pointee {
                    // TODO: Re-enable once we can support it
                    // Inner::Id {
                    //     ty,
                    //     is_const: false,
                    //     lifetime: Lifetime::Autoreleasing,
                    //     nullability: inner_nullability,
                    // } if self.kind == TyKind::MethodArgument => {
                    //     let tokens = if *inner_nullability == Nullability::NonNull {
                    //         format!("Id<{ty}, Shared>")
                    //     } else {
                    //         format!("Option<Id<{ty}, Shared>>")
                    //     };
                    //     if *nullability == Nullability::NonNull {
                    //         write!(f, "&mut {tokens}")
                    //     } else {
                    //         write!(f, "Option<&mut {tokens}>")
                    //     }
                    // }
                    // Inner::Id { .. } => {
                    //     unreachable!("there should be no id with other values: {self:?}")
                    // }
                    block @ Inner::Block { .. } => {
                        if *nullability == Nullability::NonNull {
                            write!(f, "&{block}")
                        } else {
                            write!(f, "Option<&{block}>")
                        }
                    }
                    _ => write!(f, "{ty}"),
                },
                ty => write!(f, "{ty}"),
            },
            TyKind::Struct => match &self.ty {
                Inner::Array {
                    element_type,
                    num_elements,
                } => write!(f, "[{element_type}; {num_elements}]"),
                ty => write!(f, "{ty}"),
            },
            TyKind::Enum => write!(f, "{}", self.ty),
            TyKind::FnReturn => {
                if let Inner::Void = &self.ty {
                    // Don't output anything
                    return Ok(());
                }

                write!(f, " -> {}", self.ty)
            }
        }
    }
}

/// Strip macros from unexposed types.
///
/// These appear in newer clang versions.
/// We should be able to extract data from the following macros if desired:
/// - NS_SWIFT_NAME
/// - NS_SWIFT_UNAVAILABLE
/// - NS_REFINED_FOR_SWIFT
/// - ...
fn parse_unexposed_tokens(s: &str) -> String {
    let tokens = TokenStream::from_str(s).expect("parse attributed name");
    let mut iter = tokens.into_iter().peekable();
    if let Some(TokenTree::Ident(ident)) = iter.peek() {
        let ident = ident.to_string();
        match &*ident {
            "NS_RETURNS_INNER_POINTER" | "NS_REFINED_FOR_SWIFT" => {
                iter.next();
            }
            "API_AVAILABLE"
            | "API_UNAVAILABLE"
            | "NS_SWIFT_NAME"
            | "API_DEPRECATED"
            | "API_DEPRECATED_WITH_REPLACEMENT" => {
                iter.next();
                if let Some(TokenTree::Group(_)) = iter.peek() {
                    iter.next();
                }
            }
            _ => {}
        }
    }
    TokenStream::from_iter(iter).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unexposed_tokens() {
        fn check(inp: &str, expected: &str) {
            assert_eq!(parse_unexposed_tokens(inp), expected);
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
    }
}
