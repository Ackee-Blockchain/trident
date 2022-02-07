use anchor_client::{
    anchor_lang::{
        solana_program::program_pack::Pack,
        ToAccountMetas, InstructionData, AccountDeserialize, System, Id
    },
    Client as AnchorClient, 
    Cluster,
    solana_client::{client_error::ClientErrorKind, rpc_config::RpcTransactionConfig},
    ClientError as Error,
    Program,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        loader_instruction, 
        bpf_loader, 
        system_instruction,
        signer::{Signer, keypair::Keypair},
        pubkey::Pubkey,
        instruction::Instruction,
        transaction::Transaction,
        account::Account,
    }
};
use solana_transaction_status::{UiTransactionEncoding, EncodedConfirmedTransaction};
use solana_cli_output::display::println_transaction;
use tokio::task;
use std::{time::Duration, thread::sleep};
use futures::future::try_join_all;
use fehler::throws;
use spl_token;
use spl_associated_token_account::get_associated_token_address;
use crate::TempClone;

pub struct Client {
    payer: Keypair,
    anchor_client: AnchorClient,
}

impl Client {
    pub fn new(payer: Keypair) -> Self {
        Self {
            payer: payer.clone(),
            anchor_client: AnchorClient::new_with_options(
                Cluster::Localnet,
                payer,
                CommitmentConfig::confirmed(),
            )
        }
    }

    pub fn payer(&self) -> &Keypair {
        &self.payer
    }

    pub fn anchor_client(&self) -> &AnchorClient {
        &self.anchor_client
    }

