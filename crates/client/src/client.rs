use crate::{Reader, TempClone, TestGenerator};
use anchor_client::{
    anchor_lang::{
        prelude::System, solana_program::program_pack::Pack, AccountDeserialize, Id,
        InstructionData, ToAccountMetas,
    },
    solana_client::{client_error::ClientErrorKind, rpc_config::RpcTransactionConfig},
    solana_sdk::{
        account::Account,
        bpf_loader,
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        loader_instruction,
        pubkey::Pubkey,
        signer::{keypair::Keypair, Signer},
        system_instruction,
        transaction::Transaction,
    },
    Client as AnchorClient, ClientError as Error, Cluster, Program,
};

use borsh::BorshDeserialize;
use fehler::{throw, throws};
use futures::stream::{self, StreamExt};
use log::debug;
use serde::de::DeserializeOwned;
use solana_account_decoder::parse_token::UiTokenAmount;
use solana_cli_output::display::println_transaction;
use solana_transaction_status::{EncodedConfirmedTransaction, UiTransactionEncoding};
use spl_associated_token_account::get_associated_token_address;
use std::{mem, rc::Rc};
use std::{thread::sleep, time::Duration};
use tokio::task;

// @TODO: Make compatible with the latest Anchor deps.
// https://github.com/project-serum/anchor/pull/1307#issuecomment-1022592683

/// `Client` allows you to send typed RPC requests to a Solana cluster.
pub struct Client {
    payer: Keypair,
    anchor_client: AnchorClient,
}

impl Client {
    /// Creates a new `Client` instance.
    pub fn new(payer: Keypair) -> Self {
        Self {
            payer: payer.clone(),
            anchor_client: AnchorClient::new_with_options(
                Cluster::Localnet,
                Rc::new(payer),
                CommitmentConfig::confirmed(),
            ),
        }
    }

    /// Gets client's payer.
    pub fn payer(&self) -> &Keypair {
        &self.payer
    }

    /// Gets the internal Anchor client to call Anchor client's methods directly.
    pub fn anchor_client(&self) -> &AnchorClient {
        &self.anchor_client
    }

    /// Creates [Program] instance to communicate with the selected program.
    pub fn program(&self, program_id: Pubkey) -> Program {
        self.anchor_client.program(program_id)
    }

    /// Finds out if the Solana localnet is running.
    ///
    /// Set `retry` to `true` when you want to wait until the localnet is running
    /// or until 10 retries with 500ms delays are performed.
    pub async fn is_localnet_running(&self, retry: bool) -> bool {
        let dummy_pubkey = Pubkey::new_from_array([0; 32]);
        let rpc_client = self.anchor_client.program(dummy_pubkey).rpc();
        task::spawn_blocking(move || {
            for _ in 0..(if retry { 10 } else { 1 }) {
                if rpc_client.get_health().is_ok() {
                    return true;
                }
                if retry {
                    sleep(Duration::from_millis(500));
                }
            }
            false
        })
        .await
        .expect("is_localnet_running task failed")
    }

    /// Gets deserialized data from the chosen account serialized with Anchor
    ///
    /// # Errors
    ///
    /// It fails when:
    /// - the account does not exist.
    /// - the Solana cluster is not running.
    /// - deserialization failed.
    #[throws]
    pub async fn account_data<T>(&self, account: Pubkey) -> T
    where
        T: AccountDeserialize + Send + 'static,
    {
        task::spawn_blocking(move || {
            let dummy_keypair = Keypair::new();
            let dummy_program_id = Pubkey::new_from_array([0; 32]);
            let program = Client::new(dummy_keypair).program(dummy_program_id);
            program.account::<T>(account)
        })
        .await
        .expect("account_data task failed")?
    }

    /// Gets deserialized data from the chosen account serialized with Bincode
    ///
    /// # Errors
    ///
    /// It fails when:
    /// - the account does not exist.
    /// - the Solana cluster is not running.
    /// - deserialization failed.
    #[throws]
    pub async fn account_data_bincode<T>(&self, account: Pubkey) -> T
    where
        T: DeserializeOwned + Send + 'static,
    {
        let account = self
            .get_account(account)
            .await?
            .ok_or(Error::AccountNotFound)?;

        bincode::deserialize(&account.data)
            .map_err(|_| Error::LogParseError("Bincode deserialization failed".to_string()))?
    }

