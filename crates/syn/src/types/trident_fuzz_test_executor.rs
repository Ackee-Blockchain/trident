use syn::Ident;

#[derive(Debug)]
pub struct TridentFuzzTestExecutorEnum {
    pub ident: Ident,
    pub variants: Vec<TridentFuzzTestExecutorVariant>,
}

#[derive(Debug)]
pub struct TridentFuzzTestExecutorVariant {
    pub ident: Ident,
}
