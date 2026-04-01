use std::fmt;

use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;
use trident_svm::prelude::TridentTransactionResult;

use crate::trident::progress;
use crate::AccountDiscriminator;

use super::Trident;

fn colors_enabled() -> bool {
    use std::io::IsTerminal;
    std::io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none()
}

fn cyan(text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[36m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn dim(text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[2m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn green(text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[32m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn red(text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[31m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn yellow(text: &str) -> String {
    if colors_enabled() {
        format!("\x1b[33m{}\x1b[0m", text)
    } else {
        text.to_string()
    }
}

fn short_pubkey(key: &Pubkey) -> String {
    let s = key.to_string();
    if s.len() > 11 {
        format!("{}..{}", &s[..4], &s[s.len() - 4..])
    } else {
        s
    }
}

fn header(title: &str, key: &Pubkey) -> String {
    let label = format!("{} ({})", title, short_pubkey(key));
    let bar_len = 50usize.saturating_sub(label.len() + 3);
    let bar = "─".repeat(bar_len);
    dim(&format!("── {} {}", label, bar))
}

fn header_no_key(title: &str) -> String {
    let bar_len = 50usize.saturating_sub(title.len() + 3);
    let bar = "─".repeat(bar_len);
    dim(&format!("── {} {}", title, bar))
}

impl Trident {
    /// Prints a formatted transaction result including status, compute units, and logs.
    ///
    /// Output is routed through the progress bar channel in parallel mode
    /// so it never mixes with the progress bar.
    ///
    /// # Example
    /// ```rust,ignore
    /// let result = self.trident.process_transaction(&[ix], Some("transfer"));
    /// self.trident.print_transaction_result(&result);
    /// ```
    pub fn print_transaction_result(&self, result: &TridentTransactionResult) {
        let mut out = String::new();
        out.push_str(&header_no_key("Transaction Result"));
        out.push('\n');

        let status_str = if result.is_success() {
            green("OK")
        } else {
            let err = match result.status() {
                Ok(()) => "unknown".to_string(),
                Err(e) => e.to_string(),
            };
            red(&format!("FAILED: {}", err))
        };
        out.push_str(&format!("  Status:    {}\n", status_str));
        out.push_str(&format!(
            "  CU used:   {}\n",
            result.compute_units_consumed()
        ));
        out.push_str(&format!(
            "  Timestamp: {}\n",
            result.transaction_timestamp()
        ));

        let logs = result.logs();
        if !logs.is_empty() {
            out.push_str(&format!("  {}\n", dim("Logs:")));
            for line in logs.lines() {
                out.push_str(&format!("    {}\n", dim(line)));
            }
        }

        progress::send_user_log(out);
    }

    /// Prints a deserialized program account with colored formatting.
    ///
    /// Fetches the account, deserializes it as type `T`, and pretty-prints
    /// the `Debug` output. If the account doesn't exist or deserialization
    /// fails, prints an error message instead.
    ///
    /// # Example
    /// ```rust,ignore
    /// self.trident.print_account::<MyAccountType>(&account_key, Some(8));
    /// ```
    pub fn print_account<T: BorshDeserialize + AccountDiscriminator + fmt::Debug>(
        &mut self,
        key: &Pubkey,
        discriminator_size_override: Option<usize>,
    ) {
        let mut out = String::new();
        out.push_str(&header("Account", key));
        out.push('\n');

        match self.get_account_with_type::<T>(key, discriminator_size_override) {
            Some(account) => {
                let formatted = format!("{:#?}", account);
                for line in formatted.lines() {
                    out.push_str(&format!("  {}\n", cyan(line)));
                }
            }
            None => {
                out.push_str(&format!(
                    "  {}\n",
                    yellow("Account not found or deserialization failed")
                ));
            }
        }

        progress::send_user_log(out);
    }

    /// Prints raw account metadata without deserialization.
    ///
    /// Shows owner, lamports (with SOL conversion), data length, and
    /// executable flag. Works for any account — wallets, PDAs, unknown
    /// layouts, or accounts with no data at all.
    ///
    /// # Example
    /// ```rust,ignore
    /// self.trident.print_raw_account(&some_pubkey);
    /// ```
    pub fn print_raw_account(&self, key: &Pubkey) {
        use solana_sdk::account::ReadableAccount;

        let account = self.get_account(key);
        let mut out = String::new();
        out.push_str(&header("Account", key));
        out.push('\n');

        if account.data().is_empty() && account.lamports() == 0 {
            out.push_str(&format!("  {}\n", yellow("Account not found")));
            progress::send_user_log(out);
            return;
        }

        out.push_str(&format!(
            "  Owner:      {}\n",
            cyan(&account.owner().to_string())
        ));

        let lamports = account.lamports();
        let sol = lamports as f64 / 1_000_000_000.0;
        if sol >= 0.001 {
            out.push_str(&format!(
                "  Lamports:   {} {}\n",
                cyan(&lamports.to_string()),
                dim(&format!("({:.4} SOL)", sol)),
            ));
        } else {
            out.push_str(&format!("  Lamports:   {}\n", cyan(&lamports.to_string())));
        }

        out.push_str(&format!(
            "  Data:       {} bytes\n",
            cyan(&account.data().len().to_string())
        ));
        out.push_str(&format!(
            "  Executable: {}\n",
            cyan(&account.executable().to_string())
        ));

        progress::send_user_log(out);
    }

    /// Prints program account details with colored formatting.
    ///
    /// Automatically detects the loader type (v3 upgradeable, v4, v2, native)
    /// and prints the relevant metadata. For v3 programs, fetches and prints
    /// both the program account and its ProgramData account (authority, slot).
    /// Binary data is never printed.
    ///
    /// # Example
    /// ```rust,ignore
    /// self.trident.print_program(&program_id);
    /// ```
    pub fn print_program(&mut self, key: &Pubkey) {
        use solana_sdk::account::ReadableAccount;

        let account = self.get_account(key);
        let mut out = String::new();
        out.push_str(&header("Program", key));
        out.push('\n');

        if account.data().is_empty() && account.lamports() == 0 {
            out.push_str(&format!("  {}\n", yellow("Account not found")));
            progress::send_user_log(out);
            return;
        }

        let owner = account.owner();
        out.push_str(&format!("  Owner:      {}\n", cyan(&owner.to_string())));
        out.push_str(&format!(
            "  Executable: {}\n",
            cyan(&account.executable().to_string())
        ));
        out.push_str(&format!(
            "  Lamports:   {}\n",
            cyan(&account.lamports().to_string())
        ));
        out.push_str(&format!(
            "  Data:       {} bytes\n",
            cyan(&account.data().len().to_string())
        ));

        if *owner == solana_sdk::bpf_loader_upgradeable::ID {
            self.print_program_v3(&account, &mut out);
        } else if *owner == solana_sdk::loader_v4::ID {
            Self::print_program_v4(&account, &mut out);
        } else if *owner == solana_sdk::bpf_loader::ID {
            out.push_str(&format!(
                "  Loader:     {}\n",
                cyan("BPF Loader v2 (non-upgradeable)")
            ));
            out.push_str(&format!(
                "  ELF:        {} bytes {}\n",
                cyan(&account.data().len().to_string()),
                dim("(not shown)"),
            ));
        } else if *owner == solana_sdk::native_loader::ID {
            out.push_str(&format!(
                "  Loader:     {}\n",
                cyan("Native Loader (built-in)")
            ));
        } else {
            out.push_str(&format!(
                "  {}\n",
                yellow(&format!("Unknown program owner: {}", owner)),
            ));
        }

        progress::send_user_log(out);
    }

    fn print_program_v3(
        &mut self,
        account: &solana_sdk::account::AccountSharedData,
        out: &mut String,
    ) {
        use solana_loader_v3_interface::state::UpgradeableLoaderState;
        use solana_sdk::account::ReadableAccount;

        out.push_str(&format!(
            "  Loader:     {}\n",
            cyan("BPF Loader v3 (upgradeable)")
        ));

        let state: Result<UpgradeableLoaderState, _> = bincode::deserialize(account.data());
        match state {
            Ok(UpgradeableLoaderState::Program {
                programdata_address,
            }) => {
                out.push_str(&format!(
                    "  ProgramData: {}\n",
                    cyan(&programdata_address.to_string()),
                ));

                let pd_account = self.get_account(&programdata_address);
                if pd_account.data().is_empty() {
                    out.push_str(&format!("  {}\n", yellow("ProgramData account not found"),));
                    return;
                }

                out.push('\n');
                out.push_str(&format!(
                    "  {}\n",
                    dim(&format!(
                        "── ProgramData ({}) ──",
                        short_pubkey(&programdata_address),
                    ))
                ));

                let pd_state: Result<UpgradeableLoaderState, _> =
                    bincode::deserialize(pd_account.data());
                match pd_state {
                    Ok(UpgradeableLoaderState::ProgramData {
                        slot,
                        upgrade_authority_address,
                    }) => {
                        out.push_str(&format!("  Slot:       {}\n", cyan(&slot.to_string())));
                        let auth_str = match upgrade_authority_address {
                            Some(a) => a.to_string(),
                            None => "None (immutable)".to_string(),
                        };
                        out.push_str(&format!("  Authority:  {}\n", cyan(&auth_str)));
                        const PROGRAM_DATA_METADATA_SIZE: usize = 45;
                        let elf_len = pd_account
                            .data()
                            .len()
                            .saturating_sub(PROGRAM_DATA_METADATA_SIZE);
                        out.push_str(&format!(
                            "  ELF:        {} bytes {}\n",
                            cyan(&elf_len.to_string()),
                            dim("(not shown)"),
                        ));
                        out.push_str(&format!(
                            "  Lamports:   {}\n",
                            cyan(&pd_account.lamports().to_string())
                        ));
                    }
                    _ => {
                        out.push_str(&format!(
                            "  {}\n",
                            yellow("Failed to deserialize ProgramData state"),
                        ));
                    }
                }
            }
            Ok(UpgradeableLoaderState::Buffer { authority_address }) => {
                out.push_str(&format!("  Type:       {}\n", cyan("Buffer")));
                let auth_str = match authority_address {
                    Some(a) => a.to_string(),
                    None => "None".to_string(),
                };
                out.push_str(&format!("  Authority:  {}\n", cyan(&auth_str)));
            }
            Ok(UpgradeableLoaderState::Uninitialized) => {
                out.push_str(&format!("  Type:       {}\n", cyan("Uninitialized")));
            }
            _ => {
                out.push_str(&format!(
                    "  {}\n",
                    yellow("Failed to deserialize v3 loader state"),
                ));
            }
        }
    }

    fn print_program_v4(account: &solana_sdk::account::AccountSharedData, out: &mut String) {
        use solana_sdk::account::ReadableAccount;
        use solana_sdk::loader_v4::LoaderV4State;
        use solana_sdk::loader_v4::LoaderV4Status;

        out.push_str(&format!("  Loader:     {}\n", cyan("Loader v4")));

        let data = account.data();
        let header_size = LoaderV4State::program_data_offset();
        if data.len() < header_size {
            out.push_str(&format!(
                "  {}\n",
                yellow("Account data too small for LoaderV4State header"),
            ));
            return;
        }

        // SAFETY: LoaderV4State is #[repr(C)], Copy, and we verified the buffer is large enough.
        let state: &LoaderV4State = unsafe { &*(data.as_ptr() as *const LoaderV4State) };

        out.push_str(&format!(
            "  Slot:       {}\n",
            cyan(&state.slot.to_string())
        ));

        let status_str = match state.status {
            LoaderV4Status::Retracted => "Retracted (maintenance)",
            LoaderV4Status::Deployed => "Deployed",
            LoaderV4Status::Finalized => "Finalized (immutable)",
        };
        out.push_str(&format!("  Status:     {}\n", cyan(status_str)));

        let auth_label = match state.status {
            LoaderV4Status::Finalized => "Next version",
            _ => "Authority",
        };
        out.push_str(&format!(
            "  {}:  {}\n",
            auth_label,
            cyan(&state.authority_address_or_next_version.to_string()),
        ));

        let elf_len = data.len().saturating_sub(header_size);
        out.push_str(&format!(
            "  ELF:        {} bytes {}\n",
            cyan(&elf_len.to_string()),
            dim("(not shown)"),
        ));
    }
}

#[cfg(feature = "token")]
use crate::trident::token2022::MintExtensionData;
#[cfg(feature = "token")]
use crate::trident::token2022::TokenAccountExtensionData;

#[cfg(feature = "token")]
fn pod_pubkey(p: &spl_pod::optional_keys::OptionalNonZeroPubkey) -> String {
    let key: Option<Pubkey> = Option::from(*p);
    match key {
        Some(k) => k.to_string(),
        None => "None".to_string(),
    }
}

#[cfg(feature = "token")]
fn format_token_extension(ext: &TokenAccountExtensionData) -> String {
    match ext {
        TokenAccountExtensionData::TransferFeeAmount(e) => {
            format!(
                "TransferFeeAmount: withheld = {}",
                u64::from(e.withheld_amount)
            )
        }
        TokenAccountExtensionData::ImmutableOwner(_) => "ImmutableOwner".to_string(),
        TokenAccountExtensionData::NonTransferableAccount(_) => {
            "NonTransferableAccount".to_string()
        }
        TokenAccountExtensionData::TransferHookAccount(_) => "TransferHookAccount".to_string(),
        TokenAccountExtensionData::PausableAccount(_) => "PausableAccount".to_string(),
        TokenAccountExtensionData::MemoTransfer(e) => {
            format!(
                "MemoTransfer: require_incoming = {}",
                bool::from(e.require_incoming_transfer_memos)
            )
        }
        TokenAccountExtensionData::CpiGuard(e) => {
            format!("CpiGuard: lock = {}", bool::from(e.lock_cpi))
        }
        TokenAccountExtensionData::Unknown(t) => format!("Unknown({:?})", t),
    }
}

#[cfg(feature = "token")]
fn format_mint_extension(ext: &MintExtensionData) -> String {
    match ext {
        MintExtensionData::TransferFeeConfig(e) => {
            let bp = u16::from(e.newer_transfer_fee.transfer_fee_basis_points);
            let max = u64::from(e.newer_transfer_fee.maximum_fee);
            let bp_pct = bp as f64 / 100.0;
            format!(
                "TransferFeeConfig: {}% ({} bps), max_fee = {}",
                bp_pct, bp, max
            )
        }
        MintExtensionData::MintCloseAuthority(e) => {
            format!("MintCloseAuthority: {}", pod_pubkey(&e.close_authority))
        }
        MintExtensionData::DefaultAccountState(e) => {
            let state = u8::from(e.state);
            let label = match state {
                0 => "Uninitialized",
                1 => "Initialized",
                2 => "Frozen",
                _ => "Unknown",
            };
            format!("DefaultAccountState: {} ({})", label, state)
        }
        MintExtensionData::NonTransferable(_) => "NonTransferable".to_string(),
        MintExtensionData::InterestBearingConfig(e) => {
            format!(
                "InterestBearingConfig: rate = {} bps",
                i16::from(e.current_rate)
            )
        }
        MintExtensionData::PermanentDelegate(e) => {
            format!("PermanentDelegate: {}", pod_pubkey(&e.delegate))
        }
        MintExtensionData::TransferHook(e) => {
            format!("TransferHook: program = {}", pod_pubkey(&e.program_id))
        }
        MintExtensionData::MetadataPointer(e) => {
            format!("MetadataPointer: {}", pod_pubkey(&e.metadata_address))
        }
        MintExtensionData::GroupPointer(e) => {
            format!("GroupPointer: {}", pod_pubkey(&e.group_address))
        }
        MintExtensionData::GroupMemberPointer(e) => {
            format!("GroupMemberPointer: {}", pod_pubkey(&e.member_address))
        }
        MintExtensionData::ScaledUiAmount(e) => {
            format!("ScaledUiAmount: multiplier = {}", f64::from(e.multiplier))
        }
        MintExtensionData::Pausable(_) => "Pausable".to_string(),
        MintExtensionData::TokenMetadata(e) => {
            let mut s = format!(
                "TokenMetadata: name = \"{}\", symbol = \"{}\"",
                e.name, e.symbol
            );
            if !e.uri.is_empty() {
                s.push_str(&format!(", uri = \"{}\"", e.uri));
            }
            let auth: Option<Pubkey> = Option::from(e.update_authority);
            if let Some(a) = auth {
                s.push_str(&format!(", update_authority = {}", a));
            }
            s
        }
        MintExtensionData::TokenGroup(e) => {
            format!(
                "TokenGroup: size = {}/{}",
                u64::from(e.size),
                u64::from(e.max_size)
            )
        }
        MintExtensionData::TokenGroupMember(e) => {
            format!(
                "TokenGroupMember: group = {}, member_number = {}",
                e.group,
                u64::from(e.member_number)
            )
        }
        MintExtensionData::Unknown(t) => format!("Unknown({:?})", t),
    }
}

#[cfg(feature = "token")]
impl Trident {
    /// Prints a token account with colored formatting showing mint, owner, amount,
    /// delegate, state, and any Token-2022 extensions.
    ///
    /// Fetches the token account (works with both SPL Token and Token-2022)
    /// and prints the key fields. If the account doesn't exist or isn't a
    /// valid token account, prints an error message.
    ///
    /// # Example
    /// ```rust,ignore
    /// self.trident.print_token_account(token_account_pubkey);
    /// ```
    pub fn print_token_account(&mut self, account: Pubkey) {
        let mut out = String::new();
        out.push_str(&header("Token Account", &account));
        out.push('\n');

        match self.get_token_account(account) {
            Ok(token_acc) => {
                let acc = &token_acc.account;
                out.push_str(&format!("  Mint:      {}\n", cyan(&acc.mint.to_string())));
                out.push_str(&format!("  Owner:     {}\n", cyan(&acc.owner.to_string())));
                out.push_str(&format!(
                    "  Amount:    {}\n",
                    green(&acc.amount.to_string())
                ));

                let delegate_str = match acc.delegate {
                    solana_sdk::program_option::COption::Some(d) => {
                        format!("{} (delegated: {})", d, acc.delegated_amount)
                    }
                    solana_sdk::program_option::COption::None => "None".to_string(),
                };
                out.push_str(&format!("  Delegate:  {}\n", dim(&delegate_str)));

                let state_str = format!("{:?}", acc.state);
                out.push_str(&format!("  State:     {}\n", dim(&state_str)));

                if !token_acc.extensions.is_empty() {
                    out.push_str(&format!(
                        "  {} ({})\n",
                        cyan("Extensions"),
                        token_acc.extensions.len()
                    ));
                    for ext in &token_acc.extensions {
                        out.push_str(&format!("    {}\n", dim(&format_token_extension(ext))));
                    }
                }
            }
            Err(e) => {
                out.push_str(&format!(
                    "  {}\n",
                    yellow(&format!("Failed to read token account: {:?}", e))
                ));
            }
        }

        progress::send_user_log(out);
    }

    /// Prints a mint account with colored formatting showing supply, decimals,
    /// authorities, and any Token-2022 extensions.
    ///
    /// Fetches the mint (works with both SPL Token and Token-2022)
    /// and prints the key fields. If the account doesn't exist or isn't a
    /// valid mint, prints an error message.
    ///
    /// # Example
    /// ```rust,ignore
    /// self.trident.print_mint_account(mint_pubkey);
    /// ```
    pub fn print_mint_account(&mut self, account: Pubkey) {
        let mut out = String::new();
        out.push_str(&header("Mint", &account));
        out.push('\n');

        match self.get_mint(account) {
            Ok(mint_data) => {
                let m = &mint_data.mint;

                let authority_str = match m.mint_authority {
                    solana_sdk::program_option::COption::Some(a) => a.to_string(),
                    solana_sdk::program_option::COption::None => "None (fixed supply)".to_string(),
                };
                out.push_str(&format!("  Authority: {}\n", cyan(&authority_str)));
                out.push_str(&format!("  Supply:    {}\n", green(&m.supply.to_string())));
                out.push_str(&format!("  Decimals:  {}\n", cyan(&m.decimals.to_string())));

                let freeze_str = match m.freeze_authority {
                    solana_sdk::program_option::COption::Some(a) => a.to_string(),
                    solana_sdk::program_option::COption::None => "None".to_string(),
                };
                out.push_str(&format!("  Freeze:    {}\n", dim(&freeze_str)));

                if !mint_data.extensions.is_empty() {
                    out.push_str(&format!(
                        "  {} ({})\n",
                        cyan("Extensions"),
                        mint_data.extensions.len()
                    ));
                    for ext in &mint_data.extensions {
                        out.push_str(&format!("    {}\n", dim(&format_mint_extension(ext))));
                    }
                }
            }
            Err(e) => {
                out.push_str(&format!(
                    "  {}\n",
                    yellow(&format!("Failed to read mint account: {:?}", e))
                ));
            }
        }

        progress::send_user_log(out);
    }
}
