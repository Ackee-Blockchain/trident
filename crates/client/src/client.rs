use crate::___private::Reader;
use crate::___private::TempClone;
use crate::config::Config;
use anchor_client::{
    anchor_lang::{
        prelude::System, solana_program::program_pack::Pack, AccountDeserialize, Id,
        InstructionData, ToAccountMetas,
    },
    solana_client::rpc_config::RpcTransactionConfig,
    solana_sdk::{
        account::Account,
        bpf_loader_upgradeable,
        commitment_config::CommitmentConfig,
        instruction::Instruction,
        pubkey::Pubkey,
        signature::read_keypair_file,
        signer::{keypair::Keypair, Signer},
        system_instruction,
        transaction::Transaction,
    },
    Client as AnchorClient, ClientError as Error, Cluster, Program,
};

use anchor_lang::prelude::UpgradeableLoaderState;
use borsh::BorshDeserialize;
use fehler::{throw, throws};
use futures::stream::{self, StreamExt};
use log::debug;
use serde::de::DeserializeOwned;
use solana_account_decoder::parse_token::UiTokenAmount;
use solana_cli_output::display::println_transaction;
use solana_transaction_status::{EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding};
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;
use std::{mem, rc::Rc};
use std::{thread::sleep, time::Duration};

// @TODO: Make compatible with the latest Anchor deps.
// https://github.com/project-serum/anchor/pull/1307#issuecomment-1022592683

use crate::constants::*;

type Payer = Rc<Keypair>;

/// `Client` allows you to send typed RPC requests to a Solana cluster.
pub struct Client {
    payer: Keypair,
    anchor_client: AnchorClient<Payer>,
}
/// Implement Default trait for Client, which reads keypair from default path for `solana-keygen new`
impl Default for Client {
    fn default() -> Self {
        let payer = read_keypair_file(&*shellexpand::tilde(DEFAULT_KEYPAIR_PATH))
            .unwrap_or_else(|_| panic!("Default keypair {DEFAULT_KEYPAIR_PATH} not found."));
        Self {
            payer: payer.clone(),
            anchor_client: AnchorClient::new_with_options(
                Cluster::Localnet,
                Rc::new(payer),
                CommitmentConfig::confirmed(),
            ),
        }
    }
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
    pub fn anchor_client(&self) -> &AnchorClient<Payer> {
        &self.anchor_client
    }

    /// Creates [Program] instance to communicate with the selected program.
    pub fn program(&self, program_id: Pubkey) -> Program<Payer> {
        self.anchor_client.program(program_id).unwrap()
    }

    /// Finds out if the Solana localnet is running.
    ///
    /// Set `retry` to `true` when you want to wait for up to 15 seconds until
    /// the localnet is running (until 30 retries with 500ms delays are performed).
    pub async fn is_localnet_running(&self, retry: bool) -> bool {
        let config = Config::new();

        let rpc_client = self
            .anchor_client
            .program(System::id())
            .unwrap()
            .async_rpc();

        for _ in 0..(if retry {
            config.test.validator_startup_timeout / RETRY_LOCALNET_EVERY_MILLIS
        } else {
            1
        }) {
            if rpc_client.get_health().await.is_ok() {
                return true;
            }
            if retry {
                sleep(Duration::from_millis(RETRY_LOCALNET_EVERY_MILLIS));
            }
        }
        false
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
        let program = self.anchor_client.program(System::id()).unwrap();
        program.account::<T>(account).await.unwrap()
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
        let rpc_client = self.anchor_client.program(System::id())?.async_rpc();
        rpc_client
            .get_account_with_commitment(&account, rpc_client.commitment())
            .await
            .unwrap()
            .value
    }

