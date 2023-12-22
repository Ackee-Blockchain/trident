use crate::{constants::*, Config, Reader, TempClone};

use anchor_client::ClientError as Error;
// TODO maybe can deleted
// use borsh;
use fehler::{throw, throws};
use solana_sdk::program_pack::Pack;
use solana_sdk::signer::Signer;
// TODO maybe can deleted
use futures::{self, StreamExt};
// TODO maybe can deleted
// use serde;
// TODO maybe can deleted
// use solana_account_decoder;
// TODO maybe can deleted
// use solana_cli_output;

// @TODO: Make compatible with the latest Anchor deps.
// https://github.com/project-serum/anchor/pull/1307#issuecomment-1022592683

type Payer = std::rc::Rc<solana_sdk::signer::keypair::Keypair>;

/// `Client` allows you to send typed RPC requests to a Solana cluster.
pub struct Client {
    payer: solana_sdk::signer::keypair::Keypair,
    anchor_client: anchor_client::Client<Payer>,
}

/// Implement Default trait for Client, which reads keypair from default path for `solana-keygen new`
impl Default for Client {
    fn default() -> Self {
        let payer =
            solana_sdk::signature::read_keypair_file(&*shellexpand::tilde(DEFAULT_KEYPAIR_PATH))
                .unwrap_or_else(|_| panic!("Default keypair {DEFAULT_KEYPAIR_PATH} not found."));
        Self {
            payer: payer.clone(),
            anchor_client: anchor_client::Client::new_with_options(
                anchor_client::Cluster::Localnet,
                std::rc::Rc::new(payer),
                solana_sdk::commitment_config::CommitmentConfig::confirmed(),
            ),
        }
    }
}

impl Client {
    /// Creates a new `Client` instance.
    pub fn new(payer: solana_sdk::signer::keypair::Keypair) -> Self {
        Self {
            payer: payer.clone(),
            anchor_client: anchor_client::Client::new_with_options(
                anchor_client::Cluster::Localnet,
                std::rc::Rc::new(payer),
                solana_sdk::commitment_config::CommitmentConfig::confirmed(),
            ),
        }
    }

    /// Gets client's payer.
    pub fn payer(&self) -> &solana_sdk::signer::keypair::Keypair {
        &self.payer
    }

    /// Gets the internal Anchor client to call Anchor client's methods directly.
    pub fn anchor_client(&self) -> &anchor_client::Client<Payer> {
        &self.anchor_client
    }

    /// Creates [Program] instance to communicate with the selected program.
    pub fn program(
        &self,
        program_id: &solana_sdk::pubkey::Pubkey,
    ) -> anchor_client::Program<Payer> {
        self.anchor_client.program(*program_id).unwrap()
    }

    /// Finds out if the Solana localnet is running.
    ///
    /// Set `retry` to `true` when you want to wait for up to 15 seconds until
    /// the localnet is running (until 30 retries with 500ms delays are performed).
    pub async fn is_localnet_running(&self, retry: bool) -> bool {
        let config = Config::new();

        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)
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
                std::thread::sleep(std::time::Duration::from_millis(
                    RETRY_LOCALNET_EVERY_MILLIS,
                ));
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
    pub async fn account_data<T>(&self, account: &solana_sdk::pubkey::Pubkey) -> T
    where
        T: anchor_lang::AccountDeserialize + Send + 'static,
    {
        let program = self
            .anchor_client
            .program(solana_sdk::system_program::ID)
            .unwrap();
        program.account::<T>(*account).await.unwrap()
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
    pub async fn account_data_bincode<T>(&self, account: &solana_sdk::pubkey::Pubkey) -> T
    where
        T: serde::de::DeserializeOwned + Send + 'static,
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
    pub async fn account_data_borsh<T>(&self, account: &solana_sdk::pubkey::Pubkey) -> T
    where
        T: borsh::BorshDeserialize + Send + 'static,
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
    pub async fn get_account(
        &self,
        account: &solana_sdk::pubkey::Pubkey,
    ) -> Option<solana_sdk::account::Account> {
        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)?
            .async_rpc();
        rpc_client
            .get_account_with_commitment(account, rpc_client.commitment())
            .await
            .unwrap()
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
    ///     signers: impl IntoIterator<Item = Keypair> + Send,
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
        program: solana_sdk::pubkey::Pubkey,
        instruction: impl anchor_lang::InstructionData + Send,
        accounts: impl anchor_lang::ToAccountMetas + Send,
        signers: impl IntoIterator<Item = &solana_sdk::signer::keypair::Keypair> + Send,
    ) -> solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta {
        let program = self.anchor_client.program(program).unwrap();
        let mut request = program.request().args(instruction).accounts(accounts);
        let signers = signers.into_iter().collect::<Vec<_>>();
        for signer in signers {
            request = request.signer(signer);
        }
        let signature = request.send().await.unwrap();

        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)?
            .async_rpc();
        rpc_client
            .get_transaction_with_config(
                &signature,
                anchor_client::solana_client::rpc_config::RpcTransactionConfig {
                    encoding: Some(solana_transaction_status::UiTransactionEncoding::Binary),
                    commitment: Some(solana_sdk::commitment_config::CommitmentConfig::confirmed()),
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
        instructions: &[solana_sdk::instruction::Instruction],
        signers: impl IntoIterator<Item = &solana_sdk::signer::keypair::Keypair> + Send,
    ) -> solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta {
        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)
            .unwrap()
            .async_rpc();
        let mut signers = signers.into_iter().collect::<Vec<_>>();
        signers.push(self.payer());

        let tx = &solana_sdk::transaction::Transaction::new_signed_with_payer(
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
                anchor_client::solana_client::rpc_config::RpcTransactionConfig {
                    encoding: Some(solana_transaction_status::UiTransactionEncoding::Binary),
                    commitment: Some(solana_sdk::commitment_config::CommitmentConfig::confirmed()),
                    max_supported_transaction_version: None,
                },
            )
            .await
            .unwrap();

        transaction
    }

