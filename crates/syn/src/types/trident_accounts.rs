use syn::Ident;
use syn::TypePath;

#[derive(Debug)]
pub struct TridentAccountsStruct {
    pub ident: Ident,
    pub fields: Vec<TridentAccountField>,
}

#[derive(Debug)]
pub enum TridentAccountField {
    Field(TridentField),
    CompositeField(CompositeField),
}

impl TridentAccountField {
    pub fn ident(&self) -> &Ident {
        match self {
            TridentAccountField::Field(f) => &f.ident,
            TridentAccountField::CompositeField(f) => &f.ident,
        }
    }
}

#[derive(Debug)]
pub struct CompositeField {
    pub ident: Ident,
    pub constraints: TridentConstraints,
    pub ty: String, // Store the composite type name
}

#[derive(Debug)]
pub struct TridentField {
    pub ident: Ident,
    pub ty: TridentAccountType,
    pub constraints: TridentConstraints,
}

#[derive(Debug)]
pub enum TridentAccountType {
    TridentAccount(TridentAccountTy),
}

#[derive(Debug)]
pub struct TridentAccountTy {
    pub program_type_path: TypePath,
}

#[derive(Debug, Default)]
pub struct TridentConstraints {
    pub mutable: bool,
    pub signer: bool,
    pub address: Option<syn::Expr>,
    pub skip_snapshot: bool,
    pub storage: Option<Ident>,
}
