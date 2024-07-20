use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use solana_program_runtime::invoke_context::BuiltinFunctionWithContext;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::account::Account;
use solana_sdk::instruction::Instruction;
use solana_sdk::transaction::Transaction;

use crate::error::*;
use crate::fuzz_client::FuzzClient;
use solana_sdk::system_program::ID as SYSTEM_PROGRAM_ID;
use solana_sdk::{
    account::AccountSharedData, hash::Hash, instruction::AccountMeta, program_option::COption,
    program_pack::Pack, pubkey::Pubkey, rent::Rent, signature::Keypair, signature::Signer,
};
use spl_token::state::Mint;
use tokio::runtime::Builder;

pub struct FuzzingProgramFull {
    pub program_name: String,
    pub program_id: Pubkey,
    pub entry: Option<BuiltinFunctionWithContext>,
}
impl FuzzingProgramFull {
    pub fn new(
        program_name: &str,
        program_id: &Pubkey,
        entry_fn: Option<BuiltinFunctionWithContext>,
    ) -> FuzzingProgramFull {
        Self {
            program_name: program_name.to_string(),
            program_id: *program_id,
            entry: entry_fn,
        }
    }
}

pub struct ProgramTestClientBlocking {
    ctx: ProgramTestContext,
    rt: tokio::runtime::Runtime,
}

impl ProgramTestClientBlocking {
    pub fn new(program_: &[FuzzingProgramFull]) -> Result<Self, FuzzClientError> {
        let mut program_test = ProgramTest::default();
        for x in program_ {
            match x.entry {
                Some(entry) => {
                    program_test.add_builtin_program(&x.program_name, x.program_id, entry);
                }
                None => {
                    let data = read_program(&x.program_name);

                    program_test.add_account(
                        x.program_id,
                        Account {
                            lamports: Rent::default().minimum_balance(data.len()).max(1),
                            data,
                            owner: solana_sdk::bpf_loader::id(),
                            executable: true,
                            rent_epoch: 0,
                        },
                    );
                }
            }
        }
        let rt: tokio::runtime::Runtime = Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| FuzzClientError::ClientInitError(Box::new(e)))?;

        let ctx = rt.block_on(program_test.start_with_context());
        Ok(Self { ctx, rt })
    }
}

impl FuzzClient for ProgramTestClientBlocking {
    fn set_account(&mut self, lamports: u64) -> Keypair {
        let owner = Keypair::new();
        let account = AccountSharedData::new(lamports, 0, &SYSTEM_PROGRAM_ID);
        self.ctx.set_account(&owner.pubkey(), &account);
        owner
    }

    fn set_token_account(
        &mut self,
        mint: Pubkey,
        owner: Pubkey,
        amount: u64,
        delegate: Option<Pubkey>,
        is_native: Option<u64>,
        delegated_amount: u64,
        close_authority: Option<Pubkey>,
    ) -> Pubkey {
        let mint_account_key = Keypair::new().pubkey();

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
            AccountSharedData::new(lamports, spl_token::state::Account::LEN, &spl_token::id());

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
        self.ctx.set_account(&mint_account_key, &account);

        mint_account_key
    }

    fn set_mint_account(
        &mut self,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Pubkey {
        let mint_account = Keypair::new();

        let authority = match freeze_authority {
            Some(a) => COption::Some(a),
            _ => COption::None,
        };

        let r = Rent::default();
        let lamports = r.minimum_balance(Mint::LEN);

        let mut account = AccountSharedData::new(lamports, Mint::LEN, &spl_token::id());

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
        self.ctx.set_account(&mint_account.pubkey(), &account);

        mint_account.pubkey()
    }

    fn payer(&self) -> Keypair {
        self.ctx.payer.insecure_clone()
    }

    fn get_account(&mut self, key: &Pubkey) -> Result<Option<Account>, FuzzClientError> {
        Ok(self
            .rt
            .block_on(self.ctx.banks_client.get_account_with_commitment(
                *key,
                solana_sdk::commitment_config::CommitmentLevel::Confirmed,
            ))?)
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

    fn get_last_blockhash(&self) -> Hash {
        self.ctx.last_blockhash
    }

    fn process(
        &mut self,
        instruction: Instruction,
        mut signers: Vec<Keypair>,
    ) -> Result<(), FuzzClientError> {
        let mut transaction =
            Transaction::new_with_payer(&[instruction], Some(&self.payer().pubkey()));

        transaction.message.hash();
        signers.push(self.payer().insecure_clone());
        let sig: Vec<&Keypair> = signers.iter().collect();
        transaction.sign(&sig, self.get_last_blockhash());

        Ok(self
            .rt
            .block_on(self.ctx.banks_client.process_transaction(transaction))?)
    }
    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData) {
        self.ctx.set_account(address, account);
    }

    fn get_rent(&mut self) -> Result<Rent, FuzzClientError> {
        Ok(self.rt.block_on(self.ctx.banks_client.get_rent())?)
    }
}

fn read_program(program_name: &str) -> Vec<u8> {
    let genesis_folder = std::env::var("GENESIS_FOLDER")
        .unwrap_or_else(|err| panic!("Failed to read env variable GENESIS_FOLDER: {}", err));

    let program_path = PathBuf::from(genesis_folder).join(format!("{program_name}.so"));

    let mut file = File::open(&program_path)
        .unwrap_or_else(|err| panic!("Failed to open \"{}\": {}", program_path.display(), err));

    let mut file_data = Vec::new();
    file.read_to_end(&mut file_data)
        .unwrap_or_else(|err| panic!("Failed to read \"{}\": {}", program_path.display(), err));
    file_data
}
