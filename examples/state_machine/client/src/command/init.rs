use anyhow::Error;
use fehler::throws;
use tokio::{task, fs, time::{sleep, Duration}};
use crate::anchor_helpers::{new_client, read_keypair};
use anchor_client::{
    anchor_lang::{System, Id},
    solana_sdk::{
        signer::Signer, 
        loader_instruction, 
        bpf_loader, 
        system_instruction, 
        commitment_config::CommitmentConfig
    },
    solana_client::rpc_config::RpcRequestAirdropConfig,
};

#[throws]
pub async fn init() {
    let id_keypair = read_keypair("id").await?;
    let id_pubkey = id_keypair.pubkey();

    let client = new_client(id_keypair);
    let system_program = client.program(System::id());

    // -- airdrop --

    println!("AIRDROP started");

    // @TODO commitment finalize? To make sure we have lamports available for the next step. 

    let rpc_client = system_program.rpc();
    task::spawn_blocking(move || {
        rpc_client.request_airdrop(
            &id_pubkey, 
            5_000_000_000,
        )
        // @TODO how to wait for airdropped lamports?
        // rpc_client.request_airdrop_with_config(
        //     &id_pubkey, 
        //     1_000_000_000,
        //     RpcRequestAirdropConfig {
        //         recent_blockhash: None,
        //         commitment: Some(CommitmentConfig::finalized()),
        //     }
        // )
    }).await??;

    println!("AIRDROP finished");

    // @TODO how to wait for airdropped lamports?
    sleep(Duration::from_secs(1)).await;

    // -- deploy --

    println!("DEPLOY started");

    let program_keypair = read_keypair("program").await?;
    let program_pubkey = program_keypair.pubkey();

    let program_data = fs::read("./target/deploy/turnstile.so").await?;
    let program_data_len = program_data.len();
    println!("program_data_len: {}", program_data_len);

    println!("create program account");

    let create_account_ix = system_instruction::create_account(
        &system_program.payer(),
        &program_pubkey,
        // TODO make async
        system_program.rpc().get_minimum_balance_for_rent_exemption(program_data_len)?,
        program_data_len as u64,
        &bpf_loader::id(),
    );
    let create_program_result = system_program
        .request()
        .instruction(create_account_ix)
        .signer(&program_keypair)
        // @TODO how to wait for airdropped lamports?
        // .options(CommitmentConfig::finalized())
        .send(); // TODO make async
    if let Err(error) = create_program_result {
        eprintln!("create_program error: '{}'", error);
    }

    println!("write program data");
    
    // @TODO calculate max chunk data?
    // https://github.com/solana-labs/solana/blob/3c7cb2522c23ace076af88dc1433516364fba16d/cli/src/program.rs#L1786
    // https://github.com/neodyme-labs/solana-poc-framework/blob/5ad2f995ea9d45bc7c6f2ea0f37ef5eb8c2dd77f/src/lib.rs#L270
    let mut offset = 0usize;
    // @TODO make async!
    for chunk in program_data.chunks(900) {
        println!("writing program bytes {} to {}", offset, offset + chunk.len());
        let loader_write_ix = loader_instruction::write(
            &program_pubkey, 
            &bpf_loader::id(),
            offset as u32,
            chunk.to_vec(), // @TODO optimize?
        );
        system_program
            .request()
            .instruction(loader_write_ix)
            .signer(&program_keypair)
            .options(CommitmentConfig::processed()) // @TODO remove?
            .send()?; // @TODO make async
        offset += chunk.len();
    }

    println!("start program");
    
    let loader_finalize_ix = loader_instruction::finalize(
        &program_pubkey, 
        &bpf_loader::id(),
    );
    system_program
        .request()
        .instruction(loader_finalize_ix)
        .signer(&program_keypair)
        .send()?; // TODO make async

    println!("send transaction");

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
