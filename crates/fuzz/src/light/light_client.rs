use std::cell::RefCell;
use std::collections::HashMap;
use std::mem::{size_of, transmute};

use solana_program::clock::Clock;
use solana_program::entrypoint::ProcessInstruction;
use solana_program::sysvar::{clock, rent};
use solana_program::{program_pack::Pack, rent::Rent};
use solana_sdk::account::AccountSharedData;
use solana_sdk::clock::Epoch;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::{
    account::Account, account_info::AccountInfo, instruction::Instruction, program_option::COption,
    pubkey::Pubkey, signature::Keypair, signer::Signer,
};
use spl_token::state::Mint;

use super::program_stubs::test_syscall_stubs;
use super::serialization_light::{
    deserialize_custom, get_duplicate_accounts, serialize_accounts_custom,
};
use crate::error::*;
use crate::fuzz_client::{FuzzClient, ProgramEntry};

#[repr(C)]
#[derive(Clone, Debug, Default)]

pub struct TridentAccount {
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: Epoch,
}

#[allow(clippy::too_many_arguments)]
impl TridentAccount {
    pub fn new(lamports: u64, space: usize, owner: &Pubkey) -> Self {
        Self {
            lamports,
            data: vec![0u8; space],
            owner: *owner,
            executable: false,
            rent_epoch: Epoch::default(),
        }
    }
    pub fn set_data_from_slice(&mut self, new_data: &[u8]) {
        self.data.copy_from_slice(new_data);
    }
    pub fn realloc(&mut self, newsize: usize) {
        self.data.resize(newsize, 0);
    }
}

impl From<AccountSharedData> for TridentAccount {
    fn from(value: AccountSharedData) -> Self {
        let account = Account::from(value);
        TridentAccount {
            lamports: account.lamports,
            data: account.data,
            owner: account.owner,
            executable: account.executable,
            rent_epoch: account.rent_epoch,
        }
    }
}

thread_local! {
    /// Static pointer to LightClient so that we can access it from system program stubs
    pub static LIGHT_CLIENT: RefCell<Option<usize>> = RefCell::new(None);
}

/// Sets the pointer to LightClient. This is necessary in order to access LightClient using 'get_light_client' method
fn set_light_client(new: &LightClient) {
    LIGHT_CLIENT.with(|client| unsafe { client.replace(Some(transmute::<_, usize>(new))) });
}

/// Gets the pointer to LightClient
pub(crate) fn get_light_client<'a>() -> &'a LightClient {
    let ptr = LIGHT_CLIENT.with(|client| match *client.borrow() {
        Some(val) => val,
        None => panic!("Light client not set! Maybe you have to call init() first."),
    });
    unsafe { transmute::<usize, &LightClient>(ptr) }
}

pub struct LightClient {
    pub account_storage: HashMap<Pubkey, TridentAccount>,
    pub programs: HashMap<Pubkey, ProcessInstruction>,
}

pub struct FuzzingProgramLight {
    pub program_name: String,
    pub program_id: Pubkey,
    pub entry: Option<ProgramEntry>,
}
impl FuzzingProgramLight {
    pub fn new(
        program_name: &str,
        program_id: &Pubkey,
        entry_fn: Option<ProgramEntry>,
    ) -> FuzzingProgramLight {
        Self {
            program_name: program_name.to_string(),
            program_id: *program_id,
            entry: entry_fn,
        }
    }
}

impl LightClient {
    pub fn new(program_: &[FuzzingProgramLight]) -> Result<Self, FuzzClientError> {
        let mut new_client = Self {
            account_storage: HashMap::new(),
            programs: HashMap::new(),
        };
        new_client.add_system_program();
        for x in program_ {
            match x.entry {
                Some(entry) => {
                    new_client.add_program2(x.program_id, entry);
                }
                None => return Err(FuzzClientError::NotImplemnted),
            }
        }
        new_client.add_program(spl_token::ID, spl_token::processor::Processor::process);
        // new_client.add_program(
        //     anchor_spl::token_2022::ID,
        //     spl_token_2022::processor::Processor::process,
        // );
        new_client.add_program(
            spl_associated_token_account::ID,
            spl_associated_token_account::processor::process_instruction,
        );
        new_client.add_rent()?;
        new_client.add_clock()?;

        // TODO remove this 0 and make it more sophisticated
        test_syscall_stubs(program_[0].program_id);

        Ok(new_client)
    }
    /// Initializes the LightClient before usage.
    pub fn init(&self) {
        set_light_client(self);
    }

