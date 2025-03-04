use std::collections::HashMap;
use syn::visit::{self, Visit};
use syn::Ident;
use syn::TypePath;

pub struct TridentAccountsStruct {
    pub ident: Ident,
    pub fields: Vec<TridentAccountField>,
    pub instruction_type: syn::Type,
    pub storage_type: syn::Type,
}

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

pub struct CompositeField {
    pub ident: Ident,
    pub constraints: TridentConstraints,
    pub ty: String, // Store the composite type name
}

pub struct TridentField {
    pub ident: Ident,
    pub ty: TridentAccountType,
    pub constraints: TridentConstraints,
}

pub enum TridentAccountType {
    TridentAccount(TridentAccountTy),
}

pub struct TridentAccountTy {
    pub program_type_path: TypePath,
}

#[derive(Default)]
pub struct TridentConstraints {
    pub mutable: bool,
    pub signer: bool,
    pub address: Option<syn::Expr>,
    pub skip_snapshot: bool,
    pub storage: Option<Ident>,
    pub seeds: Option<Vec<syn::Expr>>, // Store the raw expressions from the array
    pub program_id: Option<syn::Expr>,
}

pub struct SeedDependency {
    pub dependent_field: Ident,
    pub required_field: Ident,
}

// Create a visitor struct to find field dependencies
struct FieldDependencyVisitor<'a> {
    dependencies: Vec<Ident>,
    field_addresses: &'a HashMap<String, &'a Ident>,
}

impl<'a> Visit<'a> for FieldDependencyVisitor<'a> {
    fn visit_expr_method_call(&mut self, node: &'a syn::ExprMethodCall) {
        // Visit the receiver first (it might contain other method calls)
        visit::visit_expr(&mut *self, &node.receiver);

        // Check if the receiver is a path (field reference)
        if let syn::Expr::Path(path) = &*node.receiver {
            if let Some(segment) = path.path.segments.last() {
                let field_name = segment.ident.to_string();
                if let Some(&field) = self.field_addresses.get(&field_name) {
                    self.dependencies.push((*field).clone());
                }
            }
        }
    }

    fn visit_expr_call(&mut self, node: &'a syn::ExprCall) {
        // Visit function arguments
        for arg in &node.args {
            visit::visit_expr(self, arg);
        }
    }

    fn visit_expr_field(&mut self, node: &'a syn::ExprField) {
        // Visit the base expression first
        visit::visit_expr(self, &node.base);

        if let syn::Expr::Path(path) = &*node.base {
            if let Some(segment) = path.path.segments.last() {
                let field_name = segment.ident.to_string();
                if let Some(&field) = self.field_addresses.get(&field_name) {
                    self.dependencies.push((*field).clone());
                }
            }
        }
    }
}

impl TridentAccountsStruct {
    pub fn analyze_seed_dependencies(&self) -> Vec<SeedDependency> {
        let mut dependencies = Vec::new();

        let field_addresses: HashMap<String, &Ident> = self
            .fields
            .iter()
            .map(|field| (field.ident().to_string(), field.ident()))
            .collect();

        for field in &self.fields {
            if let TridentAccountField::Field(f) = field {
                if let Some(seeds) = &f.constraints.seeds {
                    for seed in seeds {
                        let mut visitor = FieldDependencyVisitor {
                            dependencies: Vec::new(),
                            field_addresses: &field_addresses,
                        };
                        visitor.visit_expr(seed);

                        for required_field in visitor.dependencies {
                            dependencies.push(SeedDependency {
                                dependent_field: f.ident.clone(),
                                required_field: required_field.clone(),
                            });
                        }
                    }
                }
            }
        }

        dependencies
    }
}
