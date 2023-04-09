use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

use crate::{
    common::{EmptyToken, KeyVal},
    interface::ClassEmitter,
};

pub(crate) enum MacroArgs {
    Declare {
        super_:
            KeyVal<syn::Token![super], crate::interface::Class, EmptyToken, syn::Token![unsafe]>,
        inherits: crate::interface::Inherits<syn::Token![unsafe]>,
        impl_attrs: Option<
            KeyVal<crate::interface::tokens::impl_attrs, ImplAttributesSpec, syn::Token![,]>,
        >,
        #[allow(unused)]
        comma_token: Option<syn::Token![,]>,
    },
    Continue {
        #[allow(unused)]
        continue_token: syn::Token![continue],
        impl_attrs: Option<
            KeyVal<crate::interface::tokens::impl_attrs, ImplAttributesSpec, syn::Token![,]>,
        >,
        #[allow(unused)]
        comma_token: Option<syn::Token![,]>,
    },
}

impl From<MacroArgs> for crate::interface::MacroArgs {
    fn from(args: MacroArgs) -> Self {
        Self::Extern(args)
    }
}

pub(crate) struct ImplAttributesSpec {
    #[allow(unused)]
    brace_token: syn::token::Brace,
    attrs: Vec<syn::Attribute>,
}

impl Parse for ImplAttributesSpec {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content;
        Ok(Self {
            brace_token: syn::braced!(content in input),
            attrs: content.call(syn::Attribute::parse_outer)?,
        })
    }
}

struct ClassContext {
    args: MacroArgs,
    item_span: proc_macro2::Span,
    class: Option<crate::interface::ItemType>,
    methods: TokenStream,
}

impl ClassContext {
    fn new(args: MacroArgs, item: &syn::ItemForeignMod) -> Self {
        Self {
            args,
            item_span: item.abi.span(),
            class: Default::default(),
            methods: Default::default(),
        }
    }
}

impl TryFrom<ClassContext> for crate::interface::ClassEmitter {
    type Error = syn::Error;

    fn try_from(ctx: ClassContext) -> Result<Self, Self::Error> {
        if let Some(item_type) = ctx.class {
            let (super_, inherits, impl_attrs) = match ctx.args {
                MacroArgs::Declare {
                    super_,
                    inherits,
                    impl_attrs,
                    ..
                } => {
                    let super_ = Some(super_.val);
                    let inherits = inherits.0.map(|kv| kv.val.classes);
                    let impl_attrs = {
                        let attrs = impl_attrs.map(|kv| kv.val.attrs).unwrap_or_default();
                        quote!(#(#attrs)*)
                    };
                    (super_, inherits, impl_attrs)
                }
                MacroArgs::Continue { impl_attrs, .. } => {
                    let impl_attrs = {
                        let attrs = impl_attrs.map(|kv| kv.val.attrs).unwrap_or_default();
                        quote!(#(#attrs)*)
                    };
                    (None, None, impl_attrs)
                }
            };

            let generics_this = item_type.generics.to_token_stream();
            let generics_this_split = {
                let (impl_generics, ty_generics, where_clause) =
                    item_type.generics.split_for_impl();
                (
                    impl_generics.to_token_stream(),
                    ty_generics.to_token_stream(),
                    where_clause.to_token_stream(),
                )
            };

            Ok(Self {
                this: crate::interface::ClassStruct::try_from(item_type)?,
                generics_this,
                generics_this_split,
                super_,
                inherits,
                attrs_impl: impl_attrs,
                methods: ctx.methods,
            })
        } else {
            Err(syn::Error::new(
                ctx.item_span,
                "objc2: no class `type` was found in `extern` block",
            ))
        }
    }
}

pub(crate) fn item_interface(
    args: MacroArgs,
    item_extern: syn::ItemForeignMod,
) -> syn::Result<TokenStream> {
    // Ensure ABI is "Objective-C"
    crate::common::check_extern_abi(&item_extern)?;

    let is_continuation = matches!(args, MacroArgs::Continue { .. });
    let mut ctx = ClassContext::new(args, &item_extern);

    for item in item_extern.items {
        match item {
            syn::ForeignItem::Type(item) => item_class(&mut ctx, item.into())?,
            syn::ForeignItem::Verbatim(tokens) => item_class(&mut ctx, syn::parse2(tokens)?)?,
            syn::ForeignItem::Fn(item) => {
                if ctx.class.is_none() {
                    return Err(syn::Error::new_spanned(
                        item,
                        "objc2: a class `type` must be declared before its methods",
                    ));
                } else {
                    let is_protocol_method = false;
                    crate::common::item_method(&mut ctx.methods, item, is_protocol_method)?;
                }
            }
            _ => {
                item_other(&item)?;
            }
        }
    }

    let emitter = ClassEmitter::try_from(ctx)?;
    let mut tokens = TokenStream::new();
    emitter.to_tokens(&mut tokens);

    if !is_continuation {
        tokens.append_all({
            let attrs_this_cfg = emitter.this.attrs_cfg;
            let class_this = emitter.this.ident;
            let message = format!("objc2: the struct `{class_this}` is not zero-sized!");
            quote!(
                #attrs_this_cfg
                const _: () = {
                    if ::objc2::__private::size_of::<#class_this>() != 0 {
                        ::objc2::__private::panic!(#message);
                    }
                };
            )
        });
    }

    Ok(tokens)
}

fn item_class(ctx: &mut ClassContext, item: crate::interface::ItemType) -> syn::Result<()> {
    if let Some(original) = &ctx.class {
        let mut error = syn::Error::new_spanned(
            item,
            "objc2: only a single class `type` is allowed per `extern` for #[interface]",
        );
        error.combine(syn::Error::new_spanned(
            original,
            "objc2: original class `type` declared here",
        ));
        return Err(error);
    } else {
        ctx.class = Some(item);
    }
    Ok(())
}

fn item_other(item: &syn::ForeignItem) -> syn::Result<()> {
    Err(syn::Error::new(
        item.span(),
        "objc2: only `type` and `fn` items are allowed in `extern` for #[interface]",
    ))
}
