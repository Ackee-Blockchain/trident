use anchor_lang::solana_program::hash::Hash;
use anchor_lang::{InstructionData, ToAccountMetas};
use assert_matches::*;
use fuzz_trdelnik::entry;
use honggfuzz::fuzz;
use solana_program_test::{
    processor, tokio::runtime::Runtime, BanksClient, BanksClientError, ProgramTest,
};

use arbitrary::Arbitrary;
use trdelnik_client::{
    anchor_lang, solana_sdk::transaction::Transaction, Instruction, Keypair, Pubkey, Signer,
};

#[derive(Arbitrary)]
pub struct FuzzData {
    input1: u8,
    // input2: u8,
}

fn main() {
    let program_id = fuzz_trdelnik::id();

    let rt = Runtime::new().unwrap();
    let program_test = ProgramTest::new("fuzz_trdelnik", program_id, processor!(entry));

    let mut ctx = rt.block_on(program_test.start_with_context());

    let counter = Keypair::new();

    let instr = fuzz_trdelnik::instruction::Initialize {};
    let accounts = fuzz_trdelnik::accounts::Initialize {
        counter: counter.pubkey(),
        user: ctx.payer.pubkey(),
        system_program: anchor_lang::system_program::ID,
    };

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id,
            accounts: accounts.to_account_metas(None),
            data: instr.data(),
        }],
        Some(&ctx.payer.pubkey().clone()),
    );
    transaction.sign(&[&ctx.payer, &counter], ctx.last_blockhash);
    let res = rt.block_on(ctx.banks_client.process_transaction(transaction));
    assert_matches!(res, Ok(()));

    loop {
        fuzz!(|fuzz_data: FuzzData| {
            rt.block_on(async {
                let res = fuzz_with_runtime(
                    &fuzz_data,
                    &mut ctx.banks_client,
                    &ctx.payer,
                    &counter,
                    ctx.last_blockhash,
                    &program_id,
                )
                .await;
                assert_matches!(res, Ok(()));
            });
        });
    }
}

async fn fuzz_with_runtime(
    fuzz_data: &FuzzData,
    banks_client: &mut BanksClient,
    payer: &Keypair,
    counter: &Keypair,
    blockhash: Hash,
    program_id: &Pubkey,
) -> Result<(), BanksClientError> {
    let instr = fuzz_trdelnik::instruction::Update {
        input1: fuzz_data.input1,
        input2: 2,
    };

    let accounts = fuzz_trdelnik::accounts::Update {
        counter: counter.pubkey(),
        authority: payer.pubkey(),
    };

    let mut transaction = Transaction::new_with_payer(
        &[Instruction {
            program_id: *program_id,
            accounts: accounts.to_account_metas(None),
            data: instr.data(),
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[payer], blockhash);

    banks_client.process_transaction(transaction).await
}
