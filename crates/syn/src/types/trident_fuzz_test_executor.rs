use syn::{Generics, Ident, Type};

#[derive(Debug)]
pub struct TridentFuzzTestExecutor {
    pub ident: Ident,
    pub generics: Generics,
    pub client_type: Type,
}
