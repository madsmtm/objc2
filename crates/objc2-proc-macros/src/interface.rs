use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
};

use crate::common::{CfgAttributes, EmptyToken, KeyVal};

pub(crate) mod external;
pub(crate) mod internal;

pub(crate) struct Class {
    attrs: TokenStream,
    class: syn::Path,
    generics: syn::Generics,
}

impl Parse for Class {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        Ok(Self {
            attrs: quote!(#(#attrs)*),
            class: input.parse()?,
            generics: input.parse()?,
        })
    }
}

pub(crate) struct Inherits<Unsafety = EmptyToken>(
    Option<KeyVal<self::tokens::inherits, InheritsSpec, syn::Token![,], Unsafety>>,
);

impl<Unsafety: Parse> Parse for Inherits<Unsafety> {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(syn::Token![,]) {
            Ok(Self(Some(input.parse()?)))
        } else {
            Ok(Self(None))
        }
    }
}

struct InheritsSpec {
    #[allow(unused)]
    bracket_token: syn::token::Bracket,
    classes: Punctuated<Class, syn::Token![,]>,
}

impl Parse for InheritsSpec {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let content;
        Ok(Self {
            bracket_token: syn::bracketed!(content in input),
            classes: content.parse_terminated(Class::parse)?,
        })
    }
}

pub(crate) enum MacroArgs {
    Intern(self::internal::MacroArgs),
    Extern(self::external::MacroArgs),
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if input.peek(syn::Token![unsafe]) {
            let unsafety = input.parse()?;
            let lookahead = input.lookahead1();
            if lookahead.peek(syn::Token![super]) {
                let super_ = KeyVal {
                    punctuation: Default::default(),
                    unsafety,
                    key: input.parse()?,
                    separator: input.parse()?,
                    val: input.parse()?,
                };
                return Ok(self::external::MacroArgs::Declare {
                    super_,
                    inherits: input.parse()?,
                    impl_attrs: if input.peek(syn::Token![,])
                        && input.peek2(self::tokens::impl_attrs)
                    {
                        Some(input.parse()?)
                    } else {
                        None
                    },
                    comma_token: input.parse()?,
                }
                .into());
            }
            if lookahead.peek(syn::Token![continue]) {
                return Ok(self::external::MacroArgs::Continue {
                    continue_token: input.parse()?,
                    impl_attrs: if input.peek(syn::Token![,])
                        && input.peek2(self::tokens::impl_attrs)
                    {
                        Some(input.parse()?)
                    } else {
                        None
                    },
                    comma_token: input.parse()?,
                }
                .into());
            }
            Err(lookahead.error())
        } else {
            Ok(self::internal::MacroArgs {
                super_class: input.parse()?,
                inherits: input.parse()?,
                comma_token: input.parse()?,
            }
            .into())
        }
    }
}

pub(crate) mod tokens {
    syn::custom_keyword!(impl_attrs);
    syn::custom_keyword!(interface);
    syn::custom_keyword!(inherits);
}

struct ItemType {
    attrs: Vec<syn::Attribute>,
    vis: syn::Visibility,
    type_token: syn::Token![type],
    ident: syn::Ident,
    generics: syn::Generics,
    semi_token: syn::token::Semi,
}

impl From<syn::ForeignItemType> for ItemType {
    fn from(item: syn::ForeignItemType) -> Self {
        let syn::ForeignItemType {
            attrs,
            vis,
            type_token,
            ident,
            semi_token,
        } = item;
        Self {
            attrs,
            vis,
            type_token,
            ident,
            generics: Default::default(),
            semi_token,
        }
    }
}

impl Parse for ItemType {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        Ok(Self {
            attrs: input.call(syn::Attribute::parse_outer)?,
            vis: input.parse()?,
            type_token: input.parse()?,
            ident: input.parse()?,
            generics: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}

impl ToTokens for ItemType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ItemType {
            attrs,
            vis,
            type_token,
            ident,
            generics,
            semi_token,
        } = self;
        tokens.append_all(quote!(
            #(#attrs)*
            #vis #type_token #ident #generics #semi_token
        ))
    }
}

pub(crate) struct ClassStruct {
    attrs_all: TokenStream,
    attrs_cfg: TokenStream,
    vis: syn::Visibility,
    ident: syn::Ident,
    fields: syn::FieldsNamed,
}