    /// Airdrops lamports to the chosen account.
    #[throws]
    pub async fn airdrop(&self, address: &solana_sdk::pubkey::Pubkey, lamports: u64) {
        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)
            .unwrap()
            .async_rpc();

        let signature = rpc_client.request_airdrop(address, lamports).await.unwrap();

        let (airdrop_result, error) = loop {
            match rpc_client.get_signature_status(&signature).await.unwrap() {
                Some(Ok(_)) => {
                    log::debug!("{} lamports airdropped", lamports);
                    break (true, None);
                }
                Some(Err(transaction_error)) => break (false, Some(transaction_error)),
                None => std::thread::sleep(std::time::Duration::from_millis(500)),
            }
        };
        if !airdrop_result {
            throw!(Error::SolanaClientError(error.unwrap().into()));
        }
    }

    /// Get balance of an account
    #[throws]
    pub async fn get_balance(&mut self, address: &solana_sdk::pubkey::Pubkey) -> u64 {
        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)?
            .async_rpc();
        rpc_client.get_balance(address).await?
    }

    /// Get token balance of an token account
    #[throws]
    pub async fn get_token_balance(
        &mut self,
        address: &solana_sdk::pubkey::Pubkey,
    ) -> solana_account_decoder::parse_token::UiTokenAmount {
        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)?
            .async_rpc();
        rpc_client.get_token_account_balance(address).await?
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
    pub async fn deploy_by_name(
        &self,
        program_keypair: &solana_sdk::signer::keypair::Keypair,
        program_name: &str,
    ) {
        log::debug!("reading program data");

        let reader = Reader::new();

        let mut program_data = reader
            .program_data(program_name)
            .await
            .expect("reading program data failed");

        log::debug!("airdropping the minimum balance required to deploy the program");

        // TODO: This will fail on devnet where airdrops are limited to 1 SOL

        self.airdrop(&self.payer().pubkey(), 5_000_000_000)
            .await
            .expect("airdropping for deployment failed");

        log::debug!("deploying program");

        self.deploy(program_keypair, std::mem::take(&mut program_data))
            .await?;

        // this will slow down the process because if we call program instruction right after deploy,
        // data are not yet completely deployed so error occures
        let deploy_done = loop {
            match self.anchor_client.program(program_keypair.pubkey()) {
                Ok(_) => {
                    log::debug!("program deployed succefully");
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    break true;
                }
                Err(_) => {
                    std::thread::sleep(std::time::Duration::from_millis(500));
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
    async fn deploy(
        &self,
        program_keypair: &solana_sdk::signer::keypair::Keypair,
        program_data: Vec<u8>,
    ) {
        const PROGRAM_DATA_CHUNK_SIZE: usize = 900;

        let program_data_len = program_data.len();

        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)
            .unwrap()
            .async_rpc();

        let system_program = self
            .anchor_client
            .program(solana_sdk::system_program::ID)
            .unwrap();

        log::debug!("program_data_len: {}", program_data_len);

        log::debug!("create program account");

        let min_balance_for_rent_exemption = rpc_client
            .get_minimum_balance_for_rent_exemption(program_data_len)
            .await
            .unwrap();

        let create_account_ix: solana_sdk::instruction::Instruction =
            solana_sdk::system_instruction::create_account(
                &self.payer.pubkey(),
                &program_keypair.pubkey(),
                min_balance_for_rent_exemption,
                program_data_len as u64,
                &solana_sdk::bpf_loader::id(),
            );
        system_program
            .request()
            .instruction(create_account_ix)
            .signer(program_keypair)
            .send()
            .await
            .unwrap();

        log::debug!("write program data");

        let mut offset = 0usize;
        let mut futures_vec = Vec::new();

        for chunk in program_data.chunks(PROGRAM_DATA_CHUNK_SIZE) {
            let loader_write_ix = solana_sdk::loader_instruction::write(
                &program_keypair.pubkey(),
                &solana_sdk::bpf_loader::id(),
                offset as u32,
                chunk.to_vec(),
            );
            futures_vec.push(async {
                let system_program = self
                    .anchor_client
                    .program(solana_sdk::system_program::ID)
                    .unwrap();
                system_program
                    .request()
                    .instruction(loader_write_ix)
                    .signer(program_keypair)
                    .send()
                    .await
                    .unwrap();
            });
            offset += chunk.len();
        }
        futures::stream::iter(futures_vec)
            .buffer_unordered(500)
            .collect::<Vec<_>>()
            .await;

        log::debug!("finalize program");

        let loader_finalize_ix = solana_sdk::loader_instruction::finalize(
            &program_keypair.pubkey(),
            &solana_sdk::bpf_loader::id(),
        );
        system_program
            .request()
            .instruction(loader_finalize_ix)
            .signer(program_keypair)
            .send()
            .await
            .unwrap();

        log::debug!("program deployed");
    }

    /// Creates accounts.
    #[throws]
    pub async fn create_account(
        &self,
        keypair: &solana_sdk::signer::keypair::Keypair,
        lamports: u64,
        space: u64,
        owner: &solana_sdk::pubkey::Pubkey,
    ) -> solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta {
        self.send_transaction(
            &[solana_sdk::system_instruction::create_account(
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
        keypair: &solana_sdk::signer::keypair::Keypair,
        space: u64,
        owner: &solana_sdk::pubkey::Pubkey,
    ) -> solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta {
        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)?
            .async_rpc();
        self.send_transaction(
            &[solana_sdk::system_instruction::create_account(
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
        mint: &solana_sdk::signer::keypair::Keypair,
        authority: &solana_sdk::pubkey::Pubkey,
        freeze_authority: Option<&solana_sdk::pubkey::Pubkey>,
        decimals: u8,
    ) -> solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta {
        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)?
            .async_rpc();
        self.send_transaction(
            &[
                solana_sdk::system_instruction::create_account(
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
                    authority,
                    freeze_authority,
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
        mint: &solana_sdk::pubkey::Pubkey,
        authority: &solana_sdk::signer::keypair::Keypair,
        account: &solana_sdk::pubkey::Pubkey,
        amount: u64,
    ) -> solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta {
        self.send_transaction(
            &[spl_token::instruction::mint_to(
                &spl_token::ID,
                mint,
                account,
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
        account: &solana_sdk::signer::keypair::Keypair,
        mint: &solana_sdk::pubkey::Pubkey,
        owner: &solana_sdk::pubkey::Pubkey,
    ) -> solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta {
        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)?
            .async_rpc();
        self.send_transaction(
            &[
                solana_sdk::system_instruction::create_account(
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
    pub async fn create_associated_token_account(
        &self,
        owner: &solana_sdk::signer::keypair::Keypair,
        mint: &solana_sdk::pubkey::Pubkey,
    ) -> solana_sdk::pubkey::Pubkey {
        self.send_transaction(
            &[
                spl_associated_token_account::instruction::create_associated_token_account(
                    &self.payer().pubkey(),
                    &owner.pubkey(),
                    mint,
                    &spl_token::ID,
                ),
            ],
            &[],
        )
        .await?;
        spl_associated_token_account::get_associated_token_address(&owner.pubkey(), mint)
    }

    /// Executes a transaction creating and filling the given account with the given data.
    /// The account is required to be empty and will be owned by bpf_loader afterwards.
    #[throws]
    pub async fn create_account_with_data(
        &self,
        account: &solana_sdk::signer::keypair::Keypair,
        data: Vec<u8>,
    ) {
        const DATA_CHUNK_SIZE: usize = 900;

        let rpc_client = self
            .anchor_client
            .program(solana_sdk::system_program::ID)?
            .async_rpc();
        self.send_transaction(
            &[solana_sdk::system_instruction::create_account(
                &self.payer().pubkey(),
                &account.pubkey(),
                rpc_client
                    .get_minimum_balance_for_rent_exemption(data.len())
                    .await?,
                data.len() as u64,
                &solana_sdk::bpf_loader::id(),
            )],
            [account],
        )
        .await?;

        let mut offset = 0usize;
        for chunk in data.chunks(DATA_CHUNK_SIZE) {
            log::debug!("writing bytes {} to {}", offset, offset + chunk.len());
            self.send_transaction(
                &[solana_sdk::loader_instruction::write(
                    &account.pubkey(),
                    &solana_sdk::bpf_loader::id(),
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

impl PrintableTransaction for solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta {
    fn print_named(&self, name: &str) {
        let tx = self.transaction.transaction.decode().unwrap();
        log::debug!("EXECUTE {} (slot {})", name, self.slot);
        match self.transaction.meta.clone() {
            Some(meta) => {
                solana_cli_output::display::println_transaction(&tx, Some(&meta), "  ", None, None)
            }
            _ => solana_cli_output::display::println_transaction(&tx, None, "  ", None, None),
        }
    }
}
