use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use std::collections::BTreeSet;
use syn::spanned::Spanned;

// Specifies whether the method (signature) is within a `trait` or `impl` block
#[derive(Debug)]
pub enum MethodContext {
    Impl,
    Trait,
}

// Rewrite a method (signature) within a `trait` or `impl` block for objc interop
pub fn rewrite_method(
    context: MethodContext,
    mut item_method: syn::ImplItemMethod,
) -> syn::Result<TokenStream> {
    let mut tokens = TokenStream::new();
    for attr in item_method.attrs.iter() {
        if let syn::AttrStyle::Outer = attr.style {
            if attr.path.is_ident("objc") {
                continue;
            }
            attr.to_tokens(&mut tokens);
        }
    }
    item_method.vis.to_tokens(&mut tokens);
    item_method.defaultness.to_tokens(&mut tokens);
    item_method.sig.to_tokens(&mut tokens);
    let (tx, rx) = oneshot::channel();
    item_method
        .block
        .brace_token
        .surround(&mut tokens, |tokens| {
            for attr in item_method.attrs.iter() {
                if let syn::AttrStyle::Inner(_) = attr.style {
                    attr.to_tokens(tokens);
                }
            }
            if item_method.block.stmts.len() == 1 {
                if let syn::Stmt::Item(syn::Item::Verbatim(verbatim)) = &item_method.block.stmts[0]
                {
                    if verbatim.to_string() == ";" {
                        match rewrite_method_block(context, &item_method) {
                            Ok(block) => {
                                tokens.append_all(block);
                            }
                            Err(err) => {
                                tx.send(Err(err)).unwrap();
                                return;
                            }
                        }
                    }
                }
            } else {
                tokens.append_all(&item_method.block.stmts);
            }
            tx.send(Ok(())).unwrap();
        });
    rx.recv().unwrap()?;
    Ok(tokens.into())
}

