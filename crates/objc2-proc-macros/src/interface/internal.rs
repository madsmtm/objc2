use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};

use crate::common::KeyVal;

pub(crate) struct MacroArgs {
    #[allow(unused)]
    pub(super) super_class: KeyVal<syn::Token![super], crate::interface::Class>,
    #[allow(unused)]
    pub(super) inherits: crate::interface::Inherits,
    #[allow(unused)]
    pub(super) comma_token: Option<syn::Token![,]>,
}

impl From<MacroArgs> for crate::interface::MacroArgs {
    fn from(args: MacroArgs) -> Self {
        Self::Intern(args)
    }
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            super_class: input.parse()?,
            inherits: input.parse()?,
            comma_token: input.parse()?,
        })
    }
}

pub(crate) fn item_interface(_args: MacroArgs, item_mod: syn::ItemMod) -> syn::Result<TokenStream> {
    if item_mod.ident != "__" {
        return Err(syn::Error::new_spanned(
            item_mod,
            "objc2: interface `mod` items must be named `__`",
        ));
    }
    Ok(item_mod.to_token_stream())
}
