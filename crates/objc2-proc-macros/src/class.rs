use crate::method::MethodContext;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    GenericArgument, PathArguments, Type,
};

enum FieldKind {
    IvarBool,
    IvarDrop,
    IvarEncode,
    PhantomData,
}

trait FieldExt {
    fn field_kind(&self) -> Option<FieldKind>;
}

impl FieldExt for syn::Field {
    fn field_kind(&self) -> Option<FieldKind> {
        if let syn::Type::Path(type_path) = &self.ty {
            if let Some(segment) = type_path.path.segments.last() {
                if segment.ident == "IvarBool" {
                    return Some(FieldKind::IvarBool);
                }
                if segment.ident == "IvarDrop" {
                    return Some(FieldKind::IvarDrop);
                }
                if segment.ident == "IvarEncode" {
                    return Some(FieldKind::IvarEncode);
                }
                if segment.ident == "PhantomData" {
                    return Some(FieldKind::PhantomData);
                }
            }
        }
        None
    }
}

#[derive(Debug)]
struct ClassMacroInput {
    superclass: syn::Ident,
    inherits: Option<Punctuated<syn::Ident, syn::Token![,]>>,
}

impl Parse for ClassMacroInput {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        mod keyword {
            syn::custom_keyword!(inherits);
        }

        let mut superclass = None;
        let mut inherits = None;

        loop {
            let lookahead = input.lookahead1();
            if superclass.is_none() && lookahead.peek(syn::Token![super]) {
                input.parse::<syn::Token![super]>()?;
                input.parse::<syn::Token![=]>()?;
                superclass = Some(input.parse()?);
            } else if inherits.is_none() && lookahead.peek(keyword::inherits) {
                input.parse::<keyword::inherits>()?;
                input.parse::<syn::Token![=]>()?;
                inherits = {
                    let content;
                    let bracket = syn::bracketed!(content in input);
                    let idents = Punctuated::parse_terminated(&content)?;
                    if idents.is_empty() {
                        let span = bracket.span;
                        let message = "#[objc]: `inherits` expects a non-empty list";
                        return Err(syn::Error::new(span, message));
                    } else {
                        Some(idents)
                    }
                };
            }
            input.parse::<Option<syn::Token![,]>>()?;
            if input.is_empty() {
                break;
            } else if true
                && (!superclass.is_none() || !input.peek(syn::Token![super]))
                && (!inherits.is_none() || !input.peek(keyword::inherits))
            {
                let span = input.span();
                let message = "#[objc]: duplicate or unexpected argument";
                return Err(syn::Error::new(span, message));
            }
        }

        if let Some(superclass) = superclass {
            Ok(ClassMacroInput {
                superclass,
                inherits,
            })
        } else {
            let span = input.span();
            let message = "#[objc]: `super` argument is required";
            return Err(syn::Error::new(span, message));
        }
    }
}

struct ClassCandidate<'a> {
    ident: &'a syn::Ident,
    superclass: syn::Ident,
    inherits: Option<Punctuated<syn::Ident, syn::Token![,]>>,
    item_struct: Option<&'a syn::ItemStruct>,
}

// #[objc] impl T { ... }

pub fn item_impl(attr: TokenStream, mut item_impl: syn::ItemImpl) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        let span = attr.span();
        let message = format!("#[objc]: unexpected arguments: `{attr}`");
        return Err(syn::Error::new(span, message));
    }
    if item_impl.trait_.is_none() && item_impl.unsafety.is_some() {
        // NOTE: inherent impls cannot be unsafe
        item_impl.unsafety = None;
    }
    let mut tokens = TokenStream::new();
    for attr in item_impl.attrs.iter() {
        if let syn::AttrStyle::Outer = attr.style {
            attr.to_tokens(&mut tokens);
        }
    }
    item_impl.defaultness.to_tokens(&mut tokens);
    item_impl.unsafety.to_tokens(&mut tokens);
    item_impl.impl_token.to_tokens(&mut tokens);
    item_impl.generics.to_tokens(&mut tokens);
    if let Some((polarity, path, for_token)) = item_impl.trait_ {
        polarity.to_tokens(&mut tokens);
        path.to_tokens(&mut tokens);
        for_token.to_tokens(&mut tokens);
    }
    item_impl.self_ty.to_tokens(&mut tokens);
    item_impl.generics.where_clause.to_tokens(&mut tokens);
    let (tx, rx) = oneshot::channel();
    item_impl.brace_token.surround(&mut tokens, |tokens| {
        for attr in item_impl.attrs.iter() {
            if let syn::AttrStyle::Inner(_) = attr.style {
                attr.to_tokens(tokens);
            }
        }
        for item in item_impl.items.into_iter() {
            if let syn::ImplItem::Method(method) = item {
                match crate::method::rewrite_method(MethodContext::Impl, method) {
                    Ok(method) => {
                        tokens.append_all(method);
                    }
                    Err(err) => {
                        tx.send(Err(err)).unwrap();
                        return;
                    }
                }
            }
        }
        tx.send(Ok(())).unwrap();
    });
    rx.recv().unwrap()?;
    Ok(tokens.into())
}

