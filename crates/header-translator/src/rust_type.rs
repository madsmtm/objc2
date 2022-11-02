use std::fmt;

use clang::{Nullability, Type, TypeKind};

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
                        match RustType::parse(param, false, Nullability::Unspecified) {
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
    kindof: Option<&mut bool>,
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

    if let Some(kindof) = kindof {
        if let Some(rest) = name.strip_prefix("__kindof") {
            name = rest.trim();
            *kindof = true;
        }
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
        let original_name = ty.get_display_name();
        println!("attributes: {original_name:?} -> {name:?} != {modified_name:?}");
        panic!(
            "could not extract all attributes from attributed type. Inner: {ty:?}, {modified_ty:?}"
        );
    }

    modified_ty
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RustType {
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

    TypeDef {
        name: String,
    },
}

impl RustType {
    pub fn is_error_out(&self) -> bool {
        if let Self::Pointer {
            nullability,
            is_const,
            pointee,
        } = self
        {
            assert_eq!(
                *nullability,
                Nullability::Nullable,
                "invalid error nullability {self:?}"
            );
            assert!(!is_const, "expected error not const {self:?}");
            if let Self::Id {
                type_,
                is_const,
                lifetime,
                nullability,
            } = &**pointee
            {
                if type_.name != "NSError" {
                    return false;
                }
                assert!(
                    type_.generics.is_empty(),
                    "expected error generics to be empty {self:?}"
                );
                assert_eq!(
                    *nullability,
                    Nullability::Nullable,
                    "invalid inner error nullability {self:?}"
                );
                assert!(!is_const, "expected inner error not const {self:?}");
                assert_eq!(
                    *lifetime,
                    Lifetime::Unspecified,
                    "invalid error lifetime {self:?}"
                );
                true
            } else {
                panic!("invalid error parameter {self:?}")
            }
        } else {
            false
        }
    }

    fn parse(ty: Type<'_>, is_consumed: bool, mut nullability: Nullability) -> Self {
        use TypeKind::*;

        // println!("{:?}, {:?}", ty, ty.get_class_type());

        let mut lifetime = Lifetime::Unspecified;
        let ty = parse_attributed(ty, &mut nullability, &mut lifetime, None);

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
                // Note: Can't handle const id pointers
                // assert!(!ty.is_const_qualified(), "expected pointee to not be const");
                let pointee = Self::parse(ty, is_consumed, Nullability::Unspecified);
                Self::Pointer {
                    nullability,
                    is_const,
                    pointee: Box::new(pointee),
                }
            }
            ObjCObjectPointer => {
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                let mut kindof = false;
                let ty = parse_attributed(ty, &mut nullability, &mut lifetime, Some(&mut kindof));
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
            BlockPointer => Self::TypeDef {
                name: "TodoBlock".to_string(),
            },
            FunctionPrototype => Self::TypeDef {
                name: "TodoFunction".to_string(),
            },
            IncompleteArray => Self::TypeDef {
                name: "TodoArray".to_string(),
            },
            ConstantArray => {
                let element_type = Self::parse(
                    ty.get_element_type().expect("array to have element type"),
                    is_consumed,
                    Nullability::Unspecified,
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
}

impl RustType {
    fn visit_lifetime(&self, mut f: impl FnMut(Lifetime)) {
        match self {
            Self::Id { lifetime, .. } => f(*lifetime),
            Self::Pointer { pointee, .. } => pointee.visit_lifetime(f),
            Self::Array { element_type, .. } => element_type.visit_lifetime(f),
            _ => {}
        }
    }

    pub fn parse_argument(ty: Type<'_>, is_consumed: bool) -> Self {
        let this = Self::parse(ty, is_consumed, Nullability::Unspecified);

        match &this {
            Self::Pointer { pointee, .. } => pointee.visit_lifetime(|lifetime| {
                if lifetime != Lifetime::Autoreleasing && lifetime != Lifetime::Unspecified {
                    panic!("unexpected lifetime {lifetime:?} in pointer argument {ty:?}");
                }
            }),
            _ => this.visit_lifetime(|lifetime| {
                if lifetime != Lifetime::Strong && lifetime != Lifetime::Unspecified {
                    panic!("unexpected lifetime {lifetime:?} in argument {ty:?}");
                }
            }),
        }

        this
    }

    pub fn parse_typedef(ty: Type<'_>) -> Self {
        match ty.get_kind() {
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
            // Because that means we can later on use ordinary Id<...> handling.
            TypeKind::ObjCObjectPointer => {
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                let type_ = GenericType::parse_objc_pointer(ty);

                match &*type_.name {
                    "NSString" => {}
                    "NSUnit" => {}        // TODO: Handle this differently
                    "TodoProtocols" => {} // TODO
                    _ => panic!("typedef declaration was not NSString: {type_:?}"),
                }

                if !type_.generics.is_empty() {
                    panic!("typedef declaration generics not empty");
                }

                Self::TypeDef { name: type_.name }
            }
            _ => {
                let this = Self::parse(ty, false, Nullability::Unspecified);

                this.visit_lifetime(|lifetime| {
                    if lifetime != Lifetime::Unspecified {
                        panic!("unexpected lifetime in typedef {this:?}");
                    }
                });

                this
            }
        }
    }

    pub fn parse_property(ty: Type<'_>, default_nullability: Nullability) -> Self {
        let this = Self::parse(ty, false, default_nullability);

        this.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in property {this:?}");
            }
        });

        this
    }

    pub fn parse_struct_field(ty: Type<'_>) -> Self {
        let this = Self::parse(ty, false, Nullability::Unspecified);

        this.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in struct field {this:?}");
            }
        });

        this
    }

    pub fn parse_enum(ty: Type<'_>) -> Self {
        let this = Self::parse(ty, false, Nullability::Unspecified);

        this.visit_lifetime(|_lifetime| {
            panic!("unexpected lifetime in enum {this:?}");
        });

        this
    }
}

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
                type_,
                // Ignore
                is_const: _,
                // Ignore
                lifetime: _,
                nullability,
            } => {
                if *nullability == Nullability::NonNull {
                    write!(f, "&{type_}")
                } else {
                    write!(f, "Option<&{type_}>")
                }
            }
            Class { nullability } => {
                if *nullability == Nullability::NonNull {
                    write!(f, "&Class")
                } else {
                    write!(f, "Option<&Class>")
                }
            }
            Sel { nullability } => {
                if *nullability == Nullability::NonNull {
                    write!(f, "Sel")
                } else {
                    write!(f, "OptionSel")
                }
            }
            ObjcBool => write!(f, "bool"),

            // Others
            Pointer {
                nullability,
                is_const,
                pointee,
            } => match &**pointee {
                // Self::Id {
                //     type_,
                //     is_const: false,
                //     lifetime: Lifetime::Autoreleasing,
                //     nullability: inner_nullability,
                // } => {
                //     let tokens = format!("Id<{type_}, Shared>");
                //     let tokens = if *inner_nullability == Nullability::NonNull {
                //         tokens
                //     } else {
                //         format!("Option<{tokens}>")
                //     };
                //
                //     let tokens = if *is_const {
                //         format!("&{tokens}")
                //     } else {
                //         format!("&mut {tokens}")
                //     };
                //     if *nullability == Nullability::NonNull {
                //         write!(f, "{tokens}")
                //     } else {
                //         write!(f, "Option<{tokens}>")
                //     }
                // }
                Self::Id {
                    type_: tokens,
                    is_const: false,
                    lifetime: _,
                    nullability: inner_nullability,
                } => {
                    let tokens = if *inner_nullability == Nullability::NonNull {
                        format!("NonNull<{tokens}>")
                    } else {
                        format!("*mut {tokens}")
                    };
                    if *nullability == Nullability::NonNull {
                        write!(f, "NonNull<{tokens}>")
                    } else if *is_const {
                        write!(f, "*const {tokens}")
                    } else {
                        write!(f, "*mut {tokens}")
                    }
                }
                Self::Id { .. } => {
                    unreachable!("there should be no id with other values: {self:?}")
                }
                Self::ObjcBool => {
                    if *nullability == Nullability::NonNull {
                        write!(f, "NonNull<Bool>")
                    } else if *is_const {
                        write!(f, "*const Bool")
                    } else {
                        write!(f, "*mut Bool")
                    }
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
            Array {
                element_type,
                num_elements,
            } => write!(f, "[{element_type}; {num_elements}]"),
            Enum { name } | Struct { name } | TypeDef { name } => write!(f, "{name}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RustTypeReturn(pub RustType);

impl RustTypeReturn {
    pub fn is_id(&self) -> bool {
        matches!(self.0, RustType::Id { .. })
    }

    pub fn new(inner: RustType) -> Self {
        inner.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in return {inner:?}");
            }
        });

        Self(inner)
    }

    pub fn parse(ty: Type<'_>) -> Self {
        Self::new(RustType::parse(ty, false, Nullability::Unspecified))
    }

    pub fn as_error(&self) -> String {
        match &self.0 {
            RustType::Id {
                type_,
                lifetime: Lifetime::Unspecified,
                is_const: false,
                nullability: Nullability::Nullable,
            } => {
                // NULL -> error
                format!(" -> Result<Id<{type_}, Shared>, Id<NSError, Shared>>")
            }
            RustType::ObjcBool => {
                // NO -> error
                format!(" -> Result<(), Id<NSError, Shared>>")
            }
            _ => panic!("unknown error result type {self:?}"),
        }
    }

    pub fn is_alloc(&self) -> bool {
        match &self.0 {
            RustType::Id {
                type_,
                lifetime: Lifetime::Unspecified,
                is_const: false,
                nullability: Nullability::NonNull,
            } => type_.name == "Self" && type_.generics.is_empty(),
            _ => false,
        }
    }
}

