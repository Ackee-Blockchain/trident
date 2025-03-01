use syn::{Generics, Ident, ImplItem};

#[derive(Debug)]
pub struct TridentFlowExecutorImpl {
    pub type_name: Box<syn::Type>,
    pub impl_block: Vec<ImplItem>,
    pub flow_methods: Vec<Ident>,
    pub init_method: Option<Ident>,
    pub generics: Generics,
}
