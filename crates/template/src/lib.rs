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
        let programs = self.build_programs_data(idls);

        // Generate files
        let test_fuzz = self
            .tera
            .render("test_fuzz.rs", &Context::from_serialize(json!({}))?)?;
        let fuzz_accounts = self.tera.render(
            "fuzz_accounts.rs",
            &Context::from_serialize(json!({"accounts": self.collect_all_accounts(idls)}))?,
        )?;
        let programs_data = self.build_programs_with_instructions_data(idls)?;
        let types = self.tera.render(
            "types.rs",
            &Context::from_serialize(json!({
                "programs": programs_data,
                "custom_types": self.collect_custom_types(idls)
            }))?,
        )?;
        let trident_toml = self.tera.render(
            "Trident.toml",
            &Context::from_serialize(json!({"programs": programs}))?,
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

    // Helper function to build program data
    fn build_programs_data(&self, idls: &[Idl]) -> Vec<serde_json::Value> {
        idls.iter()
            .map(|idl| {
                let program_id = if idl.address.is_empty() {
                    "fill corresponding program ID here"
                } else {
                    &idl.address
                };

                let program_name = if idl.metadata.name.is_empty() {
                    "fill corresponding program name here"
                } else {
                    &idl.metadata.name
                };

                json!({
                    "name": program_name,
                    "program_id": program_id,
                })
            })
            .collect()
    }

    // Helper function to build instruction data
    fn build_instruction_data(
        &self,
        instruction: &IdlInstruction,
        program_id: &str,
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
        let data_fields = self.process_data_fields(&instruction.args);

        // Check if instruction partial accounts need lifetimes
        let needs_lifetime = self.check_if_needs_lifetime(&accounts);

        Ok(json!({
            "name": name,
            "camel_name": camel_name,
            "snake_name": snake_name,
            "program_id": program_id,
            "discriminator": discriminator,
            "accounts": accounts,
            "composite_accounts": composite_accounts,
            "data_fields": data_fields,
            "needs_lifetime": needs_lifetime
        }))
    }

    // Helper function to build programs with instructions data for types.rs
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

            // First pass: collect all composite accounts for this program
            let mut composite_accounts_map = std::collections::HashMap::new();

            for instruction in &idl.instructions {
                let instruction_data = self.build_instruction_data(instruction, program_id)?;

                // Collect composite accounts for deduplication
                if let Some(composites) = instruction_data
                    .get("composite_accounts")
                    .and_then(|v| v.as_array())
                {
                    for composite in composites {
                        if let Some(name) = composite.get("camel_name").and_then(|v| v.as_str()) {
                            composite_accounts_map.insert(name.to_string(), composite.clone());
                        }
                    }
                }
            }

            // Second pass: calculate lifetime requirements with full composite knowledge
            let composite_lifetime_map =
                self.calculate_composite_lifetimes(&composite_accounts_map);

            // Third pass: rebuild instructions with correct lifetime information
            let mut instructions_data = Vec::new();
            for instruction in &idl.instructions {
                let instruction_data = self.build_instruction_data_with_lifetimes(
                    instruction,
                    program_id,
                    &composite_lifetime_map,
                )?;
                instructions_data.push(instruction_data);
            }

            // Update composite accounts with correct lifetime information
            let mut deduplicated_composites: Vec<serde_json::Value> =
                composite_accounts_map.into_values().collect();
            for composite in &mut deduplicated_composites {
                let name = composite
                    .get("camel_name")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                if let (Some(name), Some(obj)) = (name, composite.as_object_mut()) {
                    let needs_lifetime = composite_lifetime_map.get(&name).unwrap_or(&false);
                    obj.insert("needs_lifetime".to_string(), json!(*needs_lifetime));
                }
            }

            programs_data.push(json!({
                "name": program_name,
                "module_name": module_name,
                "program_id": program_id,
                "instructions": instructions_data,
                "composite_accounts": deduplicated_composites
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

                    // Process composite account itself first to get lifetime info
                    let (comp_accounts, nested_composites) = self.process_accounts(&comp.accounts);
                    let comp_needs_lifetime = self.check_if_needs_lifetime(&comp_accounts);

                    // Add to main accounts as composite reference
                    main_accounts.push(json!({
                        "name": comp.name,
                        "is_signer": false,
                        "is_writable": false,
                        "address": null,
                        "is_composite": true,
                        "composite_type_name": camel_name,
                        "has_pda_seeds": false,
                        "composite_needs_lifetime": comp_needs_lifetime
                    }));

                    composite_accounts.push(json!({
                        "name": comp.name,
                        "camel_name": camel_name,
                        "accounts": comp_accounts,
                        "nested_composites": nested_composites,
                        "needs_lifetime": comp_needs_lifetime
                    }));
                    // Don't extend here - nested composites are already included in the nested_composites field
                }
            }
        }

        // Preserve original IDL order - account order is critical for Solana programs
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

    /// Check if partial accounts need lifetimes (i.e., contain StorageMeta)
    fn check_if_needs_lifetime(&self, accounts: &[serde_json::Value]) -> bool {
        self.check_if_needs_lifetime_recursive(accounts)
    }

    fn check_if_needs_lifetime_recursive(&self, accounts: &[serde_json::Value]) -> bool {
        for account in accounts {
            // Skip accounts with constant addresses
            if account.get("address").and_then(|v| v.as_str()).is_some() {
                continue;
            }

            if let Some(has_pda_seeds) = account.get("has_pda_seeds").and_then(|v| v.as_bool()) {
                if has_pda_seeds {
                    return true;
                }
            }

            // For composite accounts, we need to check if they contain StorageMeta recursively
            if let Some(is_composite) = account.get("is_composite").and_then(|v| v.as_bool()) {
                if is_composite {
                    // Check the composite account's nested accounts
                    // For now, we'll be conservative and assume composites need lifetimes
                    // This could be improved by passing composite account data
                    return true;
                }
            }
        }
        false
    }

    /// Calculate lifetime requirements for all composite accounts recursively
    fn calculate_composite_lifetimes(
        &self,
        composite_accounts_map: &std::collections::HashMap<String, serde_json::Value>,
    ) -> std::collections::HashMap<String, bool> {
        let mut lifetime_map = std::collections::HashMap::new();
        let mut visited = std::collections::HashSet::new();

        for name in composite_accounts_map.keys() {
            self.calculate_composite_lifetime_recursive(
                name,
                composite_accounts_map,
                &mut lifetime_map,
                &mut visited,
            );
        }

        lifetime_map
    }

    #[allow(clippy::only_used_in_recursion)]
    fn calculate_composite_lifetime_recursive(
        &self,
        composite_name: &str,
        composite_accounts_map: &std::collections::HashMap<String, serde_json::Value>,
        lifetime_map: &mut std::collections::HashMap<String, bool>,
        visited: &mut std::collections::HashSet<String>,
    ) -> bool {
        // Avoid infinite recursion
        if visited.contains(composite_name) {
            return *lifetime_map.get(composite_name).unwrap_or(&false);
        }
        visited.insert(composite_name.to_string());

        if let Some(composite) = composite_accounts_map.get(composite_name) {
            if let Some(accounts) = composite.get("accounts").and_then(|v| v.as_array()) {
                let mut needs_lifetime = false;

                for account in accounts {
                    // Skip accounts with constant addresses
                    if account.get("address").and_then(|v| v.as_str()).is_some() {
                        continue;
                    }

                    // Check for direct PDA seeds
                    if let Some(has_pda_seeds) =
                        account.get("has_pda_seeds").and_then(|v| v.as_bool())
                    {
                        if has_pda_seeds {
                            needs_lifetime = true;
                            break;
                        }
                    }

                    // Check for nested composite accounts
                    if let Some(is_composite) =
                        account.get("is_composite").and_then(|v| v.as_bool())
                    {
                        if is_composite {
                            if let Some(composite_type_name) =
                                account.get("composite_type_name").and_then(|v| v.as_str())
                            {
                                if self.calculate_composite_lifetime_recursive(
                                    composite_type_name,
                                    composite_accounts_map,
                                    lifetime_map,
                                    visited,
                                ) {
                                    needs_lifetime = true;
                                    break;
                                }
                            }
                        }
                    }
                }

                lifetime_map.insert(composite_name.to_string(), needs_lifetime);
                return needs_lifetime;
            }
        }

        lifetime_map.insert(composite_name.to_string(), false);
        false
    }

    /// Build instruction data with proper lifetime information for composite accounts
    fn build_instruction_data_with_lifetimes(
        &self,
        instruction: &IdlInstruction,
        program_id: &str,
        composite_lifetime_map: &std::collections::HashMap<String, bool>,
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
            self.process_accounts_with_lifetimes(&instruction.accounts, composite_lifetime_map);
        let data_fields = self.process_data_fields(&instruction.args);

        // Check if instruction partial accounts need lifetimes
        let needs_lifetime =
            self.check_if_instruction_needs_lifetime(&accounts, composite_lifetime_map);

        Ok(json!({
            "name": name,
            "camel_name": camel_name,
            "snake_name": snake_name,
            "program_id": program_id,
            "discriminator": discriminator,
            "accounts": accounts,
            "composite_accounts": composite_accounts,
            "data_fields": data_fields,
            "needs_lifetime": needs_lifetime
        }))
    }

    fn check_if_instruction_needs_lifetime(
        &self,
        accounts: &[serde_json::Value],
        composite_lifetime_map: &std::collections::HashMap<String, bool>,
    ) -> bool {
        for account in accounts {
            // Skip accounts with constant addresses
            if account.get("address").and_then(|v| v.as_str()).is_some() {
                continue;
            }

            // Check for direct PDA seeds
            if let Some(has_pda_seeds) = account.get("has_pda_seeds").and_then(|v| v.as_bool()) {
                if has_pda_seeds {
                    return true;
                }
            }

            // Check for composite accounts that need lifetimes
            if let Some(is_composite) = account.get("is_composite").and_then(|v| v.as_bool()) {
                if is_composite {
                    if let Some(composite_type_name) =
                        account.get("composite_type_name").and_then(|v| v.as_str())
                    {
                        if *composite_lifetime_map
                            .get(composite_type_name)
                            .unwrap_or(&false)
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    #[allow(clippy::only_used_in_recursion)]
    fn process_accounts_with_lifetimes(
        &self,
        accounts: &[trident_idl_spec::IdlInstructionAccountItem],
        composite_lifetime_map: &std::collections::HashMap<String, bool>,
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

                    // Get lifetime info for this composite account
                    let comp_needs_lifetime =
                        composite_lifetime_map.get(&camel_name).unwrap_or(&false);

                    // Add to main accounts as composite reference
                    main_accounts.push(json!({
                        "name": comp.name,
                        "is_signer": false,
                        "is_writable": false,
                        "address": null,
                        "is_composite": true,
                        "composite_type_name": camel_name,
                        "has_pda_seeds": false,
                        "composite_needs_lifetime": *comp_needs_lifetime
                    }));

                    // Process composite account itself
                    let (comp_accounts, mut nested_composites) = self
                        .process_accounts_with_lifetimes(&comp.accounts, composite_lifetime_map);

                    // Update nested composites with correct lifetime information
                    for nested_composite in &mut nested_composites {
                        let nested_name = nested_composite
                            .get("camel_name")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        if let (Some(nested_name), Some(obj)) =
                            (nested_name, nested_composite.as_object_mut())
                        {
                            let nested_needs_lifetime =
                                composite_lifetime_map.get(&nested_name).unwrap_or(&false);
                            obj.insert("needs_lifetime".to_string(), json!(*nested_needs_lifetime));

                            // Also update nested accounts within this nested composite
                            if let Some(nested_accounts) =
                                obj.get_mut("accounts").and_then(|v| v.as_array_mut())
                            {
                                for nested_account in nested_accounts {
                                    let composite_type_name = nested_account
                                        .get("composite_type_name")
                                        .and_then(|v| v.as_str())
                                        .map(|s| s.to_string());
                                    let is_composite = nested_account
                                        .get("is_composite")
                                        .and_then(|v| v.as_bool())
                                        .unwrap_or(false);

                                    if is_composite {
                                        if let (
                                            Some(composite_type_name),
                                            Some(nested_account_obj),
                                        ) = (composite_type_name, nested_account.as_object_mut())
                                        {
                                            let composite_needs_lifetime = composite_lifetime_map
                                                .get(&composite_type_name)
                                                .unwrap_or(&false);
                                            nested_account_obj.insert(
                                                "composite_needs_lifetime".to_string(),
                                                json!(*composite_needs_lifetime),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    composite_accounts.push(json!({
                        "name": comp.name,
                        "camel_name": camel_name,
                        "accounts": comp_accounts,
                        "nested_composites": nested_composites,
                        "needs_lifetime": *comp_needs_lifetime
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
