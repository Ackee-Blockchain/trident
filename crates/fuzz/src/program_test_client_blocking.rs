use base64::prelude::*;
use solana_program_runtime::invoke_context::BuiltinFunctionWithContext;
use solana_program_test::ProgramTest;
use solana_program_test::ProgramTestContext;
use solana_sdk::account::Account;
use solana_sdk::account::WritableAccount;
use solana_sdk::account_info::AccountInfo;
use solana_sdk::entrypoint::ProgramResult;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::system_program::ID as SYSTEM_PROGRAM_ID;
use solana_sdk::{
    account::AccountSharedData, hash::Hash, instruction::AccountMeta, program_option::COption,
    program_pack::Pack, pubkey::Pubkey, rent::Rent, signature::Keypair, signature::Signer,
    transaction::VersionedTransaction,
};
use spl_token::state::Mint;
use tokio::runtime::Builder;
use litesvm::LiteSVM;

use crate::config::Config;
use crate::error::*;
use crate::fuzz_client::FuzzClient;

pub type ProgramEntry = for<'info> fn(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    instruction_data: &[u8],
) -> ProgramResult;

pub struct ProgramTestClientBlocking {
    //ctx: ProgramTestContext,
    svm: LiteSVM,
    rt: tokio::runtime::Runtime,
    payer: Keypair,
}

pub struct FuzzingProgram {
    pub program_name: String,
    pub program_id: Pubkey,
    pub entry: Option<BuiltinFunctionWithContext>,
}
impl FuzzingProgram {
    pub fn new(
        program_name: &str,
        program_id: &Pubkey,
        entry_fn: Option<BuiltinFunctionWithContext>,
    ) -> FuzzingProgram {
        Self {
            program_name: program_name.to_string(),
            program_id: *program_id,
            entry: entry_fn,
        }
    }
}

impl ProgramTestClientBlocking {
    pub fn new(program_: &[FuzzingProgram], config: &Config) -> Result<Self, FuzzClientError> {
        let mut program_test = ProgramTest::default();
        let mut svm = LiteSVM::default()
            .with_builtins(None)
            .with_lamports(1_100_000u64.wrapping_mul(LAMPORTS_PER_SOL))
            .with_sysvars()
            .with_spl_programs();

        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 1_000_000u64.wrapping_mul(LAMPORTS_PER_SOL)).unwrap();

        for x in program_ {
            if let Some(entry) = x.entry {
                println!("adding builtin {:?}", x.program_id);
                svm.add_builtin(x.program_id, &x.program_name, entry);

                //program_test.add_builtin_program(&x.program_name, x.program_id, entry);

                assert!(svm.get_account(&x.program_id).unwrap().executable);
            }
        }

        for account in config.fuzz.accounts.iter() {
            svm.set_account(
                account.pubkey,
                Account {
                    lamports: account.account.lamports,
                    data: BASE64_STANDARD
                    .decode(&account.account.data)
                    .unwrap_or_else(|err| panic!("Failed to base64 decode: {err}")),
                    owner: account.account.owner,
                    executable: account.account.executable,
                    rent_epoch: account.account.rent_epoch,
                },
            ).unwrap();

            /*
            program_test.add_account_with_base64_data(
                account.pubkey,
                account.account.lamports,
                account.account.owner,
                &account.account.data,
            )
            */
        }

        for program in config.fuzz.programs.iter() {
            println!("adding program {:?}", program.address);
            svm.add_program(program.address, &program.data);

            assert!(svm.get_account(&program.address).unwrap().executable);
            /*
            println!("adding program {:?}", program.address);
            svm.set_account(
                program.address,
                Account {
                    lamports: Rent::default().minimum_balance(program.data.len()).max(1),
                    data: program.data.clone(),
                    owner: solana_sdk::bpf_loader::id(),
                    executable: true,
                    rent_epoch: 0,
                },
            ).unwrap();

            program_test.add_account(
                program.address,
                Account {
                    lamports: Rent::default().minimum_balance(program.data.len()).max(1),
                    data: program.data.clone(),
                    owner: solana_sdk::bpf_loader::id(),
                    executable: true,
                    rent_epoch: 0,
                },
            );
            */
        }

        let rt: tokio::runtime::Runtime = Builder::new_current_thread().enable_all().build()?;

        //let ctx = rt.block_on(program_test.start_with_context());

