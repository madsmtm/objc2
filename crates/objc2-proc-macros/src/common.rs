use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

#[derive(Default)]
pub(crate) struct EmptyToken;

impl Parse for EmptyToken {
    fn parse(_input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self)
    }
}

impl ToTokens for EmptyToken {
    fn to_tokens(&self, _tokens: &mut TokenStream) {}
}

/// Helper type for parsing the UserInfo specification fields.
pub(crate) struct KeyVal<Key, Val, Punctuation = EmptyToken, Unsafety = EmptyToken> {
    #[allow(unused)]
    pub(crate) punctuation: Punctuation,
    #[allow(unused)]
    pub(crate) unsafety: Unsafety,
    #[allow(unused)]
    pub(crate) key: Key,
    #[allow(unused)]
    pub(crate) separator: syn::Token![=],
    pub(crate) val: Val,
}

impl<Key, Val, Punctuation, Unsafety> Parse for KeyVal<Key, Val, Punctuation, Unsafety>
where
    Key: Parse,
    Val: Parse,
    Punctuation: Parse,
    Unsafety: Parse,
{
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(KeyVal {
            punctuation: input.parse()?,
            unsafety: input.parse()?,
            key: input.parse()?,
            separator: input.parse()?,
            val: input.parse()?,
        })
    }
}

#[derive(Clone, Copy, Default)]
#[repr(transparent)]
pub(crate) struct CfgAttributes<'a> {
    attrs: &'a [syn::Attribute],
}

impl<'a> Iterator for CfgAttributes<'a> {
    type Item = &'a syn::Attribute;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.attrs.get(0).filter(|attr| {
            self.attrs = &self.attrs[1..];
            attr.path.is_ident("cfg")
        })
    }
}

impl<'a> From<&'a [syn::Attribute]> for CfgAttributes<'a> {
    #[inline]
    fn from(attrs: &'a [syn::Attribute]) -> Self {
        Self { attrs }
    }
}

pub(crate) struct MethodSpecification {
    #[allow(unused)]
    pub(crate) optional: bool,
    pub(crate) selector: String,
    pub(crate) managed: Option<RetainSemantics>,
    pub(crate) throws: bool,
}

impl MethodSpecification {
    pub(crate) fn doc_alias(&self, sig: &syn::Signature) -> impl Iterator<Item = &str> + '_ {
        let alias = &self.selector[..self.selector.len() - 1];
        if sig.ident.unraw() != alias {
            Some(alias)
        } else {
            None
        }
        .into_iter()
    }
}

pub(crate) enum RetainSemantics {
    CopyOrMutCopy,
    Init,
    New,
    Other,
    Unspecified,
}

