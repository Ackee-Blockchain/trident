use syn::Ident;

pub struct TridentSelectorEnum {
    pub ident: Ident,
    pub variants: Vec<TridentSelectorVariant>,
}

pub struct TridentSelectorVariant {
    pub ident: Ident,
}
