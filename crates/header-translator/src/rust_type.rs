use clang::{Nullability, Type, TypeKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};

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

    // Objective-C
    Id {
        name: String,
        // generics: Vec<String>,
        is_return: bool,
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

    TypeDef(String),
}

impl RustType {
    pub fn is_id(&self) -> bool {
        matches!(self, Self::Id { .. })
    }

    pub fn typedef_is_id(ty: Type<'_>) -> Option<String> {
        use TypeKind::*;

        // Note: When we encounter a typedef declaration like this:
        // typedef NSString* NSAbc;
        //
        // We parse it as one of:
        // type NSAbc = NSString;
        // struct NSAbc(NSString);
        //
        // Instead of:
        // type NSAbc = *const NSString;
        //
        // Because that means we can use ordinary Id<...> handling.
        match ty.get_kind() {
            ObjCObjectPointer => {
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                let name = ty.get_display_name();
                match ty.get_kind() {
                    ObjCInterface => {
                        match &*name {
                            "NSString" => {}
                            "NSUnit" => {} // TODO: Handle this differently
                            _ => panic!("typedef interface was not NSString {:?}", ty),
                        }
                        Some(name)
                    }
                    ObjCObject => Some(name),
                    _ => panic!("pointee was not objcinterface nor objcobject: {:?}", ty),
                }
            }
            ObjCId => panic!("unexpected ObjCId {:?}", ty),
            _ => None,
        }
    }

    pub fn parse(mut ty: Type<'_>, is_return: bool, is_consumed: bool) -> Self {
        use TypeKind::*;

        // println!("{:?}, {:?}", ty, ty.get_class_type());

        let mut nullability = Nullability::Unspecified;
        let mut kind = ty.get_kind();
        while kind == Attributed {
            let new = ty
                .get_nullability()
                .expect("attributed type to have nullability");
            nullability = match (nullability, new) {
                (Nullability::NonNull, Nullability::Nullable) => Nullability::Nullable,
                (Nullability::NonNull, _) => Nullability::NonNull,
                (Nullability::Nullable, _) => Nullability::Nullable,
                (Nullability::Unspecified, new) => new,
            };
            ty = ty
                .get_modified_type()
                .expect("attributed type to have modified type");
            kind = ty.get_kind();
        }

        match kind {
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
                name: "Object".to_string(),
                is_return,
                nullability,
            },
            ObjCClass => Self::Class { nullability },
            ObjCSel => Self::Sel { nullability },
            Pointer => {
                let is_const = ty.is_const_qualified();
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                let pointee = Self::parse(ty, false, is_consumed);
                let pointee = match pointee {
                    Self::Id {
                        name,
                        is_return,
                        nullability,
                    } => {
                        let is_const = ty.is_const_qualified();
                        Self::Pointer {
                            nullability,
                            is_const,
                            pointee: Box::new(Self::TypeDef(name)),
                        }
                    }
                    pointee => pointee,
                };

                Self::Pointer {
                    nullability,
                    is_const,
                    pointee: Box::new(pointee),
                }
            }
            ObjCObjectPointer => {
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                match ty.get_kind() {
                    ObjCInterface => {
                        let base_ty = ty
                            .get_objc_object_base_type()
                            .expect("interface to have base type");
                        if base_ty != ty {
                            // TODO: Figure out what the base type is
                            panic!("base {:?} was not equal to {:?}", base_ty, ty);
                        }
                        let name = ty.get_display_name();
                        Self::Id {
                            name,
                            is_return,
                            nullability,
                        }
                    }
                    ObjCObject => Self::TypeDef("TodoGenerics".to_string()),
                    Attributed => Self::TypeDef("TodoAttributed".to_string()),
                    _ => panic!("pointee was not objcinterface: {:?}", ty),
                }
            }
            Typedef => {
                let typedef_name = ty.get_typedef_name().expect("typedef has name");
                match &*typedef_name {
                    "BOOL" => Self::ObjcBool,
                    "instancetype" => {
                        if !is_return {
                            panic!("instancetype in non-return position")
                        }
                        Self::Id {
                            name: "Self".to_string(),
                            is_return,
                            nullability,
                        }
                    }
                    _ => {
                        if Self::typedef_is_id(ty.get_canonical_type()).is_some() {
                            Self::Id {
                                name: typedef_name,
                                is_return,
                                nullability,
                            }
                        } else {
                            Self::TypeDef(typedef_name)
                        }
                    }
                }
            }
            BlockPointer => Self::TypeDef("TodoBlock".to_string()),
            FunctionPrototype => Self::TypeDef("TodoFunction".to_string()),
            IncompleteArray => Self::TypeDef("TodoArray".to_string()),
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

            // Objective-C
            Id {
                name,
                is_return,
                nullability,
            } => {
                let tokens = format_ident!("{}", name);
                let tokens = if *is_return {
                    quote!(Id<#tokens, Shared>)
                } else {
                    quote!(&#tokens)
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
            } => {
                if *nullability == Nullability::NonNull {
                    quote!(NonNull<#pointee>)
                } else {
                    if *is_const {
                        quote!(*const #pointee)
                    } else {
                        quote!(*mut #pointee)
                    }
                }
            }
            Array {
                element_type,
                num_elements,
            } => quote!([#element_type; #num_elements]),
            TypeDef(s) => {
                let x = format_ident!("{}", s);
                quote!(#x)
            }
        };
        tokens.append_all(result);
    }
}
