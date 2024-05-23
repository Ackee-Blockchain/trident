//! The `idl` module contains structs and functions for Anchor program code parsing.
//!
//! [Idl] example:
//!
//! ```rust,ignore
//! Idl {
//!     programs: [
//!         IdlProgram {
//!             name: IdlName {
//!                 snake_case: "turnstile",
//!                 upper_camel_case: "Turnstile",
//!             },
//!             id: "[216u8 , 55u8 , 200u8 , 93u8 , 189u8 , 81u8 , 94u8 , 109u8 , 14u8 , 249u8 , 244u8 , 106u8 , 68u8 , 214u8 , 222u8 , 190u8 , 9u8 , 25u8 , 199u8 , 75u8 , 79u8 , 230u8 , 94u8 , 137u8 , 51u8 , 187u8 , 193u8 , 48u8 , 87u8 , 222u8 , 175u8 , 163u8]",
//!             instruction_account_pairs: [
//!                 (
//!                     IdlInstruction {
//!                         name: IdlName {
//!                             snake_case: "initialize",
//!                             upper_camel_case: "Initialize",
//!                         },
//!                         parameters: [],
//!                     },
//!                     IdlAccountGroup {
//!                         name: IdlName {
//!                             snake_case: "initialize",
//!                             upper_camel_case: "Initialize",
//!                         },
//!                         accounts: [
//!                             (
//!                                 "state",
//!                                 "anchor_lang :: solana_program :: pubkey :: Pubkey",
//!                             ),
//!                             (
//!                                 "user",
//!                                 "anchor_lang :: solana_program :: pubkey :: Pubkey",
//!                             ),
//!                             (
//!                                 "system_program",
//!                                 "anchor_lang :: solana_program :: pubkey :: Pubkey",
//!                             ),
//!                         ],
//!                     },
//!                 ),
//!                 (
//!                     IdlInstruction {
//!                         name: IdlName {
//!                             snake_case: "coin",
//!                             upper_camel_case: "Coin",
//!                         },
//!                         parameters: [
//!                             (
//!                                 "dummy_arg",
//!                                 "String",
//!                             ),
//!                         ],
//!                     },
//!                     IdlAccountGroup {
//!                         name: IdlName {
//!                             snake_case: "update_state",
//!                             upper_camel_case: "UpdateState",
//!                         },
//!                         accounts: [
//!                             (
//!                                 "state",
//!                                 "anchor_lang :: solana_program :: pubkey :: Pubkey",
//!                             ),
//!                         ],
//!                     },
//!                 ),
//!                 (
//!                     IdlInstruction {
//!                         name: IdlName {
//!                             snake_case: "push",
//!                             upper_camel_case: "Push",
//!                         },
//!                         parameters: [],
//!                     },
//!                     IdlAccountGroup {
//!                         name: IdlName {
//!                             snake_case: "update_state",
//!                             upper_camel_case: "UpdateState",
//!                         },
//!                         accounts: [
//!                             (
//!                                 "state",
//!                                 "anchor_lang :: solana_program :: pubkey :: Pubkey",
//!                             ),
//!                         ],
//!                     },
//!                 ),
//!             ],
//!         },
//!     ],
//! }
//! ```

use crate::constants::*;
use heck::{ToSnakeCase, ToUpperCamelCase};
use quote::ToTokens;
use syn::{visit::Visit, File};
use thiserror::Error;

const ACCOUNT_MOD_PREFIX: &str = "__client_accounts_";
const MOD_PRIVATE: &str = "__private";
const MOD_INSTRUCTION: &str = "instruction";
const MOD_GLOBAL: &str = "__global";
const ID_IDENT: &str = "ID";
const ACCOUNTS_IDENT: &str = "__accounts";

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    RustParsingError(#[from] syn::Error),
    #[error("missing or invalid program item: '{0}'")]
    MissingOrInvalidProgramItems(String),
}

struct ModPub {
    pub mod_name: String,
    pub is_pub: bool,
}

struct FullPathFinder {
    target_item_name: String,
    current_module: String,
    found_path: Option<String>,
    module_pub: Vec<ModPub>,
}

pub fn find_item_path(target_item_name: &str, syn_file: &File) -> Option<String> {
    let mut finder = FullPathFinder {
        target_item_name: target_item_name.to_string(),
        current_module: "".to_string(),
        found_path: None,
        module_pub: vec![],
    };
    finder.visit_file(syn_file);
    finder.found_path
}

