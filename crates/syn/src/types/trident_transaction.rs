use syn::Ident;

pub struct TridentTransactionStruct {
    pub ident: Ident,
    pub fields: Vec<TridentTransactionField>,
    pub custom_name: Option<String>,
}

pub struct TridentTransactionField {
    pub ident: Ident,
    pub ty: String,
}