// Rewrite a method block within a `trait` or `impl` block for objc interop
fn rewrite_method_block(
    context: MethodContext,
    item_method: &syn::ImplItemMethod,
) -> syn::Result<TokenStream> {
    // TODO: handle `optional`
    let MethodSpecification {
        item_method,
        mut selector,
        receiver,
        semantics,
        ..
    } = elaborate_method(context, item_method)?;

    // null-terminate the selector string
    selector.push('\u{0}');

    // construct the selector caching block syntax fragment
    let cached_sel = quote!({
        static __CACHED_SEL: objc2::__macro_helpers::CachedSel = objc2::__macro_helpers::CachedSel::new();
        __CACHED_SEL.get(#selector)
    });

    // construct the receiver syntax fragment
    let receiver = match receiver {
        MethodReceiver::Class => quote!(<Self as objc2::ClassType>::class()),
        MethodReceiver::SelfType { .. } => quote!(&self),
        MethodReceiver::This { .. } => quote!(this),
    };

    // extract the method args
    let args = item_method.get_args().filter_map(|arg| {
        if let syn::FnArg::Typed(ty) = arg {
            Some(&*ty.pat)
        } else {
            None
        }
    });

    // handle the managed method case (returns `Id`)
    if let Some(semantics) = semantics {
        // compute the semantics syntax fragment
        let semantics = if let MethodSemantics::Unspecified = semantics {
            quote!(RetainSemantics<{objc2::__macro_helpers::retain_semantics(#selector)}>)
        } else {
            quote::format_ident!("{semantics}").into_token_stream()
        };
        // compute the block syntax fragment
        let block = quote!(
            #[allow(unused_unsafe)]
            unsafe {
                <objc2::__macro_helpers::#semantics as objc2::__macro_helpers::MsgSendId<_,_>>::send_message_id(
                    #receiver,
                    #cached_sel,
                    (#(#args,)*),
                )
            }
        );
        return Ok(block);
    }

    // handle the non-managed method case
    let block = quote!(
        #[allow(unused_unsafe)]
        unsafe {
            objc2::MessageReceiver::send_message::<_, _>(
                #receiver,
                #cached_sel,
                (#(#args,)*),
            )
        }
    );
    Ok(block)
}

// A candidate method for objc interop rewriting (prior to elaboration)
#[derive(Debug)]
struct MethodCandidate<'a> {
    context: MethodContext,
    item_method: &'a syn::ImplItemMethod,
    meta: Option<syn::Meta>,
}

// A fully elaborated method suitable for objc interop
struct MethodSpecification<'a> {
    context: MethodContext,
    item_method: &'a syn::ImplItemMethod,
    selector: String,
    receiver: MethodReceiver<'a>,
    semantics: Option<MethodSemantics>,
    out_params: BTreeSet<usize>,
    optional: bool,
}

// Try to elaborate a candidate method into a full method specification
impl<'a> TryFrom<MethodCandidate<'a>> for MethodSpecification<'a> {
    type Error = syn::Error;

    fn try_from(candidate: MethodCandidate<'a>) -> Result<Self, Self::Error> {
        use syn::{Lit, Meta, MetaList, NestedMeta};

        let MethodCandidate {
            context,
            item_method,
            meta,
        } = candidate;
        let mut selector = String::new();
        let receiver = get_method_receiver(&item_method.sig);
        let mut method_return = None;
        let mut semantics = None::<MethodSemantics>;
        let mut out_params = BTreeSet::new();
        let mut optional = false;

        if let Some(meta) = &meta {
            match meta {
                // TODO:: maybe warn about being redundant (with no arguments)
                Meta::Path(path) => {
                    debug_assert!(path.is_ident("objc"));
                }
                Meta::List(MetaList { path, nested, .. }) => {
                    debug_assert!(path.is_ident("objc"));

                    let mut processor = crate::objc::MetaProcessor::new();

                    for nested_meta in nested {
                        if let NestedMeta::Meta(meta) = nested_meta {
                            match meta {
                                // handle `sel = <selector>` argument
                                Meta::NameValue(name_value) if name_value.path.is_ident("sel") => {
                                    processor.ensure_unique(&name_value.path)?;
                                    // check that `<selector>` is a string
                                    if let Lit::Str(str) = &name_value.lit {
                                        // parse `<selector>`
                                        selector = Selector::from(str).into();
                                    } else {
                                        let span = name_value.lit.span();
                                        let key = "sel";
                                        let r#type = "a string";
                                        return Err(crate::objc::error_invalid_value_type(
                                            span, key, r#type,
                                        ));
                                    }
                                }
                                // handle `managed = <semantics>` argument
                                Meta::NameValue(name_value)
                                    if name_value.path.is_ident("managed") =>
                                {
                                    processor.ensure_unique(&name_value.path)?;
                                    // ensure we only parse `managed` once
                                    if semantics.is_some() {
                                        let span = name_value.span();
                                        let message =
                                            "#[objc]: argument `managed` already specified";
                                        return Err(syn::Error::new(span, message));
                                    }
                                    // analyze the method return type (if not already done)
                                    match method_return
                                        .get_or_insert_with(|| item_method.get_return())
                                    {
                                        // check for allowed return types for `managed`
                                        MethodReturn::Managed { .. }
                                        | MethodReturn::Result {
                                            ownership: Some(_), ..
                                        } => {
                                            // okay
                                        }
                                        _ => {
                                            let span = item_method.sig.output.span();
                                            let message = "#[objc]: return must be `Id<T, _>` or `Result<Id<T, O>, _>` for `managed`";
                                            return Err(syn::Error::new(span, message));
                                        }
                                    }
                                    // check that `<semantics>` is a string
                                    if let Lit::Str(str) = &name_value.lit {
                                        // parse `<semantics>`
                                        semantics = Some(str.try_into()?);
                                    } else {
                                        let span = name_value.lit.span();
                                        let key = "managed";
                                        let r#type = "a string";
                                        return Err(crate::objc::error_invalid_value_type(
                                            span, key, r#type,
                                        ));
                                    }
                                }
                                // handle `managed` (no value) argument
                                Meta::Path(path) if path.is_ident("managed") => {
                                    processor.ensure_unique(&path)?;
                                    // ensure we only parse `managed` once
                                    if semantics.is_some() {
                                        let span = path.span();
                                        let message =
                                            "#[objc]: argument `managed` already specified";
                                        return Err(syn::Error::new(span, message));
                                    }
                                    // analyze the method return type (if not already done)
                                    match method_return
                                        .get_or_insert_with(|| item_method.get_return())
                                    {
                                        // check for allowed return types for `managed`
                                        MethodReturn::Managed { .. }
                                        | MethodReturn::Result {
                                            ownership: Some(_), ..
                                        } => {
                                            // okay
                                        }
                                        _ => {
                                            let span = item_method.sig.output.span();
                                            let message = "#[objc]: return must be `Id<T, _>` or `Result<Id<T, O>, _>` for `managed`";
                                            return Err(syn::Error::new(span, message));
                                        }
                                    }
                                    // use `unspecified` for semantics since no value was given
                                    semantics = Some(MethodSemantics::Unspecified);
                                }
                                // handle `optional` argument
                                Meta::Path(path) if path.is_ident("optional") => {
                                    processor.ensure_unique(&path)?;
                                    // check that method context is a trait (protocol)
                                    if let MethodContext::Trait = context {
                                        optional = true;
                                    } else {
                                        let span = meta.span();
                                        let message = "#[objc]: `optional` is only allowed for trait (protocol) methods";
                                        return Err(syn::Error::new(span, message));
                                    }
                                }
                                // FIXME: have `catch` as a special case (just for `NSError`)
                                // handle `out` argument
                                Meta::Path(path) if path.is_ident("out") => {
                                    // analyze the method return type (if not already done)
                                    match method_return
                                        .get_or_insert_with(|| item_method.get_return())
                                    {
                                        // check for allowed return types for `catch`
                                        MethodReturn::Result { .. } => {
                                            // TODO: generalize
                                            let last_arg = item_method.sig.inputs.len() - 1;
                                            out_params.insert(last_arg);
                                        }
                                        _ => {
                                            let span = item_method.sig.output.span();
                                            let message = "#[objc]: return must be `Result<T, Id<NSError, O>>` for `out`";
                                            return Err(syn::Error::new(span, message));
                                        }
                                    }
                                }
                                _ => {
                                    let span = meta.span();
                                    return Err(crate::objc::error_invalid_argument(span));
                                }
                            }
                        } else {
                            let span = nested_meta.span();
                            return Err(crate::objc::error_invalid_argument(span));
                        }
                    }
                }
                _ => {
                    let span = meta.span();
                    return Err(crate::objc::error_invalid_argument(span));
                }
            }
        }

        // synthesize selector if not specified by attribute
        if selector.is_empty() {
            let receiver = &receiver;
            let sig = &item_method.sig;
            let candidate = SelectorCandidate { receiver, sig };
            selector = Selector::try_from(candidate)?.into();
        }

        // synthesize semantics if not specified by attribute
        if semantics.is_none() {
            // analyze the method return type (if not already done)
            match method_return.get_or_insert_with(|| item_method.get_return()) {
                MethodReturn::Managed { .. }
                | MethodReturn::Result {
                    ownership: Some(_), ..
                } => {
                    semantics = Some(MethodSemantics::Unspecified);
                }
                _ => {}
            }
        }

        Ok(MethodSpecification {
            context,
            item_method,
            selector,
            receiver,
            semantics,
            out_params,
            optional,
        })
    }
}

// The method receiver type
#[derive(Debug)]
enum MethodReceiver<'a> {
    Class,
    SelfType { arg: &'a syn::FnArg },
    This { arg: &'a syn::FnArg },
}

struct SelectorCandidate<'a> {
    receiver: &'a MethodReceiver<'a>,
    sig: &'a syn::Signature,
}

// Wrapper struct for selector (for impls)
struct Selector(String);

// Create a selector for a string
impl From<&syn::LitStr> for Selector {
    fn from(str: &syn::LitStr) -> Self {
        Selector(str.value())
    }
}

// Synthesize a selector from a method signature
impl<'a> TryFrom<SelectorCandidate<'a>> for Selector {
    type Error = syn::Error;

    fn try_from(candidate: SelectorCandidate<'_>) -> Result<Self, Self::Error> {
        use MethodReceiver::*;
        let mut count = 0usize;
        let mut prev_was_underscore = false;
        let mut sel = String::new();
        for c in candidate.sig.ident.to_string().chars() {
            if c == '_' {
                if prev_was_underscore {
                    let span = candidate.sig.ident.span();
                    let message = "#[objc]: subsequent underscores in `fn` name disallowed; specify selector if this is needed";
                    return Err(syn::Error::new(span, message));
                }
                sel.push(':');
                count += 1;
                prev_was_underscore = true;
            } else {
                sel.push(c);
                prev_was_underscore = false;
            }
        }
        let arg_count = match candidate.receiver {
            SelfType { .. } | This { .. } => candidate.sig.inputs.len() - 1,
            Class => 0,
        };
        if arg_count == count + 1 {
            sel.push(':');
        } else if 0 < arg_count || 0 < count {
            let span = candidate.sig.span();
            let message = "#[objc]: number of `_` and argument count disagree";
            return Err(syn::Error::new(span, message));
        }
        Ok(Selector(sel))
    }
}

impl From<Selector> for String {
    fn from(sel: Selector) -> Self {
        sel.0
    }
}

// Retain semantics for a method
#[derive(Debug)]
enum MethodSemantics {
    Init,
    New,
    Other,
    Unspecified,
}

impl std::fmt::Display for MethodSemantics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl TryFrom<&syn::LitStr> for MethodSemantics {
    type Error = syn::Error;

    fn try_from(str: &syn::LitStr) -> Result<Self, Self::Error> {
        let value = str.value();
        match value.as_str() {
            "init" => Ok(MethodSemantics::Init),
            "new" => Ok(MethodSemantics::New),
            "other" => Ok(MethodSemantics::Other),
            _ => {
                let value = value.to_ascii_lowercase();
                if ["init", "new", "other"].contains(&value.as_str()) {
                    let span = str.span();
                    let message = r#"`managed` value must be a lowercase string"#;
                    Err(syn::Error::new(span, message))
                } else {
                    let span = str.span();
                    let message = r#"`managed` value must be one of { "init", "new", "other" }"#;
                    Err(syn::Error::new(span, message))
                }
            }
        }
    }
}

// Ownership for the `Id` type
#[derive(Debug)]
enum ObjectOwnership {
    Owned,
    Shared,
}

// The form of the method return type
#[derive(Debug)]
enum MethodReturn<'a> {
    // no explicit return type (implict `()`)
    Default,
    // return type of `Id<T, O>`
    Managed {
        r#type: &'a syn::Type,
        ownership: ObjectOwnership,
    },
    // return type of `Result<T, Id<NSError, Shared>>`
    Result {
        r#type: &'a syn::Type,
        ownership: Option<ObjectOwnership>,
    },
    // any other return type `T`
    Typical {
        r#type: &'a syn::Type,
    },
}

trait ItemMethodExt {
    fn get_return(&self) -> MethodReturn<'_>;
    fn get_receiver(&self) -> MethodReceiver<'_>;
    fn get_args(&self) -> Box<dyn Iterator<Item = &syn::FnArg> + '_>;
}

impl ItemMethodExt for syn::TraitItemMethod {
    fn get_return(&self) -> MethodReturn<'_> {
        get_method_return(&self.sig)
    }

    fn get_receiver(&self) -> MethodReceiver<'_> {
        get_method_receiver(&self.sig)
    }

    fn get_args(&self) -> Box<dyn Iterator<Item = &syn::FnArg> + '_> {
        let iter = self.sig.inputs.iter();
        if self.sig.receiver().is_some() {
            Box::new(iter.skip(1))
        } else {
            Box::new(iter)
        }
    }
}

