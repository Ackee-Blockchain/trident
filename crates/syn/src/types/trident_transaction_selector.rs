use syn::Ident;

#[derive(Debug)]
pub struct TridentSelectorEnum {
    pub ident: Ident,
    pub variants: Vec<TridentSelectorVariant>,
}

#[derive(Debug)]
pub struct TridentSelectorVariant {
    pub ident: Ident,
}
