use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream};

pub(crate) struct MacroArgs;

impl Parse for MacroArgs {
    fn parse(_input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self)
    }
}

pub(crate) mod tokens {
    syn::custom_keyword!(protocol);
}

#[allow(unused)]
pub(crate) fn item_trait(_args: MacroArgs, item_trait: syn::ItemTrait) -> syn::Result<TokenStream> {
    let syn::ItemTrait {
        attrs,
        vis,
        unsafety,
        auto_token,
        trait_token,
        ident,
        generics,
        colon_token,
        supertraits,
        brace_token,
        mut items,
    } = item_trait;

    if unsafety.is_none() {
        return Err(syn::Error::new_spanned(
            trait_token,
            "objc2: protocol traits are expected to be `unsafe`",
        ));
    }

    let mut methods = TokenStream::new();
    for mut item in items.into_iter() {
        match item {
            syn::TraitItem::Method(item) => {
                let is_protocol_method = true;
                crate::common::item_method(
                    &mut methods,
                    TraitItemMethod(item).into(),
                    is_protocol_method,
                )?;
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    item,
                    "objc2: only methods are allowed in `protocol` traits",
                ));
            }
        }
    }

    let mut tokens = TokenStream::new();

    tokens.append_all(attrs);
    vis.to_tokens(&mut tokens);
    unsafety.to_tokens(&mut tokens);
    auto_token.to_tokens(&mut tokens);
    trait_token.to_tokens(&mut tokens);
    ident.to_tokens(&mut tokens);
    generics.to_tokens(&mut tokens);
    if !supertraits.is_empty() {
        item_trait
            .colon_token
            .unwrap_or_default()
            .to_tokens(&mut tokens);
        supertraits.to_tokens(&mut tokens);
    }
    generics.where_clause.to_tokens(&mut tokens);
    item_trait
        .brace_token
        .surround(&mut tokens, |tokens| tokens.append_all(methods));

    tokens.append_all({
        let ident_string = ident.to_string();
        quote!(
            unsafe impl<T> #ident for ::objc2::runtime::ProtocolObject<T>
            where
                T: ?::objc2::__private::Sized + ::objc2::ProtocolType + #ident
            {}

            // SAFETY: The specified name is ensured by caller to be a protocol,
            // and is correctly defined.
            unsafe impl ::objc2::ProtocolType for dyn #ident {
                const NAME: &'static ::objc2::__private::str = #ident_string;
                const __INNER: () = ();
            }

            // SAFETY: Anything that implements the protocol `$name` is valid to
            // convert to `ProtocolObject<dyn $name>`.
            unsafe impl<T> ::objc2::runtime::ImplementedBy<T> for dyn #ident
            where
                T: ?::objc2::__private::Sized + ::objc2::Message + #ident
            {
                const __INNER: () = ();
            }
        )
    });

    Ok(tokens)
}

struct TraitItemMethod(syn::TraitItemMethod);

impl From<TraitItemMethod> for syn::ForeignItemFn {
    fn from(item: TraitItemMethod) -> Self {
        let TraitItemMethod(syn::TraitItemMethod {
            attrs,
            sig,
            default,
            semi_token,
        }) = item;
        debug_assert!(default.is_none());
        Self {
            attrs,
            vis: syn::Visibility::Inherited,
            sig,
            semi_token: semi_token.unwrap_or_default(),
        }
    }
}
