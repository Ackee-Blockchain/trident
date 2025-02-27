use syn::{GenericParam, Ident, WhereClause};

#[derive(Debug)]
pub struct TridentFuzzTestExecutor {
    pub ident: Ident,
    pub generics: Vec<GenericParam>,
    pub where_clause: Option<WhereClause>,
}