impl ItemMethodExt for syn::ImplItemMethod {
    fn get_return(&self) -> MethodReturn<'_> {
        get_method_return(&self.sig)
    }

    fn get_receiver(&self) -> MethodReceiver<'_> {
        get_method_receiver(&self.sig)
    }

    fn get_args(&self) -> Box<dyn Iterator<Item = &syn::FnArg> + '_> {
        let iter = self.sig.inputs.iter();
        if self.sig.receiver().is_some() {
            Box::new(iter.skip(1))
        } else {
            Box::new(iter)
        }
    }
}

// Compute the return type from a method signature
fn get_method_return(sig: &syn::Signature) -> MethodReturn<'_> {
    use syn::ReturnType;
    match &sig.output {
        ReturnType::Default => MethodReturn::Default,
        ReturnType::Type(_, ty) => get_method_return_from_type(ty),
    }
}

// Compute a method return type from a given type
fn get_method_return_from_type(r#type: &syn::Type) -> MethodReturn<'_> {
    use syn::{GenericArgument, PathArguments, Type};
    if let Type::Path(type_path) = r#type {
        if let Some(segment) = type_path.path.segments.last() {
            // check for `Result<S, E>`
            if segment.ident == "Result" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(ty)) = args.args.last() {
                        // check that `E = Id<T, O>`
                        // FIXME: should we check for `Shared` here too?
                        if let MethodReturn::Managed { r#type: error, .. } =
                            get_method_return_from_type(ty)
                        {
                            if let Type::Path(type_path) = error {
                                if let Some(segment) = type_path.path.segments.last() {
                                    // check that `T = NSError`
                                    if segment.ident == "NSError" {
                                        if let Some(GenericArgument::Type(r#type)) =
                                            args.args.first()
                                        {
                                            if let MethodReturn::Managed { r#type, ownership } =
                                                get_method_return_from_type(r#type)
                                            {
                                                // handle the case where `S = Id<_, _>` case
                                                let ownership = ownership.into();
                                                return MethodReturn::Result { r#type, ownership };
                                            } else {
                                                // handle the case where `S` is any other type
                                                let ownership = None;
                                                return MethodReturn::Result { r#type, ownership };
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            // check for `Id<T, O>`
            if segment.ident == "Id" {
                if let PathArguments::AngleBracketed(args) = &segment.arguments {
                    if let Some(GenericArgument::Type(ty)) = args.args.last() {
                        if let Type::Path(type_path) = &*ty {
                            if let Some(segment) = type_path.path.segments.last() {
                                // check for `O = Owned`
                                if segment.ident == "Owned" {
                                    if let Some(GenericArgument::Type(ty)) = args.args.first() {
                                        let r#type = ty;
                                        let ownership = ObjectOwnership::Owned;
                                        return MethodReturn::Managed { r#type, ownership };
                                    }
                                }
                                // check for `O = Shared`
                                if segment.ident == "Shared" {
                                    if let Some(GenericArgument::Type(ty)) = args.args.first() {
                                        let r#type = ty;
                                        let ownership = ObjectOwnership::Shared;
                                        return MethodReturn::Managed { r#type, ownership };
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    MethodReturn::Typical { r#type }
}

// Compute the receiver from a method signature
fn get_method_receiver(sig: &syn::Signature) -> MethodReceiver<'_> {
    use syn::{FnArg, Pat, PatIdent, PatType};
    if let Some(arg) = sig.inputs.first() {
        match arg {
            FnArg::Receiver(_) => {
                return MethodReceiver::SelfType { arg };
            }
            FnArg::Typed(PatType { pat, .. }) => {
                if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
                    if ident == "self" {
                        return MethodReceiver::SelfType { arg };
                    }
                    if ident == "this" {
                        return MethodReceiver::This { arg };
                    }
                }
            }
        }
    }
    MethodReceiver::Class
}

// Elaborate a method candidate into a method specification
fn elaborate_method(
    context: MethodContext,
    item_method: &syn::ImplItemMethod,
) -> syn::Result<MethodSpecification<'_>> {
    let meta = item_method
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident("objc"))
        .map(|attr| attr.parse_meta())
        .transpose()?;
    let candidate = MethodCandidate {
        context,
        item_method,
        meta,
    };
    candidate.try_into()
}
