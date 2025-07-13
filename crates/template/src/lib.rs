use convert_case::{Case, Casing};
use serde_json::json;
use sha2::{Digest, Sha256};
use tera::{Context, Tera};
use trident_idl_spec::{Idl, IdlInstruction, IdlType, IdlTypeDef, IdlTypeDefTy};

/// Simple template engine for Trident code generation
pub struct TridentTemplates {
    tera: Tera,
}

impl TridentTemplates {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            (
                "instruction.rs",
                include_str!("../templates/instruction.rs.tera"),
            ),
            (
                "transaction.rs",
                include_str!("../templates/transaction.rs.tera"),
            ),
            (
                "test_fuzz.rs",
                include_str!("../templates/test_fuzz.rs.tera"),
            ),
            (
                "fuzz_accounts.rs",
                include_str!("../templates/fuzz_accounts.rs.tera"),
            ),
            ("types.rs", include_str!("../templates/types.rs.tera")),
        ])?;
        Ok(Self { tera })
    }

    /// Generate all templates from IDLs
    pub fn generate(
        &self,
        idls: &[Idl],
        lib_names: &[String],
    ) -> Result<GeneratedFiles, Box<dyn std::error::Error>> {
        let mut instructions = Vec::new();
        let mut transactions = Vec::new();
        let programs = self.build_programs_data(idls, lib_names);

        // Process instructions for each IDL
        for idl in idls.iter() {
            let program_id = if idl.address.is_empty() {
                "fill corresponding program ID here"
            } else {
                &idl.address
            };

            for instruction in &idl.instructions {
                let template_data = self.build_instruction_data(instruction, program_id)?;
                let snake_name = &template_data["snake_name"].as_str().unwrap();

                let context = Context::from_serialize(json!({"instruction": template_data}))?;

                instructions.push((
                    snake_name.to_string(),
                    self.tera.render("instruction.rs", &context)?,
                ));
                transactions.push((
                    snake_name.to_string(),
                    self.tera.render("transaction.rs", &context)?,
                ));
            }
        }

        // Generate other files
        let test_fuzz = self.tera.render(
            "test_fuzz.rs",
            &Context::from_serialize(json!({"programs": programs}))?,
        )?;
        let fuzz_accounts = self.tera.render(
            "fuzz_accounts.rs",
            &Context::from_serialize(json!({"accounts": self.collect_all_accounts(idls)}))?,
        )?;
        let custom_types = self.tera.render(
            "types.rs",
            &Context::from_serialize(json!({"custom_types": self.collect_custom_types(idls)}))?,
        )?;

        // Generate mod files (clone to avoid borrowing issues)
        let instructions_mod = self.generate_mod_from_names(
            &instructions
                .iter()
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>(),
        );
        let transactions_mod = self.generate_mod_from_names(
            &transactions
                .iter()
                .map(|(name, _)| name.clone())
                .collect::<Vec<_>>(),
        );

        Ok(GeneratedFiles {
            instructions,
            transactions,
            test_fuzz,
            instructions_mod,
            transactions_mod,
            custom_types,
            fuzz_accounts,
        })
    }

    // Helper function to build program data
    fn build_programs_data(&self, idls: &[Idl], lib_names: &[String]) -> Vec<serde_json::Value> {
        idls.iter()
            .enumerate()
            .map(|(idx, idl)| {
                let program_id = if idl.address.is_empty() {
                    "fill corresponding program ID here"
                } else {
                    &idl.address
                };
                let lib_name = lib_names.get(idx).unwrap_or(&idl.metadata.name);

                json!({
                    "name": idl.metadata.name,
                    "program_id": program_id,
                    "lib_name": lib_name
                })
            })
            .collect()
    }

    // Helper function to build instruction data
    fn build_instruction_data(
        &self,
        instruction: &IdlInstruction,
        program_id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let name = &instruction.name;
        let camel_name = name.to_case(Case::UpperCamel);
        let snake_name = name.to_case(Case::Snake);

        let discriminator = if instruction.discriminator.is_empty() {
            self.generate_discriminator(name)
        } else {
            instruction.discriminator.clone()
        };

        let (accounts, composite_accounts) = self.process_accounts(&instruction.accounts);
        let data_fields = self.process_data_fields(&instruction.args);

        Ok(json!({
            "name": name,
            "camel_name": camel_name,
            "snake_name": snake_name,
            "program_id": program_id,
            "discriminator": discriminator,
            "accounts": accounts,
            "composite_accounts": composite_accounts,
            "data_fields": data_fields
        }))
    }

    // Helper function to process data fields
    fn process_data_fields(&self, args: &[trident_idl_spec::IdlField]) -> Vec<serde_json::Value> {
        args.iter()
            .map(|field| {
                json!({
                    "name": field.name,
                    "rust_type": self.idl_type_to_rust(&field.ty)
                })
            })
            .collect()
    }

    #[allow(clippy::only_used_in_recursion)]
    /// Simplified account processing
    fn process_accounts(
        &self,
        accounts: &[trident_idl_spec::IdlInstructionAccountItem],
    ) -> (Vec<serde_json::Value>, Vec<serde_json::Value>) {
        let mut main_accounts = Vec::new();
        let mut composite_accounts = Vec::new();

        for account in accounts {
            match account {
                trident_idl_spec::IdlInstructionAccountItem::Single(acc) => {
                    main_accounts.push(json!({
                        "name": acc.name,
                        "is_signer": acc.signer,
                        "is_writable": acc.writable,
                        "address": acc.address,
                        "is_composite": false,
                        "composite_type_name": null
                    }));
                }
                trident_idl_spec::IdlInstructionAccountItem::Composite(comp) => {
                    let camel_name = comp.name.to_case(Case::UpperCamel);

                    // Add to main accounts as composite reference
                    main_accounts.push(json!({
                        "name": comp.name,
                        "is_signer": false,
                        "is_writable": false,
                        "address": null,
                        "is_composite": true,
                        "composite_type_name": camel_name
                    }));

                    // Process composite account itself
                    let (comp_accounts, nested_composites) = self.process_accounts(&comp.accounts);
                    composite_accounts.push(json!({
                        "name": comp.name,
                        "camel_name": camel_name,
                        "accounts": comp_accounts,
                        "nested_composites": nested_composites
                    }));
                    // Don't extend here - nested composites are already included in the nested_composites field
                }
            }
        }

        (main_accounts, composite_accounts)
    }

    #[allow(clippy::only_used_in_recursion)]
    /// Simple type conversion
    fn idl_type_to_rust(&self, idl_type: &IdlType) -> String {
        match idl_type {
            IdlType::Bool => "bool".to_string(),
            IdlType::U8 => "u8".to_string(),
            IdlType::I8 => "i8".to_string(),
            IdlType::U16 => "u16".to_string(),
            IdlType::I16 => "i16".to_string(),
            IdlType::U32 => "u32".to_string(),
            IdlType::I32 => "i32".to_string(),
            IdlType::F32 => "f32".to_string(),
            IdlType::U64 => "u64".to_string(),
            IdlType::I64 => "i64".to_string(),
            IdlType::F64 => "f64".to_string(),
            IdlType::U128 => "u128".to_string(),
            IdlType::I128 => "i128".to_string(),
            IdlType::U256 => "u256".to_string(),
            IdlType::I256 => "i256".to_string(),
            IdlType::Bytes => "Vec<u8>".to_string(),
            IdlType::String => "String".to_string(),
            IdlType::Pubkey | IdlType::PublicKey => "TridentPubkey".to_string(),
            IdlType::Option(inner) => format!("Option<{}>", self.idl_type_to_rust(inner)),
            IdlType::Vec(inner) => format!("Vec<{}>", self.idl_type_to_rust(inner)),
            IdlType::Array(inner, len) => {
                let len_str = match len {
                    trident_idl_spec::IdlArrayLen::Value(n) => n.to_string(),
                    _ => "0".to_string(),
                };
                format!("[{}; {}]", self.idl_type_to_rust(inner), len_str)
            }
            IdlType::Defined(defined) => match defined {
                trident_idl_spec::DefinedType::Simple(name) => name.clone(),
                trident_idl_spec::DefinedType::Complex { name, .. } => name.clone(),
            },
            IdlType::Generic(name) => name.clone(),
            _ => "UnknownType".to_string(),
        }
    }

    /// Generate discriminator
    fn generate_discriminator(&self, name: &str) -> Vec<u8> {
        let preimage = format!("global:{}", name.to_case(Case::Snake));
        let mut hasher = Sha256::new();
        hasher.update(preimage);
        hasher.finalize()[..8].to_vec()
    }

    /// Collect all accounts for fuzz_accounts
    fn collect_all_accounts(&self, idls: &[Idl]) -> Vec<serde_json::Value> {
        let mut accounts = std::collections::HashSet::new();
        for idl in idls {
            for instruction in &idl.instructions {
                self.collect_accounts_recursive(&instruction.accounts, &mut accounts);
            }
        }
        accounts
            .into_iter()
            .map(|name| json!({ "name": name }))
            .collect()
    }

    #[allow(clippy::only_used_in_recursion)]
    fn collect_accounts_recursive(
        &self,
        accounts: &[trident_idl_spec::IdlInstructionAccountItem],
        acc: &mut std::collections::HashSet<String>,
    ) {
        for account in accounts {
            match account {
                trident_idl_spec::IdlInstructionAccountItem::Single(a) => {
                    acc.insert(a.name.clone());
                }
                trident_idl_spec::IdlInstructionAccountItem::Composite(c) => {
                    acc.insert(c.name.clone());
                    self.collect_accounts_recursive(&c.accounts, acc);
                }
            }
        }
    }

    /// Collect custom types
    fn collect_custom_types(&self, idls: &[Idl]) -> Vec<serde_json::Value> {
        idls.iter()
            .flat_map(|idl| &idl.types)
            .map(|type_def| self.convert_type_def_to_template_data(type_def))
            .collect()
    }

    /// Convert IDL type definition to template data (simplified)
    fn convert_type_def_to_template_data(&self, type_def: &IdlTypeDef) -> serde_json::Value {
        match &type_def.ty {
            IdlTypeDefTy::Struct { fields } => json!({
                "type": "struct",
                "name": type_def.name,
                "fields": fields.as_ref().map(|f| self.convert_fields_to_template_data(f))
            }),
            IdlTypeDefTy::Enum { variants } => json!({
                "type": "enum",
                "name": type_def.name,
                "variants": variants.iter().map(|v| json!({
                    "name": v.name,
                    "fields": v.fields.as_ref().map(|f| self.convert_fields_to_template_data(f))
                })).collect::<Vec<_>>()
            }),
            IdlTypeDefTy::Type { .. } => json!({
                "type": "type_alias",
                "name": type_def.name
            }),
        }
    }

    /// Helper to convert fields to template data
    fn convert_fields_to_template_data(
        &self,
        fields: &trident_idl_spec::IdlDefinedFields,
    ) -> serde_json::Value {
        match fields {
            trident_idl_spec::IdlDefinedFields::Named(named) => json!({
                "type": "named",
                "fields": named.iter().map(|field| json!({
                    "name": field.name,
                    "rust_type": self.idl_type_to_rust(&field.ty)
                })).collect::<Vec<_>>()
            }),
            trident_idl_spec::IdlDefinedFields::Tuple(tuple) => json!({
                "type": "tuple",
                "fields": tuple.iter().enumerate().map(|(i, field_type)| json!({
                    "name": format!("field_{}", i),
                    "rust_type": self.idl_type_to_rust(field_type)
                })).collect::<Vec<_>>()
            }),
        }
    }

    fn generate_mod_from_names(&self, names: &[String]) -> String {
        let mut content = String::new();
        for name in names {
            content.push_str(&format!("pub mod {};\n", name));
        }
        for name in names {
            content.push_str(&format!("pub use {}::*;\n", name));
        }
        content
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedFiles {
    pub instructions: Vec<(String, String)>,
    pub transactions: Vec<(String, String)>,
    pub test_fuzz: String,
    pub instructions_mod: String,
    pub transactions_mod: String,
    pub custom_types: String,
    pub fuzz_accounts: String,
}

impl Default for TridentTemplates {
    fn default() -> Self {
        Self::new().expect("Failed to create template engine")
    }
}
