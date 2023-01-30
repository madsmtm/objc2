use proc_macro2::{Span, TokenStream};
use std::collections::HashSet;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
};

pub struct ItemType {
    pub attrs: Vec<syn::Attribute>,
    pub vis: syn::Visibility,
    pub type_token: syn::Token![type],
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub semi_token: syn::Token![;],
}

pub enum ItemObjC {
    ItemImpl(syn::ItemImpl),
    ItemStruct(syn::ItemStruct),
    ItemTrait(syn::ItemTrait),
    ItemType(ItemType),
}

// NOTE: definition from syn crate
fn data_struct(
    input: ParseStream<'_>,
) -> syn::Result<(
    Option<syn::WhereClause>,
    syn::Fields,
    Option<syn::Token![;]>,
)> {
    let mut lookahead = input.lookahead1();
    let mut where_clause = None;
    if lookahead.peek(syn::Token![where]) {
        where_clause = Some(input.parse()?);
        lookahead = input.lookahead1();
    }

    if where_clause.is_none() && lookahead.peek(syn::token::Paren) {
        let fields = input.parse()?;

        lookahead = input.lookahead1();
        if lookahead.peek(syn::Token![where]) {
            where_clause = Some(input.parse()?);
            lookahead = input.lookahead1();
        }

        if lookahead.peek(syn::Token![;]) {
            let semi = input.parse()?;
            Ok((where_clause, syn::Fields::Unnamed(fields), Some(semi)))
        } else {
            Err(lookahead.error())
        }
    } else if lookahead.peek(syn::token::Brace) {
        let fields = input.parse()?;
        Ok((where_clause, syn::Fields::Named(fields), None))
    } else if lookahead.peek(syn::Token![;]) {
        let semi = input.parse()?;
        Ok((where_clause, syn::Fields::Unit, Some(semi)))
    } else {
        Err(lookahead.error())
    }
}

