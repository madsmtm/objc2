use clang::{Nullability, Type, TypeKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, Clone)]
pub struct RustType {
    pub tokens: TokenStream,
    pub is_id: bool,
}

impl RustType {
    fn new(tokens: TokenStream) -> Self {
        Self {
            tokens,
            is_id: false,
        }
    }

    fn new_id(tokens: TokenStream, is_return: bool, nullability: Nullability) -> Self {
        let tokens = if is_return {
            quote!(Id<#tokens, Shared>)
        } else {
            quote!(&#tokens)
        };
        let tokens = if nullability == Nullability::NonNull {
            tokens
        } else {
            quote!(Option<#tokens>)
        };
        Self {
            tokens,
            is_id: true,
        }
    }

    pub fn parse(mut ty: Type<'_>, is_return: bool) -> Self {
        use TypeKind::*;

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

        let tokens = match kind {
            Void => quote!(c_void),
            Bool => quote!(bool),
            CharS | CharU => quote!(c_char),
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
            ObjCId => {
                return Self::new_id(quote!(Object), is_return, nullability);
            }
            ObjCClass => {
                if nullability == Nullability::NonNull {
                    quote!(&Class)
                } else {
                    quote!(Option<&Class>)
                }
            }
            ObjCSel => {
                if nullability == Nullability::NonNull {
                    quote!(Sel)
                } else {
                    quote!(Option<Sel>)
                }
            }
            Pointer => {
                println!("{:?}", &ty);
                let pointee = ty.get_pointee_type().expect("pointer type to have pointee");
                println!("{:?}", &pointee);
                let Self { tokens, .. } = Self::parse(pointee, false);

                if nullability == Nullability::NonNull {
                    quote!(NonNull<#tokens>)
                } else {
                    if ty.is_const_qualified() {
                        quote!(*const #tokens)
                    } else {
                        quote!(*mut #tokens)
                    }
                }
            }
            ObjCObjectPointer => {
                let ty = ty.get_pointee_type().expect("pointer type to have pointee");
                let tokens = match ty.get_kind() {
                    ObjCInterface => {
                        let base_ty = ty
                            .get_objc_object_base_type()
                            .expect("interface to have base type");
                        if base_ty != ty {
                            // TODO: Figure out what the base type is
                            panic!("base {:?} was not equal to {:?}", base_ty, ty);
                        }
                        let ident = format_ident!("{}", ty.get_display_name());
                        quote!(#ident)
                    }
                    ObjCObject => {
                        quote!(TodoGenerics)
                    }
                    Attributed => {
                        quote!(TodoAttributed)
                    }
                    _ => panic!("pointee was not objcinterface: {:?}", ty),
                };
                return Self::new_id(tokens, is_return, nullability);
            }
            Typedef if ty.get_display_name() == "instancetype" => {
                if !is_return {
                    panic!("instancetype in non-return position")
                }
                return Self::new_id(quote!(Self), is_return, nullability);
            }
            Typedef => {
                let display_name = ty.get_display_name();
                let display_name = display_name.strip_prefix("const ").unwrap_or(&display_name);
                // TODO: Handle typedefs properly
                match &*display_name {
                    "BOOL" => quote!(bool),
                    display_name => {
                        let ident = format_ident!("{}", display_name);
                        quote!(#ident)
                    }
                }
            }
            BlockPointer => {
                quote!(TodoBlock)
            }
            FunctionPrototype => {
                quote!(TodoFunction)
            }
            IncompleteArray => quote!(TodoArray),
            ConstantArray => {
                let Self { tokens, .. } = Self::parse(
                    ty.get_element_type().expect("array to have element type"),
                    false,
                );
                let num_elements = ty
                    .get_size()
                    .expect("constant array to have element length");
                quote!([#tokens; #num_elements])
            }
            _ => {
                panic!("Unsupported type: {:?}", ty)
            }
        };
        Self::new(tokens)
    }
}