    /// Gets deserialized data from the chosen account serialized with Borsh
    ///
    /// # Errors
    ///
    /// It fails when:
    /// - the account does not exist.
    /// - the Solana cluster is not running.
    /// - deserialization failed.
    #[throws]
    pub async fn account_data_borsh<T>(&self, account: Pubkey) -> T
    where
        T: BorshDeserialize + Send + 'static,
    {
        let account = self
            .get_account(account)
            .await?
            .ok_or(Error::AccountNotFound)?;

        T::try_from_slice(&account.data)
            .map_err(|_| Error::LogParseError("Bincode deserialization failed".to_string()))?
    }

    /// Returns all information associated with the account of the provided [Pubkey].
    ///
    /// # Errors
    ///
    /// It fails when the Solana cluster is not running.
    #[throws]
    pub async fn get_account(&self, account: Pubkey) -> Option<Account> {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        task::spawn_blocking(move || {
            rpc_client.get_account_with_commitment(&account, rpc_client.commitment())
        })
        .await
        .expect("get_account task failed")?
        .value
    }

    /// Sends the Anchor instruction with associated accounts and signers.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use trdelnik_client::*;
    ///
    /// pub async fn initialize(
    ///     client: &Client,
    ///     state: Pubkey,
    ///     user: Pubkey,
    ///     system_program: Pubkey,
    ///     signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    /// ) -> Result<EncodedConfirmedTransaction, ClientError> {
    ///     Ok(client
    ///         .send_instruction(
    ///             PROGRAM_ID,
    ///             turnstile::instruction::Initialize {},
    ///             turnstile::accounts::Initialize {
    ///                 state: a_state,
    ///                 user: a_user,
    ///                 system_program: a_system_program,
    ///             },
    ///             signers,
    ///         )
    ///         .await?)
    /// }
    /// ```
    #[throws]
    pub async fn send_instruction(
        &self,
        program: Pubkey,
        instruction: impl InstructionData + Send + 'static,
        accounts: impl ToAccountMetas + Send + 'static,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> EncodedConfirmedTransaction {
        let payer = self.payer().clone();
        let signature = task::spawn_blocking(move || {
            let program = Client::new(payer).program(program);
            let mut request = program.request().args(instruction).accounts(accounts);
            let signers = signers.into_iter().collect::<Vec<_>>();
            for signer in &signers {
                request = request.signer(signer);
            }
            request.send()
        })
        .await
        .expect("send instruction task failed")?;

        let rpc_client = self.anchor_client.program(System::id()).rpc();
        task::spawn_blocking(move || {
            rpc_client.get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Binary),
                    commitment: Some(CommitmentConfig::confirmed()),
                },
            )
        })
        .await
        .expect("get transaction task failed")?
    }

    /// Sends the transaction with associated instructions and signers.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// #[throws]
    /// pub async fn create_account(
    ///     &self,
    ///     keypair: &Keypair,
    ///     lamports: u64,
    ///     space: u64,
    ///     owner: &Pubkey,
    /// ) -> EncodedConfirmedTransaction {
    ///     self.send_transaction(
    ///         &[system_instruction::create_account(
    ///             &self.payer().pubkey(),
    ///             &keypair.pubkey(),
    ///             lamports,
    ///             space,
    ///             owner,
    ///         )],
    ///         [keypair],
    ///     )
    ///     .await?
    /// }
    /// ```
    #[throws]
    pub async fn send_transaction(
        &self,
        instructions: &[Instruction],
        signers: impl IntoIterator<Item = &Keypair> + Send,
    ) -> EncodedConfirmedTransaction {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        let mut signers = signers.into_iter().collect::<Vec<_>>();
        signers.push(self.payer());

        let tx = &Transaction::new_signed_with_payer(
            instructions,
            Some(&self.payer.pubkey()),
            &signers,
            rpc_client
                .get_latest_blockhash()
                .expect("Error while getting recent blockhash"),
        );
        // @TODO make this call async with task::spawn_blocking
        let signature = rpc_client.send_and_confirm_transaction(tx)?;
        let transaction = task::spawn_blocking(move || {
            rpc_client.get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Binary),
                    commitment: Some(CommitmentConfig::confirmed()),
                },
            )
        })
        .await
        .expect("get transaction task failed")?;

        transaction
    }

    /// Airdrops lamports to the chosen account.
    #[throws]
    pub async fn airdrop(&self, address: Pubkey, lamports: u64) {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        task::spawn_blocking(move || -> Result<(), Error> {
            let signature = rpc_client.request_airdrop(&address, lamports)?;
            for _ in 0..5 {
                match rpc_client.get_signature_status(&signature)? {
                    Some(Ok(_)) => {
                        debug!("{} lamports airdropped", lamports);
                        return Ok(());
                    }
                    Some(Err(transaction_error)) => {
                        throw!(Error::SolanaClientError(transaction_error.into()));
                    }
                    None => sleep(Duration::from_millis(500)),
                }
            }
            throw!(Error::SolanaClientError(
                ClientErrorKind::Custom(
                    "Airdrop transaction has not been processed yet".to_owned(),
                )
                .into(),
            ));
        })
        .await
        .expect("airdrop task failed")?
    }

    /// Get balance of an account
    #[throws]
    pub async fn get_balance(&mut self, address: Pubkey) -> u64 {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        task::spawn_blocking(move || rpc_client.get_balance(&address))
            .await
            .expect("get_balance task failed")?
    }

    /// Get token balance of an token account
    #[throws]
    pub async fn get_token_balance(&mut self, address: Pubkey) -> UiTokenAmount {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        task::spawn_blocking(move || rpc_client.get_token_account_balance(&address))
            .await
            .expect("get_token_balance task failed")?
    }

    /// Deploys all programs from the 'programs' folder
    #[throws]
    pub async fn deploy_all_programs(&self) {
        let root = TestGenerator::discover_root().expect("project root (Anchor.toml) not found");
        let programs = TestGenerator::get_programs(root.as_path())
            .await
            .expect("can not get the list of programs");

        debug!("found program(s): {:?}", programs);

        for (i, (program_name, _)) in programs.iter().enumerate() {
            debug!("deploying program no. {} '{}'", i, program_name);

            let program_keypair = crate::program_keypair(i);
            self.deploy_by_name(&program_keypair, program_name.as_str())
                .await
                .expect("deployment failed");
        }
    }

    /// Deploys a program based on it's name.
    /// This function wraps boilerplate code required for the successful deployment of a program,
    /// i.e. SOLs airdrop etc.
    ///
    /// # Arguments
    ///
    /// * `program_keypair` - [Keypair] used for the program
    /// * `program_name` - Name of the program to be deployed
    ///
    /// # Example:
    ///
    /// *Project structure*
    ///
    /// ```text
    /// project/
    /// - programs/
    ///   - awesome_contract/
    ///     - ...
    ///     - Cargo.toml
    ///   - turnstile/
    ///     - ...
    ///     - Cargo.toml
    /// - ...
    /// - Cargo.toml
    /// ```
    ///
    /// *Code*
    ///
    /// ```rust,ignore
    /// client.deploy_program(program_keypair(0), "awesome_contract");
    /// client.deploy_program(program_keypair(1), "turnstile");
    /// ```
    #[throws]
    pub async fn deploy_by_name(&self, program_keypair: &Keypair, program_name: &str) {
        debug!("reading program data");

        let reader = Reader::new();
        let mut program_data = reader
            .program_data(program_name)
            .await
            .expect("reading program data failed");

        debug!("airdropping the minimum balance required to deploy the program");

        // TODO: This will fail on devnet where airdrops are limited to 1 SOL
        self.airdrop(self.payer().pubkey(), 5_000_000_000)
            .await
            .expect("airdropping for deployment failed");

        debug!("deploying program");

        self.deploy(program_keypair.clone(), mem::take(&mut program_data))
            .await
            .expect("deploying program failed");
    }

    /// Deploys the program.
    #[throws]
    async fn deploy(&self, program_keypair: Keypair, program_data: Vec<u8>) {
        const PROGRAM_DATA_CHUNK_SIZE: usize = 900;

        let program_pubkey = program_keypair.pubkey();
        let system_program = self.anchor_client.program(System::id());

        let program_data_len = program_data.len();
        debug!("program_data_len: {}", program_data_len);

        debug!("create program account");

        let rpc_client = system_program.rpc();
        let min_balance_for_rent_exemption = task::spawn_blocking(move || {
            rpc_client.get_minimum_balance_for_rent_exemption(program_data_len)
        })
        .await
        .expect("crate program account task failed")?;

        let create_account_ix = system_instruction::create_account(
            &system_program.payer(),
            &program_pubkey,
            min_balance_for_rent_exemption,
            program_data_len as u64,
            &bpf_loader::id(),
        );
        {
            let program_keypair = Keypair::from_bytes(&program_keypair.to_bytes()).unwrap();
            let payer = self.payer().clone();
            task::spawn_blocking(move || {
                let system_program = Client::new(payer).program(System::id());
                system_program
                    .request()
                    .instruction(create_account_ix)
                    .signer(&program_keypair)
                    .send()
            })
            .await
            .expect("create program account task failed")?;
        }

        debug!("write program data");

        let mut offset = 0usize;
        let mut futures = Vec::new();
        for chunk in program_data.chunks(PROGRAM_DATA_CHUNK_SIZE) {
            let program_keypair = Keypair::from_bytes(&program_keypair.to_bytes()).unwrap();
            let loader_write_ix = loader_instruction::write(
                &program_pubkey,
                &bpf_loader::id(),
                offset as u32,
                chunk.to_vec(),
            );
            let payer = self.payer().clone();

            futures.push(async move {
                task::spawn_blocking(move || {
                    let system_program = Client::new(payer).program(System::id());
                    system_program
                        .request()
                        .instruction(loader_write_ix)
                        .signer(&program_keypair)
                        .send()
                })
                .await
                .expect("write program data task failed")
            });
            offset += chunk.len();
        }
        stream::iter(futures)
            .buffer_unordered(100)
            .collect::<Vec<_>>()
            .await;

        debug!("finalize program");

        let loader_finalize_ix = loader_instruction::finalize(&program_pubkey, &bpf_loader::id());
        let payer = self.payer().clone();
        task::spawn_blocking(move || {
            let system_program = Client::new(payer).program(System::id());
            system_program
                .request()
                .instruction(loader_finalize_ix)
                .signer(&program_keypair)
                .send()
        })
        .await
        .expect("finalize program account task failed")?;

        debug!("program deployed");
    }

    /// Creates accounts.
    #[throws]
    pub async fn create_account(
        &self,
        keypair: &Keypair,
        lamports: u64,
        space: u64,
        owner: &Pubkey,
    ) -> EncodedConfirmedTransaction {
        self.send_transaction(
            &[system_instruction::create_account(
                &self.payer().pubkey(),
                &keypair.pubkey(),
                lamports,
                space,
                owner,
            )],
            [keypair],
        )
        .await?
    }

    /// Creates rent exempt account.
    #[throws]
    pub async fn create_account_rent_exempt(
        &mut self,
        keypair: &Keypair,
        space: u64,
        owner: &Pubkey,
    ) -> EncodedConfirmedTransaction {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        self.send_transaction(
            &[system_instruction::create_account(
                &self.payer().pubkey(),
                &keypair.pubkey(),
                rpc_client.get_minimum_balance_for_rent_exemption(space as usize)?,
                space,
                owner,
            )],
            [keypair],
        )
        .await?
    }

    /// Executes a transaction constructing a token mint.
    #[throws]
    pub async fn create_token_mint(
        &self,
        mint: &Keypair,
        authority: Pubkey,
        freeze_authority: Option<Pubkey>,
        decimals: u8,
    ) -> EncodedConfirmedTransaction {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        self.send_transaction(
            &[
                system_instruction::create_account(
                    &self.payer().pubkey(),
                    &mint.pubkey(),
                    rpc_client
                        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?,
                    spl_token::state::Mint::LEN as u64,
                    &spl_token::ID,
                ),
                spl_token::instruction::initialize_mint(
                    &spl_token::ID,
                    &mint.pubkey(),
                    &authority,
                    freeze_authority.as_ref(),
                    decimals,
                )
                .unwrap(),
            ],
            [mint],
        )
        .await?
    }

    /// Executes a transaction that mints tokens from a mint to an account belonging to that mint.
    #[throws]
    pub async fn mint_tokens(
        &self,
        mint: Pubkey,
        authority: &Keypair,
        account: Pubkey,
        amount: u64,
    ) -> EncodedConfirmedTransaction {
        self.send_transaction(
            &[spl_token::instruction::mint_to(
                &spl_token::ID,
                &mint,
                &account,
                &authority.pubkey(),
                &[],
                amount,
            )
            .unwrap()],
            [authority],
        )
        .await?
    }

    /// Executes a transaction constructing a token account of the specified mint. The account needs to be empty and belong to system for this to work.
    /// Prefer to use [create_associated_token_account] if you don't need the provided account to contain the token account.
    #[throws]
    pub async fn create_token_account(
        &self,
        account: &Keypair,
        mint: &Pubkey,
        owner: &Pubkey,
    ) -> EncodedConfirmedTransaction {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        self.send_transaction(
            &[
                system_instruction::create_account(
                    &self.payer().pubkey(),
                    &account.pubkey(),
                    rpc_client
                        .get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)?,
                    spl_token::state::Account::LEN as u64,
                    &spl_token::ID,
                ),
                spl_token::instruction::initialize_account(
                    &spl_token::ID,
                    &account.pubkey(),
                    mint,
                    owner,
                )
                .unwrap(),
            ],
            [account],
        )
        .await?
    }

    /// Executes a transaction constructing the associated token account of the specified mint belonging to the owner. This will fail if the account already exists.
    #[throws]
    pub async fn create_associated_token_account(&self, owner: &Keypair, mint: Pubkey) -> Pubkey {
        self.send_transaction(
            &[
                spl_associated_token_account::create_associated_token_account(
                    &self.payer().pubkey(),
                    &owner.pubkey(),
                    &mint,
                ),
            ],
            &[],
        )
        .await?;
        get_associated_token_address(&owner.pubkey(), &mint)
    }

    /// Executes a transaction creating and filling the given account with the given data.
    /// The account is required to be empty and will be owned by bpf_loader afterwards.
    #[throws]
    pub async fn create_account_with_data(&self, account: &Keypair, data: Vec<u8>) {
        const DATA_CHUNK_SIZE: usize = 900;

        let rpc_client = self.anchor_client.program(System::id()).rpc();
        self.send_transaction(
            &[system_instruction::create_account(
                &self.payer().pubkey(),
                &account.pubkey(),
                rpc_client.get_minimum_balance_for_rent_exemption(data.len())?,
                data.len() as u64,
                &bpf_loader::id(),
            )],
            [account],
        )
        .await?;

        let mut offset = 0usize;
        for chunk in data.chunks(DATA_CHUNK_SIZE) {
            debug!("writing bytes {} to {}", offset, offset + chunk.len());
            self.send_transaction(
                &[loader_instruction::write(
                    &account.pubkey(),
                    &bpf_loader::id(),
                    offset as u32,
                    chunk.to_vec(),
                )],
                [account],
            )
            .await?;
            offset += chunk.len();
        }
    }
}

/// Utility trait for printing transaction results.
pub trait PrintableTransaction {
    /// Pretty print the transaction results, tagged with the given name for distinguishability.
    fn print_named(&self, name: &str);

    /// Pretty print the transaction results.
    fn print(&self) {
        self.print_named("");
    }
}

impl PrintableTransaction for EncodedConfirmedTransaction {
    fn print_named(&self, name: &str) {
        let tx = self.transaction.transaction.decode().unwrap();
        debug!("EXECUTE {} (slot {})", name, self.slot);
        println_transaction(&tx, &self.transaction.meta, "  ", None, None);
    }
}
