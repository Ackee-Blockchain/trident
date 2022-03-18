use crate::{
    account::{AccountFieldVisibility, AccountQueryBuilder, KeyedAccount},
    display::{
        writeln_styled, AccountDisplayFormat, DisplayKeyedAccount, DisplayUpgradeableProgram,
        ProgramDisplayFormat,
    },
    error::{ExplorerError, Result},
    program::ProgramFieldVisibility,
};
use pretty_hex::*;
use solana_sdk::{
    account_utils::StateMut, bpf_loader, bpf_loader_deprecated, bpf_loader_upgradeable,
    bpf_loader_upgradeable::UpgradeableLoaderState,
};
use std::fmt::Write;

pub fn get_account_string(
    account: &KeyedAccount,
    _visibility: &AccountFieldVisibility,
    format: AccountDisplayFormat,
) -> Result<String> {
    let data = account.account.data.clone();
    let account = DisplayKeyedAccount::from_keyed_account(account);

    let mut account_string = format.formatted_account_string(&account)?;

    if let AccountDisplayFormat::Trdelnik = format {
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
    format: ProgramDisplayFormat,
) -> Result<String> {
    if program.account.owner == bpf_loader::id()
        || program.account.owner == bpf_loader_deprecated::id()
    {
        let mapped_format = match format {
            ProgramDisplayFormat::Trdelnik => AccountDisplayFormat::Trdelnik,
            ProgramDisplayFormat::JSONPretty => AccountDisplayFormat::JSONPretty,
            ProgramDisplayFormat::JSON => AccountDisplayFormat::JSON,
        };

        // nothing interesting, we can return the account string
        Ok(get_account_string(
            program,
            &AccountFieldVisibility::new_all_enabled(),
            mapped_format,
        )?)
    } else if program.account.owner == bpf_loader_upgradeable::id() {
        if let Ok(UpgradeableLoaderState::Program {
            programdata_address,
        }) = program.account.state()
        {
            if let Ok(programdata_account) = AccountQueryBuilder::with_pubkey(programdata_address)
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
                    let mut program_string = format.formatted_program_string(&program)?;
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