impl TryFrom<syn::ItemStruct> for ClassStruct {
    type Error = syn::Error;
    fn try_from(item: syn::ItemStruct) -> Result<Self, Self::Error> {
        let syn::ItemStruct {
            attrs,
            vis,
            ident,
            generics,
            fields,
            ..
        } = item;
        let attrs_cfg = CfgAttributes::from(&*attrs);
        let mut fields = if let syn::Fields::Named(fields) = fields {
            fields
        } else {
            syn::FieldsNamed {
                brace_token: Default::default(),
                named: Default::default(),
            }
        };
        for generic in generics.params.into_iter() {
            if let syn::GenericParam::Type(ty) = generic {
                let ty_string = ty.ident.to_string();
                if ty_string.ends_with("Ownership") {
                    let ty_ownership_ident = ty.ident;
                    let ty_ident = format_ident!(
                        "{}",
                        ty_string
                            .strip_suffix("Ownership")
                            .expect("already checked suffix")
                    );
                    fields.named.push(syn::Field {
                        attrs: vec![syn::Attribute {
                            pound_token: Default::default(),
                            style: syn::AttrStyle::Outer,
                            bracket_token: Default::default(),
                            path: syn::PathSegment {
                                ident: format_ident!("allow"),
                                arguments: Default::default(),
                            }
                            .into(),
                            tokens: quote!((non_snake_case)),
                        }],
                        vis: syn::Visibility::Inherited,
                        ident: Some(format_ident!("__objc2_generic_{}", ty_ident)),
                        colon_token: Default::default(),
                        ty: make_phantom_field(ty_ident, ty_ownership_ident),
                    });
                }
            }
        }
        Ok(Self {
            attrs_all: quote!(#(#attrs)*),
            attrs_cfg: quote!(#(#attrs_cfg)*),
            vis,
            ident,
            fields,
        })
    }
}

fn make_phantom_field(ty_ident: syn::Ident, ty_ownership_ident: syn::Ident) -> syn::Type {
    syn::Type::Path(syn::TypePath {
        qself: Default::default(),
        path: syn::Path {
            leading_colon: Some(Default::default()),
            segments: Punctuated::from_iter([
                syn::PathSegment::from(syn::Ident::new("objc2", Span::call_site())),
                syn::PathSegment::from(syn::Ident::new("__private", Span::call_site())),
                syn::PathSegment {
                    ident: syn::Ident::new("PhantomData", Span::call_site()),
                    arguments: syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Default::default(),
                            args: Punctuated::from_iter([syn::GenericArgument::Type(
                                syn::Type::from(syn::TypePtr {
                                    star_token: Default::default(),
                                    const_token: None,
                                    mutability: Some(Default::default()),
                                    elem: Box::new(syn::Type::from(syn::TypeTuple {
                                        paren_token: Default::default(),
                                        elems: Punctuated::from_iter([
                                            syn::Type::from(syn::TypePath {
                                                qself: Default::default(),
                                                path: syn::Path::from(ty_ident),
                                            }),
                                            syn::Type::from(syn::TypePath {
                                                qself: Default::default(),
                                                path: syn::Path::from(ty_ownership_ident),
                                            }),
                                        ]),
                                    })),
                                }),
                            )]),
                            gt_token: Default::default(),
                        },
                    ),
                },
            ]),
        },
    })
}

impl TryFrom<ItemType> for ClassStruct {
    type Error = syn::Error;
    fn try_from(item: ItemType) -> Result<Self, Self::Error> {
        let ItemType {
            attrs,
            vis,
            ident,
            generics,
            semi_token,
            ..
        } = item;
        let item_struct = syn::ItemStruct {
            attrs,
            vis,
            struct_token: Default::default(),
            ident,
            generics,
            fields: syn::Fields::Unit,
            semi_token: Some(semi_token),
        };
        ClassStruct::try_from(item_struct)
    }
}

struct ClassEmitter {
    this: ClassStruct,
    generics_this: TokenStream,
    generics_this_split: (TokenStream, TokenStream, TokenStream),
    super_: Option<Class>,
    inherits: Option<Punctuated<Class, syn::Token![,]>>,
    attrs_impl: TokenStream,
    methods: TokenStream,
}

