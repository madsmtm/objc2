use std::fmt;

use clang::{CallingConvention, EntityKind, Nullability, Type, TypeKind};
use serde::Deserialize;
use tracing::{debug_span, error, warn};

use crate::context::Context;
use crate::method::MemoryManagement;

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
    Protocols(Vec<(String, String)>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum IdType {
    Class {
        library: String,
        name: String,
        params: TypeParams,
        ownership: Option<Ownership>,
    },
    TypeDef {
        library: String,
        name: String,
    },
    GenericParam {
        name: String,
    },
    AnyObject {
        protocols: Vec<(String, String)>,
    },
    AnyProtocol,
    AnyClass {
        protocols: Vec<(String, String)>,
    },
    Allocated,
    Self_ {
        ownership: Option<Ownership>,
    },
}

impl IdType {
    fn name(&self) -> &str {
        match self {
            Self::Class { name, .. } => name,
            Self::AnyObject { protocols } => match &**protocols {
                [] => "Object",
                [(_, name)] if name == "NSCopying" || name == "NSMutableCopying" => "Object",
                [(_, name)] => name,
                // TODO: Handle this better
                _ => "TodoProtocols",
            },
            Self::TypeDef { name, .. } => name,
            Self::GenericParam { name } => name,
            Self::AnyProtocol => "Protocol",
            // TODO: Handle this better
            Self::AnyClass { .. } => "TodoClass",
            Self::Allocated => "Allocated",
            Self::Self_ { .. } => "Self",
        }
    }

    #[allow(dead_code)]
    fn library(&self) -> Option<&str> {
        match self {
            Self::Class { library, .. } => Some(library),
            Self::AnyObject { protocols } => match &**protocols {
                [(_, name)] if name == "NSCopying" || name == "NSMutableCopying" => None,
                [(library, _)] => Some(library),
                _ => None,
            },
            Self::TypeDef { library, .. } => Some(library),
            _ => None,
        }
    }

    fn ownership(&self) -> Ownership {
        match self {
            Self::Class {
                ownership: Some(ownership),
                ..
            }
            | Self::Self_ {
                ownership: Some(ownership),
                ..
            } => ownership.clone(),
            _ => Ownership::Shared,
        }
    }

    fn parse_objc_pointer(ty: Type<'_>, context: &Context<'_>) -> Self {
        let generics: Vec<_> = ty
            .get_objc_type_arguments()
            .into_iter()
            .map(|param| {
                match RustType::parse(param, false, Nullability::Unspecified, false, context) {
                    RustType::Id {
                        ty,
                        is_const: _,
                        lifetime: _,
                        nullability: _,
                    } => ty,
                    RustType::Class { nullability: _ } => Self::AnyClass { protocols: vec![] },
                    param => {
                        panic!("invalid generic parameter {param:?} in {ty:?}")
                    }
                }
            })
            .collect();

        let protocols: Vec<_> = ty
            .get_objc_protocol_declarations()
            .into_iter()
            .map(|entity| {
                (
                    context
                        .get_library_and_file_name(&entity)
                        .expect("protocol library")
                        .0,
                    entity.get_display_name().expect("protocol name"),
                )
            })
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

                if name == "Protocol" {
                    Self::AnyProtocol
                } else {
                    let (library, _) = context
                        .get_library_and_file_name(
                            &ty.get_declaration().expect("ObjCInterface declaration"),
                        )
                        .expect("ObjCInterface declaration library");
                    Self::Class {
                        library,
                        name,
                        params: TypeParams::Empty,
                        ownership: None,
                    }
                }
            }
            TypeKind::ObjCObject => {
                let base_ty = ty
                    .get_objc_object_base_type()
                    .expect("object to have base type");
                let name = base_ty.get_display_name();

                match base_ty.get_kind() {
                    TypeKind::ObjCId => {
                        assert_eq!(name, "id");

                        if !generics.is_empty() {
                            panic!("generics not empty: {ty:?}, {generics:?}");
                        }

                        Self::AnyObject { protocols }
                    }
                    TypeKind::ObjCInterface => {
                        let (library, _) = context
                            .get_library_and_file_name(
                                &base_ty
                                    .get_declaration()
                                    .expect("ObjCObject -> ObjCInterface declaration"),
                            )
                            .expect("ObjCObject -> ObjCInterface declaration library");

                        if !generics.is_empty() && !protocols.is_empty() {
                            panic!("got object with both protocols and generics: {name:?}, {protocols:?}, {generics:?}");
                        }

                        if generics.is_empty() && protocols.is_empty() {
                            panic!("got object with empty protocols and generics: {name:?}");
                        }

                        Self::Class {
                            library,
                            name,
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
}

impl fmt::Display for IdType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())?;

        if let Self::Class {
            params: TypeParams::Generics(generics),
            ..
        } = &self
        {
            write!(f, "<")?;
            for generic in generics {
                write!(f, "{generic},")?;
            }
            write!(f, ">")?;
        }

        Ok(())
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

/// Parse attributes.
///
/// This is _very_ ugly, but required because libclang doesn't expose most
/// of these.
fn parse_attributed<'a>(
    ty: Type<'a>,
    nullability: &mut Nullability,
    lifetime: &mut Lifetime,
    kindof: &mut bool,
    inside_partial_array: bool,
) -> Type<'a> {
    let mut modified_ty = ty;
    while let TypeKind::Attributed = modified_ty.get_kind() {
        modified_ty = modified_ty
            .get_modified_type()
            .expect("attributed type to have modified type");
    }

    if modified_ty == ty {
        return ty;
    }

    let mut name = &*ty.get_display_name();
    let mut modified_name = &*modified_ty.get_display_name();

    fn get_inner_fn(name: &str) -> &str {
        let (_, name) = name.split_once('(').expect("fn to have begin parenthesis");
        let (name, _) = name.split_once(')').expect("fn to have end parenthesis");
        name.trim()
    }

    let parse_const = |name: &mut &str| {
        if ty.is_const_qualified() {
            if let Some(rest) = name.strip_suffix("const") {
                *name = rest.trim();
            }
            if !modified_ty.is_const_qualified() {
                // TODO: Fix this
                warn!("unnecessarily stripped const");
            }
        }
    };

    fn parse_lifetime<'a>(
        name: &mut &'a str,
        modified_name: &'a str,
        lifetime: &mut Lifetime,
        prefix: bool,
    ) {
        let strip_method = if prefix {
            str::strip_prefix
        } else {
            str::strip_suffix
        };

        let mut strip_lifetime = |stripped: &'static str| {
            // If the string is present in the name, and _not_ in the modified
            // name, then naturally we should strip it so that they match.
            if let Some(rest) = strip_method(name, stripped) {
                if strip_method(modified_name, stripped).is_none() {
                    *name = rest.trim();
                    return true;
                }
            }

            false
        };

        if strip_lifetime("__unsafe_unretained") {
            *lifetime = Lifetime::Unretained;
        } else if strip_lifetime("__strong") {
            *lifetime = Lifetime::Strong;
        } else if strip_lifetime("__weak") {
            *lifetime = Lifetime::Weak;
        } else if strip_lifetime("__autoreleasing") {
            *lifetime = Lifetime::Autoreleasing;
        }
    }

    let parse_nullability = |name: &mut &str, nullability: &mut Nullability| {
        if let Some(rest) = name.strip_suffix("_Nullable") {
            assert_eq!(
                ty.get_nullability(),
                Some(Nullability::Nullable),
                "nullable"
            );
            *nullability = Nullability::Nullable;
            *name = rest.trim();
        } else if let Some(rest) = name.strip_suffix("_Nonnull") {
            assert_eq!(ty.get_nullability(), Some(Nullability::NonNull), "nonnull");
            *nullability = match nullability {
                Nullability::Nullable => Nullability::Nullable,
                _ => Nullability::NonNull,
            };
            *name = rest.trim();
        } else if let Some(rest) = name.strip_suffix("_Null_unspecified") {
            assert_eq!(
                ty.get_nullability(),
                Some(Nullability::Unspecified),
                "unspecified"
            );
            // Do nothing
            *name = rest.trim();
        } else {
            assert_eq!(
                ty.get_nullability(),
                None,
                "expected no nullability attribute on {name:?}"
            );
        }
    };

    match modified_ty.get_kind() {
        TypeKind::ConstantArray => {
            let (res, _) = name.split_once('[').expect("array to end with [");
            name = res.trim();
            let (res, _) = modified_name.split_once('[').expect("array to end with [");
            modified_name = res.trim();
        }
        TypeKind::IncompleteArray => {
            name = name
                .strip_suffix("[]")
                .expect("array to end with []")
                .trim();
            modified_name = modified_name
                .strip_suffix("[]")
                .expect("array to end with []")
                .trim();
        }
        TypeKind::BlockPointer => {
            name = get_inner_fn(name);
            modified_name = get_inner_fn(modified_name);
        }
        TypeKind::Pointer => {
            if modified_ty
                .get_pointee_type()
                .expect("pointer to have pointee")
                .get_kind()
                == TypeKind::FunctionPrototype
            {
                name = get_inner_fn(name);
                modified_name = get_inner_fn(modified_name);
            }
        }
        _ => {}
    }

    parse_const(&mut name);
    if inside_partial_array {
        parse_lifetime(&mut name, modified_name, lifetime, true);
    }
    parse_lifetime(&mut name, modified_name, lifetime, false);
    parse_nullability(&mut name, nullability);
    parse_lifetime(&mut name, modified_name, lifetime, false);

    if name != modified_name {
        if let Some(rest) = name.strip_prefix("__kindof") {
            name = rest.trim();
            *kindof = true;
        }

        if name != modified_name {
            let original_name = ty.get_display_name();
            error!("attributes: {original_name:?} -> {name:?} != {modified_name:?}");
            panic!(
                "could not extract all attributes from attributed type. Inner: {ty:?}, {modified_ty:?}"
            );
        }
    }

    modified_ty
}

