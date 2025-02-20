use syn::Ident;

// Add these new structures for TridentInstruction
#[derive(Debug)]
pub struct TridentInstructionStruct {
    pub ident: Ident,
    pub accounts_field: String,
    pub remaining_accounts_field: Option<String>,
    pub program_id: String,
    pub discriminator: Vec<u8>,
}
