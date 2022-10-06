use clang::{Nullability, Type, TypeKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

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
                        match RustType::parse(param, false, false) {
                            RustType::Id {
                                type_,
                                is_return: _,
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

impl ToTokens for GenericType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let generics = &self.generics;
        tokens.append_all(quote!(#name <#(#generics),*>));
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
        is_return: bool,
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

    TypeDef {
        name: String,
    },
}

impl RustType {
    pub fn is_id(&self) -> bool {
        matches!(self, Self::Id { .. })
    }

    fn parse(ty: Type<'_>, is_return: bool, is_consumed: bool) -> Self {
        use TypeKind::*;

        // println!("{:?}, {:?}", ty, ty.get_class_type());

        let mut nullability = Nullability::Unspecified;
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
                is_return,
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
                let pointee = Self::parse(ty, false, is_consumed);
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
                    is_return,
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
                    "int164_t" => Self::I64,
                    "uint64_t" => Self::U64,
                    "instancetype" => {
                        if !is_return {
                            panic!("instancetype in non-return position")
                        }
                        Self::Id {
                            type_: GenericType {
                                name: "Self".to_string(),
                                generics: Vec::new(),
                            },
                            is_return,
                            is_const: ty.is_const_qualified(),
                            lifetime,
                            nullability,
                        }
                    }
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
                                    is_return,
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
                    false,
                    is_consumed,
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

    pub fn parse_return(ty: Type<'_>) -> Self {
        let this = Self::parse(ty, true, false);

        this.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in return {this:?}");
            }
        });

        this
    }

    pub fn parse_argument(ty: Type<'_>, is_consumed: bool) -> Self {
        let this = Self::parse(ty, false, is_consumed);

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
        let this = Self::parse(ty, false, false);

        this.visit_lifetime(|lifetime| {
            if lifetime != Lifetime::Unspecified {
                panic!("unexpected lifetime in typedef {this:?}");
            }
        });

        this
    }
}

impl ToTokens for RustType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        use RustType::*;
        let result = match self {
            // Primitives
            Void => quote!(c_void),
            C99Bool => panic!("C99's bool is unsupported"), // quote!(bool)
            Char => quote!(c_char),
            SChar => quote!(c_schar),
            UChar => quote!(c_uchar),
            Short => quote!(c_short),
            UShort => quote!(c_ushort),
            Int => quote!(c_int),
            UInt => quote!(c_uint),
            Long => quote!(c_long),
            ULong => quote!(c_ulong),
            LongLong => quote!(c_longlong),
            ULongLong => quote!(c_ulonglong),
            Float => quote!(c_float),
            Double => quote!(c_double),
            I8 => quote!(i8),
            U8 => quote!(u8),
            I16 => quote!(i16),
            U16 => quote!(u16),
            I32 => quote!(i32),
            U32 => quote!(u32),
            I64 => quote!(i64),
            U64 => quote!(u64),

            // Objective-C
            Id {
                type_,
                is_return,
                // Ignore
                is_const: _,
                // Ignore
                lifetime: _,
                nullability,
            } => {
                let tokens = if *is_return {
                    quote!(Id<#type_, Shared>)
                } else {
                    quote!(&#type_)
                };
                if *nullability == Nullability::NonNull {
                    tokens
                } else {
                    quote!(Option<#tokens>)
                }
            }
            Class { nullability } => {
                if *nullability == Nullability::NonNull {
                    quote!(&Class)
                } else {
                    quote!(Option<&Class>)
                }
            }
            Sel { nullability } => {
                if *nullability == Nullability::NonNull {
                    quote!(Sel)
                } else {
                    quote!(Option<Sel>)
                }
            }
            ObjcBool => quote!(bool),

            // Others
            Pointer {
                nullability,
                is_const,
                pointee,
            } => match &**pointee {
                Self::Id {
                    type_: tokens,
                    is_return: false,
                    is_const: false,
                    lifetime: Lifetime::Autoreleasing,
                    nullability: inner_nullability,
                } => {
                    let tokens = quote!(Id<#tokens, Shared>);
                    let tokens = if *inner_nullability == Nullability::NonNull {
                        tokens
                    } else {
                        quote!(Option<#tokens>)
                    };

                    let tokens = if *is_const {
                        quote!(&#tokens)
                    } else {
                        quote!(&mut #tokens)
                    };
                    if *nullability == Nullability::NonNull {
                        tokens
                    } else {
                        quote!(Option<#tokens>)
                    }
                }
                Self::Id {
                    type_: tokens,
                    is_return: false,
                    is_const: false,
                    lifetime: Lifetime::Unspecified,
                    nullability: inner_nullability,
                } => {
                    if tokens.name != "NSError" {
                        println!("id*: {self:?}");
                    }
                    let tokens = if *inner_nullability == Nullability::NonNull {
                        quote!(NonNull<#tokens>)
                    } else {
                        quote!(*mut #tokens)
                    };
                    if *nullability == Nullability::NonNull {
                        quote!(NonNull<#tokens>)
                    } else if *is_const {
                        quote!(*const #tokens)
                    } else {
                        quote!(*mut #tokens)
                    }
                }
                Self::Id { .. } => {
                    unreachable!("there should be no id with other values: {self:?}")
                }
                pointee => {
                    if *nullability == Nullability::NonNull {
                        quote!(NonNull<#pointee>)
                    } else if *is_const {
                        quote!(*const #pointee)
                    } else {
                        quote!(*mut #pointee)
                    }
                }
            },
            Array {
                element_type,
                num_elements,
            } => quote!([#element_type; #num_elements]),
            TypeDef { name } => {
                let name = format_ident!("{}", name);
                quote!(#name)
            }
        };
        tokens.append_all(result);
    }
}
