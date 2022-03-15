use crate::{
    account::{AccountFieldVisibility, AccountQueryBuilder, KeyedAccount},
    display::{
        DisplayAccountFormat, DisplayKeyedAccount, DisplayProgramFormat, DisplayUpgradeableProgram,
    },
    error::{ExplorerError, Result},
    program::ProgramFieldVisibility,
};
use console::style;
use pretty_hex::*;
use solana_sdk::{
    account_utils::StateMut, bpf_loader, bpf_loader_deprecated, bpf_loader_upgradeable,
    bpf_loader_upgradeable::UpgradeableLoaderState,
};
use std::fmt::Write;

pub fn get_account_string(
    account: &KeyedAccount,
    visibility: &AccountFieldVisibility,
    format: DisplayAccountFormat,
) -> Result<String> {
    let data = account.account.data.clone();
    let account = DisplayKeyedAccount::from_keyed_account(account);

    let account_string = match format {
        DisplayAccountFormat::Trdelnik => {
            let mut account_string = format!("{}", account);
            if !data.is_empty() {
                writeln!(&mut account_string)?;
                writeln!(&mut account_string, "{}:", style("Data Dump:").bold())?;
                writeln!(&mut account_string, "{:?}", data.hex_dump())?;
            }
            account_string
        }
        DisplayAccountFormat::JSONPretty => serde_json::to_string_pretty(&account)?,
        DisplayAccountFormat::JSON => serde_json::to_string(&account)?,
    };
    Ok(account_string)
}

pub async fn get_program_string(
    program_id: &KeyedAccount,
    visibility: &ProgramFieldVisibility,
    format: DisplayProgramFormat,
) -> Result<String> {
    if program_id.account.owner == bpf_loader::id()
        || program_id.account.owner == bpf_loader_deprecated::id()
    {
        Ok(get_account_string(
            program_id,
            &AccountFieldVisibility::new_all_enabled(),
            DisplayAccountFormat::Trdelnik,
        )?)
    } else if program_id.account.owner == bpf_loader_upgradeable::id() {
        if let Ok(UpgradeableLoaderState::Program {
            programdata_address,
        }) = program_id.account.state()
        {
            if let Ok(programdata_account) = AccountQueryBuilder::with_pubkey(programdata_address)
                .build()
                .fetch_one()
                .await
            {
                let programdata_account = programdata_account.account;
                if let Ok(UpgradeableLoaderState::ProgramData {
                    upgrade_authority_address,
                    slot,
                }) = programdata_account.state()
                {
                    let program = DisplayUpgradeableProgram {
                        program_id: program_id.pubkey.to_string(),
                        owner: program_id.account.owner.to_string(),
                        programdata_address: programdata_address.to_string(),
                        authority: upgrade_authority_address
                            .map(|pubkey| pubkey.to_string())
                            .unwrap_or_else(|| "none".to_string()),
                        last_deploy_slot: slot,
                        data_len: programdata_account.data.len()
                            - UpgradeableLoaderState::programdata_data_offset()?,
                        lamports: programdata_account.lamports,
                    };
                    Ok(format!("{}", program))
                } else {
                    Err(ExplorerError::Custom(format!(
                        "Program {} has been closed",
                        program_id.pubkey
                    )))
                }
            } else {
                Err(ExplorerError::Custom(format!(
                    "Program {} has been closed",
                    program_id.pubkey
                )))
            }
        } else {
            Err(ExplorerError::Custom(format!(
                "{} is not a Program account",
                program_id.pubkey
            )))
        }
    } else {
        Ok(format!(
            "{} is not a pubkey of an on-chain BPF program.",
            program_id.pubkey
        ))
    }
}