impl fmt::Display for RustTypeReturn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            RustType::Void => Ok(()),
            RustType::Id {
                type_,
                // Ignore
                is_const: _,
                // Ignore
                lifetime: _,
                nullability,
            } => {
                if *nullability == Nullability::NonNull {
                    write!(f, " -> Id<{type_}, Shared>")
                } else {
                    write!(f, " -> Option<Id<{type_}, Shared>>")
                }
            }
            RustType::Class { nullability } => {
                // SAFETY: TODO
                if *nullability == Nullability::NonNull {
                    write!(f, "-> &'static Class")
                } else {
                    write!(f, "-> Option<&'static Class>")
                }
            }
            type_ => write!(f, " -> {type_}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RustTypeStatic {
    inner: RustType,
}

impl RustTypeStatic {
    pub fn parse(ty: Type<'_>) -> Self {
        let inner = RustType::parse(ty, false, Nullability::Unspecified);

        inner.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Strong && lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in var {inner:?}");
            }
        });

        Self { inner }
    }
}

impl fmt::Display for RustTypeStatic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.inner {
            RustType::Id {
                type_,
                is_const: false,
                lifetime: Lifetime::Strong | Lifetime::Unspecified,
                nullability,
            } => {
                if *nullability == Nullability::NonNull {
                    write!(f, "&'static {type_}")
                } else {
                    write!(f, "Option<&'static {type_}>")
                }
            }
            ty @ RustType::Id { .. } => panic!("invalid static {ty:?}"),
            ty => write!(f, "{ty}"),
        }
    }
}
