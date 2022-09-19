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
        generics: Vec<String>,
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

    TypeDef {
        name: String,
    },
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

    pub fn parse_attributed(ty: &mut Type<'_>, nullability: &mut Nullability, kindof: &mut bool) {
        while ty.get_kind() == TypeKind::Attributed {
            match (
                ty.get_display_name().starts_with("__kindof"),
                ty.get_nullability(),
            ) {
                (false, Some(new)) => {
                    *nullability = match (*nullability, new) {
                        (Nullability::NonNull, Nullability::Nullable) => Nullability::Nullable,
                        (Nullability::NonNull, _) => Nullability::NonNull,
                        (Nullability::Nullable, _) => Nullability::Nullable,
                        (Nullability::Unspecified, new) => new,
                    }
                }
                (true, None) => *kindof = true,
                _ => panic!("invalid attributed type: {:?}", ty),
            }
            *ty = ty
                .get_modified_type()
                .expect("attributed type to have modified type");
        }
    }

    pub fn parse(mut ty: Type<'_>, is_return: bool, is_consumed: bool) -> Self {
        use TypeKind::*;

        // println!("{:?}, {:?}", ty, ty.get_class_type());

        let mut nullability = Nullability::Unspecified;
        let mut kindof = false;
        Self::parse_attributed(&mut ty, &mut nullability, &mut kindof);

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
                name: "Object".to_string(),
                generics: Vec::new(),
                is_return,
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
                let mut ty = ty.get_pointee_type().expect("pointer type to have pointee");
                Self::parse_attributed(&mut ty, &mut nullability, &mut kindof);
                match ty.get_kind() {
                    ObjCInterface => {
                        let generics = ty.get_objc_type_arguments();
                        if !generics.is_empty() {
                            panic!("not empty: {:?}, {:?}", ty, generics);
                        }
                        let name = ty.get_display_name();
                        Self::Id {
                            name,
                            generics: Vec::new(),
                            is_return,
                            nullability,
                        }
                    }
                    ObjCObject => {
                        let generics = ty
                            .get_objc_type_arguments()
                            .into_iter()
                            .map(|param| {
                                match Self::parse(param, false, false) {
                                    Self::Id {
                                        name,
                                        generics,
                                        is_return,
                                        nullability,
                                    } => name,
                                    // TODO: Handle this better
                                    Self::Class { nullability } => "TodoClass".to_string(),
                                    param => {
                                        panic!("invalid generic parameter {:?} in {:?}", param, ty)
                                    }
                                }
                            })
                            .collect();
                        let base_ty = ty
                            .get_objc_object_base_type()
                            .expect("object to have base type");
                        let name = base_ty.get_display_name();
                        Self::Id {
                            name,
                            generics,
                            is_return,
                            nullability,
                        }
                    }
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
                            generics: Vec::new(),
                            is_return,
                            nullability,
                        }
                    }
                    _ => {
                        if Self::typedef_is_id(ty.get_canonical_type()).is_some() {
                            let generics = ty.get_objc_type_arguments();
                            if !generics.is_empty() {
                                panic!("not empty: {:?}, {:?}", ty, generics);
                            }
                            Self::Id {
                                name: typedef_name,
                                generics: Vec::new(),
                                is_return,
                                nullability,
                            }
                        } else {
                            Self::TypeDef { name: typedef_name }
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
                generics,
                is_return,
                nullability,
            } => {
                let tokens = format_ident!("{}", name);
                let generics = generics.iter().map(|param| format_ident!("{}", param));
                let tokens = quote!(#tokens <#(#generics),*>);
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
                let tokens = match &**pointee {
                    Self::Id {
                        name,
                        generics,
                        is_return,
                        nullability,
                    } => {
                        let name = format_ident!("{}", name);
                        let generics = generics.iter().map(|param| format_ident!("{}", param));
                        if *nullability == Nullability::NonNull {
                            quote!(NonNull<#name <#(#generics),*>>)
                        } else {
                            // TODO: const?
                            quote!(*mut #name)
                        }
                    }
                    pointee => quote!(#pointee),
                };
                if *nullability == Nullability::NonNull {
                    quote!(NonNull<#tokens>)
                } else {
                    if *is_const {
                        quote!(*const #tokens)
                    } else {
                        quote!(*mut #tokens)
                    }
                }
            }
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