pub(crate) fn item_method(
    methods: &mut TokenStream,
    mut item: syn::ForeignItemFn,
    is_protocol_method: bool,
) -> syn::Result<()> {
    let span = item.span();
    let spec = crate::common::extract_method_specification(&mut item.attrs, span)?;
    let syn::ForeignItemFn {
        attrs, vis, sig, ..
    } = &item;

    let alias = spec.doc_alias(&item.sig);

    let mut receiver = TokenStream::new();
    let mut args = TokenStream::new();

    let mut args_iter = item.sig.inputs.iter();

    match args_iter.next() {
        Some(syn::FnArg::Receiver(_)) => {
            receiver = quote!(self);
        }
        Some(syn::FnArg::Typed(syn::PatType { pat, .. })) => match &**pat {
            syn::Pat::Ident(pi) if pi.ident == "self" || pi.ident == "this" => {
                receiver = pat.to_token_stream();
            }
            _ => {
                args.append_all(quote!(#pat,));
            }
        },
        None => {}
    }
    for arg in args_iter {
        match arg {
            syn::FnArg::Typed(syn::PatType { pat, .. }) => {
                args.append_all(quote!(#pat,));
            }
            syn::FnArg::Receiver(_) => unreachable!("already consumed receiver"),
        }
    }

    let selector = &spec.selector;

    let message_sender = {
        let throws_prefix = if spec.managed.is_none() && spec.throws {
            "__"
        } else {
            ""
        };
        let id = if spec.managed.is_some() { "_id" } else { "" };
        let throws_suffix = if spec.throws { "_error" } else { "" };
        let method = format_ident!("{throws_prefix}send_message{id}{throws_suffix}");
        if let Some(semantics) = &spec.managed {
            match semantics {
                RetainSemantics::CopyOrMutCopy => quote!(
                    <::objc2::__macro_helpers::CopyOrMutCopy as ::objc2::__macro_helpers::MsgSendId<_, _>>::#method
                ),
                RetainSemantics::Init => quote!(
                    <::objc2::__macro_helpers::Init as ::objc2::__macro_helpers::MsgSendId<_, _>>::#method
                ),
                RetainSemantics::New => quote!(
                    <::objc2::__macro_helpers::New as ::objc2::__macro_helpers::MsgSendId<_, _>>::#method
                ),
                RetainSemantics::Other => quote!(
                    <::objc2::__macro_helpers::Other as ::objc2::__macro_helpers::MsgSendId<_, _>>::#method
                ),
                RetainSemantics::Unspecified => quote!(
                    <::objc2::__macro_helpers::RetainSemantics<{
                        ::objc2::__macro_helpers::retain_semantics(#selector)
                    }> as ::objc2::__macro_helpers::MsgSendId<_, _>>::#method
                ),
            }
        } else {
            quote!(::objc2::MessageReceiver::#method)
        }
    };

    if receiver.is_empty() {
        receiver = quote!(<Self as ::objc2::ClassType>::class());
    }

    let body = quote!(
        #message_sender(
            #receiver,
            {
                static __OBJC2_CACHED_SEL: ::objc2::__macro_helpers::CachedSel = ::objc2::__macro_helpers::CachedSel::new();
                unsafe {
                    __OBJC2_CACHED_SEL.get(#selector)
                }
            },
            (#args)
        )
    );
    let body = if item.sig.unsafety.is_none() {
        quote!(unsafe { #body })
    } else {
        body
    };

    let where_clause = if is_protocol_method {
        quote!(
            where
                Self: ::objc2::__private::Sized + ::objc2::ClassType,
        )
    } else {
        quote!()
    };

    methods.append_all(quote!(
        #(#attrs)*
        #(#[doc(alias = #alias)])*
        #vis #sig #where_clause {
            #body
        }
    ));

    Ok(())
}

pub(crate) fn extract_method_specification(
    attrs: &mut Vec<syn::Attribute>,
    span: proc_macro2::Span,
) -> syn::Result<MethodSpecification> {
    let mut optional = false;
    let mut selector = String::new();
    let mut managed = None;
    let mut throws = false;

    for i in 0..attrs.len() {
        let attr = &attrs[i];
        if matches!(attr.style, syn::AttrStyle::Outer) {
            if let syn::Meta::List(syn::MetaList { path, nested, .. }) = attr.parse_meta()? {
                let mut segments = path.segments.iter();
                if let (Some(seg0), Some(seg1)) = (segments.next(), segments.next()) {
                    if attr.path.leading_colon.is_none()
                        && seg0.ident == "objc2"
                        && seg1.ident == "method"
                        && segments.next().is_none()
                    {
                        attrs.remove(i);
                        for meta in nested {
                            match meta {
                                syn::NestedMeta::Meta(meta) => {
                                    match meta {
                                        syn::Meta::Path(path) if path.is_ident("optional") => {
                                            optional = true;
                                        }
                                        syn::Meta::NameValue(name_value)
                                            if name_value.path.is_ident("sel") =>
                                        {
                                            if let syn::Lit::Str(str) = name_value.lit {
                                                selector.push_str(&str.value());
                                                selector.push('\0');
                                            } else {
                                                return Err(syn::Error::new_spanned(name_value.lit, "objc2: expected string literal for `sel` value"));
                                            }
                                        }
                                        syn::Meta::NameValue(name_value)
                                            if name_value.path.is_ident("managed") =>
                                        {
                                            if let syn::Lit::Str(str) = name_value.lit {
                                                match str.value().as_str() {
                                                    "CopyOrMutCopy" => {
                                                        managed =
                                                            Some(RetainSemantics::CopyOrMutCopy);
                                                    }
                                                    "Init" => {
                                                        managed = Some(RetainSemantics::Init);
                                                    }
                                                    "New" => {
                                                        managed = Some(RetainSemantics::New);
                                                    }
                                                    "Other" => {
                                                        managed = Some(RetainSemantics::Other);
                                                    }
                                                    _ => {}
                                                }
                                            } else {
                                                return Err(syn::Error::new_spanned(name_value.lit, "objc2: expected string literal for `managed` value"));
                                            }
                                        }
                                        syn::Meta::Path(path) if path.is_ident("managed") => {
                                            managed = Some(RetainSemantics::Unspecified);
                                        }
                                        syn::Meta::Path(path) if path.is_ident("throws") => {
                                            throws = true;
                                        }
                                        _ => {
                                            return Err(syn::Error::new_spanned(
                                                meta,
                                                "objc2: unexpected argument",
                                            ));
                                        }
                                    }
                                }
                                _ => {
                                    return Err(syn::Error::new_spanned(
                                        meta,
                                        "objc2: unexpected argument",
                                    ));
                                }
                            }
                        }
                        return Ok(MethodSpecification {
                            optional,
                            selector,
                            managed,
                            throws,
                        });
                    }
                }
            }
        }
    }

    Err(syn::Error::new(
        span,
        "objc2: methods must have an `#[objc2::method]` attribute",
    ))
}

pub(crate) fn check_extern_abi(item_extern: &syn::ItemForeignMod) -> syn::Result<()> {
    if item_extern
        .abi
        .name
        .as_ref()
        .map(|lit| lit.value())
        .as_deref()
        != Some("Objective-C")
    {
        return Err(syn::Error::new_spanned(
            &item_extern.abi,
            r#"objc2: ABI must be "Objective-C""#,
        ));
    }
    Ok(())
}