    fn add_rent(&mut self) -> Result<(), FuzzClientError> {
        let rent = Rent::default();
        let size = size_of::<Rent>();
        let mut data = vec![0; size];
        bincode::serialize_into(&mut data[..], &rent)
            .map_err(|e| FuzzClientError::ClientInitError(e))?;

        let lamports = rent.minimum_balance(data.len());

        let mut account = TridentAccount::new(lamports, size, &solana_program::sysvar::id());

        account.set_data_from_slice(&data[..]);
        self.account_storage.insert(rent::id(), account);
        Ok(())
    }

    fn add_clock(&mut self) -> Result<(), FuzzClientError> {
        let clock = Clock::default();
        let rent = Rent::default();
        let size = size_of::<Clock>();
        let mut data = vec![0; size];
        bincode::serialize_into(&mut data[..], &clock)
            .map_err(|e| FuzzClientError::ClientInitError(e))?;

        let lamports = rent.minimum_balance(data.len());

        let mut account = TridentAccount::new(lamports, size, &solana_program::sysvar::id());

        account.set_data_from_slice(&data[..]);
        self.account_storage.insert(clock::id(), account);
        Ok(())
    }

    fn add_system_program(&mut self) {
        let rent = Rent::default().minimum_balance(0).max(1);
        let program = TridentAccount {
            executable: true,
            lamports: rent,
            ..Default::default()
        };
        self.account_storage
            .insert(solana_sdk::system_program::ID, program);
    }

    /// Add new arbitrary program to the client. Starting from Anchor 0.29.0, the entry point signature
    /// has more restrictive lifetime requirements. Use this method to add programs written in Anchor 0.29.0 and above.
    ///
    /// - `program_id` is the address of your program
    /// - `process_function` is the closure that will be called to enter the program and process instructions
    pub fn add_program2(&mut self, program_id: Pubkey, process_function: ProgramEntry) {
        let x = unsafe {
            transmute::<ProgramEntry, solana_sdk::entrypoint::ProcessInstruction>(process_function)
        };
        self.programs.insert(program_id, x);

        let rent = Rent::default().minimum_balance(0).max(1);
        let program = TridentAccount {
            executable: true,
            lamports: rent,
            ..Default::default()
        };
        self.account_storage.insert(program_id, program);
    }

    /// Add new arbitrary program to the client.
    ///
    /// HINT: Use the [`add_program2`] method for programs written in Anchor 0.29.0 and above.
    ///
    /// - `program_id` is the address of your program
    /// - `process_function` is the closure that will be called to enter the program and process instructions
    pub fn add_program(
        &mut self,
        program_id: Pubkey,
        process_function: solana_sdk::entrypoint::ProcessInstruction,
    ) {
        self.programs.insert(program_id, process_function);

        let rent = Rent::default().minimum_balance(0).max(1);
        let program = TridentAccount {
            executable: true,
            lamports: rent,
            ..Default::default()
        };
        self.account_storage.insert(program_id, program);
    }
    fn pre_ix_accounts(
        &self,
        instruction_accounts: &[AccountMeta],
    ) -> Vec<Option<&TridentAccount>> {
        instruction_accounts
            .iter()
            .map(|m| self.account_storage.get(&m.pubkey))
            .collect::<Vec<_>>()
    }
    fn post_ix_accounts_update(&mut self, instruction_account_infos: &[AccountInfo]) {
        for account in instruction_account_infos.iter() {
            if account.is_writable {
                if is_closed(account) {
                    // TODO if we remove the account, what about AccountStorage?
                    self.account_storage.remove(account.key);
                } else {
                    let account_data = self.account_storage.entry(*account.key).or_default();
                    account_data.data = account.data.borrow().to_vec();
                    account_data.lamports = account.lamports.borrow().to_owned();
                    account_data.owner = account.owner.to_owned();
                    // TODO check data can be resized
                    // TODO check lamports sum is constant
                }
            }
        }
    }
}

