use syn::{Attribute, Ident, Meta, NestedMeta};

// Taken from `nom-derive`:
// https://github.com/rust-bakery/nom-derive/blob/5315891a0016b15094d4d0201f7d3ac803e4fc57/nom-derive-impl/src/enums.rs#L60-L90
pub(crate) fn get_repr(attrs: &[Attribute]) -> Option<Ident> {
    for attr in attrs {
        if let Ok(Meta::List(metalist)) = attr.parse_meta() {
            if let Some(ident) = metalist.path.get_ident() {
                if ident == "repr" {
                    for n in metalist.nested.iter() {
                        match n {
                            NestedMeta::Meta(meta) => match meta {
                                Meta::Path(path) => {
                                    if let Some(word) = path.get_ident() {
                                        return Some(word.clone());
                                    } else {
                                        panic!("unsupported nested type for 'repr'")
                                    }
                                }
                                _ => panic!("unsupported nested type for 'repr'"),
                            },
                            _ => panic!("unsupported meta type for 'repr'"),
                        }
                    }
                }
            }
        }
    }
    None
}
