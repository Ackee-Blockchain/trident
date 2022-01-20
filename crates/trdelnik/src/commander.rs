use fehler::throws;
use thiserror::Error;
use tokio::{process::{Command, Child}};
use std::{borrow::Cow, io, string::FromUtf8Error, collections::BTreeMap};
use solana_sdk::signer::keypair::Keypair;
use tokio::fs;
use cargo_metadata::MetadataCommand;
use futures::future::try_join_all;
use quote::{quote, ToTokens};
use heck::{ToUpperCamelCase, ToSnakeCase};
use crate::Client;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0:?}")]
    IoError(#[from] io::Error),
    #[error("{0:?}")]
    Utf8Error(#[from] FromUtf8Error),
    #[error("{0:?}")]
    RustParsingError(#[from] syn::Error),
    #[error("localnet is not running")]
    LocalnetIsNotRunning,
    #[error("localnet is still running")]
    LocalnetIsStillRunning,
    #[error("build programs failed")]
    BuildProgramsFailed,
    #[error("read program code failed: '{0}'")]
    ReadProgramCodeFailed(String),
    #[error("missing program item: '{0}'")]
    MissingOrInvalidProgramItems(&'static str),
}

pub struct LocalnetHandle {
    solana_test_validator_process: Child,
}

impl LocalnetHandle {
    #[throws]
    /// _Note_: Manual kill: `kill -9 $(lsof -t -i:8899)`
    pub async fn stop(mut self) {
        self.solana_test_validator_process.kill().await?;
        if Client::new(Keypair::new()).is_localnet_running(false).await {
            Err(Error::LocalnetIsStillRunning)?
        }
        fs::remove_dir_all("test-ledger").await?;
        println!("localnet stopped and its ledger deleted");
    }
}

pub struct Commander {
    root: Cow<'static, str>
}

impl Commander {
    pub fn new() -> Self {
        Self {
            root: "../../".into()
        }
    }

    pub fn with_root(root: impl Into<Cow<'static, str>>) -> Self {
        Self {
            root: root.into()
        }
    }

    #[throws]
    pub async fn build_programs(&self) {
        let success = Command::new("cargo")
            .arg("build-bpf")
            .spawn()?
            .wait()
            .await?
            .success();
        if !success {
            Err(Error::BuildProgramsFailed)?;
        }
    }

    #[throws]
    pub async fn generate_idls(&self) {
        let cargo_toml_data = MetadataCommand::new()
            .no_deps()
            .exec()
            .unwrap();

        let program_names = cargo_toml_data
            .packages
            .into_iter()
            .filter_map(|package| {
                match package.manifest_path.iter().nth_back(2) {
                    Some("programs") => Some(package.name),
                    _ => None,
                }
            });

        let name_code_pairs = program_names.map(|name| async move {
            let output = Command::new("cargo")
                .arg("+nightly")
                .arg("rustc")
                .args(&["--package", &name])
                .arg("--profile=check")
                .arg("--")
                .arg("-Zunpretty=expanded")
                .output()
                .await?;
            if output.status.success() {
                let code = String::from_utf8(output.stdout)?;
                Ok((name, code))
            } else {
                let error_text = String::from_utf8(output.stderr)?;
                Err(Error::ReadProgramCodeFailed(error_text))
            }
        });
        let name_code_pairs = try_join_all(name_code_pairs).await?;

        for (name, code) in name_code_pairs {
            println!("{name}");
            println!("------");
            println!("{:#?}", parse_to_idl_program(name, &code).await);
        }
    }

    #[throws]
    pub async fn start_localnet(&self) -> LocalnetHandle {
        let process = Command::new("solana-test-validator")
            .arg("-C")
            .arg([&self.root, "config.yml"].concat())
            .arg("-r")
            .arg("-q")
            .spawn()?;
        if !Client::new(Keypair::new()).is_localnet_running(true).await {
            Err(Error::LocalnetIsNotRunning)?
        }
        println!("localnet started");
        LocalnetHandle {
            solana_test_validator_process: process,
        }
    }
}

#[derive(Debug)]
struct Idl {
    programs: Vec<IdlProgram>
}

#[derive(Debug)]
struct IdlName {
    snake_case: String,
    upper_camel_case: String,
}

#[derive(Debug)]
struct IdlProgram {
    name: IdlName,
    id: String,
    instruction_account_pairs: Vec<(IdlInstruction, IdlAccountGroup)>
}

#[derive(Debug)]
struct IdlInstruction {
    name: IdlName,
    parameters: Vec<(String, String)>
}

#[derive(Debug)]
struct IdlAccountGroup {
    name: IdlName,
    accounts: Vec<(String, String)>
}

async fn parse_to_idl_program(name: String, code: &str) -> Result<IdlProgram, Error> {
    let mut root_items = syn::parse_file(&code)?.items.into_iter();

    // ------ get program id ------

    // input example:
    // ```
    // pub static ID: anchor_lang::solana_program::pubkey::Pubkey =
    //     anchor_lang::solana_program::pubkey::Pubkey::new_from_array([216u8, 55u8,
    //                                                                  200u8, 93u8,
    //                                                                  189u8, 81u8,
    //                                                                  94u8, 109u8,
    //                                                                  14u8, 249u8,
    //                                                                  244u8, 106u8,
    //                                                                  68u8, 214u8,
    //                                                                  222u8, 190u8,
    //                                                                  9u8, 25u8,
    //                                                                  199u8, 75u8,
    //                                                                  79u8, 230u8,
    //                                                                  94u8, 137u8,
    //                                                                  51u8, 187u8,
    //                                                                  193u8, 48u8,
    //                                                                  87u8, 222u8,
    //                                                                  175u8,
    //                                                                  163u8]);
    // ```

    let program_id_bytes = root_items.find_map(|item| {
        let item_static = match item {
            syn::Item::Static(item_static) if item_static.ident == "ID" => item_static,
            _ => None?,
        };
        let new_pubkey_call = match *item_static.expr {
            syn::Expr::Call(new_pubkey_call) => new_pubkey_call,
            _ => None?,
        };
        let pubkey_bytes = match new_pubkey_call.args.into_iter().next() {
            Some(syn::Expr::Array(pubkey_bytes)) => pubkey_bytes,
            _ => None?,
        };
        Some(pubkey_bytes)
    }).ok_or_else(|| Error::MissingOrInvalidProgramItems("static ID"))?;

    // ------ get instruction_item_fns ------

    // input example:
    // ```
    // mod __private {
    //     pub mod __global {
    //         use super::*;
    //         #[inline(never)]
    //         pub fn initialize(program_id: &Pubkey, accounts: &[AccountInfo],
    //                           ix_data: &[u8]) -> ProgramResult {
    //             let ix =
    //                 instruction::Initialize::deserialize(&mut &ix_data[..]).map_err(|_|
    //                                                                                     anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
    //             let instruction::Initialize = ix;
    //             let mut remaining_accounts: &[AccountInfo] = accounts;
    //             let mut accounts =
    //                 Initialize::try_accounts(program_id, &mut remaining_accounts,
    //                                          ix_data)?;
    //             turnstile::initialize(Context::new(program_id, &mut accounts,
    //                                                remaining_accounts))?;
    //             accounts.exit(program_id)
    //         }
    // ```

    let instruction_item_fns = root_items.find_map(|item| {
        let item_mod_private = match item {
            syn::Item::Mod(item_mod) if item_mod.ident == "__private" => item_mod,
            _ => None?,
        };
        let items = item_mod_private.content.map(|(_, items)| items).unwrap_or_default();
        let item_mod_global = items.into_iter().find_map(|item| {
            match item {
                syn::Item::Mod(item_mod) if item_mod.ident == "__global" => Some(item_mod),
                _ => None,
            }
        })?;
        let items = item_mod_global.content.map(|(_, items)| items).unwrap_or_default();
        Some(
            items.into_iter().filter_map(|item| {
                match item {
                    syn::Item::Fn(item_fn) => Some(item_fn),
                    _ => None,
                }
            })
        )
    }).ok_or_else(|| Error::MissingOrInvalidProgramItems("_private + __global modules"))?;

    // ------ get instruction + account group names ------

    // input example:
    // ```
    //         pub fn initialize(program_id: &Pubkey, accounts: &[AccountInfo],
    //                           ix_data: &[u8]) -> ProgramResult {
    //             let ix =
    //                 instruction::Initialize::deserialize(&mut &ix_data[..]).map_err(|_|
    //                                                                                     anchor_lang::__private::ErrorCode::InstructionDidNotDeserialize)?;
    //             let instruction::Initialize = ix;
    //             let mut remaining_accounts: &[AccountInfo] = accounts;
    //             let mut accounts =
    //                 Initialize::try_accounts(program_id, &mut remaining_accounts,
    //                                          ix_data)?;
    //             turnstile::initialize(Context::new(program_id, &mut accounts,
    //                                                remaining_accounts))?;
    //             accounts.exit(program_id)
    //         }
    // ```

    let mut instruction_account_pairs = Vec::new();
    instruction_item_fns.into_iter().map(|item_fn| {
        // stmt example: `let mut accounts = UpdateState::try_accounts(program_id, &mut remaining_accounts, ix_data)?;`
        let account_group_name = item_fn.block.stmts.into_iter().find_map(|stmt| {
            let local = if let syn::Stmt::Local(local) = stmt {
                local
            } else { 
                None?
            };
            if !matches!(&local.pat, syn::Pat::Ident(pat_ident) if pat_ident.ident == "accounts") {
                None?
            }
            let init_expr = *local.init?.1;
            let expr_try_expr = match init_expr {
                syn::Expr::Try(expr_try) => *expr_try.expr,
                _ => None?
            };
            let expr_call_func = match expr_try_expr {
                syn::Expr::Call(expr_call) => *expr_call.func,
                _ => None?
            };
            let account_group_name = match expr_call_func {
                syn::Expr::Path(expr_path) => expr_path.path.segments.into_iter().next()?.ident,
                _ => None?
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
    }).try_for_each(|pair| {
        if let Some(pair) = pair {
            Ok(instruction_account_pairs.push(pair))
        } else {
            Err(Error::MissingOrInvalidProgramItems("statement with `accounts`"))
        }
    })?;

    // ------ get instruction parameters ------

    // input example:
    // ```
    // pub mod instruction {
    //     use super::*;
    //     pub mod state {
    //         use super::*;
    //     }
    // // **
    //     pub struct Initialize;
    // // **
    //     pub struct Coin {
    //         pub dummy_arg: String,   
    //     }
    // ```

    let mut instruction_mod_items = root_items.find_map(|item| {
        match item {
            syn::Item::Mod(item_mod) if item_mod.ident == "instruction" => {
                Some(item_mod.content?.1.into_iter())
            }
            _ => None,
        }
    }).ok_or_else(|| Error::MissingOrInvalidProgramItems("instruction mod"))?;

    for (idl_instruction, _) in &mut instruction_account_pairs {
        let instruction_struct_name = &idl_instruction.name.upper_camel_case;

        let instruction_item_struct_fields = instruction_mod_items.find_map(|item| {
            let instruction_item_struct = match item {
                syn::Item::Struct(item_struct) if item_struct.ident == instruction_struct_name => item_struct,
                _ => None?,
            };
            let fields = match instruction_item_struct.fields {
                syn::Fields::Named(fields_named) => fields_named.named,
                syn::Fields::Unit => syn::punctuated::Punctuated::new(),
                syn::Fields::Unnamed(_) => None?, 
            };
            Some(fields.into_iter())
        }).ok_or_else(|| Error::MissingOrInvalidProgramItems("instruction struct"))?;

        idl_instruction.parameters = instruction_item_struct_fields.map(|field| {
            let parameter_name = field.ident.unwrap().to_string();
            let parameter_id_type = field.ty.into_token_stream().to_string();
            (parameter_name, parameter_id_type)
        }).collect();
    }

    // ------ get accounts ------

    // input example:
    // ```
    // pub(crate) mod __client_accounts_initialize {
    //     use super::*;
    //     use anchor_lang::prelude::borsh;
    //     pub struct Initialize {
    //         pub state: anchor_lang::solana_program::pubkey::Pubkey,
    //         pub user: anchor_lang::solana_program::pubkey::Pubkey,
    //         pub system_program: anchor_lang::solana_program::pubkey::Pubkey,
    //     }
    // ```

    let mut group_name_with_accounts = BTreeMap::<String, Vec<(String, String)>>::new();

    for (_, idl_account_group) in &mut instruction_account_pairs {
        let account_struct_name = &idl_account_group.name.upper_camel_case;
        if let Some(accounts) = group_name_with_accounts.get(account_struct_name) {
            idl_account_group.accounts = accounts.clone();
            continue;
        }

        let account_mod_name = ["__client_accounts_", &idl_account_group.name.snake_case].concat();

        let account_item_struct_fields = root_items.find_map(|item| {
            let account_mod_item = match item {
                syn::Item::Mod(item_mod) if item_mod.ident == account_mod_name => item_mod,
                _ => None?,
            };
            let account_item_struct = account_mod_item.content?.1.into_iter().find_map(|item| {
                match item {
                    syn::Item::Struct(item_struct) if item_struct.ident == account_struct_name => Some(item_struct),
                    _ => None,
                }
            })?;
            let fields = match account_item_struct.fields {
                syn::Fields::Named(fields_named) => fields_named.named,
                syn::Fields::Unit => syn::punctuated::Punctuated::new(),
                syn::Fields::Unnamed(_) => None?, 
            };
            Some(fields.into_iter())
        }).ok_or_else(|| Error::MissingOrInvalidProgramItems("account module + struct"))?;

        let accounts = account_item_struct_fields.map(|field| {
            let account_name = field.ident.unwrap().to_string();
            let account_id_type = field.ty.into_token_stream().to_string();
            (account_name, account_id_type)
        }).collect::<Vec<_>>();
        group_name_with_accounts.insert(account_struct_name.clone(), accounts.clone());
        idl_account_group.accounts = accounts;
    }

    // ------ // ------

    Ok(IdlProgram { 
        name: IdlName {
            upper_camel_case: name.to_upper_camel_case(),
            snake_case: name,
        },
        id: program_id_bytes.into_token_stream().to_string(),
        instruction_account_pairs, 
    })
}
