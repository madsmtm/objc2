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

fn handle_reserved(s: &str) -> &str {
    match s {
        "type" => "type_",
        s => s,
    }
}

fn get_id_type(tokens: TokenStream, is_return: bool, nullability: Nullability) -> TokenStream {
    let tokens = if is_return {
        quote!(Id<#tokens, Shared>)
    } else {
        quote!(&#tokens)
    };
    if nullability == Nullability::NonNull {
        quote!(#tokens)
    } else {
        quote!(Option<#tokens>)
    }
}

fn get_rust_type(mut ty: Type<'_>, is_return: bool) -> (TokenStream, bool) {
    use TypeKind::*;

    let mut nullability = Nullability::Nullable;
    let mut kind = ty.get_kind();
    if kind == Attributed {
        nullability = ty
            .get_nullability()
            .expect("attributed type to have nullability");
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
            return (get_id_type(quote!(Object), is_return, nullability), true);
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
            let (pointee_tokens, _) = get_rust_type(pointee, false);

            if nullability == Nullability::NonNull {
                quote!(NonNull<#pointee_tokens>)
            } else {
                if ty.is_const_qualified() {
                    quote!(*const #pointee_tokens)
                } else {
                    quote!(*mut #pointee_tokens)
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
            return (get_id_type(tokens, is_return, nullability), true);
        }
        Typedef if ty.get_display_name() == "instancetype" => {
            if !is_return {
                panic!("instancetype in non-return position")
            }
            return (get_id_type(quote!(Self), is_return, nullability), true);
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
            let (element_type, _) = get_rust_type(
                ty.get_element_type().expect("array to have element type"),
                false,
            );
            let num_elements = ty
                .get_size()
                .expect("constant array to have element length");
            quote!([#element_type; #num_elements])
        }
        _ => {
            panic!("Unsupported type: {:?}", ty)
        }
    };
    (tokens, false)
}

// One of EntityKind::ObjCInstanceMethodDecl or EntityKind::ObjCClassMethodDecl
fn parse_method(entity: Entity<'_>) -> Option<TokenStream> {
    // println!("Method {:?}", entity.get_display_name());
    // println!("Availability: {:?}", entity.get_platform_availability());
    // TODO: Handle `NSConsumesSelf` and `NSReturnsRetained`
    // println!("Children: {:?}", entity.get_children());

    let selector = entity.get_name().expect("method selector");
    let fn_name = get_rust_name(&selector);

    if entity.is_variadic() {
        println!("Can't handle variadic method {}", selector);
        return None;
    }

    println!("{}", selector);

    let result_type = entity.get_result_type().expect("method return type");
    let (ret, is_id) = if result_type.get_kind() == TypeKind::Void {
        (quote! {}, false)
    } else {
        let (return_item, is_id) = get_rust_type(result_type, true);
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

    let arguments: Vec<_> = entity
        .get_arguments()
        .expect("method arguments")
        .into_iter()
        .map(|arg| {
            (
                format_ident!(
                    "{}",
                    handle_reserved(&arg.get_name().expect("arg display name"))
                ),
                arg.get_type().expect("argument type"),
            )
        })
        .collect();

    let fn_args = arguments.iter().map(|(param, arg_ty)| {
        let (ty, _) = get_rust_type(arg_ty.clone(), false);
        quote!(#param: #ty)
    });

    let method_call = if selector.contains(':') {
        let split_selector: Vec<_> = selector.split(':').filter(|sel| !sel.is_empty()).collect();
        assert!(
            arguments.len() == split_selector.len(),
            "incorrect method argument length",
        );

        let iter = arguments
            .iter()
            .zip(split_selector)
            .map(|((param, _), sel)| {
                let sel = format_ident!("{}", sel);
                quote!(#sel: #param)
            });
        quote!(#(#iter),*)
    } else {
        assert_eq!(arguments.len(), 0, "too many arguments");
        let sel = format_ident!("{}", selector);
        quote!(#sel)
    };

    match entity.get_kind() {
        EntityKind::ObjCInstanceMethodDecl => Some(quote! {
            pub unsafe fn #fn_name(&self #(, #fn_args)*) #ret {
                #macro_name![self, #method_call]
            }
        }),
        EntityKind::ObjCClassMethodDecl => Some(quote! {
            pub unsafe fn #fn_name(#(#fn_args),*) #ret {
                #macro_name![Self::class(), #method_call]
            }
        }),
        _ => panic!("unknown method kind"),
    }
}

pub fn get_tokens(entity: &Entity<'_>) -> TokenStream {
    match entity.get_kind() {
        EntityKind::InclusionDirective | EntityKind::MacroExpansion | EntityKind::ObjCClassRef => {
            TokenStream::new()
        }
        EntityKind::ObjCInterfaceDecl => {
            // entity.get_mangled_objc_names()
            let name = format_ident!("{}", entity.get_name().expect("class name"));
            // println!("Availability: {:?}", entity.get_platform_availability());
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
                    EntityKind::ObjCInstanceMethodDecl | EntityKind::ObjCClassMethodDecl => {
                        if let Some(tokens) = parse_method(entity) {
                            methods.push(tokens);
                        }
                    }
                    EntityKind::ObjCPropertyDecl => {
                        // println!(
                        //     "Property {:?}, {:?}",
                        //     entity.get_display_name().unwrap(),
                        //     entity.get_objc_attributes().unwrap()
                        // );
                        // methods.push(quote! {});
                    }
                    EntityKind::TemplateTypeParameter => {
                        println!("TODO: Template parameters")
                    }
                    EntityKind::VisibilityAttr => {
                        // NS_CLASS_AVAILABLE_MAC??
                        println!("TODO: VisibilityAttr")
                    }
                    EntityKind::UnexposedAttr => {}
                    _ => panic!("Unknown in ObjCInterfaceDecl {:?}", entity),
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
            let meta = if let Some(doc) = entity.get_name() {
                quote!(#[doc = #doc])
            } else {
                // Some categories don't have a name. Example: NSClipView
                quote!()
            };
            let mut class = None;
            let mut protocols = Vec::new();
            let mut methods = Vec::new();

            entity.visit_children(|entity, _parent| {
                match entity.get_kind() {
                    EntityKind::ObjCClassRef => {
                        if class.is_some() {
                            panic!("could not find unique category class")
                        }
                        class = Some(entity);
                    }
                    EntityKind::ObjCProtocolRef => {
                        protocols.push(entity);
                    }
                    EntityKind::ObjCInstanceMethodDecl | EntityKind::ObjCClassMethodDecl => {
                        if let Some(tokens) = parse_method(entity) {
                            methods.push(tokens);
                        }
                    }
                    EntityKind::ObjCPropertyDecl => {
                        // println!(
                        //     "Property {:?}, {:?}",
                        //     entity.get_display_name().unwrap(),
                        //     entity.get_objc_attributes().unwrap()
                        // );
                        // methods.push(quote! {});
                    }
                    EntityKind::UnexposedAttr => {}
                    _ => panic!("Unknown in ObjCCategoryDecl {:?}", entity),
                }
                EntityVisitResult::Continue
            });

            let class = class.expect("could not find category class");
            let class_name = format_ident!("{}", class.get_name().expect("class name"));

            quote! {
                #meta
                impl #class_name {
                    #(#methods)*
                }
            }
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
