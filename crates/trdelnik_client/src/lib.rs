use anchor_client::{
    anchor_lang::{System, Id},
    Client as AnchorClient, 
    Cluster,
    solana_client::client_error::ClientErrorKind,
    ClientError,
    Program,
};
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signer::{Signer, keypair::Keypair}, 
    loader_instruction, 
    bpf_loader, 
    system_instruction, 
};
use tokio::{fs, task};
use std::{time::Duration, thread::sleep, str::FromStr};
use futures::future::try_join_all;
use fehler::throws;
use anyhow::Error;

// @TODO REFACTOR

// @TODO define custom errors with `thiserror` and remove `anyhow` from deps 

#[throws]
pub async fn read_pubkey(name: &str) -> Pubkey {
    let path = format!("./keys/{}_pub.json", name);
    let key: String = serde_json::from_str(&fs::read_to_string(path).await?)?;
    Pubkey::from_str(&key)?
}

#[throws]
pub async fn read_keypair(name: &str) -> Keypair {
    let path = format!("./keys/{}.json", name);
    let bytes: Vec<u8> = serde_json::from_str(&fs::read_to_string(path).await?)?;
    Keypair::from_bytes(&bytes)?
}

pub struct TrdelnikClient {
    anchor_client: AnchorClient
}

impl TrdelnikClient {
    pub fn new(payer: Keypair) -> Self {
        Self {
            anchor_client: AnchorClient::new_with_options(
                Cluster::Localnet,
                payer,
                CommitmentConfig::confirmed(),
            )
        }
    }

    pub fn anchor_client(&self) -> &AnchorClient {
        &self.anchor_client
    }

    pub fn program(&self, program_id: Pubkey) -> Program {
        self.anchor_client.program(program_id)
    }

    pub async fn airdrop(&self, address: Pubkey, lamports: u64) -> Result<(), ClientError> {
        let rpc_client = self.anchor_client.program(System::id()).rpc();
        task::spawn_blocking(move || -> Result<(), ClientError> {
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
                    Some(Err(transaction_error)) => Err(ClientError::SolanaClientError(transaction_error.into()))?,
                    None => sleep(Duration::from_millis(500)),
                }
            }
            Err(ClientError::SolanaClientError(ClientErrorKind::Custom("Airdrop transaction has not been processed yet".to_owned()).into()))?
        }).await.expect("airdrop task failed")
    }

    pub async fn deploy(&self, program_keypair: Keypair, program_data: Vec<u8>) -> Result<(), ClientError> {
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
        
        // @TODO calculate max chunk data?
        // https://github.com/solana-labs/solana/blob/3c7cb2522c23ace076af88dc1433516364fba16d/cli/src/program.rs#L1786
        // https://github.com/neodyme-labs/solana-poc-framework/blob/5ad2f995ea9d45bc7c6f2ea0f37ef5eb8c2dd77f/src/lib.rs#L270
        let mut offset = 0usize;
        let mut futures = Vec::new();
        for chunk in program_data.chunks(900) { // @TODO optimize?
            let program_keypair = Keypair::from_bytes(&program_keypair.to_bytes()).unwrap();
            // println!("writing program bytes {} to {}", offset, offset + chunk.len());
            let loader_write_ix = loader_instruction::write(
                &program_pubkey, 
                &bpf_loader::id(),
                offset as u32,
                chunk.to_vec(), // @TODO optimize?
            );
            let system_program = self.anchor_client.program(System::id());
            futures.push(async {
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
        Ok(())
    }
}
