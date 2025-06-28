use syn::Ident;

pub struct TridentFuzzTestMethodsStruct {
    pub ident: Ident,
    pub client_field: Ident,
    pub metrics_field: Ident,
    pub rng_field: Ident,
}