fn parse_rest_of_impl(
    input: ParseStream<'_>,
    mut attrs: Vec<syn::Attribute>,
    unsafety: Option<syn::Token![unsafe]>,
) -> syn::Result<syn::ItemImpl> {
    let impl_token = input.parse::<syn::Token![impl]>()?;

    let has_generics = input.peek(syn::Token![<])
        && (input.peek2(syn::Token![>])
            || input.peek2(syn::Token![#])
            || (input.peek2(syn::Ident) || input.peek2(syn::Lifetime))
                && (input.peek3(syn::Token![:])
                    || input.peek3(syn::Token![,])
                    || input.peek3(syn::Token![>])
                    || input.peek3(syn::Token![=]))
            || input.peek2(syn::Token![const]));
    let mut generics: syn::Generics = if has_generics {
        input.parse()?
    } else {
        syn::Generics::default()
    };

    let polarity = None::<syn::Token![!]>;
    let first_ty_span = input.span();
    let mut first_ty: syn::Type = input.parse()?;
    let self_ty: syn::Type;
    let trait_;

    let is_impl_for = input.peek(syn::Token![for]);
    if is_impl_for {
        let for_token: syn::Token![for] = input.parse()?;
        let mut first_ty_ref = &first_ty;
        while let syn::Type::Group(ty) = first_ty_ref {
            first_ty_ref = &ty.elem;
        }
        if let syn::Type::Path(syn::TypePath { qself: None, .. }) = first_ty_ref {
            while let syn::Type::Group(ty) = first_ty {
                first_ty = *ty.elem;
            }
            if let syn::Type::Path(syn::TypePath { qself: None, path }) = first_ty {
                trait_ = Some((polarity, path, for_token));
            } else {
                unreachable!();
            }
        } else {
            return Err(syn::Error::new(first_ty_span, "expected trait path"));
        }
        self_ty = input.parse()?;
    } else {
        trait_ = None;
        self_ty = first_ty;
    }

    if input.peek(syn::Token![for]) {
        let span = impl_token.span();
        let message = "[objc]: expected inherent impl";
        return Err(syn::Error::new(span, message));
    }

    generics.where_clause = input.parse()?;

    let content;
    let brace_token = syn::braced!(content in input);
    attrs.extend(syn::Attribute::parse_inner(&content)?);

    let mut items = Vec::new();
    while !content.is_empty() {
        items.push(content.parse()?);
    }
    return Ok(syn::ItemImpl {
        attrs,
        defaultness: None,
        unsafety,
        impl_token,
        generics,
        trait_: None,
        self_ty: Box::new(self_ty),
        brace_token,
        items,
    });
}

fn parse_rest_of_struct(
    input: ParseStream<'_>,
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
) -> syn::Result<syn::ItemStruct> {
    let struct_token = input.parse()?;
    let ident = input.parse()?;
    let generics = input.parse()?;
    let (where_clause, fields, semi_token) = data_struct(input)?;
    return Ok(syn::ItemStruct {
        attrs,
        vis,
        struct_token,
        ident,
        generics: syn::Generics {
            where_clause,
            ..generics
        },
        fields,
        semi_token,
    });
}

fn parse_rest_of_trait(
    input: ParseStream<'_>,
    mut attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    unsafety: Option<syn::Token![unsafe]>,
) -> syn::Result<syn::ItemTrait> {
    let auto_token = input.parse()?;
    let trait_token = input.parse()?;
    let ident = input.parse()?;
    let mut generics = input.parse::<syn::Generics>()?;

    let colon_token: Option<syn::Token![:]> = input.parse()?;

    let mut supertraits = syn::punctuated::Punctuated::new();
    if colon_token.is_some() {
        loop {
            if input.peek(syn::Token![where]) || input.peek(syn::token::Brace) {
                break;
            }
            supertraits.push_value(input.parse()?);
            if input.peek(syn::Token![where]) || input.peek(syn::token::Brace) {
                break;
            }
            supertraits.push_punct(input.parse()?);
        }
    }

    generics.where_clause = input.parse()?;

    let content;
    let brace_token = syn::braced!(content in input);
    attrs.extend(syn::Attribute::parse_inner(&content)?);
    let mut items = Vec::new();
    while !content.is_empty() {
        items.push(content.parse()?);
    }

    Ok(syn::ItemTrait {
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
        items,
    })
}

fn parse_rest_of_type(
    input: ParseStream<'_>,
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
) -> syn::Result<self::ItemType> {
    let type_token = input.parse()?;
    let ident = input.parse()?;
    let generics = {
        let mut generics = input.parse::<syn::Generics>()?;
        generics.where_clause = input.parse()?;
        generics
    };
    let semi_token = input.parse()?;
    Ok(ItemType {
        attrs,
        vis,
        type_token,
        ident,
        generics,
        semi_token,
    })
}

impl Parse for ItemObjC {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        // parse attributes
        let attrs = input.call(syn::Attribute::parse_outer)?;
        let vis = input.parse()?;

        let lookahead = input.lookahead1();

        // parse `type`
        if lookahead.peek(syn::Token![type]) {
            let item_type = parse_rest_of_type(input, attrs, vis)?;
            return Ok(ItemObjC::ItemType(item_type));
        }

        // parse `struct`
        if lookahead.peek(syn::Token![struct]) {
            let item_struct = parse_rest_of_struct(input, attrs, vis)?;
            return Ok(ItemObjC::ItemStruct(item_struct));
        }

        // parse `unsafe`
        if lookahead.peek(syn::Token![unsafe]) {
            let unsafety = Some(input.parse()?);

            let lookahead = input.lookahead1();

            // parse impl
            if lookahead.peek(syn::Token![impl]) {
                let item_impl = parse_rest_of_impl(input, attrs, unsafety)?;
                return Ok(ItemObjC::ItemImpl(item_impl));
            }

            // parse `trait`
            if lookahead.peek(syn::Token![trait]) {
                let item_trait = parse_rest_of_trait(input, attrs, vis, unsafety)?;
                return Ok(ItemObjC::ItemTrait(item_trait));
            }

            return Err(lookahead.error());
        }

        Err(lookahead.error())
    }
}

pub fn objc(attr: TokenStream, item: TokenStream) -> syn::Result<TokenStream> {
    let item = syn::parse2::<ItemObjC>(item)?;
    match item {
        ItemObjC::ItemImpl(item_impl) => crate::class::item_impl(attr, item_impl),
        ItemObjC::ItemStruct(item_struct) => crate::class::item_struct(attr, item_struct),
        ItemObjC::ItemTrait(item_trait) => crate::protocol::item_trait(attr, item_trait),
        ItemObjC::ItemType(item_type) => crate::class::item_type(attr, item_type),
    }
}

pub struct MetaProcessor<'m> {
    processed: HashSet<&'m syn::Path>,
}

impl<'m> MetaProcessor<'m> {
    pub fn new() -> Self {
        let processed = HashSet::default();
        Self { processed }
    }

    pub fn ensure_unique(&mut self, path: &'m syn::Path) -> syn::Result<()> {
        if !self.processed.insert(path) {
            let span = path.span();
            let message = "#[objc]: duplicate argument";
            Err(syn::Error::new(span, message))
        } else {
            Ok(())
        }
    }
}

pub fn error_invalid_argument(span: Span) -> syn::Error {
    let message = "#[objc]: invalid argument";
    syn::Error::new(span, message)
}

pub fn error_invalid_value_type(span: Span, key: &str, r#type: &str) -> syn::Error {
    let message = format!("#[objc]: value for `{key}` must be {}", r#type);
    syn::Error::new(span, message)
}