        Ok(Self { rt, svm, payer })
    }
}

/// Converts Anchor 0.29.0 and higher entrypoint into the runtime's entrypoint style
///
/// Starting Anchor 0.29.0 the accounts are passed by reference https://github.com/coral-xyz/anchor/pull/2656
/// and the lifetime requirements are `accounts: &'a [AccountInfo<'a>]` instead of `accounts: &'a [AccountInfo<'b>]`.
/// The new requirements require the slice of AccountInfos and the contained Accounts to have the same lifetime but
/// the previous version is more general. The compiler implies that `'b` must live at least as long as `'a` or longer.
///
/// The transaction data is serialized and again deserialized to the `&[AccountInfo<_>]` slice just before invoking
/// the entry point and the modified account data is copied to the original accounts just after the the entry point.
/// After that the `&[AccountInfo<_>]` slice goes out of scope entirely and therefore `'a` == `'b`. So it _SHOULD_ be
/// safe to do this conversion in this testing scenario.
///
/// Do not use this conversion in any on-chain programs!
#[macro_export]
macro_rules! convert_entry {
    ($entry:expr) => {
        unsafe { core::mem::transmute::<ProgramEntry, ProcessInstruction>($entry) }
    };
}

impl FuzzClient for ProgramTestClientBlocking {
    fn set_account(&mut self, lamports: u64) -> Keypair {
        let owner = Keypair::new();
        //let account = AccountSharedData::new(lamports, 0, &SYSTEM_PROGRAM_ID);
        self.svm.set_account(owner.pubkey(), Account {
            lamports,
            data: vec![],
            owner: SYSTEM_PROGRAM_ID,
            executable: false,
            rent_epoch: 0,
        });
        //self.ctx.set_account(&owner.pubkey(), &account);
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
        //self.ctx.set_account(&mint_account_key, account);

        self.svm.set_account(mint_account_key, Account {
            lamports: lamports,
            data: data,
            owner: spl_token::id(),
            executable: true,
            rent_epoch: 0,
        });

        mint_account_key
    }

    fn set_mint_account(
        &mut self,
        decimals: u8,
        owner: &Pubkey,
        freeze_authority: Option<Pubkey>,
    ) -> Pubkey {
        todo!();

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
        //self.ctx.set_account(&mint_account.pubkey(), &account);

        mint_account.pubkey()
    }

    fn payer(&self) -> Keypair {
        self.payer.insecure_clone()
        //self.ctx.payer.insecure_clone()
    }

    fn get_account(&mut self, key: &Pubkey) -> Result<Option<Account>, FuzzClientError> {
        Ok(self.svm.get_account(key))
        /*
        Ok(self
            .rt
            .block_on(self.ctx.banks_client.get_account_with_commitment(
                *key,
                solana_sdk::commitment_config::CommitmentLevel::Confirmed,
            ))?)
            */
    }
    fn get_accounts(
        &mut self,
        metas: &[AccountMeta],
    ) -> Result<Vec<Option<Account>>, FuzzClientErrorWithOrigin> {
        Ok(metas
            .iter()
            .map(|m| self.svm.get_account(&m.pubkey))
            .collect())

        /*
        let result: Vec<_> = metas
            .iter()
            .map(|m| {
                self.get_account(&m.pubkey)
                    .map_err(|e| e.with_origin(Origin::Account(m.pubkey)))
            })
            .collect();
        result.into_iter().collect()
        */
    }

    fn get_last_blockhash(&self) -> Hash {
        self.svm.latest_blockhash()
        //self.ctx.last_blockhash
    }
    fn process_transaction(
        &mut self,
        transaction: impl Into<VersionedTransaction>,
    ) -> Result<(), FuzzClientError> {
        self.svm.send_transaction(transaction).unwrap();

        Ok(())

        /*
        Ok(self
            .rt
            .block_on(self.ctx.banks_client.process_transaction(transaction))?)
            */
    }

    fn set_account_custom(&mut self, address: &Pubkey, account: &AccountSharedData) {
        self.svm.set_account(*address, account.clone().into()).unwrap();

        //self.ctx.set_account(address, account);
    }

    fn get_rent(&mut self) -> Result<Rent, FuzzClientError> {
        println!("getting rent");
        Ok(self.svm.get_sysvar::<Rent>())
        //Ok(self.rt.block_on(self.ctx.banks_client.get_rent())?)
    }
}
