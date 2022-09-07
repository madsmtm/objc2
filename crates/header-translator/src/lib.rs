use std::collections::HashMap;
use std::path::{Path, PathBuf};

use clang::source::File;
use clang::{Clang, Entity, EntityKind, EntityVisitResult, Index, Nullability, Type, TypeKind};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

fn get_rust_name(selector: &str) -> Ident {
    format_ident!(
        "{}",
        selector.trim_end_matches(|c| c == ':').replace(':', "_")
    )
}

fn get_rust_type(ty: &Type<'_>) -> (TokenStream, bool) {
    use TypeKind::*;

    let tokens = match ty.get_kind() {
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
        // ObjCId => quote!(Option<Id<Object, Shared>>),
        // ObjCClass => quote!(Option<&Class>),
        // ObjCSel => quote!(Option<Sel>),
        Pointer => {
            let pointee = ty.get_pointee_type().expect("pointer type to have pointee");
            let (ty_tokens, _) = get_rust_type(&pointee);

            // TODO: Nullability
            if ty.is_const_qualified() {
                quote!(*const #ty_tokens)
            } else {
                quote!(*mut #ty_tokens)
            }
        }
        ObjCInterface => {
            let base_ty = ty
                .get_objc_object_base_type()
                .expect("interface to have base type");
            if base_ty != *ty {
                // TODO: Figure out what the base type is
                panic!("base {:?} was not equal to {:?}", base_ty, ty);
            }
            let ident = format_ident!("{}", ty.get_display_name());
            quote!(#ident)
        }
        // ObjCObjectPointer => quote!(Option<Id<Object, Shared>>),
        Attributed => {
            let nullability = ty
                .get_nullability()
                .expect("attributed type to have nullability");
            let modified = ty
                .get_modified_type()
                .expect("attributed type to have modified type");
            match modified.get_kind() {
                ObjCObjectPointer => {
                    let pointee = modified
                        .get_pointee_type()
                        .expect("pointer type to have pointee");
                    let (ty_tokens, _) = get_rust_type(&pointee);
                    if nullability == Nullability::NonNull {
                        return (quote!(Id<#ty_tokens, Shared>), true);
                    } else {
                        return (quote!(Option<Id<#ty_tokens, Shared>>), true);
                    }
                }
                Typedef => {
                    let (ty_tokens, _) = get_rust_type(&modified);
                    if nullability == Nullability::NonNull {
                        return (quote!(Id<#ty_tokens, Shared>), true);
                    } else {
                        return (quote!(Option<Id<#ty_tokens, Shared>>), true);
                    }
                }
                _ => panic!("Unsupported attributed type: {:?}", modified),
            }
        }
        Typedef => {
            let display_name = ty.get_display_name();
            if display_name == "instancetype" {
                quote!(Self)
            } else {
                let ident = format_ident!("{}", ty.get_display_name());
                quote!(#ident)
            }
        }
        _ => panic!("Unsupported type: {:?}", ty),
    };
    (tokens, false)
}

pub fn get_tokens(entity: &Entity<'_>) -> TokenStream {
    match entity.get_kind() {
        EntityKind::InclusionDirective | EntityKind::MacroExpansion | EntityKind::ObjCClassRef => {
            TokenStream::new()
        }
        EntityKind::ObjCInterfaceDecl => {
            let name = format_ident!("{}", entity.get_name().expect("class name"));
            let mut superclass = None;
            let mut protocols = Vec::new();
            let mut methods = Vec::new();

            entity.visit_children(|entity, _parent| {
                match entity.get_kind() {
                    EntityKind::ObjCSuperClassRef => {
                        superclass = Some(entity);
                    }
                    EntityKind::ObjCClassRef => {
                        println!("ObjCClassRef: {:?}", entity.get_display_name());
                    }
                    EntityKind::ObjCProtocolRef => {
                        protocols.push(entity);
                    }
                    kind @ (EntityKind::ObjCInstanceMethodDecl
                    | EntityKind::ObjCClassMethodDecl) => {
                        // TODO: Handle `NSConsumesSelf` and `NSReturnsRetained`
                        println!("Children: {:?}", entity.get_children());

                        let selector = entity.get_name().expect("method selector");
                        let fn_name = get_rust_name(&selector);

                        if entity.is_variadic() {
                            panic!("Can't handle variadic methods");
                        }

                        let args = Vec::<i32>::new();
                        let method_call = Vec::<i32>::new();

                        let result_type = entity.get_result_type().expect("method return type");
                        let (ret, is_id) = if result_type.get_kind() == TypeKind::Void {
                            (quote! {}, false)
                        } else {
                            let (return_item, is_id) = get_rust_type(&result_type);
                            (
                                quote! {
                                    -> #return_item
                                },
                                is_id,
                            )
                        };

                        let macro_name = if is_id {
                            format_ident!("msg_send_id")
                        } else {
                            format_ident!("msg_send")
                        };

                        if EntityKind::ObjCInstanceMethodDecl == kind {
                            methods.push(quote! {
                                pub unsafe fn #fn_name(&self #(, #args)*) #ret {
                                    #macro_name![self, #(#method_call),*]
                                }
                            });
                        } else {
                            methods.push(quote! {
                                pub unsafe fn #fn_name(#(#args),*) #ret {
                                    #macro_name![Self::class(), #(#method_call),*]
                                }
                            });
                        }
                    }
                    EntityKind::ObjCPropertyDecl => {
                        methods.push(quote! {});
                    }
                    _ => {
                        println!("{:?}: {:?}", entity.get_kind(), entity.get_display_name());
                    }
                }
                EntityVisitResult::Continue
            });

            let superclass = superclass.expect("only classes with a superclass is supported");
            let superclass_name =
                format_ident!("{}", superclass.get_name().expect("superclass name"));

            quote! {
                extern_class!(
                    #[derive(Debug)]
                    struct #name;

                    unsafe impl ClassType for #name {
                        type Super = #superclass_name;
                    }
                );

                impl #name {
                    #(#methods)*
                }
            }
        }
        EntityKind::ObjCCategoryDecl => {
            quote! {}
        }
        _ => {
            println!(
                "Unknown: {:?}: {}",
                entity.get_kind(),
                entity
                    .get_display_name()
                    .unwrap_or_else(|| "`None`".to_string())
            );
            TokenStream::new()
        }
    }
}

pub fn create_rust_file(entities: &[Entity<'_>]) -> TokenStream {
    let mut iter = entities.iter().map(get_tokens);
    quote! {
        #(#iter)*
    }
}
