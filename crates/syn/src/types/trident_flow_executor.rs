use syn::{Generics, Ident, ImplItem};

#[derive(Default)]
pub struct FlowConstraints {
    pub ignore: bool,
    // Future fields can be added here (e.g., weight: Option<u32>)
}

pub struct FlowMethod {
    pub ident: Ident,
    pub constraints: FlowConstraints,
}

pub struct TridentFlowExecutorImpl {
    pub type_name: Box<syn::Type>,
    pub impl_block: Vec<ImplItem>,
    pub flow_methods: Vec<FlowMethod>,
    pub init_method: Option<Ident>,
    pub end_method: Option<Ident>,
    pub generics: Generics,
}