// #[objc] struct T { ... }

pub fn item_struct(attr: TokenStream, item_struct: syn::ItemStruct) -> syn::Result<TokenStream> {
    let ClassMacroInput {
        superclass,
        inherits,
    } = Parser::parse2(ClassMacroInput::parse, attr)?;

    let syn::ItemStruct { ident, .. } = &item_struct;

    let mut tokens = TokenStream::new();

    for attr in item_struct.attrs.iter() {
        if let syn::AttrStyle::Outer = attr.style {
            attr.to_tokens(&mut tokens);
        }
    }

    tokens.append_all(quote!(#[repr(C)]));

    item_struct.vis.to_tokens(&mut tokens);
    item_struct.struct_token.to_tokens(&mut tokens);
    item_struct.ident.to_tokens(&mut tokens);
    item_struct.generics.to_tokens(&mut tokens);

    if let syn::Fields::Named(fields) = &item_struct.fields {
        item_struct.generics.where_clause.to_tokens(&mut tokens);

        // FIXME: accumulate ivars helper stuff
        let (tx, rx) = oneshot::channel();
        fields.brace_token.surround(&mut tokens, |tokens| {
            let manually_drop = {
                let superclass = format_ident!("{}", superclass);
                quote!(__inner: objc2::__macro_helpers::ManuallyDrop<#superclass>)
            };
            tokens.append_all(manually_drop);

            let mut ivars_helper_mod_items = TokenStream::new();
            let mut ivars_declare_fn_block = TokenStream::new();

            ivars_helper_mod_items.append_all(quote!(use super::*;));

            for pair in fields.named.pairs() {
                if let Some(field) = pair.value().field_kind() {
                    match field {
                        FieldKind::IvarBool => {
                            todo!();
                        }
                        FieldKind::IvarDrop => {
                            todo!();
                        }
                        FieldKind::IvarEncode => {
                            todo!();
                        }
                        FieldKind::PhantomData => {
                            todo!();
                        }
                    }
                } else {
                    let span = pair.value().span();
                    let message = "#[obj]: all fields must be of type: `IvarBool`, `IvarDrop`, `IvarEncode`, or `PhantomData`";
                    tx.send(Err(syn::Error::new(span, message))).unwrap();
                    return;
                }
            }

            let ivars_helper_mod = quote!(
                mod ivars {
                    #ivars_helper_mod_items
                    pub(super) fn __objc2_declare_ivars(__objc2_builder: &mut objc2::declare::ClassBuilder) {
                        #ivars_declare_fn_block
                    }
                }
            );

            tx.send(Ok(ivars_helper_mod)).unwrap();
        });
        let ivars_helper_mod = rx.recv().unwrap()?;
    } else {
        let span = item_struct.fields.span();
        let message = "#[objc]: class structs must be defined using named fields";
        return Err(syn::Error::new(span, message));
    }

    Ok(tokens)
}

// #[objc] type T;

pub fn item_type(attr: TokenStream, item_type: crate::objc::ItemType) -> syn::Result<TokenStream> {
    let ClassMacroInput {
        superclass,
        inherits,
    } = Parser::parse2(ClassMacroInput::parse, attr)?;

    let crate::objc::ItemType { ident, .. } = &item_type;

    todo!()
}

struct TokensOrDefault<'a, T: 'a>(&'a Option<T>);

impl<'a, T> ToTokens for TokensOrDefault<'a, T>
where
    T: ToTokens + Default,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self.0 {
            Some(t) => t.to_tokens(tokens),
            None => T::default().to_tokens(tokens),
        }
    }
}