impl<'ast> syn::visit::Visit<'ast> for FullPathFinder {
    fn visit_item(&mut self, item: &'ast syn::Item) {
        if let Some(_found_path) = &self.found_path {
            return;
        }

        // INFO this will only look for enum or struct
        match item {
            syn::Item::Enum(syn::ItemEnum { ident, .. })
            | syn::Item::Struct(syn::ItemStruct { ident, .. }) => {
                if *ident == self.target_item_name {
                    // Found the target item, construct the full path.
                    self.found_path = Some(format!("{}::{}", self.current_module, ident));
                    for x in &self.module_pub {
                        if !x.is_pub {
                            println!(
                                "{WARNING} {} is private. Prefix with pub to access via fully qualified path of {}",
                                x.mod_name,ident
                            );
                        }
                    }
                    return;
                }
            }

            _ => {}
        }

        syn::visit::visit_item(self, item);
    }

    fn visit_item_mod(&mut self, module: &'ast syn::ItemMod) {
        let old_module = self.current_module.clone();
        self.current_module = format!("{}::{}", self.current_module, module.ident);

        let is_pub = matches!(module.vis, syn::Visibility::Public(_));

        self.module_pub.push(ModPub {
            mod_name: module.ident.to_string(),
            is_pub,
        });

        syn::visit::visit_item_mod(self, module);

        self.module_pub.pop();
        self.current_module = old_module;
    }
}

#[derive(Debug, Default)]
pub struct Idl {
    pub programs: Vec<IdlProgram>,
}

#[derive(Debug, Clone)]
pub struct IdlName {
    pub snake_case: String,
    pub upper_camel_case: String,
}

#[derive(Debug, Clone)]
pub struct IdlProgram {
    pub name: IdlName,
    pub id: String,
    pub instruction_account_pairs: Vec<(IdlInstruction, IdlAccountGroup)>,
}

