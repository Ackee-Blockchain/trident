use std::collections::HashMap;

use convert_case::{Case, Casing};
use trident_idl_spec::{Idl, IdlInstruction};

use crate::process_program_id;

pub(crate) struct ModDefinition {
    pub module: syn::ItemMod,
    pub use_statement: syn::ItemUse,
}

#[derive(Default)]
pub struct Template {
    pub(crate) instructions_mod: Vec<ModDefinition>,
    pub(crate) instructions: HashMap<String, syn::File>,
    pub(crate) transactions_mod: Vec<ModDefinition>,
    pub(crate) transactions: HashMap<String, syn::File>,
    pub(crate) fuzz_transactions: Vec<syn::Variant>,
    pub(crate) account_storages: HashMap<String, syn::Field>,
    pub(crate) test_fuzz: Option<syn::File>,
    pub(crate) custom_types: Vec<syn::Item>,
}

impl Template {
    pub fn create_template(&mut self, idls: &[Idl], lib_names: &[String]) {
        let mut program_ids = HashMap::new();

        for idl in idls {
            // Assign program IDs to program names.
            let program_name = idl.metadata.name.clone();
            let program_id = idl.address.clone();

            program_ids.insert(program_name, program_id);

            self.process_idl(idl);
        }

        self.test_fuzz(&program_ids, lib_names);
    }

    fn process_idl(&mut self, idl: &Idl) {
        let program_id = process_program_id(idl);

        idl.instructions.iter().for_each(|instruction| {
            self.process_instruction(&program_id, instruction);
        });
        self.custom_types(idl);
    }
    fn process_instruction(&mut self, program_id: &String, instruction: &IdlInstruction) {
        self.fuzz_transaction_variant(instruction);
        self.transaction(instruction);
        self.instruction(instruction, program_id);
        self.account_storage(instruction);
    }
    pub(crate) fn get_camel_identifier(&self, instruction: &IdlInstruction) -> String {
        instruction.name.to_case(Case::UpperCamel)
    }
    pub(crate) fn get_snake_identifier(&self, instruction: &IdlInstruction) -> String {
        instruction.name.to_case(Case::Snake)
    }
}
