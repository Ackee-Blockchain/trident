use convert_case::Case;
use convert_case::Casing;
use serde_json::json;
use sha2::Digest;
use sha2::Sha256;
use std::collections::HashMap;
use std::collections::HashSet;
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
        let programs_data = self.build_programs_data(idls)?;

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
                "programs": programs_data
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

    /// Build programs data for template rendering
    fn build_programs_data(&self, idls: &[Idl]) -> Result<Vec<serde_json::Value>, TemplateError> {
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
            let mut seen_composites = HashSet::new();

            for instruction in &idl.instructions {
                let instruction_data = self.build_instruction_data(instruction)?;

                // Collect composite accounts for deduplication (preserving first occurrence order)
                if let Some(composites) = instruction_data
                    .get("composite_accounts")
                    .and_then(|v| v.as_array())
                {
                    for composite in composites {
                        if let Some(name) = composite.get("camel_name").and_then(|v| v.as_str()) {
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
                "composite_accounts": composite_accounts,
                "data_accounts": self.collect_accounts_with_discriminators(idl),
                "errors": self.collect_errors(idl),
                "custom_types": self.collect_custom_types_for_program(idl)
            }));
        }

        Ok(programs_data)
    }

    /// Build instruction data for template
    fn build_instruction_data(
        &self,
        instruction: &IdlInstruction,
    ) -> Result<serde_json::Value, TemplateError> {
        let name = &instruction.name;
        let camel_name = name.to_case(Case::UpperCamel);
        let snake_name = name.to_case(Case::Snake);

        let discriminator = if instruction.discriminator.is_empty() {
            self.generate_discriminator(name)
        } else {
            instruction.discriminator.clone()
        };

        let (accounts, composite_accounts) = self.process_accounts(&instruction.accounts);

        Ok(json!({
            "name": name,
            "camel_name": camel_name,
            "snake_name": snake_name,
            "discriminator": discriminator,
            "accounts": accounts,
            "composite_accounts": composite_accounts,
            "data_fields": self.process_data_fields(&instruction.args)
        }))
    }

    #[allow(clippy::only_used_in_recursion)]
    /// Process accounts from instruction
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

                    main_accounts.push(json!({
                        "name": comp.name,
                        "is_signer": false,
                        "is_writable": false,
                        "address": null,
                        "is_composite": true,
                        "composite_type_name": camel_name
                    }));

                    let (comp_accounts, nested_composites) = self.process_accounts(&comp.accounts);

                    composite_accounts.push(json!({
                        "name": comp.name,
                        "camel_name": camel_name,
                        "accounts": comp_accounts,
                        "nested_composites": nested_composites
                    }));
                }
            }
        }

        (main_accounts, composite_accounts)
    }

    /// Process data fields for instruction
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
    /// Convert IDL type to Rust type string
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

    /// Generate discriminator from instruction name
    fn generate_discriminator(&self, name: &str) -> Vec<u8> {
        let preimage = format!("global:{}", name.to_case(Case::Snake));
        let mut hasher = Sha256::new();
        hasher.update(preimage);
        hasher.finalize()[..8].to_vec()
    }

    /// Collect all accounts for fuzz_accounts template
    fn collect_all_accounts(&self, idls: &[Idl]) -> Vec<serde_json::Value> {
        let mut accounts = Vec::new();
        for idl in idls {
            for instruction in &idl.instructions {
                self.collect_accounts_recursive(&instruction.accounts, &mut accounts);
            }
        }

        // Deduplicate while preserving order
        let mut seen = HashSet::new();
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

    /// Collect custom types for a single program/IDL
    fn collect_custom_types_for_program(&self, idl: &Idl) -> Vec<serde_json::Value> {
        let mut custom_types = Vec::new();
        let mut seen_names = HashSet::new();

        // Collect types from the `types` field
        for type_def in &idl.types {
            if seen_names.insert(type_def.name.clone()) {
                custom_types.push(self.convert_type_def_to_template_data(type_def));
            }
        }

        // Collect account types from older IDLs with `ty` field
        for account in &idl.accounts {
            if let Some(ty) = &account.ty {
                if seen_names.insert(account.name.clone()) {
                    custom_types.push(self.convert_type_def_ty_to_template_data(&account.name, ty));
                }
            }
        }

        custom_types
    }

    /// Collect accounts with discriminators
    fn collect_accounts_with_discriminators(&self, idl: &Idl) -> Vec<serde_json::Value> {
        let type_map: HashMap<&str, &IdlTypeDef> =
            idl.types.iter().map(|t| (t.name.as_str(), t)).collect();

        idl.accounts
            .iter()
            .map(|account| {
                let fields = type_map
                    .get(account.name.as_str())
                    .map(|td| self.convert_type_def_to_template_data(td))
                    .or_else(|| {
                        account
                            .ty
                            .as_ref()
                            .map(|ty| self.convert_type_def_ty_to_template_data(&account.name, ty))
                    });

                json!({
                    "name": account.name,
                    "discriminator": account.discriminator,
                    "fields": fields
                })
            })
            .collect()
    }

    /// Collect errors from IDL
    fn collect_errors(&self, idl: &Idl) -> Vec<serde_json::Value> {
        idl.errors
            .iter()
            .map(|error| {
                json!({
                    "code": error.code,
                    "name": error.name,
                    "msg": error.msg
                })
            })
            .collect()
    }

    /// Convert type definition to template data
    fn convert_type_def_to_template_data(&self, type_def: &IdlTypeDef) -> serde_json::Value {
        self.convert_type_def_ty_to_template_data(&type_def.name, &type_def.ty)
    }

    /// Convert type definition body to template data
    fn convert_type_def_ty_to_template_data(
        &self,
        name: &str,
        ty: &IdlTypeDefTy,
    ) -> serde_json::Value {
        match ty {
            IdlTypeDefTy::Struct { fields } => json!({
                "type": "struct",
                "name": name,
                "fields": fields.as_ref().map(|f| self.convert_fields_to_template_data(f))
            }),
            IdlTypeDefTy::Enum { variants } => json!({
                "type": "enum",
                "name": name,
                "variants": variants.iter().map(|v| json!({
                    "name": v.name,
                    "fields": v.fields.as_ref().map(|f| self.convert_fields_to_template_data(f))
                })).collect::<Vec<_>>()
            }),
            IdlTypeDefTy::Type { .. } => json!({
                "type": "type_alias",
                "name": name
            }),
        }
    }

    /// Convert fields to template data
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