    pub fn program(&self, program_id: Pubkey) -> Program {
        self.anchor_client.program(program_id)
    }

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
        }).await.expect("is_localnet_running task failed")
    }

    #[throws]
    pub async fn account_data<T>(&self, account: Pubkey) -> T
        where T: AccountDeserialize + Send + 'static
    {
        let dummy_pubkey = Pubkey::new_from_array([0; 32]);
        let program = self.program(dummy_pubkey);
        task::spawn_blocking(move || {
            program.account::<T>(account)
        })
        .await.expect("account_data task failed")?
    }

    #[throws]
    pub async fn get_account(&self, account: Pubkey) -> Option<Account> {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        task::spawn_blocking(move || rpc_client
            .get_account_with_commitment(&account, rpc_client.commitment())
            .expect("get_account task failed")
            .value
        )
        .await
        .expect("get_account task failed")
    }

    #[throws]
    pub async fn send_instruction(
        &self, 
        program: Pubkey,
        instruction: impl InstructionData + Send + 'static,
        accounts: impl ToAccountMetas + Send + 'static,
        signers: impl IntoIterator<Item = Keypair> + Send + 'static,
    ) -> EncodedConfirmedTransaction {
        let program = self.program(program);
        let signature = task::spawn_blocking(move || {
            let mut request = program
                .request()
                .args(instruction)
                .accounts(accounts);
            let signers = signers.into_iter().collect::<Vec<_>>();
            for signer in &signers {
                request = request.signer(signer);
            }
            request.send()
        }).await.expect("send instruction task failed")?;

        let rpc_client = self.anchor_client.program(System::id()).rpc();
        task::spawn_blocking(move || {
            rpc_client.get_transaction_with_config(
                &signature, 
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Binary),
                    commitment: Some(CommitmentConfig::confirmed()),
                }
            )
        }).await.expect("get transaction task failed")?
    }

    #[throws]
    pub async fn send_transaction(
        &self,
        instructions: &[Instruction],
        signers: impl IntoIterator<Item = &Keypair> + Send,
    ) -> EncodedConfirmedTransaction {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        let mut signers = signers.into_iter().collect::<Vec<_>>();
        signers.push(self.payer());
        
        let tx =
            &Transaction::new_signed_with_payer(
                instructions,
                Some(&self.payer.pubkey()),
                &signers,
                rpc_client.get_recent_blockhash()
                    .expect("Error while getting recent blockhash")
                    .0,
            );
        // @TODO make this call async with task::spawn_blocking
        let signature = rpc_client.send_and_confirm_transaction(tx)?;
        let transaction = task::spawn_blocking(move || {
            rpc_client.get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    encoding: Some(UiTransactionEncoding::Binary),
                    commitment: Some(CommitmentConfig::confirmed()),
                }
            )
        }).await.expect("get transaction task failed")?;
        
        transaction
    }

    #[throws]
    pub async fn airdrop(&self, address: Pubkey, lamports: u64) {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        task::spawn_blocking(move || -> Result<(), Error> {
            let signature = rpc_client.request_airdrop(
                &address, 
                lamports,
            )?;
            for _ in 0..5 {
                match rpc_client.get_signature_status(&signature)? {
                    Some(Ok(_)) => { 
                        println!("{} lamports airdropped", lamports);
                        return Ok(())
                    },
                    Some(Err(transaction_error)) => Err(Error::SolanaClientError(transaction_error.into()))?,
                    None => sleep(Duration::from_millis(500)),
                }
            }
            Err(Error::SolanaClientError(ClientErrorKind::Custom("Airdrop transaction has not been processed yet".to_owned()).into()))?
        }).await.expect("airdrop task failed")?
    }

    #[throws]
    pub async fn deploy(&self, program_keypair: Keypair, program_data: Vec<u8>) {
        const PROGRAM_DATA_CHUNK_SIZE: usize = 900;

        let program_pubkey = program_keypair.pubkey();
        let system_program = self.anchor_client.program(System::id());

        let program_data_len = program_data.len();
        println!("program_data_len: {}", program_data_len);

        println!("create program account");

        let rpc_client = system_program.rpc();
        let min_balance_for_rent_exemption = task::spawn_blocking(move || {
            rpc_client.get_minimum_balance_for_rent_exemption(program_data_len)
        }).await.expect("crate program account task failed")?;

        let create_account_ix = system_instruction::create_account(
            &system_program.payer(),
            &program_pubkey,
            min_balance_for_rent_exemption,
            program_data_len as u64,
            &bpf_loader::id(),
        );
        {
            let system_program = self.anchor_client.program(System::id());
            let program_keypair = Keypair::from_bytes(&program_keypair.to_bytes()).unwrap();
            task::spawn_blocking(move || {
                system_program
                    .request()
                    .instruction(create_account_ix)
                    .signer(&program_keypair)
                    .send()
            }).await.expect("create program account task failed")?;
        }

        println!("write program data");

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
            let system_program = self.anchor_client.program(System::id());
            futures.push(async move {
                task::spawn_blocking(move || {
                    system_program
                        .request()
                        .instruction(loader_write_ix) 
                        .signer(&program_keypair)
                        .send()
                }).await.expect("write program data task failed")
            });
            offset += chunk.len();
        }
        try_join_all(futures).await?;

        println!("finalize program");
        
        let loader_finalize_ix = loader_instruction::finalize(
            &program_pubkey, 
            &bpf_loader::id(),
        );
        task::spawn_blocking(move || {
            system_program
                .request()
                .instruction(loader_finalize_ix)
                .signer(&program_keypair)
                .send()
        }).await.expect("finalize program account task failed")?;

        println!("program deployed");
    }

    #[throws]
    pub async fn create_account(
        &self, 
        keypair: &Keypair, 
        lamports: u64, 
        space: u64, 
        owner: &Pubkey
    ) -> EncodedConfirmedTransaction {
        self.send_transaction(
            &[system_instruction::create_account(
                &self.payer().pubkey(), 
                &keypair.pubkey(), 
                lamports, 
                space, 
                owner)
            ],
            [keypair]
        ).await?
    }

    #[throws]
    pub async fn create_account_rent_exempt(
        &mut self, 
        keypair: &Keypair, 
        space: u64, 
        owner: &Pubkey
    ) -> EncodedConfirmedTransaction { 
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        self.send_transaction(
            &[system_instruction::create_account(
                &self.payer().pubkey(), 
                &keypair.pubkey(), 
                rpc_client.get_minimum_balance_for_rent_exemption(space as usize)?,
                space, 
                owner)
            ],
            [keypair]
        ).await?
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
                    rpc_client.get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?,
                    spl_token::state::Mint::LEN as u64,
                    &spl_token::ID
                ),
                spl_token::instruction::initialize_mint(
                    &spl_token::ID, 
                    &mint.pubkey(), 
                    &authority, 
                    freeze_authority.as_ref(), 
                    decimals
                )
                .unwrap()
            ],
            [mint]
        ).await?
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
            &[
                spl_token::instruction::mint_to(
                    &spl_token::ID,
                    &mint,
                    &account,
                    &authority.pubkey(),
                    &[], 
                    amount,
                )
                .unwrap()
            ], 
            [authority]
        ).await?
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
                    rpc_client.get_minimum_balance_for_rent_exemption(spl_token::state::Account::LEN)?,
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
            [account]
        ).await?
    }

    /// Executes a transaction constructing the associated token account of the specified mint belonging to the owner. This will fail if the account already exists.
    #[throws]
    pub async fn create_associated_token_account(
        &self,
        owner: &Keypair, 
        mint: Pubkey,
    ) -> Pubkey {
        self.send_transaction(
            &[
                spl_associated_token_account::create_associated_token_account(
                    &self.payer().pubkey(),
                    &owner.pubkey(),
                    &mint,
                ),
            ],
            &[],
        ).await?;
        get_associated_token_address(&owner.pubkey(), &mint)
    }

    /// Executes a transaction creating and filling the given account with the given data.
    /// The account is required to be empty and will be owned by bpf_loader afterwards.
    #[throws]
    pub async fn create_account_with_data(&self, account: &Keypair, data: Vec<u8>) {
        const DATA_CHUNK_SIZE: usize = 900;
        
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        self.send_transaction(
            &[
                system_instruction::create_account(
                    &self.payer().pubkey(), 
                    &account.pubkey(),
                    rpc_client.get_minimum_balance_for_rent_exemption(data.len())?,
                    data.len() as u64,
                    &bpf_loader::id(),
                )
            ],
            [account]
        ).await?;

        let mut offset = 0usize;
        for chunk in data.chunks(DATA_CHUNK_SIZE) {
            println!("writing bytes {} to {}", offset, offset + chunk.len());
            self.send_transaction(
                &[loader_instruction::write(
                    &account.pubkey(),
                    &bpf_loader::id(),
                    offset as u32,
                    chunk.to_vec(),
                )],
                [account],
            ).await?;
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
        println!("EXECUTE {} (slot {})", name, self.slot);
        println_transaction(&tx, &self.transaction.meta, "  ", None, None);
    }
}