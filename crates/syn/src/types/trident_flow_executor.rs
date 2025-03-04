use syn::{Generics, Ident, ImplItem};

pub struct TridentFlowExecutorImpl {
    pub type_name: Box<syn::Type>,
    pub impl_block: Vec<ImplItem>,
    pub flow_methods: Vec<Ident>,
    pub init_method: Option<Ident>,
    pub generics: Generics,
    pub args: FlowExecutorArgs,
}

#[derive(Debug, Default)]
pub struct FlowExecutorArgs {
    pub random_tail: bool,
    // More fields can be added here in the future
}
