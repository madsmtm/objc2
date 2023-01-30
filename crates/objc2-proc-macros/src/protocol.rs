use proc_macro2::TokenStream;
use syn::spanned::Spanned;

pub(crate) fn item_trait(
    attr: TokenStream,
    item_trait: syn::ItemTrait,
) -> syn::Result<TokenStream> {
    if !attr.is_empty() {
        let span = attr.span();
        let message = format!("#[objc]: unexpected arguments: `{attr}`");
        return Err(syn::Error::new(span, message));
    }
    todo!()
}