    /// Sends the Anchor instruction with associated accounts and signers.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use trident_client::*;
    ///
    /// pub async fn initialize(
    ///     client: &Client,
    ///     state: Pubkey,
    ///     user: Pubkey,
    ///     system_program: Pubkey,
    ///     signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    /// ) -> Result<EncodedConfirmedTransactionWithStatusMeta, ClientError> {
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
    ) -> EncodedConfirmedTransactionWithStatusMeta {
        let program = self.anchor_client.program(program).unwrap();
        let mut request = program.request().args(instruction).accounts(accounts);
        let signers = signers.into_iter().collect::<Vec<_>>();
        for signer in &signers {
            request = request.signer(signer);
        }
        let signature = request.send().await.unwrap();

        let rpc_client = self.anchor_client.program(System::id())?.async_rpc();
        rpc_client
            .get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Binary),
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: None,
                },
            )
            .await
            .unwrap()
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
    ) -> EncodedConfirmedTransactionWithStatusMeta {
        let rpc_client = self
            .anchor_client
            .program(System::id())
            .unwrap()
            .async_rpc();
        let mut signers = signers.into_iter().collect::<Vec<_>>();
        signers.push(self.payer());

        let tx = &Transaction::new_signed_with_payer(
            instructions,
            Some(&self.payer.pubkey()),
            &signers,
            rpc_client.get_latest_blockhash().await.unwrap(),
        );
        // @TODO make this call async with task::spawn_blocking
        let signature = rpc_client.send_and_confirm_transaction(tx).await.unwrap();
        let transaction = rpc_client
            .get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Binary),
                    commitment: Some(CommitmentConfig::confirmed()),
                    max_supported_transaction_version: None,
                },
            )
            .await
            .unwrap();

        transaction
    }

    /// Airdrops lamports to the chosen account.
    #[throws]
    pub async fn airdrop(&self, address: Pubkey, lamports: u64) {
        let rpc_client = self
            .anchor_client
            .program(System::id())
            .unwrap()
            .async_rpc();

        let signature = rpc_client
            .request_airdrop(&address, lamports)
            .await
            .unwrap();

        let (airdrop_result, error) = loop {
            match rpc_client.get_signature_status(&signature).await.unwrap() {
                Some(Ok(_)) => {
                    debug!("{} lamports airdropped", lamports);
                    break (true, None);
                }
                Some(Err(transaction_error)) => break (false, Some(transaction_error)),
                None => sleep(Duration::from_millis(500)),
            }
        };
        if !airdrop_result {
            throw!(Error::SolanaClientError(error.unwrap().into()));
        }
    }

    /// Get balance of an account
    #[throws]
    pub async fn get_balance(&mut self, address: &Pubkey) -> u64 {
        let rpc_client = self.anchor_client.program(System::id())?.async_rpc();
        rpc_client.get_balance(address).await?
    }

    /// Get token balance of an token account
    #[throws]
    pub async fn get_token_balance(&mut self, address: Pubkey) -> UiTokenAmount {
        let rpc_client = self.anchor_client.program(System::id())?.async_rpc();
        rpc_client.get_token_account_balance(&address).await?
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

        self.airdrop(self.payer().pubkey(), 5_000_000_000_000)
            .await
            .expect("airdropping for deployment failed");

        debug!("deploying program");

        self.deploy(program_keypair.clone(), mem::take(&mut program_data))
            .await?;

        // this will slow down the process because if we call program instruction right after deploy,
        // data are not yet completely deployed so error occures
        let deploy_done = loop {
            match self.anchor_client.program(program_keypair.pubkey()) {
                Ok(_) => {
                    debug!("program deployed succefully");
                    sleep(Duration::from_millis(1000));
                    break true;
                }
                Err(_) => {
                    sleep(Duration::from_millis(500));
                }
            }
        };

        // this is not necessarry, but we want to avoid "throws" macro warning of unused code
        if !deploy_done {
            throw!(Error::ProgramError(
                solana_sdk::program_error::ProgramError::Custom(0)
            ));
        }
    }

    /// Deploys the program.
    #[throws]
    async fn deploy(&self, program_keypair: Keypair, program_data: Vec<u8>) {
        const PROGRAM_DATA_CHUNK_SIZE: usize = 900;
        let buffer_account = Keypair::new();

        let program_data_len = program_data.len();
        let size_of_buffer = UpgradeableLoaderState::size_of_buffer(program_data_len);

        let rpc_client = self
            .anchor_client
            .program(System::id())
            .unwrap()
            .async_rpc();

        let system_program = self.anchor_client.program(System::id()).unwrap();

        debug!("program_data_len: {}", program_data_len);

        debug!("create program account");

        let min_balance_for_rent_exemption = rpc_client
            .get_minimum_balance_for_rent_exemption(program_data_len)
            .await
            .unwrap();

        let min_balance_for_rent_exemption_buffer = rpc_client
            .get_minimum_balance_for_rent_exemption(size_of_buffer)
            .await
            .unwrap();

        let create_account_ixs = bpf_loader_upgradeable::create_buffer(
            &self.payer.pubkey(),
            &buffer_account.pubkey(),
            &self.payer.pubkey(),
            min_balance_for_rent_exemption_buffer,
            program_data_len,
        )
        .unwrap();

        debug!("number of ixs = {}", create_account_ixs.len());

        let mut ix_builder = system_program.request();
        for ix in create_account_ixs {
            ix_builder = ix_builder.instruction(ix);
        }

        ix_builder
            .signer(&buffer_account)
            .signer(&self.payer)
            .send()
            .await
            .unwrap();

        debug!("write program data");

        let mut offset = 0usize;
        let mut futures_vec = Vec::new();

        for chunk in program_data.chunks(PROGRAM_DATA_CHUNK_SIZE) {
            let loader_write_ix = bpf_loader_upgradeable::write(
                &buffer_account.pubkey(),
                &self.payer.pubkey(),
                offset as u32,
                chunk.to_vec(),
            );
            futures_vec.push(async {
                let system_program = self.anchor_client.program(System::id()).unwrap();
                system_program
                    .request()
                    .instruction(loader_write_ix)
                    .signer(&self.payer)
                    .send()
                    .await
                    .unwrap();
            });
            offset += chunk.len();
        }
        stream::iter(futures_vec)
            .buffer_unordered(500)
            .collect::<Vec<_>>()
            .await;

        debug!("deploy program");

        let deploy_ixs = bpf_loader_upgradeable::deploy_with_max_program_len(
            &self.payer.pubkey(),
            &program_keypair.pubkey(),
            &buffer_account.pubkey(),
            &self.payer.pubkey(),
            min_balance_for_rent_exemption,
            program_data_len,
        )
        .unwrap();

        let mut ix_builder = system_program.request();
        for ix in deploy_ixs {
            ix_builder = ix_builder.instruction(ix);
        }

        ix_builder
            .signer(&self.payer)
            .signer(&program_keypair)
            .send()
            .await
            .unwrap();

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
    ) -> EncodedConfirmedTransactionWithStatusMeta {
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
    ) -> EncodedConfirmedTransactionWithStatusMeta {
        let rpc_client = self.anchor_client.program(System::id())?.async_rpc();
        self.send_transaction(
            &[system_instruction::create_account(
                &self.payer().pubkey(),
                &keypair.pubkey(),
                rpc_client
                    .get_minimum_balance_for_rent_exemption(space as usize)
                    .await?,
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
    ) -> EncodedConfirmedTransactionWithStatusMeta {
        let rpc_client = self.anchor_client.program(System::id())?.async_rpc();
        self.send_transaction(
            &[
                system_instruction::create_account(
                    &self.payer().pubkey(),
                    &mint.pubkey(),
                    rpc_client
                        .get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)
                        .await?,
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
    ) -> EncodedConfirmedTransactionWithStatusMeta {
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
    ) -> EncodedConfirmedTransactionWithStatusMeta {
        let rpc_client = self.anchor_client.program(System::id())?.async_rpc();
        self.send_transaction(
            &[
                system_instruction::create_account(
                    &self.payer().pubkey(),
                    &account.pubkey(),
                    rpc_client
                        .get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)
                        .await?,
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
            &[create_associated_token_account(
                &self.payer().pubkey(),
                &owner.pubkey(),
                &mint,
                &spl_token::ID,
            )],
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

        let rpc_client = self.anchor_client.program(System::id())?.async_rpc();
        self.send_transaction(
            &[system_instruction::create_account(
                &self.payer().pubkey(),
                &account.pubkey(),
                rpc_client
                    .get_minimum_balance_for_rent_exemption(data.len())
                    .await?,
                data.len() as u64,
                &bpf_loader_upgradeable::id(),
            )],
            [account],
        )
        .await?;

        let mut offset = 0usize;
        for chunk in data.chunks(DATA_CHUNK_SIZE) {
            debug!("writing bytes {} to {}", offset, offset + chunk.len());
            self.send_transaction(
                &[bpf_loader_upgradeable::write(
                    &account.pubkey(),
                    &bpf_loader_upgradeable::id(),
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

impl PrintableTransaction for EncodedConfirmedTransactionWithStatusMeta {
    fn print_named(&self, name: &str) {
        let tx = self.transaction.transaction.decode().unwrap();
        debug!("EXECUTE {} (slot {})", name, self.slot);
        match self.transaction.meta.clone() {
            Some(meta) => println_transaction(&tx, Some(&meta), "  ", None, None),
            _ => println_transaction(&tx, None, "  ", None, None),
        }
    }
}
