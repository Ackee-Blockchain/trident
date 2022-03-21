use crate::{
    account::{AccountFieldVisibility, AccountQueryBuilder, DisplayKeyedAccount, KeyedAccount},
    config::ExplorerConfig,
    display::DisplayFormat,
    error::{ExplorerError, Result},
    program::{DisplayUpgradeableProgram, ProgramFieldVisibility},
    transaction::{
        DisplayRawTransaction, RawTransactionFieldVisibility, TransactionFieldVisibility, DisplayTransaction,
    },
};
use console::style;
use pretty_hex::*;
use solana_client::rpc_config::RpcTransactionConfig;
use solana_sdk::{
    account_utils::StateMut, bpf_loader, bpf_loader_deprecated, bpf_loader_upgradeable,
    bpf_loader_upgradeable::UpgradeableLoaderState, commitment_config::CommitmentConfig,
    message::Message, native_token, pubkey::Pubkey, signature::Signature,
};
use solana_transaction_status::UiTransactionEncoding;
use std::fmt::{self, Write};

pub fn pretty_lamports_to_sol(lamports: u64) -> String {
    let sol_str = format!("{:.9}", native_token::lamports_to_sol(lamports));
    sol_str
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

pub fn classify_account(message: &Message, index: usize) -> String {
    let mut account_type = String::new();
    let mut started = false;
    if index == 0 {
        account_type.push_str("[Fee Payer]");
        started = true;
    }
    if message.is_writable(index) {
        if started {
            account_type.push(' ');
        }
        account_type.push_str("[Writable]");
        started = true;
    }
    if message.is_signer(index) {
        if started {
            account_type.push(' ');
        }
        account_type.push_str("[Signer]");
        started = true;
    }
    if message.maybe_executable(index) {
        if started {
            account_type.push(' ');
        }
        account_type.push_str("[Program]");
    }
    account_type
}

pub fn write_styled(f: &mut dyn fmt::Write, name: &str, value: &str) -> fmt::Result {
    let styled_value = if value.is_empty() {
        style("(not set)").italic()
    } else {
        style(value)
    };
    write!(f, "{} {}", style(name).bold(), styled_value)
}

pub fn writeln_styled(f: &mut dyn fmt::Write, name: &str, value: &str) -> fmt::Result {
    let styled_value = if value.is_empty() {
        style("(not set)").italic()
    } else {
        style(value)
    };
    writeln!(f, "{} {}", style(name).bold(), styled_value)
}

pub async fn print_account(
    pubkey: &Pubkey,
    visibility: &AccountFieldVisibility,
    format: DisplayFormat,
) -> Result<()> {
    let query = AccountQueryBuilder::with_pubkey(pubkey).build();
    let account = query.fetch_one().await?;
    let result = get_account_string(&account, visibility, format)?;
    println!("{}", result);
    Ok(())
}

pub async fn print_program(
    program_id: &Pubkey,
    visibility: &ProgramFieldVisibility,
    format: DisplayFormat,
) -> Result<()> {
    let query = AccountQueryBuilder::with_pubkey(program_id).build();
    let account = query.fetch_one().await?;
    let result = get_program_string(&account, visibility, format).await?;
    println!("{}", result);
    Ok(())
}

pub async fn print_raw_transaction(
    signature: &Signature,
    visibility: &RawTransactionFieldVisibility,
    format: DisplayFormat,
) -> Result<()> {
    let result = get_raw_transaction_string(signature, visibility, format).await?;
    println!("{}", result);
    Ok(())
}

pub async fn print_transaction(
    signature: &Signature,
    visibility: &TransactionFieldVisibility,
    format: DisplayFormat,
) -> Result<()> {
    let result = get_transaction_string(signature, visibility, format).await?;
    println!("{}", result);
    Ok(())
}

pub fn get_account_string(
    account: &KeyedAccount,
    _visibility: &AccountFieldVisibility,
    format: DisplayFormat,
) -> Result<String> {
    let data = account.account.data.clone();
    let account = DisplayKeyedAccount::from_keyed_account(account);

    let mut account_string = format.formatted_string(&account)?;

    if let DisplayFormat::Cli = format {
        if !data.is_empty() {
            writeln!(&mut account_string)?; // newline
            writeln!(&mut account_string)?; // newline

            writeln_styled(
                &mut account_string,
                "Raw Account Data:",
                &format!("{} bytes", data.len()),
            )?;
            // Show hexdump of not more than MAX_BYTES_SHOWN bytes
            const MAX_BYTES_SHOWN: usize = 64;
            let len = data.len();
            let (end, finished) = if MAX_BYTES_SHOWN > len {
                (len, true)
            } else {
                (MAX_BYTES_SHOWN, false)
            };
            let raw_account_data = &data[..end];
            let cfg = HexConfig {
                title: false,
                width: 16,
                group: 0,
                chunk: 2,
                ..HexConfig::default()
            };
            writeln!(&mut account_string, "{:?}", raw_account_data.hex_conf(cfg))?;
            if !finished {
                write!(&mut account_string, "... (skipped)")?;
            }
        }
    };

    Ok(account_string)
}

pub async fn get_program_string(
    program: &KeyedAccount,
    _visibility: &ProgramFieldVisibility,
    format: DisplayFormat,
) -> Result<String> {
    if program.account.owner == bpf_loader::id()
        || program.account.owner == bpf_loader_deprecated::id()
    {
        // nothing interesting, we can return the account string
        Ok(get_account_string(
            program,
            &AccountFieldVisibility::new_all_enabled(),
            format,
        )?)
    } else if program.account.owner == bpf_loader_upgradeable::id() {
        if let Ok(UpgradeableLoaderState::Program {
            programdata_address,
        }) = program.account.state()
        {
            if let Ok(programdata_account) = AccountQueryBuilder::with_pubkey(&programdata_address)
                .build()
                .fetch_one()
                .await
            {
                if let Ok(UpgradeableLoaderState::ProgramData {
                    upgrade_authority_address,
                    slot,
                }) = programdata_account.account.state()
                {
                    let program = DisplayUpgradeableProgram::from(
                        program,
                        &programdata_account,
                        slot,
                        &upgrade_authority_address,
                    );
                    let mut program_string = format.formatted_string(&program)?;
                    writeln!(&mut program_string)?;
                    writeln!(&mut program_string)?;
                    writeln_styled(
                        &mut program_string,
                        "Followed by Raw Program Data (program.so):",
                        &format!(
                            "{} bytes",
                            program
                                .programdata_account
                                .data
                                .raw_program_data_following_in_bytes
                        ),
                    )?;

                    // Show hexdump of not more than MAX_BYTES_SHOWN bytes
                    const MAX_BYTES_SHOWN: usize = 64;
                    let len = programdata_account.account.data.len();
                    let offset = UpgradeableLoaderState::programdata_data_offset().unwrap();
                    let (end, finished) = if offset + MAX_BYTES_SHOWN > len {
                        (len, true)
                    } else {
                        (offset + MAX_BYTES_SHOWN, false)
                    };
                    let raw_program_data = &programdata_account.account.data[offset..end];
                    let cfg = HexConfig {
                        title: false,
                        width: 16,
                        group: 0,
                        chunk: 2,
                        ..HexConfig::default()
                    };
                    write!(&mut program_string, "{:?}", raw_program_data.hex_conf(cfg))?;
                    if !finished {
                        writeln!(&mut program_string)?;
                        write!(&mut program_string, "... (skipped)")?;
                    }

                    Ok(program_string)
                } else {
                    Err(ExplorerError::Custom(format!(
                        "Program {} has been closed",
                        program.pubkey
                    )))
                }
            } else {
                Err(ExplorerError::Custom(format!(
                    "Program {} has been closed",
                    program.pubkey
                )))
            }
        } else {
            Err(ExplorerError::Custom(format!(
                "{} is not a Program account",
                program.pubkey
            )))
        }
    } else {
        Err(ExplorerError::Custom(format!(
            "{} is not a pubkey of an on-chain BPF program.",
            program.pubkey
        )))
    }
}

pub async fn get_raw_transaction_string(
    signature: &Signature,
    _visibility: &RawTransactionFieldVisibility,
    format: DisplayFormat,
) -> Result<String> {
    let explorer_config = ExplorerConfig::default();
    let rpc_client = explorer_config.rpc_client();
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Json),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    let confirmed_transaction = rpc_client
        .get_transaction_with_config(signature, config)
        .await?;

    let display_transaction = DisplayRawTransaction::from(signature, &confirmed_transaction)?;

    let transaction_string = format.formatted_string(&display_transaction)?;

    Ok(transaction_string)
}

pub async fn get_transaction_string(
    signature: &Signature,
    _visibility: &TransactionFieldVisibility,
    format: DisplayFormat,
) -> Result<String> {
    let explorer_config = ExplorerConfig::default();
    let rpc_client = explorer_config.rpc_client();
    let config = RpcTransactionConfig {
        encoding: Some(UiTransactionEncoding::Binary),
        commitment: Some(CommitmentConfig::confirmed()),
        max_supported_transaction_version: Some(0),
    };

    let confirmed_transaction = rpc_client
        .get_transaction_with_config(signature, config)
        .await?;

    let display_transaction = DisplayTransaction::from(signature, &confirmed_transaction)?;

    let transaction_string = format.formatted_string(&display_transaction)?;

    Ok(transaction_string)
}
