use std::fmt;

use clang::{CallingConvention, Nullability, Type, TypeKind};

use crate::method::MemoryManagement;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericType {
    pub name: String,
    pub generics: Vec<GenericType>,
}

impl GenericType {
    pub fn parse_objc_pointer(ty: Type<'_>) -> Self {
        match ty.get_kind() {
            TypeKind::ObjCInterface => {
                let generics = ty.get_objc_type_arguments();
                if !generics.is_empty() {
                    panic!("generics not empty: {ty:?}, {generics:?}");
                }
                let protocols = ty.get_objc_protocol_declarations();
                if !protocols.is_empty() {
                    panic!("protocols not empty: {ty:?}, {protocols:?}");
                }
                let name = ty.get_display_name();
                Self {
                    name,
                    generics: Vec::new(),
                }
            }
            TypeKind::ObjCObject => {
                let base_ty = ty
                    .get_objc_object_base_type()
                    .expect("object to have base type");
                let mut name = base_ty.get_display_name();

                let generics: Vec<_> = ty
                    .get_objc_type_arguments()
                    .into_iter()
                    .map(|param| {
                        match RustType::parse(param, false, Nullability::Unspecified, false) {
                            RustType::Id {
                                type_,
                                is_const: _,
                                lifetime: _,
                                nullability: _,
                            } => type_,
                            // TODO: Handle this better
                            RustType::Class { nullability: _ } => Self {
                                name: "TodoClass".to_string(),
                                generics: Vec::new(),
                            },
                            param => {
                                panic!("invalid generic parameter {:?} in {:?}", param, ty)
                            }
                        }
                    })
                    .collect();

                let protocols: Vec<_> = ty
                    .get_objc_protocol_declarations()
                    .into_iter()
                    .map(|entity| entity.get_display_name().expect("protocol name"))
                    .collect();
                if !protocols.is_empty() {
                    if name == "id" && generics.is_empty() && protocols.len() == 1 {
                        name = protocols[0].clone();
                    } else {
                        name = "TodoProtocols".to_string();
                    }
                }

                Self { name, generics }
            }
            _ => panic!("pointee was neither objcinterface nor objcobject: {ty:?}"),
        }
    }
}