/// Strip macros from unexposed type.
///
/// These appear in newer clang versions.
/// We should be able to extract data from the following macros if desired:
/// - NS_SWIFT_NAME
/// - NS_SWIFT_UNAVAILABLE
/// - NS_REFINED_FOR_SWIFT
fn parse_unexposed<'a>(ty: Type<'a>, nullability: &mut Nullability) -> Type<'a> {
    let mut modified_ty = ty;
    while let TypeKind::Unexposed = modified_ty.get_kind() {
        modified_ty = modified_ty
            .get_modified_type()
            .expect("attributed type to have modified type");
    }

    if modified_ty == ty {
        return ty;
    }

    if let Some(new @ (Nullability::NonNull | Nullability::Nullable)) = ty.get_nullability() {
        *nullability = new;
    }

    modified_ty
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum RustType {
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
        pointee: Box<RustType>,
    },
    IncompleteArray {
        nullability: Nullability,
        is_const: bool,
        pointee: Box<RustType>,
    },
    Array {
        element_type: Box<RustType>,
        num_elements: usize,
    },
    Enum {
        name: String,
    },
    Struct {
        name: String,
    },
    Fn {
        is_variadic: bool,
        arguments: Vec<Ty>,
        result_type: Box<Ty>,
    },
    Block {
        arguments: Vec<Ty>,
        result_type: Box<Ty>,
    },

    TypeDef {
        name: String,
    },
}