impl ClassEmitter {
    fn class_this(&self, tokens: &mut TokenStream) {
        if let Some(super_) = &self.super_ {
            let attrs_this_all = &self.this.attrs_all;
            let attrs_this_cfg = &self.this.attrs_cfg;
            let attrs_super = &super_.attrs;
            let attrs_impl = &self.attrs_impl;

            let class_this = &self.this.ident;
            let class_this_name = class_this.to_string();
            let class_super = &super_.class;

            let vis = &self.this.vis;
            let fields = self.this.fields.named.iter();

            let generics_this = &self.generics_this;
            let (impl_generics_this, ty_generics_this, where_clause_this) =
                &self.generics_this_split;

            let (_, ty_generics_super, _) = {
                let generics = &super_.generics;
                generics.split_for_impl()
            };

            let deny_unwind_safe_impl = if !generics_this.is_empty() {
                quote!(notunwindsafe: ::objc2::__private::PhantomData<&'static mut ()>,)
            } else {
                quote!()
            };

            tokens.append_all(quote!(
                #attrs_super
                #attrs_this_all
                #[repr(C)]
                #vis struct #class_this #generics_this {
                    #(#fields,)*
                    __inner: #class_super #ty_generics_super,
                    #deny_unwind_safe_impl
                }
            ));
            tokens.append_all(quote!(
                #attrs_this_cfg
                #attrs_impl
                unsafe impl #impl_generics_this ::objc2::ClassType for #class_this #ty_generics_this #where_clause_this {
                    type Super = #class_super #ty_generics_super;
                    const NAME: &'static ::objc2::__private::str = #class_this_name;

                    #[inline]
                    fn class() -> &'static ::objc2::runtime::Class {
                        ::objc2::__class_inner!(
                            #class_this_name,
                            ::objc2::__hash_idents!(#class_this),
                        )
                    }

                    #[inline]
                    fn as_super(&self) -> &Self::Super {
                        &self.__inner
                    }

                    #[inline]
                    fn as_super_mut(&mut self) -> &mut Self::Super {
                        &mut self.__inner
                    }
                }
            ));
        }
    }

    fn impls_this(&self, tokens: &mut TokenStream) {
        if !self.methods.is_empty() {
            let attrs_this_cfg = &self.this.attrs_cfg;
            let attrs_impl = &self.attrs_impl;
            let class_this = &self.this.ident;
            let methods = &self.methods;
            let (impl_generics, ty_generics, where_clause) = &self.generics_this_split;
            tokens.append_all(quote!(
                #attrs_this_cfg
                #attrs_impl
                impl #impl_generics #class_this #ty_generics #where_clause {
                    #![allow(non_snake_case)]
                    #methods
                }
            ));
        }
    }

    fn impls_encoding(&self, tokens: &mut TokenStream) {
        if let Some(super_) = &self.super_ {
            let attrs_this_cfg = &self.this.attrs_cfg;
            let attrs_super = &super_.attrs;
            let class_this = &self.this.ident;
            let class_super = &super_.class;

            let (impl_generics_this, ty_generics_this, where_clause_this) =
                &self.generics_this_split;

            let (_, ty_generics_super, _) = {
                let generics = &super_.generics;
                generics.split_for_impl()
            };

            tokens.append_all(quote!(
                #attrs_this_cfg
                #attrs_super
                unsafe impl #impl_generics_this ::objc2::RefEncode for #class_this #ty_generics_this #where_clause_this {
                    const ENCODING_REF: ::objc2::Encoding = <#class_super #ty_generics_super as ::objc2::RefEncode>::ENCODING_REF;
                }

                #attrs_this_cfg
                unsafe impl #impl_generics_this ::objc2::Message for #class_this #ty_generics_this #where_clause_this {}
            ));
        }
    }

    fn impls_super(&self, tokens: &mut TokenStream) {
        if let Some(super_) = &self.super_ {
            let attrs_this_cfg = &self.this.attrs_cfg;
            let attrs_super = &super_.attrs;
            let class_this = &self.this.ident;
            let class_super = &super_.class;

            let (impl_generics_this, ty_generics_this, where_clause_this) =
                &self.generics_this_split;

            let (_, ty_generics_super, _) = {
                let generics = &super_.generics;
                generics.split_for_impl()
            };

            tokens.append_all(quote!(
                #attrs_super
                #attrs_this_cfg
                impl #impl_generics_this ::objc2::__private::Deref for #class_this #ty_generics_this #where_clause_this {
                    type Target = #class_super #ty_generics_super;
                    #[inline]
                    fn deref(&self) -> &Self::Target {
                        &self.__inner
                    }
                }

                #attrs_super
                #attrs_this_cfg
                impl #impl_generics_this ::objc2::__private::DerefMut for #class_this #ty_generics_this #where_clause_this {
                    #[inline]
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self.__inner
                    }
                }

                #attrs_this_cfg
                impl #impl_generics_this ::objc2::__private::AsRef<Self> for #class_this #ty_generics_this #where_clause_this {
                    #[inline]
                    fn as_ref(&self) -> &Self {
                        self
                    }
                }

                #attrs_this_cfg
                impl #impl_generics_this ::objc2::__private::AsMut<Self> for #class_this #ty_generics_this #where_clause_this {
                    #[inline]
                    fn as_mut(&mut self) -> &mut Self {
                        self
                    }
                }
            ));
        }
    }

    fn impls_inherits(&self, tokens: &mut TokenStream) {
        let attrs_this_cfg = &self.this.attrs_cfg;
        let class_this = &self.this.ident;

        let (impl_generics_this, ty_generics_this, where_clause_this) = &self.generics_this_split;

        if let Some(classes) = &self.inherits {
            let classes = self.super_.iter().chain(classes);
            for that in classes {
                let attrs_that = &that.attrs;
                let class_that = &that.class;

                let (_, ty_generics_that, _) = {
                    let generics = &that.generics;
                    generics.split_for_impl()
                };

                tokens.append_all(quote!(
                    #attrs_that
                    #attrs_this_cfg
                    impl #impl_generics_this ::objc2::__private::AsRef<#class_that #ty_generics_that> for #class_this #ty_generics_this #where_clause_this {
                        #[inline]
                        fn as_ref(&self) -> &#class_that #ty_generics_that {
                            &*self
                        }
                    }

                    #attrs_that
                    #attrs_this_cfg
                    impl #impl_generics_this ::objc2::__private::AsMut<#class_that #ty_generics_that> for #class_this #ty_generics_this #where_clause_this {
                        #[inline]
                        fn as_mut(&mut self) -> &mut #class_that #ty_generics_that {
                            &mut *self
                        }
                    }

                    #attrs_that
                    #attrs_this_cfg
                    impl #impl_generics_this ::objc2::__private::Borrow<#class_that #ty_generics_that> for #class_this #ty_generics_this #where_clause_this {
                        #[inline]
                        fn borrow(&self) -> &#class_that #ty_generics_that {
                            &*self
                        }
                    }

                    #attrs_that
                    #attrs_this_cfg
                    impl #impl_generics_this ::objc2::__private::BorrowMut<#class_that #ty_generics_that> for #class_this #ty_generics_this #where_clause_this {
                        #[inline]
                        fn borrow_mut(&mut self) -> &mut #class_that #ty_generics_that {
                            &mut *self
                        }
                    }
                ));
            }

            tokens.append_all(quote!(
                #attrs_this_cfg
                impl #impl_generics_this ::objc2::__private::AsRef<::objc2::runtime::Object> for #class_this #ty_generics_this #where_clause_this {
                    #[inline]
                    fn as_ref(&self) -> &::objc2::runtime::Object {
                        &*self
                    }
                }

                #attrs_this_cfg
                impl #impl_generics_this ::objc2::__private::AsMut<::objc2::runtime::Object> for #class_this #ty_generics_this #where_clause_this {
                    #[inline]
                    fn as_mut(&mut self) -> &mut ::objc2::runtime::Object {
                        &mut *self
                    }
                }

                #attrs_this_cfg
                impl #impl_generics_this ::objc2::__private::Borrow<::objc2::runtime::Object> for #class_this #ty_generics_this #where_clause_this {
                    #[inline]
                    fn borrow(&self) -> &::objc2::runtime::Object {
                        &*self
                    }
                }

                #attrs_this_cfg
                impl #impl_generics_this ::objc2::__private::BorrowMut<::objc2::runtime::Object> for #class_this #ty_generics_this #where_clause_this {
                    #[inline]
                    fn borrow_mut(&mut self) -> &mut ::objc2::runtime::Object {
                        &mut *self
                    }
                }
            ));
        }
    }
}

impl ToTokens for ClassEmitter {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.class_this(tokens);
        self.impls_encoding(tokens);
        self.impls_super(tokens);
        self.impls_inherits(tokens);
        self.impls_this(tokens);
    }
}
