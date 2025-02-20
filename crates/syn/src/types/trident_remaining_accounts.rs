use syn::Ident;

// Add this new structure for TridentRemainingAccounts
#[derive(Debug)]
pub struct TridentRemainingAccountsStruct {
    pub ident: Ident,
    pub field_name: Ident,
}
