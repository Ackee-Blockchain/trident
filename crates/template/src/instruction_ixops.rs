use convert_case::{Case, Casing};
use quote::format_ident;
use std::collections::HashMap;
use syn::parse_quote;

use trident_idl_spec::Idl;

use crate::{
    get_accounts::get_accounts, get_data::get_data, instruction_account::InstructionAccount,
    process_discriminator, process_program_id,
};

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
