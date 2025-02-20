use syn::Ident;

// Add these new structures for TridentTransaction
#[derive(Debug)]
pub struct TridentTransactionStruct {
    pub ident: Ident,
    pub fields: Vec<TridentTransactionField>,
    pub custom_name: Option<String>,
}

#[derive(Debug)]
pub struct TridentTransactionField {
    pub ident: Ident,
    pub ty: String,
}