impl FuzzClient for LightClient {
    fn get_rent(&mut self) -> Result<Rent, FuzzClientError> {
        Ok(solana_sdk::rent::Rent::default())
    }
    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData) {
        self.account_storage
            .insert(*address, account.to_owned().into());
    }

    fn set_account(&mut self, lamports: u64) -> solana_sdk::signature::Keypair {
        let new_account = Keypair::new();

        let new_account_info = TridentAccount::new(lamports, 0, &solana_sdk::system_program::ID);

        self.account_storage
            .insert(new_account.pubkey(), new_account_info);
        new_account
    }

    fn set_token_account(
        &mut self,
        mint: solana_sdk::pubkey::Pubkey,
        owner: solana_sdk::pubkey::Pubkey,
        amount: u64,
        delegate: Option<solana_sdk::pubkey::Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<solana_sdk::pubkey::Pubkey>,
    ) -> solana_sdk::pubkey::Pubkey {
        let token_account_key = Keypair::new().pubkey();

        let delegate = match delegate {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let is_native = match is_native {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let close_authority = match close_authority {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let r = Rent::default();
        let lamports = r.minimum_balance(spl_token::state::Account::LEN);

        let mut account =
            TridentAccount::new(lamports, spl_token::state::Account::LEN, &spl_token::id());

        let token_account = spl_token::state::Account {
            mint,
            owner,
            amount,
            delegate,
            state: spl_token::state::AccountState::Initialized,
            is_native,
            delegated_amount,
            close_authority,
        };

        let mut data = vec![0u8; spl_token::state::Account::LEN];
        spl_token::state::Account::pack(token_account, &mut data[..]).unwrap();
        account.set_data_from_slice(&data);

        self.account_storage.insert(token_account_key, account);

        token_account_key
    }

    fn set_mint_account(
        &mut self,
        decimals: u8,
        owner: &solana_sdk::pubkey::Pubkey,
        freeze_authority: Option<solana_sdk::pubkey::Pubkey>,
    ) -> solana_sdk::pubkey::Pubkey {
        let mint_account_key = Keypair::new().pubkey();

        let authority = match freeze_authority {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let r = Rent::default();
        let lamports = r.minimum_balance(Mint::LEN);

        let mut account = TridentAccount::new(lamports, Mint::LEN, &spl_token::id());

        let mint = Mint {
            is_initialized: true,
            mint_authority: COption::Some(*owner),
            freeze_authority: authority,
            decimals,
            ..Default::default()
        };

        let mut data = vec![0u8; Mint::LEN];
        Mint::pack(mint, &mut data[..]).unwrap();
        account.set_data_from_slice(&data);
        self.account_storage.insert(mint_account_key, account);

        mint_account_key
    }

    fn payer(&self) -> solana_sdk::signature::Keypair {
        todo!()
    }

    fn get_account(
        &mut self,
        key: &solana_sdk::pubkey::Pubkey,
    ) -> Result<Option<solana_sdk::account::Account>, FuzzClientError> {
        let storage = &self.account_storage; //.borrow();
        match storage.get(key) {
            Some(account) => Ok(Some(Account {
                lamports: account.lamports,
                data: account.data.clone(),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            })),
            None => Ok(None),
        }
    }

    fn get_accounts(
        &mut self,
        metas: &[AccountMeta],
    ) -> Result<Vec<Option<Account>>, FuzzClientErrorWithOrigin> {
        let result: Vec<_> = metas
            .iter()
            .map(|m| {
                self.get_account(&m.pubkey)
                    .map_err(|e| e.with_origin(Origin::Account(m.pubkey)))
            })
            .collect();
        result.into_iter().collect()
    }

    fn get_last_blockhash(&self) -> solana_sdk::hash::Hash {
        todo!()
    }

    fn process(
        &mut self,
        instruction: Instruction,
        _signers: Vec<Keypair>,
    ) -> Result<(), FuzzClientError> {
        let mut instruction = instruction;

        // This is actually de-duplication
        // It returns map with unique accounts
        let de_duplicate_accounts = get_duplicate_accounts(&mut instruction.accounts);

        // We expect duplicate accounts will be only minority so we return references to all accounts
        let trident_accounts = self.pre_ix_accounts(&instruction.accounts);

        let mut serialized_accounts =
            serialize_accounts_custom(&instruction, &de_duplicate_accounts, &trident_accounts);

        let deserialized_account_infos =
            unsafe { deserialize_custom(&mut serialized_accounts.as_slice_mut()[0] as *mut u8) };

        match self.programs.get(&instruction.program_id) {
            Some(entrypoint) => (entrypoint)(
                &instruction.program_id,
                &deserialized_account_infos,
                &instruction.data,
            )
            .map_err(FuzzClientError::ProgramError)?,
            None if instruction.program_id == solana_sdk::system_program::ID => {
                solana_program::program::invoke(&instruction, &deserialized_account_infos)
                    .map_err(FuzzClientError::ProgramError)?
            }
            None => Err(FuzzClientError::ProgramNotFound(instruction.program_id))?,
        };

        self.post_ix_accounts_update(&deserialized_account_infos);
        Ok(())
    }
}

pub fn is_closed(info: &AccountInfo) -> bool {
    info.owner == &solana_sdk::system_program::ID && info.data_is_empty() && info.lamports() == 0
}
