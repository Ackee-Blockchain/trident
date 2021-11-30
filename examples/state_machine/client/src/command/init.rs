use anyhow::Error;
use fehler::throws;
use tokio::{task, fs};
use crate::anchor_helpers::{new_client, read_keypair};
use anchor_client::{
    anchor_lang::{System, Id},
    solana_sdk::{
        signer::{Signer, keypair::Keypair}, 
        loader_instruction, 
        bpf_loader, 
        system_instruction, 
    },
    solana_client::client_error::{ClientErrorKind, ClientError}
};
use futures::future::try_join_all;
use std::{time::Duration, thread::sleep};

#[throws]
pub async fn init() {
    let id_keypair = read_keypair("id").await?;
    let id_pubkey = id_keypair.pubkey();

    let client = new_client(id_keypair);
    let system_program = client.program(System::id());

    // -- airdrop --

    println!("AIRDROP started");

    let rpc_client = system_program.rpc();
    task::spawn_blocking(move || -> Result<(), ClientError> {
        let signature = rpc_client.request_airdrop(
            &id_pubkey, 
            5_000_000_000,
        )?;
        for _ in 0..5 {
            match rpc_client.get_signature_status(&signature)? {
                Some(Ok(_)) => return Ok(()),
                Some(Err(transaction_error)) => Err(transaction_error)?,
                None => sleep(Duration::from_millis(500)),
            }
        }
        Err(ClientErrorKind::Custom("Airdrop transaction has not been processed yet".to_owned()))?
    }).await??;

    println!("AIRDROP finished");

    // -- deploy --

    println!("DEPLOY started");

    let program_keypair = read_keypair("program").await?;
    let program_pubkey = program_keypair.pubkey();

    let program_data = fs::read("./target/deploy/turnstile.so").await?;
    let program_data_len = program_data.len();
    println!("program_data_len: {}", program_data_len);

    println!("create program account");

    let rpc_client = system_program.rpc();
    let min_balance_for_rent_exemption = task::spawn_blocking(move || {
        rpc_client.get_minimum_balance_for_rent_exemption(program_data_len)
    }).await??;

    let create_account_ix = system_instruction::create_account(
        &system_program.payer(),
        &program_pubkey,
        min_balance_for_rent_exemption,
        program_data_len as u64,
        &bpf_loader::id(),
    );
    let create_program_result = {
        let system_program = client.program(System::id());
        let program_keypair = Keypair::from_bytes(&program_keypair.to_bytes()).unwrap();
        task::spawn_blocking(move || {
            system_program
                .request()
                .instruction(create_account_ix)
                .signer(&program_keypair)
                .send()
        }).await?
    };
    if let Err(error) = create_program_result {
        eprintln!("create_program error: '{}'", error);
    }

    println!("write program data");
    
    // @TODO calculate max chunk data?
    // https://github.com/solana-labs/solana/blob/3c7cb2522c23ace076af88dc1433516364fba16d/cli/src/program.rs#L1786
    // https://github.com/neodyme-labs/solana-poc-framework/blob/5ad2f995ea9d45bc7c6f2ea0f37ef5eb8c2dd77f/src/lib.rs#L270
    let mut offset = 0usize;
    let mut futures = Vec::new();
    for chunk in program_data.chunks(900) {
        let program_keypair = Keypair::from_bytes(&program_keypair.to_bytes()).unwrap();
        // println!("writing program bytes {} to {}", offset, offset + chunk.len());
        let loader_write_ix = loader_instruction::write(
            &program_pubkey, 
            &bpf_loader::id(),
            offset as u32,
            chunk.to_vec(), // @TODO optimize?
        );
        let system_program = client.program(System::id());
        futures.push(task::spawn_blocking(move || {
            system_program
                .request()
                .instruction(loader_write_ix)
                .signer(&program_keypair)
                .send()
        }));
        offset += chunk.len();
    }
    try_join_all(futures).await?;

    println!("finalize program");
    
    let loader_finalize_ix = loader_instruction::finalize(
        &program_pubkey, 
        &bpf_loader::id(),
    );
    {
        let system_program = client.program(System::id());
        let program_keypair = Keypair::from_bytes(&program_keypair.to_bytes()).unwrap();
        task::spawn_blocking(move || {
            system_program
                .request()
                .instruction(loader_finalize_ix)
                .signer(&program_keypair)
                .send()
        }).await??;
    }

    println!("DEPLOY finished");

    // -- init state --

    println!("INIT STATE started");

    let program = client.program(program_pubkey);

    let state = read_keypair("state").await?;
    task::spawn_blocking(move || {
        program
            .request()
            .args(turnstile::instruction::Initialize)
            .accounts(turnstile::accounts::Initialize { 
                state: state.pubkey(),
                user: program.payer(),
                system_program: System::id()
            })
            .signer(&state)
            .send()
    }).await??;

    println!("INIT STATE finished");

    println!("Initialized");
}
