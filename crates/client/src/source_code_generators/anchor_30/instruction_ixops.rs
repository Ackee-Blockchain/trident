use std::collections::HashMap;

use anchor_lang_idl_spec::{Idl, IdlInstruction, IdlInstructionAccount, IdlInstructionAccountItem};
use convert_case::{Case, Casing};
use quote::format_ident;
use syn::parse_quote;

use super::fuzz_instructions_generator::ProgramAccount;

// Generate implementation of IxOps trait for each instruction
pub(crate) fn get_instruction_ixops(
    idl: &Idl,
    program_accounts: &HashMap<String, Option<ProgramAccount>>,
) -> Vec<syn::ItemImpl> {
    // let module_name: syn::Ident = parse_str(&idl.metadata.name).unwrap();
    let program_name = &idl.metadata.name;
    let program_id = &idl.address;

    idl.instructions
        .iter()
        .fold(Vec::new(), |mut instructions_ixops_impl, instruction| {
            let instruction_name = instruction.name.to_case(Case::UpperCamel);
            let discriminator = &instruction.discriminator;

            let instruction_ident_name_modified: syn::Ident =
                format_ident!("{}", &instruction_name);

            let accounts = get_accounts(instruction, program_accounts, program_name);

            let data = get_data(instruction, program_accounts, program_name);

            let doc_comment = format!(
                "IxOps implementation for `{}` with all required functions.",
                instruction_ident_name_modified
            );
            let ix_impl: syn::ItemImpl = parse_quote! {
                #[doc = #doc_comment]
                impl IxOps for #instruction_ident_name_modified {
                    type IxAccounts = FuzzAccounts;

                    /// Definition of the instruction DISCRIMINATOR.
                    fn get_discriminator(&self) -> Vec<u8> {
                        vec![#(#discriminator),*]
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

fn get_accounts(
    instruction: &IdlInstruction,
    program_accounts: &HashMap<String, Option<ProgramAccount>>,
    program_name: &str,
) -> Vec<syn::Block> {
    let mut account_implementations = vec![];

    for account in &instruction.accounts {
        match account {
            IdlInstructionAccountItem::Composite(_) => {
                panic!("Composite accounts not supported yet")
            }
            IdlInstructionAccountItem::Single(single) => {
                let account_impl = process_single_account(single, program_accounts, program_name);

                account_implementations.push(account_impl);
            }
        }
    }

    account_implementations
}

fn get_data(
    instruction: &IdlInstruction,
    _program_accounts: &HashMap<String, Option<ProgramAccount>>,
    _program_name: &str,
) -> Vec<syn::Block> {
    let mut args_implementations = vec![];

    for arg in &instruction.args {
        let arg_name = &arg.name;
        let ident_short = format_ident!("{}", arg_name);
        match arg.ty {
            anchor_lang_idl_spec::IdlType::Pubkey => {
                let pubkey_arg = parse_quote!({
                    let #ident_short: Pubkey = todo!();
                    args.extend(borsh::to_vec(&#ident_short).unwrap());
                });
                args_implementations.push(pubkey_arg);
            }
            _ => {
                let other_arg = parse_quote!({
                    args.extend(borsh::to_vec(&self.data.#ident_short).unwrap());
                });
                args_implementations.push(other_arg);
            }
        }
    }

    args_implementations
}

fn process_single_account(
    single: &IdlInstructionAccount,
    program_accounts: &HashMap<String, Option<ProgramAccount>>,
    program_name: &str,
) -> syn::Block {
    let account_name = &single.name;
    let ident_long = format_ident!("{}_{}", account_name, program_name);
    let ident_short = format_ident!("{}", account_name);

    let program_account = program_accounts.get(account_name).unwrap_or(&None);

    match program_account {
        Some(account_type) => match account_type {
            ProgramAccount::Keypair(writable, signer) => {
                handle_keypair_account(&ident_short, &ident_long, *writable, *signer)
            }

            ProgramAccount::Pda(_idl_pda, writable, _signer) => {
                handle_pda_account(&ident_short, &ident_long, *writable)
            }

            ProgramAccount::Constant(address, writable, signer) => {
                handle_constant_account(address, *writable, *signer)
            }
        },
        None => handle_unknown_account(&ident_short),
    }
}

fn handle_keypair_account(
    ident_short: &syn::Ident,
    ident_long: &syn::Ident,
    writable: bool,
    signer: bool,
) -> syn::Block {
    match (writable, signer) {
        (true, true) => {
            parse_quote!(
                {
                    let #ident_short = fuzz_accounts.#ident_long.get_or_create_account(
                        self.accounts.#ident_short,
                        client,
                        500 * LAMPORTS_PER_SOL,
                    );
                    account_metas.push(AccountMeta::new(#ident_short.pubkey(), #signer));
                    signers.push(#ident_short.insecure_clone());
                }
            )
        }
        (true, false) => {
            parse_quote!(
                {
                    let #ident_short = fuzz_accounts.#ident_long.get_or_create_account(
                        self.accounts.#ident_short,
                        client,
                        500 * LAMPORTS_PER_SOL,
                        );
                    account_metas.push(AccountMeta::new(#ident_short.pubkey(), #signer));
                }

            )
        }
        (false, true) => {
            parse_quote!(
                {
                    let #ident_short = fuzz_accounts.#ident_long.get_or_create_account(
                        self.accounts.#ident_short,
                        client,
                        500 * LAMPORTS_PER_SOL,
                    );
                    account_metas.push(AccountMeta::new_readonly(#ident_short.pubkey(), #signer));
                    signers.push(#ident_short.insecure_clone());
                }
            )
        }
        (false, false) => {
            parse_quote!(
                {
                    let #ident_short = fuzz_accounts.#ident_long.get_or_create_account(
                        self.accounts.#ident_short,
                        client,
                        500 * LAMPORTS_PER_SOL,
                    );
                    account_metas.push(AccountMeta::new_readonly(#ident_short.pubkey(), #signer));
                }

            )
        }
    }
}

fn handle_pda_account(
    ident_short: &syn::Ident,
    ident_long: &syn::Ident,
    writable: bool,
) -> syn::Block {
    match writable {
        true => {
            parse_quote!(
                {
                    let #ident_short = fuzz_accounts.#ident_long.get_or_create_account(
                        self.accounts.#ident_short,
                        client,
                        &[todo!()],
                        &self.get_program_id(),
                    );
                    account_metas.push(AccountMeta::new(#ident_short, false));
                }

            )
        }
        false => {
            parse_quote!(
                {
                    let #ident_short = fuzz_accounts.#ident_long.get_or_create_account(
                        self.accounts.#ident_short,
                        client,
                        &[todo!()],
                        &self.get_program_id(),
                    );
                    account_metas.push(AccountMeta::new_readonly(#ident_short, false));
                }
            )
        }
    }
}

fn handle_constant_account(address: &str, writable: bool, signer: bool) -> syn::Block {
    match (writable, signer) {
        (true, true) => {
            parse_quote!({
                account_metas.push(AccountMeta::new(pubkey!(#address), true));
            })
        }
        (true, false) => {
            parse_quote!({
                account_metas.push(AccountMeta::new(pubkey!(#address), false));
            })
        }
        (false, true) => {
            parse_quote!({
                account_metas.push(AccountMeta::new_readonly(pubkey!(#address), true));
            })
        }
        (false, false) => {
            parse_quote!({
                account_metas.push(AccountMeta::new_readonly(pubkey!(#address), false));
            })
        }
    }
}

fn handle_unknown_account(ident_short: &syn::Ident) -> syn::Block {
    parse_quote!(
        {
            let #ident_short = todo!();
            account_metas.push(todo!());
        }

    )
}
