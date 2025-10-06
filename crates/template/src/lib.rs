use convert_case::Case;
use convert_case::Casing;
use serde_json::json;
use sha2::Digest;
use sha2::Sha256;
use tera::Context;
use tera::Tera;
use trident_idl_spec::Idl;
use trident_idl_spec::IdlInstruction;
use trident_idl_spec::IdlType;
use trident_idl_spec::IdlTypeDef;
use trident_idl_spec::IdlTypeDefTy;

use crate::error::TemplateError;

pub mod error;

/// Simple template engine for Trident code generation
pub struct TridentTemplates {
    tera: Tera,
}

impl TridentTemplates {
    pub fn new() -> Result<Self, TemplateError> {
        let mut tera = Tera::default();
        tera.add_raw_templates(vec![
            (
                "test_fuzz.rs",
                include_str!("../templates/test_fuzz.rs.tera"),
            ),
            (
                "fuzz_accounts.rs",
                include_str!("../templates/fuzz_accounts.rs.tera"),
            ),
            ("types.rs", include_str!("../templates/types.rs.tera")),
            (
                "Trident.toml",
                include_str!("../templates/Trident.toml.tera"),
            ),
            (
                "Cargo_fuzz.toml",
                include_str!("../templates/Cargo_fuzz.toml.tera"),
            ),
        ])?;
        Ok(Self { tera })
    }

    /// Generate all templates from IDLs
    pub fn generate(
        &self,
        idls: &[Idl],
        trident_version: &str,
    ) -> Result<GeneratedFiles, TemplateError> {
        let programs_data = self.build_programs_with_instructions_data(idls)?;

        // Generate files
        let test_fuzz = self
            .tera
            .render("test_fuzz.rs", &Context::from_serialize(json!({}))?)?;
        let fuzz_accounts = self.tera.render(
            "fuzz_accounts.rs",
            &Context::from_serialize(json!({"accounts": self.collect_all_accounts(idls)}))?,
        )?;
        let types = self.tera.render(
            "types.rs",
            &Context::from_serialize(json!({
                "programs": programs_data,
                "custom_types": self.collect_custom_types(idls)
            }))?,
        )?;
        let trident_toml = self.tera.render(
            "Trident.toml",
            &Context::from_serialize(json!({"programs": programs_data}))?,
        )?;
        let cargo_fuzz_toml = self.tera.render(
            "Cargo_fuzz.toml",
            &Context::from_serialize(json!({
                "trident_version": trident_version,
            }))?,
        )?;

        Ok(GeneratedFiles {
            test_fuzz,
            types,
            fuzz_accounts,
            trident_toml,
            cargo_fuzz_toml,
        })
    }