impl fmt::Display for GenericType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.generics.is_empty() {
            write!(f, "<")?;
            for generic in &self.generics {
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
    let mut modified_ty = ty.clone();
    while modified_ty.get_kind() == TypeKind::Attributed {
        // println!("{ty:?}, {modified_ty:?}");
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

    match modified_ty.get_kind() {
        TypeKind::ConstantArray => {
            let (res, _) = name.split_once("[").expect("array to end with [");
            name = res.trim();
            let (res, _) = modified_name.split_once("[").expect("array to end with [");
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

    if ty.is_const_qualified() {
        if let Some(rest) = name.strip_suffix("const") {
            name = rest.trim();
        }
        if !modified_ty.is_const_qualified() {
            // TODO: Fix this
            println!("unnecessarily stripped const");
        }
    }

    if inside_partial_array {
        if let Some(rest) = name.strip_prefix("__unsafe_unretained") {
            *lifetime = Lifetime::Unretained;
            name = rest.trim();
        }
    }

    if let Some(rest) = name.strip_suffix("__unsafe_unretained") {
        *lifetime = Lifetime::Unretained;
        name = rest.trim();
    } else if let Some(rest) = name.strip_suffix("__strong") {
        *lifetime = Lifetime::Strong;
        name = rest.trim();
    } else if let Some(rest) = name.strip_suffix("__weak") {
        *lifetime = Lifetime::Weak;
        name = rest.trim();
    } else if let Some(rest) = name.strip_suffix("__autoreleasing") {
        *lifetime = Lifetime::Autoreleasing;
        name = rest.trim();
    }

    if let Some(rest) = name.strip_suffix("_Nullable") {
        assert_eq!(
            ty.get_nullability(),
            Some(Nullability::Nullable),
            "nullable"
        );
        *nullability = Nullability::Nullable;
        name = rest.trim();
    } else if let Some(rest) = name.strip_suffix("_Nonnull") {
        assert_eq!(ty.get_nullability(), Some(Nullability::NonNull), "nonnull");
        *nullability = match nullability {
            Nullability::Nullable => Nullability::Nullable,
            _ => Nullability::NonNull,
        };
        name = rest.trim();
    } else if let Some(rest) = name.strip_suffix("_Null_unspecified") {
        assert_eq!(
            ty.get_nullability(),
            Some(Nullability::Unspecified),
            "unspecified"
        );
        // Do nothing
        name = rest.trim();
    } else {
        assert_eq!(
            ty.get_nullability(),
            None,
            "expected no nullability attribute on {name:?}"
        );
    }

    if name != modified_name {
        if let Some(rest) = name.strip_prefix("__kindof") {
            name = rest.trim();
            *kindof = true;
        }

        if name != modified_name {
            let original_name = ty.get_display_name();
            println!("attributes: {original_name:?} -> {name:?} != {modified_name:?}");
            panic!(
                "could not extract all attributes from attributed type. Inner: {ty:?}, {modified_ty:?}"
            );
        }
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

    // Objective-C
    Id {
        type_: GenericType,
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
    ) -> Self {
        use TypeKind::*;

        // println!("{:?}, {:?}", ty, ty.get_class_type());

        let mut kindof = false;
        let mut lifetime = Lifetime::Unspecified;
        let ty = parse_attributed(
            ty,
            &mut nullability,
            &mut lifetime,
            &mut kindof,
            inside_partial_array,
        );

        // println!("{:?}: {:?}", ty.get_kind(), ty.get_display_name());

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
                type_: GenericType {
                    name: "Object".to_string(),
                    generics: Vec::new(),
                },
                is_const: ty.is_const_qualified(),
                lifetime,
                nullability,
            },
            ObjCClass => Self::Class { nullability },
            ObjCSel => Self::Sel { nullability },
            Pointer => {
                let is_const = ty.is_const_qualified();
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                let pointee = Self::parse(ty, is_consumed, Nullability::Unspecified, false);
                Self::Pointer {
                    nullability,
                    is_const,
                    pointee: Box::new(pointee),
                }
            }
            BlockPointer => {
                let is_const = ty.is_const_qualified();
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                match Self::parse(ty, is_consumed, Nullability::Unspecified, false) {
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
                let type_ = GenericType::parse_objc_pointer(ty);

                Self::Id {
                    type_,
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
                        type_: GenericType {
                            name: "Self".to_string(),
                            generics: Vec::new(),
                        },
                        is_const: ty.is_const_qualified(),
                        lifetime,
                        nullability,
                    },
                    _ => {
                        let ty = ty.get_canonical_type();
                        match ty.get_kind() {
                            ObjCObjectPointer => {
                                let ty =
                                    ty.get_pointee_type().expect("pointer type to have pointee");
                                let type_ = GenericType::parse_objc_pointer(ty);
                                if !type_.generics.is_empty() {
                                    panic!("typedef generics not empty");
                                }

                                Self::Id {
                                    type_: GenericType {
                                        name: typedef_name,
                                        generics: Vec::new(),
                                    },
                                    is_const: ty.is_const_qualified(),
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
                    .map(Ty::parse_fn_argument)
                    .collect();

                let result_type = ty.get_result_type().expect("fn type to have result type");
                let result_type = Ty::parse_fn_result(result_type);

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
                let pointee = Self::parse(ty, is_consumed, Nullability::Unspecified, true);
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
                panic!("Unsupported type: {:?}", ty)
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
/// requires.
impl fmt::Display for RustType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use RustType::*;
        match self {
            // Primitives
            Void => write!(f, "c_void"),
            C99Bool => panic!("C99's bool is unsupported"), // write!(f, "bool")
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

            // Objective-C
            Id {
                type_: ty,
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
            } => write!(f, "[{element_type}; {num_elements}]"),
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
    InMethodReturn,
    InFnDeclReturn,
    InMethodReturnWithError,
    InStatic,
    InTypedef,
    InMethodArgument,
    InFnDeclArgument,
    InStructEnum,
    InFnArgument,
    InFnReturn,
    InBlockArgument,
    InBlockReturn,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ty {
    ty: RustType,
    kind: TyKind,
}

impl Ty {
    pub const VOID_RESULT: Self = Self {
        ty: RustType::Void,
        kind: TyKind::InMethodReturn,
    };

    pub fn parse_method_argument(ty: Type<'_>, is_consumed: bool) -> Self {
        let ty = RustType::parse(ty, is_consumed, Nullability::Unspecified, false);

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
            kind: TyKind::InMethodArgument,
        }
    }

    pub fn parse_method_return(ty: Type<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in return {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::InMethodReturn,
        }
    }

    pub fn parse_function_argument(ty: Type<'_>) -> Self {
        let mut this = Self::parse_method_argument(ty, false);
        this.kind = TyKind::InFnDeclArgument;
        this
    }

    pub fn parse_function_return(ty: Type<'_>) -> Self {
        let mut this = Self::parse_method_return(ty);
        this.kind = TyKind::InFnDeclReturn;
        this
    }

    pub fn parse_typedef(ty: Type<'_>) -> Option<Self> {
        let mut ty = RustType::parse(ty, false, Nullability::Unspecified, false);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in typedef {ty:?}");
            }
        });

        match &mut ty {
            // Handled by Stmt::EnumDecl
            RustType::Enum { .. } => None,
            // Handled above and in Stmt::StructDecl
            // The rest is only `NSZone`
            RustType::Struct { name } => {
                assert_eq!(name, "_NSZone", "invalid struct in typedef");
                None
            }
            // Opaque structs
            RustType::Pointer { pointee, .. } if matches!(&**pointee, RustType::Struct { .. }) => {
                **pointee = RustType::Void;
                Some(Self {
                    ty,
                    kind: TyKind::InTypedef,
                })
            }
            RustType::IncompleteArray { .. } => {
                unimplemented!("incomplete array in struct")
            }
            _ => Some(Self {
                ty,
                kind: TyKind::InTypedef,
            }),
        }
    }

    pub fn parse_property(ty: Type<'_>, default_nullability: Nullability) -> Self {
        let ty = RustType::parse(ty, false, default_nullability, false);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in property {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::InMethodArgument,
        }
    }

    pub fn parse_property_return(ty: Type<'_>, default_nullability: Nullability) -> Self {
        let ty = RustType::parse(ty, false, default_nullability, false);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in property {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::InMethodReturn,
        }
    }

    pub fn parse_struct_field(ty: Type<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in struct field {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::InStructEnum,
        }
    }

    pub fn parse_enum(ty: Type<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false);

        ty.visit_lifetime(|_lifetime| {
            panic!("unexpected lifetime in enum {ty:?}");
        });

        Self {
            ty,
            kind: TyKind::InStructEnum,
        }
    }

    pub fn parse_static(ty: Type<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Strong && lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in var {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::InStatic,
        }
    }

    fn parse_fn_argument(ty: Type<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Strong {
                panic!("unexpected lifetime {lifetime:?} in fn argument {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::InFnArgument,
        }
    }

    fn parse_fn_result(ty: Type<'_>) -> Self {
        let ty = RustType::parse(ty, false, Nullability::Unspecified, false);

        ty.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime {lifetime:?} in fn result {ty:?}");
            }
        });

        Self {
            ty,
            kind: TyKind::InFnReturn,
        }
    }

    fn set_block(&mut self) {
        self.kind = match self.kind {
            TyKind::InFnArgument => TyKind::InBlockArgument,
            TyKind::InFnReturn => TyKind::InBlockReturn,
            _ => unreachable!("set block kind"),
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
                type_: ty,
                is_const: id_is_const,
                lifetime,
                nullability: id_nullability,
            } = &**pointee
            {
                if ty.name != "NSError" {
                    return false;
                }
                assert_eq!(
                    *nullability,
                    Nullability::Nullable,
                    "invalid error nullability {self:?}"
                );
                assert!(!is_const, "expected error not const {self:?}");

                assert!(
                    ty.generics.is_empty(),
                    "expected error generics to be empty {self:?}"
                );
                assert_eq!(
                    *id_nullability,
                    Nullability::Nullable,
                    "invalid inner error nullability {self:?}"
                );
                assert!(!id_is_const, "expected inner error not const {self:?}");
                assert_eq!(
                    *lifetime,
                    Lifetime::Unspecified,
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
                type_: ty,
                lifetime: Lifetime::Unspecified,
                is_const: false,
                nullability: Nullability::NonNull,
            } if ty.name == "Self" && ty.generics.is_empty() => {
                ty.name = "Allocated".into();
                ty.generics = vec![GenericType {
                    name: "Self".into(),
                    generics: vec![],
                }];
            }
            _ => panic!("invalid alloc return type {self:?}"),
        }
    }

    pub fn set_is_error(&mut self) {
        assert_eq!(self.kind, TyKind::InMethodReturn);
        self.kind = TyKind::InMethodReturnWithError;
    }

    /// Related result types
    /// https://clang.llvm.org/docs/AutomaticReferenceCounting.html#related-result-types
    pub fn fix_related_result_type(&mut self, is_class: bool, selector: &str) {
        if let RustType::Id { type_, .. } = &mut self.ty {
            if type_.name == "Object" {
                assert!(type_.generics.is_empty(), "Object return generics empty");
                if (is_class && MemoryManagement::is_new(&selector))
                    || (is_class && MemoryManagement::is_alloc(&selector))
                    || (!is_class && MemoryManagement::is_init(&selector))
                    || (!is_class && selector == "self")
                {
                    type_.name = "Self".into();
                }
            }
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            TyKind::InMethodReturn => {
                if let RustType::Void = &self.ty {
                    // Don't output anything
                    return Ok(());
                }

                write!(f, " -> ")?;

                match &self.ty {
                    RustType::Id {
                        type_: ty,
                        // Ignore
                        is_const: _,
                        // Ignore
                        lifetime: _,
                        nullability,
                    } => {
                        if *nullability == Nullability::NonNull {
                            write!(f, "Id<{ty}, Shared>")
                        } else {
                            write!(f, "Option<Id<{ty}, Shared>>")
                        }
                    }
                    RustType::Class { nullability } => {
                        if *nullability == Nullability::NonNull {
                            write!(f, "&'static Class")
                        } else {
                            write!(f, "Option<&'static Class>")
                        }
                    }
                    RustType::ObjcBool => write!(f, "bool"),
                    ty => write!(f, "{ty}"),
                }
            }
            TyKind::InMethodReturnWithError => match &self.ty {
                RustType::Id {
                    type_: ty,
                    lifetime: Lifetime::Unspecified,
                    is_const: false,
                    nullability: Nullability::Nullable,
                } => {
                    // NULL -> error
                    write!(f, " -> Result<Id<{ty}, Shared>, Id<NSError, Shared>>")
                }
                RustType::ObjcBool => {
                    // NO -> error
                    write!(f, " -> Result<(), Id<NSError, Shared>>")
                }
                _ => panic!("unknown error result type {self:?}"),
            },
            TyKind::InStatic => match &self.ty {
                RustType::Id {
                    type_: ty,
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
            TyKind::InTypedef => match &self.ty {
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
                    type_: ty,
                    is_const: _,
                    lifetime: _,
                    nullability,
                } => {
                    match &*ty.name {
                        "NSString" => {}
                        "NSUnit" => {}        // TODO: Handle this differently
                        "TodoProtocols" => {} // TODO
                        _ => panic!("typedef declaration was not NSString: {ty:?}"),
                    }

                    if !ty.generics.is_empty() {
                        panic!("typedef declaration generics not empty");
                    }

                    assert_ne!(*nullability, Nullability::NonNull);
                    write!(f, "{ty}")
                }
                ty => write!(f, "{ty}"),
            },
            TyKind::InMethodArgument | TyKind::InFnDeclArgument => match &self.ty {
                RustType::Id {
                    type_: ty,
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
                RustType::ObjcBool if self.kind == TyKind::InMethodArgument => write!(f, "bool"),
                ty @ RustType::Pointer {
                    nullability,
                    is_const: false,
                    pointee,
                } => match &**pointee {
                    // TODO: Re-enable once we can support it
                    // RustType::Id {
                    //     type_: ty,
                    //     is_const: false,
                    //     lifetime: Lifetime::Autoreleasing,
                    //     nullability: inner_nullability,
                    // } if self.kind == TyKind::InMethodArgument => {
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
            TyKind::InStructEnum => write!(f, "{}", self.ty),
            TyKind::InFnArgument | TyKind::InBlockArgument => write!(f, "{}", self.ty),
            TyKind::InFnDeclReturn | TyKind::InFnReturn => {
                if let RustType::Void = &self.ty {
                    // Don't output anything
                    return Ok(());
                }

                write!(f, " -> {}", self.ty)
            }
            TyKind::InBlockReturn => match &self.ty {
                RustType::Void => write!(f, "()"),
                ty => write!(f, "{ty}"),
            },
        }
    }
}