#[derive(Debug, Clone)]
pub struct IdlInstruction {
    pub name: IdlName,
    pub parameters: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct IdlAccountGroup {
    pub name: IdlName,
    pub accounts: Vec<(String, String)>,
}

pub fn parse_to_idl_program(name: String, code: &str) -> Result<IdlProgram, Error> {
    let mut static_program_id = None::<syn::ItemStatic>;
    let mut mod_private = None::<syn::ItemMod>;
    let mut mod_instruction = None::<syn::ItemMod>;
    let mut account_mods = Vec::<syn::ItemMod>::new();

    let syn_file = syn::parse_file(code)?;

    for item in syn_file.items.iter() {
        match item {
            syn::Item::Static(item_static) if item_static.ident == ID_IDENT => {
                static_program_id = Some(item_static.clone());
            }
            syn::Item::Mod(item_mod) => match item_mod.ident.to_string().as_str() {
                MOD_PRIVATE => mod_private = Some(item_mod.clone()),
                MOD_INSTRUCTION => mod_instruction = Some(item_mod.clone()),
                _ => set_account_modules(&mut account_mods, item_mod),
            },
            _ => (),
        }
    }

    let static_program_id = static_program_id.ok_or(Error::MissingOrInvalidProgramItems(
        "missing static ID".to_string(),
    ))?;
    let mod_private = mod_private.ok_or(Error::MissingOrInvalidProgramItems(
        "missing mod private".to_string(),
    ))?;
    let mod_instruction = mod_instruction.ok_or(Error::MissingOrInvalidProgramItems(
        "missing mod instruction".to_string(),
    ))?;

    // ------ get program id ------
    // Obtain Program ID
    //
    // input example:
    //
    // ```
    //
    // pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    // anchor_lang::solana_program::pubkey::Pubkey::new_from_array([
    //     222u8, 219u8, 96u8, 222u8, 150u8, 129u8, 32u8, 71u8, 184u8, 221u8, 54u8, 221u8, 224u8,
    //     97u8, 103u8, 133u8, 11u8, 126u8, 234u8, 11u8, 186u8, 25u8, 119u8, 161u8, 48u8, 137u8, 77u8,
    //     249u8, 144u8, 153u8, 133u8, 92u8,
    // ]);
    //
    // ```

    let program_id_bytes = {
        let new_pubkey_call = match *static_program_id.expr {
            syn::Expr::Call(new_pubkey_call) => new_pubkey_call,
            _ => {
                return Err(Error::MissingOrInvalidProgramItems(
                    "static ID: new pubkey call not found".to_string(),
                ))
            }
        };
        match new_pubkey_call.args.into_iter().next() {
            Some(syn::Expr::Array(pubkey_bytes)) => pubkey_bytes,
            _ => {
                return Err(Error::MissingOrInvalidProgramItems(
                    "static ID: pubkey bytes not found".to_string(),
                ))
            }
        }
    };

    // ------ get instruction_item_fns ------
    // Obtain Instructions as whole, parse at next step
    //
    // input example:
    //
    // ```
    //
    // mod __private {
    //     use super::*;
    //     #[doc = r" __global mod defines wrapped handlers for global instructions."]
    //     pub mod __global {
    //         use super::*;
    //         #[inline(never)]
    //         pub fn init_vesting(
    //             __program_id: &Pubkey,
    //             __accounts: &[AccountInfo],
    //             __ix_data: &[u8],
    //         ) -> anchor_lang::Result<()> {
    //             ::solana_program::log::sol_log("Instruction: InitVesting");
    //             let ix = instruction::InitVesting::deserialize(&mut &__ix_data[..])
    //                 .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
    //             let instruction::InitVesting {
    //                 recipient,
    //                 amount,
    //                 start_at,
    //                 end_at,
    //                 interval,
    //                 input_option,
    //             } = ix;
    //             let mut __bumps = std::collections::BTreeMap::new();
    //             let mut __reallocs = std::collections::BTreeSet::new();
    //             let mut __remaining_accounts: &[AccountInfo] = __accounts;
    //             let mut __accounts = InitVesting::try_accounts(
    //                 __program_id,
    //                 &mut __remaining_accounts,
    //                 __ix_data,
    //                 &mut __bumps,
    //                 &mut __reallocs,
    //             )?;
    //             let result = fuzz_example3::init_vesting(
    //                 anchor_lang::context::Context::new(
    //                     __program_id,
    //                     &mut __accounts,
    //                     __remaining_accounts,
    //                     __bumps,
    //                 ),
    //                 recipient,
    //                 amount,
    //                 start_at,
    //                 end_at,
    //                 interval,
    //                 input_option,
    //             )?;
    //             __accounts.exit(__program_id)
    //         }
    //     }
    // }
    //
    // ```

    let instruction_item_fns = {
        let items = mod_private
            .content
            .map(|(_, items)| items)
            .unwrap_or_default();
        let item_mod_global = items
            .into_iter()
            .find_map(|item| match item {
                syn::Item::Mod(item_mod) if item_mod.ident == MOD_GLOBAL => Some(item_mod),
                _ => None?,
            })
            .ok_or(Error::MissingOrInvalidProgramItems(
                "mod private: mod global not found".to_string(),
            ))?;
        let items = item_mod_global
            .content
            .map(|(_, items)| items)
            .unwrap_or_default();
        items.into_iter().filter_map(|item| match item {
            syn::Item::Fn(item_fn) => Some(item_fn),
            _ => None,
        })
    };

    // ------ get instruction + account group names ------
    // This will obtain:
    // IdlInstruction
    //      - name
    //      - empty parameters vector
    // IdlAccountGroup (Which is actually Context from Anchor perspective)
    //      - name
    //      - empty accounts vector
    // input example:
    //
    // ```
    //
    //         pub fn init_vesting(
    //             __program_id: &Pubkey,
    //             __accounts: &[AccountInfo],
    //             __ix_data: &[u8],
    //         ) -> anchor_lang::Result<()> {
    //             ::solana_program::log::sol_log("Instruction: InitVesting");
    //             let ix = instruction::InitVesting::deserialize(&mut &__ix_data[..])
    //                 .map_err(|_| anchor_lang::error::ErrorCode::InstructionDidNotDeserialize)?;
    //             let instruction::InitVesting {
    //                 recipient,
    //                 amount,
    //                 start_at,
    //                 end_at,
    //                 interval,
    //                 input_option,
    //             } = ix;
    //             let mut __bumps = std::collections::BTreeMap::new();
    //             let mut __reallocs = std::collections::BTreeSet::new();
    //             let mut __remaining_accounts: &[AccountInfo] = __accounts;
    //
    // *** we are looking for this part ***
    //
    //             let mut __accounts = InitVesting::try_accounts(
    //                 __program_id,
    //                 &mut __remaining_accounts,
    //                 __ix_data,
    //                 &mut __bumps,
    //                 &mut __reallocs,
    //             )?;
    // *************************************
    //             let result = fuzz_example3::init_vesting(
    //                 anchor_lang::context::Context::new(
    //                     __program_id,
    //                     &mut __accounts,
    //                     __remaining_accounts,
    //                     __bumps,
    //                 ),
    //                 recipient,
    //                 amount,
    //                 start_at,
    //                 end_at,
    //                 interval,
    //                 input_option,
    //             )?;
    //             __accounts.exit(__program_id)
    //         }
    //
    // ```

    let mut instruction_account_pairs = Vec::new();
    instruction_item_fns
        .into_iter()
        .map(|item_fn| {
            // stmt example: `let mut accounts = UpdateState::try_accounts(program_id, &mut remaining_accounts, ix_data)?;`
            let account_group_name = item_fn.block.stmts.into_iter().find_map(|stmt| {
                let local = if let syn::Stmt::Local(local) = stmt {
                    local
                } else {
                    None?
                };
                if !matches!(&local.pat, syn::Pat::Ident(pat_ident) if pat_ident.ident == ACCOUNTS_IDENT) {
                    None?
                }
                let init_expr = *local.init?.1;
                let expr_try_expr = match init_expr {
                    syn::Expr::Try(expr_try) => *expr_try.expr,
                    _ => None?,
                };
                let expr_call_func = match expr_try_expr {
                    syn::Expr::Call(expr_call) => *expr_call.func,
                    _ => None?,
                };
                let account_group_name = match expr_call_func {
                    syn::Expr::Path(expr_path) => expr_path.path.segments.into_iter().next()?.ident,
                    _ => None?,
                };
                Some(account_group_name.to_string())
            })?;

            let instruction_name = item_fn.sig.ident.to_string();
            let idl_instruction = IdlInstruction {
                name: IdlName {
                    upper_camel_case: instruction_name.to_upper_camel_case(),
                    snake_case: instruction_name,
                },
                parameters: Vec::new(),
            };
            let idl_account = IdlAccountGroup {
                name: IdlName {
                    snake_case: account_group_name.to_snake_case(),
                    upper_camel_case: account_group_name,
                },
                accounts: Vec::new(),
            };
            Some((idl_instruction, idl_account))
        })
        .try_for_each(|pair| {
            if let Some(pair) = pair {
                instruction_account_pairs.push(pair);
                Ok(())
            } else {
                Err(Error::MissingOrInvalidProgramItems(
                    "statement with `accounts` not found".to_string(),
                ))
            }
        })?;

    // ------ get instruction parameters ------
    // This will obtain input parameters for every instruction
    // For custom types such as Struct or Enum, which can be also at instruction
    // input, we look for fully qualified path such as:
    // fuzz_example3::instructions::initialize::CustomInputOption
    //
    // input example:
    //
    // ```
    //
    // pub mod instruction {
    //     use super::*;
    //     #[doc = r" Instruction."]
    //     pub struct InitVesting {
    //         pub recipient: Pubkey,
    //         pub amount: u64,
    //         pub start_at: u64,
    //         pub end_at: u64,
    //         pub interval: u64,
    //         pub input_option: CustomInputOption,
    //     }
    //      ...
    //      ...
    //      ...
    //      pub struct WithdrawUnlocked;
    //      ...
    //      ...
    //      ...
    // }
    //
    // ```

    let mut instruction_mod_items = mod_instruction
        .content
        .ok_or(Error::MissingOrInvalidProgramItems(
            "instruction mod: empty content".to_string(),
        ))?
        .1
        .into_iter();

    for (idl_instruction, _) in &mut instruction_account_pairs {
        let instruction_struct_name = &idl_instruction.name.upper_camel_case;

        let instruction_item_struct_fields = instruction_mod_items
            .find_map(|item| {
                let instruction_item_struct = match item {
                    syn::Item::Struct(item_struct)
                        if item_struct.ident == instruction_struct_name =>
                    {
                        item_struct
                    }
                    _ => None?,
                };
                let fields = match instruction_item_struct.fields {
                    syn::Fields::Named(fields_named) => fields_named.named,
                    syn::Fields::Unit => syn::punctuated::Punctuated::new(),
                    syn::Fields::Unnamed(_) => None?,
                };
                Some(fields.into_iter())
            })
            .ok_or(Error::MissingOrInvalidProgramItems(format!(
                "instruction struct: missing struct for item {}",
                instruction_struct_name,
            )))?;

        idl_instruction.parameters = instruction_item_struct_fields
            .map(|field| {
                let parameter_name = field.ident.unwrap().to_string();
                let parameter_id_type = field.ty.into_token_stream().to_string();

                let type_name = parameter_id_type
                    .clone()
                    .replace(' ', "")
                    .split("::")
                    .last()
                    .unwrap_or(&parameter_id_type)
                    .to_string();

                if let Some(path) = find_item_path(&type_name, &syn_file) {
                    let name = name.to_snake_case();
                    let tmp_final_path = format!("{name}{path}");
                    (parameter_name, tmp_final_path)
                } else {
                    (parameter_name, parameter_id_type)
                }
            })
            .collect();
    }

    // ------ get accounts ------
    // This will obtain corresponding Context for every Instruction
    // input example:
    //
    // ```
    //
    // pub(crate) mod __client_accounts_init_vesting {
    //     use super::*;
    //     use anchor_lang::prelude::borsh;
    //     #[doc = " Generated client accounts for [`InitVesting`]."]
    //     pub struct InitVesting {
    //         pub sender: anchor_lang::solana_program::pubkey::Pubkey,
    //         pub sender_token_account: anchor_lang::solana_program::pubkey::Pubkey,
    //         pub escrow: anchor_lang::solana_program::pubkey::Pubkey,
    //         pub escrow_token_account: anchor_lang::solana_program::pubkey::Pubkey,
    //         pub mint: anchor_lang::solana_program::pubkey::Pubkey,
    //         pub token_program: anchor_lang::solana_program::pubkey::Pubkey,
    //         pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    //     }
    // }
    //
    // ```

    for account_mod_item in account_mods {
        let account_struct_name = account_mod_item
            .ident
            .to_string()
            .strip_prefix(ACCOUNT_MOD_PREFIX)
            .unwrap()
            .to_upper_camel_case();

        let account_item_struct = account_mod_item
            .content
            .ok_or(Error::MissingOrInvalidProgramItems(format!(
                "account mod: empty content for item {}",
                account_mod_item.ident
            )))?
            .1
            .into_iter()
            .find_map(|item| match item {
                syn::Item::Struct(item_struct) if item_struct.ident == account_struct_name => {
                    Some(item_struct)
                }
                _ => None?,
            })
            .ok_or(Error::MissingOrInvalidProgramItems(format!(
                "account mod: struct with name {} not found",
                account_struct_name
            )))?;

        let account_item_struct_fields = match account_item_struct.fields {
            syn::Fields::Named(fields_named) => fields_named.named,
            syn::Fields::Unit => syn::punctuated::Punctuated::new(),
            syn::Fields::Unnamed(_) => {
                return Err(Error::MissingOrInvalidProgramItems(
                    "account struct: unnamed fields not allowed".to_string(),
                ))
            }
        };

        let accounts = account_item_struct_fields
            .into_iter()
            .map(|field| {
                let account_name = field.ident.unwrap().to_string();
                let account_id_type = field.ty.into_token_stream().to_string();
                (account_name, account_id_type)
            })
            .collect::<Vec<_>>();

        for (_, idl_account_group) in &mut instruction_account_pairs {
            if idl_account_group.name.upper_camel_case == account_struct_name {
                idl_account_group.accounts = accounts.clone();
            }
        }
    }

    // ------ // ------

    Ok(IdlProgram {
        name: IdlName {
            upper_camel_case: name.to_upper_camel_case(),
            snake_case: name.to_snake_case(),
        },
        id: program_id_bytes.into_token_stream().to_string(),
        instruction_account_pairs,
    })
}

fn set_account_modules(account_modules: &mut Vec<syn::ItemMod>, item_module: &syn::ItemMod) {
    if item_module
        .ident
        .to_string()
        .starts_with(ACCOUNT_MOD_PREFIX)
    {
        account_modules.push(item_module.clone());
        return;
    }
    let modules = &item_module
        .content
        .as_ref()
        .ok_or(Error::MissingOrInvalidProgramItems(
            "account mod: empty content".to_string(),
        ))
        .unwrap()
        .1;
    for module in modules.iter() {
        if let syn::Item::Mod(nested_module) = module {
            set_account_modules(account_modules, nested_module);
        }
    }
}