    // Helper function to build programs with instructions data
    fn build_programs_with_instructions_data(
        &self,
        idls: &[Idl],
    ) -> Result<Vec<serde_json::Value>, TemplateError> {
        let mut programs_data = Vec::new();

        for idl in idls.iter() {
            let program_id = if idl.address.is_empty() {
                "fill corresponding program ID here"
            } else {
                &idl.address
            };

            let program_name = if idl.metadata.name.is_empty() {
                "unknown_program"
            } else {
                &idl.metadata.name
            };

            let module_name = program_name.to_case(Case::Snake);

            // Process instructions and collect composite accounts (preserving IDL order)
            let mut instructions_data = Vec::new();
            let mut composite_accounts = Vec::new();
            let mut seen_composites = std::collections::HashSet::new();

            for instruction in &idl.instructions {
                let instruction_data = self.build_instruction_data_with_lifetimes(
                    instruction,
                    program_id,
                    &std::collections::HashMap::new(),
                )?;

                // Collect composite accounts for deduplication (preserving first occurrence order)
                if let Some(composites) = instruction_data
                    .get("composite_accounts")
                    .and_then(|v| v.as_array())
                {
                    for composite in composites {
                        if let Some(name) = composite.get("camel_name").and_then(|v| v.as_str()) {
                            // Only add if not already seen (preserves first occurrence and IDL order)
                            if seen_composites.insert(name.to_string()) {
                                composite_accounts.push(composite.clone());
                            }
                        }
                    }
                }

                instructions_data.push(instruction_data);
            }

            programs_data.push(json!({
                "name": program_name,
                "module_name": module_name,
                "program_id": program_id,
                "instructions": instructions_data,
                "composite_accounts": composite_accounts
            }));
        }

        Ok(programs_data)
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
            IdlType::Pubkey | IdlType::PublicKey => "Pubkey".to_string(),
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

    /// Collect all accounts for fuzz_accounts (preserving IDL order and deterministic)
    fn collect_all_accounts(&self, idls: &[Idl]) -> Vec<serde_json::Value> {
        let mut accounts = Vec::new();
        for idl in idls {
            for instruction in &idl.instructions {
                self.collect_accounts_recursive(&instruction.accounts, &mut accounts);
            }
        }

        // Deduplicate while preserving order (keep first occurrence)
        let mut seen = std::collections::HashSet::new();
        accounts.retain(|name| seen.insert(name.clone()));

        accounts
            .into_iter()
            .map(|name| json!({ "name": name }))
            .collect()
    }

    #[allow(clippy::only_used_in_recursion)]
    fn collect_accounts_recursive(
        &self,
        accounts: &[trident_idl_spec::IdlInstructionAccountItem],
        acc: &mut Vec<String>,
    ) {
        for account in accounts {
            match account {
                trident_idl_spec::IdlInstructionAccountItem::Single(a) => {
                    acc.push(a.name.clone());
                }
                trident_idl_spec::IdlInstructionAccountItem::Composite(c) => {
                    acc.push(c.name.clone());
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

    /// Extract seeds from PDA seeds
    fn extract_seeds(
        &self,
        seeds: &[trident_idl_spec::IdlSeed],
    ) -> (Vec<String>, Vec<String>, Vec<String>) {
        let mut static_seeds = Vec::new();
        let mut account_seeds = Vec::new();
        let mut arg_seeds = Vec::new();

        for seed in seeds {
            match seed {
                trident_idl_spec::IdlSeed::Const(const_seed) => {
                    // Convert byte array to Rust byte array literal
                    let bytes_str = format!(
                        "[{}]",
                        const_seed
                            .value
                            .iter()
                            .map(|b| format!("{}u8", b))
                            .collect::<Vec<_>>()
                            .join(", ")
                    );
                    static_seeds.push(bytes_str);
                }
                trident_idl_spec::IdlSeed::Account(account_seed) => {
                    // Account reference for PDA seeds
                    account_seeds.push(account_seed.path.clone());
                }
                trident_idl_spec::IdlSeed::Arg(arg_seed) => {
                    // Argument reference for PDA seeds
                    arg_seeds.push(arg_seed.path.clone());
                }
            }
        }

        (static_seeds, account_seeds, arg_seeds)
    }

    /// Build instruction data
    fn build_instruction_data_with_lifetimes(
        &self,
        instruction: &IdlInstruction,
        program_id: &str,
        _composite_lifetime_map: &std::collections::HashMap<String, bool>,
    ) -> Result<serde_json::Value, TemplateError> {
        let name = &instruction.name;
        let camel_name = name.to_case(Case::UpperCamel);
        let snake_name = name.to_case(Case::Snake);

        let discriminator = if instruction.discriminator.is_empty() {
            self.generate_discriminator(name)
        } else {
            instruction.discriminator.clone()
        };

        let (accounts, composite_accounts) =
            self.process_accounts_with_lifetimes(&instruction.accounts);
        let data_fields = self.process_data_fields(&instruction.args);

        Ok(json!({
            "name": name,
            "camel_name": camel_name,
            "snake_name": snake_name,
            "program_id": program_id,
            "discriminator": discriminator,
            "accounts": accounts,
            "composite_accounts": composite_accounts,
            "data_fields": data_fields,
            "needs_lifetime": false
        }))
    }

    #[allow(clippy::only_used_in_recursion)]
    fn process_accounts_with_lifetimes(
        &self,
        accounts: &[trident_idl_spec::IdlInstructionAccountItem],
    ) -> (Vec<serde_json::Value>, Vec<serde_json::Value>) {
        let mut main_accounts = Vec::new();
        let mut composite_accounts = Vec::new();

        for account in accounts {
            match account {
                trident_idl_spec::IdlInstructionAccountItem::Single(acc) => {
                    let has_pda_seeds = acc.pda.is_some();
                    let (static_seeds, account_seeds, arg_seeds) = if let Some(pda) = &acc.pda {
                        self.extract_seeds(&pda.seeds)
                    } else {
                        (Vec::new(), Vec::new(), Vec::new())
                    };

                    main_accounts.push(json!({
                        "name": acc.name,
                        "is_signer": acc.signer,
                        "is_writable": acc.writable,
                        "address": acc.address,
                        "is_composite": false,
                        "composite_type_name": null,
                        "has_pda_seeds": has_pda_seeds,
                        "composite_needs_lifetime": false,
                        "static_seeds": static_seeds,
                        "account_seeds": account_seeds,
                        "arg_seeds": arg_seeds
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
                        "composite_type_name": camel_name,
                        "has_pda_seeds": false,
                        "composite_needs_lifetime": false
                    }));

                    // Process composite account itself
                    let (comp_accounts, nested_composites) =
                        self.process_accounts_with_lifetimes(&comp.accounts);

                    composite_accounts.push(json!({
                        "name": comp.name,
                        "camel_name": camel_name,
                        "accounts": comp_accounts,
                        "nested_composites": nested_composites,
                        "needs_lifetime": false
                    }));
                }
            }
        }

        // Preserve original IDL order - account order is critical for Solana programs
        (main_accounts, composite_accounts)
    }
}

#[derive(Debug, Clone)]
pub struct GeneratedFiles {
    pub test_fuzz: String,
    pub types: String,
    pub fuzz_accounts: String,
    pub trident_toml: String,
    pub cargo_fuzz_toml: String,
}

impl Default for TridentTemplates {
    fn default() -> Self {
        Self::new().expect("Failed to create template engine")
    }
}
