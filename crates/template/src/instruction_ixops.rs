use convert_case::{Case, Casing};
use quote::format_ident;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use syn::parse_quote;

use trident_idl_spec::{Idl, IdlInstruction};

use crate::{
    get_accounts::get_accounts, get_data::get_data, instruction_account::InstructionAccount,
};

pub const SIGHASH_GLOBAL_NAMESPACE: &str = "global";

// Generate implementation of IxOps trait for each instruction
pub(crate) fn get_instruction_ixops(
    idl: &Idl,
    instructions_accounts: &HashMap<String, InstructionAccount>,
) -> Vec<syn::ItemImpl> {
    idl.instructions
        .iter()
        .fold(Vec::new(), |mut instructions_ixops_impl, instruction| {
            let instruction_name = instruction.name.to_case(Case::UpperCamel);
            let instruction_name_ident: syn::Ident = format_ident!("{}", &instruction_name);

            let instruction_discriminator = process_discriminator(instruction);
            let program_id = process_program_id(idl);
            let data = get_data(instruction, instructions_accounts);
            let accounts = get_accounts(instruction, instructions_accounts);

            let doc_comment = format!(
                "IxOps implementation for `{}` with all required functions.",
                instruction_name_ident
            );
            let ix_impl: syn::ItemImpl = parse_quote! {
                #[doc = #doc_comment]
                impl IxOps for #instruction_name_ident {
                    type IxAccounts = FuzzAccounts;

                    /// Definition of the instruction DISCRIMINATOR.
                    fn get_discriminator(&self) -> Vec<u8> {
                        vec![#(#instruction_discriminator),*]
                    }

                    /// Definition of the program ID that the Instruction is associated with.
                    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
                        pubkey!(#program_id)
                    }

                    /// Definition of the Instruction data.
                    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
                    /// or customize the data as needed.
                    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
                    fn get_data(
                        &self,
                        client: &mut impl FuzzClient,
                        fuzz_accounts: &mut FuzzAccounts,
                    ) -> Result<Vec<u8>, FuzzingError> {
                        let mut args: Vec<u8> = self.get_discriminator();

                        #(#data)*

                        Ok(args)
                    }

                    /// Definition of of the accounts required by the Instruction.
                    /// To utilize accounts stored in `FuzzAccounts`, use
                    /// `fuzz_accounts.account_name.get_or_create_account()`.
                    /// If no signers are required, leave the vector empty.
                    /// For AccountMetas use <program>::accounts::<corresponding_metas>
                    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
                    fn get_accounts(
                        &self,
                        client: &mut impl FuzzClient,
                        fuzz_accounts: &mut FuzzAccounts,
                    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
                        let mut account_metas = vec![];
                        let mut signers = vec![];

                        #(#accounts)*

                        Ok((signers, account_metas))
                    }
                }
            };

            instructions_ixops_impl.push(ix_impl);
            instructions_ixops_impl
        })
}

fn process_discriminator(instruction: &IdlInstruction) -> Vec<u8> {
    // if discriminator is not provided, generate it
    if instruction.discriminator.is_empty() {
        gen_discriminator(SIGHASH_GLOBAL_NAMESPACE, &instruction.name).to_vec()
    } else {
        // if discriminator is provided, use it
        instruction.discriminator.clone()
    }
}

fn gen_discriminator(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{namespace}:{name}");

    let mut hasher = Sha256::new();
    hasher.update(preimage);

    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(&hasher.finalize().as_slice()[..8]);
    sighash
}

fn process_program_id(idl: &Idl) -> String {
    // if program ID is present, use it
    if !idl.address.is_empty() {
        idl.address.clone()
    } else {
        // if program ID is not present, use placeholder
        // We might be able to parse it form program, but it
        // might not be necesarry as newer versions of IDL will contain it
        "fill corresponding program ID here".to_string()
    }
}