impl RustType {
    fn parse(
        ty: Type<'_>,
        is_consumed: bool,
        mut nullability: Nullability,
        inside_partial_array: bool,
        context: &Context<'_>,
    ) -> Self {
        let _span = debug_span!("ty", ?ty, is_consumed);

        // debug!("{:?}, {:?}", ty, ty.get_class_type());

        let mut kindof = false;
        let mut lifetime = Lifetime::Unspecified;
        let ty = parse_attributed(
            ty,
            &mut nullability,
            &mut lifetime,
            &mut kindof,
            inside_partial_array,
        );

        let ty = parse_unexposed(ty, &mut nullability);

        // debug!("{:?}: {:?}", ty.get_kind(), ty.get_display_name());

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
            ObjCId => Self::Id {
                ty: IdType::AnyObject { protocols: vec![] },
                is_const: ty.is_const_qualified(),
                lifetime,
                nullability,
            },
            ObjCClass => Self::Class { nullability },
            ObjCSel => Self::Sel { nullability },
            Pointer => {
                let is_const = ty.is_const_qualified();
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                let pointee =
                    Self::parse(ty, is_consumed, Nullability::Unspecified, false, context);
                Self::Pointer {
                    nullability,
                    is_const,
                    pointee: Box::new(pointee),
                }
            }
            BlockPointer => {
                let is_const = ty.is_const_qualified();
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                match Self::parse(ty, is_consumed, Nullability::Unspecified, false, context) {
                    Self::Fn {
                        is_variadic: false,
                        mut arguments,
                        mut result_type,
                    } => {
                        for arg in &mut arguments {
                            arg.set_block();
                        }
                        result_type.set_block();
                        Self::Pointer {
                            nullability,
                            is_const,
                            pointee: Box::new(Self::Block {
                                arguments,
                                result_type,
                            }),
                        }
                    }
                    pointee => panic!("unexpected pointee in block: {pointee:?}"),
                }
            }
            ObjCObjectPointer => {
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                let mut kindof = false;
                let ty = parse_attributed(ty, &mut nullability, &mut lifetime, &mut kindof, false);

                Self::Id {
                    ty: IdType::parse_objc_pointer(ty, context),
                    is_const: ty.is_const_qualified(),
                    lifetime,
                    nullability,
                }
            }
            Typedef => {
                let typedef_name = ty.get_typedef_name().expect("typedef has name");
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
                        is_const: ty.is_const_qualified(),
                        lifetime,
                        nullability,
                    },
                    _ => {
                        let declaration = ty.get_declaration();
                        let ty = ty.get_canonical_type();
                        match ty.get_kind() {
                            ObjCObjectPointer => {
                                let ty =
                                    ty.get_pointee_type().expect("pointer type to have pointee");
                                let declaration =
                                    declaration.expect("typedef ObjCObjectPointer declaration");
                                let is_const = ty.is_const_qualified();

                                assert!(
                                    ty.get_objc_type_arguments().is_empty(),
                                    "typedef generics not empty"
                                );

                                let ty = if let EntityKind::TemplateTypeParameter =
                                    declaration.get_kind()
                                {
                                    IdType::GenericParam { name: typedef_name }
                                } else {
                                    let (library, _) = context
                                        .get_library_and_file_name(&declaration)
                                        .expect("ObjCObjectPointer library");
                                    IdType::TypeDef {
                                        library,
                                        name: typedef_name,
                                    }
                                };

                                Self::Id {
                                    ty,
                                    is_const,
                                    lifetime,
                                    nullability,
                                }
                            }
                            _ => Self::TypeDef { name: typedef_name },
                        }
                    }
                }
            }
            Elaborated => {
                let ty = ty.get_elaborated_type().expect("elaborated");
                match ty.get_kind() {
                    TypeKind::Record => {
                        let name = ty
                            .get_display_name()
                            .trim_start_matches("struct ")
                            .to_string();
                        Self::Struct { name }
                    }
                    TypeKind::Enum => {
                        let name = ty
                            .get_display_name()
                            .trim_start_matches("enum ")
                            .to_string();
                        Self::Enum { name }
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
                    .map(|ty| Ty::parse_fn_argument(ty, context))
                    .collect();

                let result_type = ty.get_result_type().expect("fn type to have result type");
                let result_type = Ty::parse_fn_result(result_type, context);

                Self::Fn {
                    is_variadic: ty.is_variadic(),
                    arguments,
                    result_type: Box::new(result_type),
                }
            }
            IncompleteArray => {
                let is_const = ty.is_const_qualified();
                let ty = ty
                    .get_element_type()
                    .expect("incomplete array to have element type");
                let pointee = Self::parse(ty, is_consumed, Nullability::Unspecified, true, context);
                Self::IncompleteArray {
                    nullability,
                    is_const,
                    pointee: Box::new(pointee),
                }
            }
            ConstantArray => {
                let element_type = Self::parse(
                    ty.get_element_type().expect("array to have element type"),
                    is_consumed,
                    Nullability::Unspecified,
                    false,
                    context,
                );
                let num_elements = ty
                    .get_size()
                    .expect("constant array to have element length");
                Self::Array {
                    element_type: Box::new(element_type),
                    num_elements,
                }
            }
            _ => {
                panic!("Unsupported type: {ty:?}")
            }
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
}

/// This is sound to output in (almost, c_void is not a valid return type) any
/// context. `Ty` is then used to change these types into something nicer when
/// required.
impl fmt::Display for RustType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RustType::*;
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
                    write!(f, "){result_type}")?;

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
            Enum { name } | Struct { name } | TypeDef { name } => write!(f, "{name}"),
            Self::Fn { .. } => write!(f, "TodoFunction"),
            Block {
                arguments,
                result_type,
            } => {
                write!(f, "Block<(")?;
                for arg in arguments {
                    write!(f, "{arg}, ")?;
                }
                write!(f, "), {result_type}>")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TyKind {
    MethodReturn,
    FnDeclReturn,
    MethodReturnWithError,
    Static,
    Typedef,
    MethodArgument,
    FnDeclArgument,
    Struct,
    Enum,
    FnArgument,
    FnReturn,
    BlockArgument,
    BlockReturn,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ty {
    ty: RustType,
    kind: TyKind,
}

impl Ty {
    pub const VOID_RESULT: Self = Self {
        ty: RustType::Void,
        kind: TyKind::MethodReturn,
    };

    pub fn parse_method_argument(ty: Type<'_>, is_consumed: bool, context: &Context<'_>) -> Self {
        let ty = RustType::parse(ty, is_consumed, Nullability::Unspecified, false, context);

        match &ty {
            RustType::Pointer { pointee, .. } => pointee.visit_lifetime(|lifetime| {
                if lifetime != Lifetime::Autoreleasing && lifetime != Lifetime::Unspecified {
                    panic!("unexpected lifetime {lifetime:?} in pointer argument {ty:?}");
                }
            }),
            RustType::IncompleteArray { pointee, .. } => pointee.visit_lifetime(|lifetime| {
                if lifetime != Lifetime::Unretained && lifetime != Lifetime::Unspecified {
                    panic!("unexpected lifetime {lifetime:?} in incomplete array argument {ty:?}");
                }
            }),
            _ => ty.visit_lifetime(|lifetime| {
                if lifetime != Lifetime::Strong && lifetime != Lifetime::Unspecified {
                    panic!("unexpected lifetime {lifetime:?} in argument {ty:?}");
                }
            }),
        }

        Self {
            ty,
            kind: TyKind::MethodArgument,
        }
    }

    pub fn parse_method_return(ty: Type<'_>, context: &Context<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in return {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::MethodReturn,
        }
    }

    pub fn parse_function_argument(ty: Type<'_>, context: &Context<'_>) -> Self {
        let mut this = Self::parse_method_argument(ty, false, context);
        this.kind = TyKind::FnDeclArgument;
        this
    }

    pub fn parse_function_return(ty: Type<'_>, context: &Context<'_>) -> Self {
        let mut this = Self::parse_method_return(ty, context);
        this.kind = TyKind::FnDeclReturn;
        this
    }

    pub fn parse_typedef(ty: Type<'_>, typedef_name: &str, context: &Context<'_>) -> Option<Self> {
        let mut ty = RustType::parse(ty, false, Nullability::Unspecified, false, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in typedef {ty:?}");
            }
        });

        match &mut ty {
            // Handled by Stmt::EnumDecl
            RustType::Enum { .. } => None,
            // Handled above and in Stmt::StructDecl
            RustType::Struct { name } if name == typedef_name => None,
            RustType::Struct { name } if name == typedef_name => {
                warn!(name, "invalid struct in typedef");
                None
            }
            // Opaque structs
            RustType::Pointer { pointee, .. } if matches!(&**pointee, RustType::Struct { .. }) => {
                **pointee = RustType::Void;
                Some(Self {
                    ty,
                    kind: TyKind::Typedef,
                })
            }
            RustType::IncompleteArray { .. } => {
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
        default_nullability: Nullability,
        context: &Context<'_>,
    ) -> Self {
        let ty = RustType::parse(ty, false, default_nullability, false, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in property {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::MethodArgument,
        }
    }

    pub fn parse_property_return(
        ty: Type<'_>,
        default_nullability: Nullability,
        context: &Context<'_>,
    ) -> Self {
        let ty = RustType::parse(ty, false, default_nullability, false, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in property {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::MethodReturn,
        }
    }

    pub fn parse_struct_field(ty: Type<'_>, context: &Context<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in struct field {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::Struct,
        }
    }

    pub fn parse_enum(ty: Type<'_>, context: &Context<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false, context);

        ty.visit_lifetime(|_lifetime| {
            panic!("unexpected lifetime in enum {ty:?}");
        });

        Self {
            ty,
            kind: TyKind::Enum,
        }
    }

    pub fn parse_static(ty: Type<'_>, context: &Context<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Strong && lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in var {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::Static,
        }
    }

    fn parse_fn_argument(ty: Type<'_>, context: &Context<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Strong {
                panic!("unexpected lifetime {lifetime:?} in fn argument {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::FnArgument,
        }
    }

    fn parse_fn_result(ty: Type<'_>, context: &Context<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false, context);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime {lifetime:?} in fn result {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::FnReturn,
        }
    }

    fn set_block(&mut self) {
        self.kind = match self.kind {
            TyKind::FnArgument => TyKind::BlockArgument,
            TyKind::FnReturn => TyKind::BlockReturn,
            _ => unreachable!("set block kind"),
        }
    }

    pub(crate) fn set_ownership(&mut self, mut get_ownership: impl FnMut(&str) -> Ownership) {
        assert!(matches!(
            self.kind,
            TyKind::MethodReturn | TyKind::MethodReturnWithError
        ));
        if let RustType::Id { ty, .. } = &mut self.ty {
            match ty {
                IdType::Class {
                    ownership, name, ..
                } => {
                    *ownership = Some(get_ownership(name));
                }
                IdType::Self_ { ownership } => {
                    *ownership = Some(get_ownership("Self"));
                }
                _ => {}
            }
        }
    }
}

impl Ty {
    pub fn argument_is_error_out(&self) -> bool {
        if let RustType::Pointer {
            nullability,
            is_const,
            pointee,
        } = &self.ty
        {
            if let RustType::Id {
                ty:
                    IdType::Class {
                        library,
                        name,
                        params: TypeParams::Empty,
                        ownership: None,
                    },
                is_const: id_is_const,
                lifetime,
                nullability: id_nullability,
            } = &**pointee
            {
                if name != "NSError" {
                    return false;
                }
                assert_eq!(*library, "Foundation", "invalid error library {self:?}");
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
                assert!(
                    matches!(lifetime, Lifetime::Unspecified | Lifetime::Autoreleasing),
                    "invalid error lifetime {self:?}"
                );
                return true;
            }
        }
        false
    }

    pub fn is_id(&self) -> bool {
        matches!(self.ty, RustType::Id { .. })
    }

    pub fn set_is_alloc(&mut self) {
        match &mut self.ty {
            RustType::Id {
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
        assert_eq!(self.kind, TyKind::MethodReturn);
        self.kind = TyKind::MethodReturnWithError;
    }

    pub fn is_error(&self) -> bool {
        match &self.kind {
            TyKind::MethodReturn => false,
            TyKind::MethodReturnWithError => true,
            _ => panic!("invalid is_error usage"),
        }
    }

    pub fn is_instancetype(&self) -> bool {
        matches!(
            &self.ty,
            RustType::Id {
                ty: IdType::Self_ { .. },
                ..
            }
        )
    }

    pub fn is_typedef_to(&self, s: &str) -> bool {
        matches!(&self.ty, RustType::TypeDef { name } if name == s)
    }

    /// Related result types
    /// <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#related-result-types>
    pub fn fix_related_result_type(&mut self, is_class: bool, selector: &str) {
        if let RustType::Id {
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
        if let RustType::Id {
            ty: IdType::Class { name, .. },
            ..
        } = &self.ty
        {
            name == "NSString"
        } else {
            false
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TyKind::MethodReturn => {
                if let RustType::Void = &self.ty {
                    // Don't output anything
                    return Ok(());
                }

                write!(f, " -> ")?;

                match &self.ty {
                    RustType::Id {
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
                    RustType::Class { nullability } => {
                        if *nullability == Nullability::NonNull {
                            write!(f, "&'static Class")
                        } else {
                            write!(f, "Option<&'static Class>")
                        }
                    }
                    RustType::C99Bool => {
                        panic!("C99's bool as Objective-C method return is unsupported")
                    }
                    RustType::ObjcBool => write!(f, "bool"),
                    ty => write!(f, "{ty}"),
                }
            }
            TyKind::MethodReturnWithError => match &self.ty {
                RustType::Id {
                    ty,
                    lifetime: Lifetime::Unspecified,
                    is_const: false,
                    nullability: Nullability::Nullable,
                } => {
                    // NULL -> error
                    write!(
                        f,
                        " -> Result<Id<{ty}, {}>, Id<NSError, Shared>>",
                        ty.ownership()
                    )
                }
                RustType::ObjcBool => {
                    // NO -> error
                    write!(f, " -> Result<(), Id<NSError, Shared>>")
                }
                _ => panic!("unknown error result type {self:?}"),
            },
            TyKind::Static => match &self.ty {
                RustType::Id {
                    ty,
                    is_const: false,
                    lifetime: Lifetime::Strong | Lifetime::Unspecified,
                    nullability,
                } => {
                    if *nullability == Nullability::NonNull {
                        write!(f, "&'static {ty}")
                    } else {
                        write!(f, "Option<&'static {ty}>")
                    }
                }
                ty @ RustType::Id { .. } => panic!("invalid static {ty:?}"),
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
                RustType::Id {
                    ty:
                        ty @ IdType::Class {
                            library,
                            name,
                            params: TypeParams::Empty,
                            ownership: None,
                        },
                    is_const: _,
                    lifetime: _,
                    nullability: Nullability::Nullable | Nullability::Unspecified,
                } if library == "Foundation" && name == "NSString" => {
                    write!(f, "{ty}")
                }
                RustType::Id {
                    ty: ty @ IdType::AnyObject { .. },
                    ..
                } => write!(f, "{ty}"),
                ty @ RustType::Id { .. } => {
                    panic!("typedef declaration was not NSString: {ty:?}");
                }
                ty => write!(f, "{ty}"),
            },
            TyKind::MethodArgument | TyKind::FnDeclArgument => match &self.ty {
                RustType::Id {
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
                RustType::Class { nullability } => {
                    if *nullability == Nullability::NonNull {
                        write!(f, "&Class")
                    } else {
                        write!(f, "Option<&Class>")
                    }
                }
                RustType::C99Bool if self.kind == TyKind::MethodArgument => {
                    panic!("C99's bool as Objective-C method argument is unsupported")
                }
                RustType::ObjcBool if self.kind == TyKind::MethodArgument => write!(f, "bool"),
                ty @ RustType::Pointer {
                    nullability,
                    is_const: false,
                    pointee,
                } => match &**pointee {
                    // TODO: Re-enable once we can support it
                    // RustType::Id {
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
                    // RustType::Id { .. } => {
                    //     unreachable!("there should be no id with other values: {self:?}")
                    // }
                    block @ RustType::Block { .. } => {
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
                RustType::Array {
                    element_type,
                    num_elements,
                } => write!(f, "[{element_type}; {num_elements}]"),
                ty => write!(f, "{ty}"),
            },
            TyKind::Enum => write!(f, "{}", self.ty),
            TyKind::FnArgument | TyKind::BlockArgument => write!(f, "{}", self.ty),
            TyKind::FnDeclReturn | TyKind::FnReturn => {
                if let RustType::Void = &self.ty {
                    // Don't output anything
                    return Ok(());
                }

                write!(f, " -> {}", self.ty)
            }
            TyKind::BlockReturn => match &self.ty {
                RustType::Void => write!(f, "()"),
                ty => write!(f, "{ty}"),
            },
        }
    }
}
