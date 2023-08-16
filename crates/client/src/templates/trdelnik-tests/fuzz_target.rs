use assert_matches::*;
use fuzz_trdelnik::entry;

use trdelnik_client::fuzzing::*;

use program_client::fuzz_trdelnik_instruction::*;

#[derive(Arbitrary)]
pub struct FuzzData {
    input1: u8,
    input2: u8,
}

fn main() {
    let program_id = fuzz_trdelnik::id();

    let rt = Runtime::new().unwrap();
    let program_test = ProgramTest::new("fuzz_trdelnik", program_id, processor!(entry));

    let mut ctx = rt.block_on(program_test.start_with_context());

    let counter = Keypair::new();

    let init_ix = initialize_ix(counter.pubkey(), ctx.payer.pubkey(), SYSTEM_PROGRAM_ID);
    let mut transaction =
        Transaction::new_with_payer(&[init_ix], Some(&ctx.payer.pubkey().clone()));

    transaction.sign(&[&ctx.payer, &counter], ctx.last_blockhash);
    let res = rt.block_on(ctx.banks_client.process_transaction(transaction));
    assert_matches!(res, Ok(()));

    loop {
        fuzz!(|fuzz_data: FuzzData| {
            rt.block_on(async {
                let res = fuzz_update_ix(
                    &fuzz_data,
                    &mut ctx.banks_client,
                    &ctx.payer,
                    &counter,
                    ctx.last_blockhash,
                )
                .await;
                assert_matches!(res, Ok(()));
            });
        });
    }
}

async fn fuzz_update_ix(
    fuzz_data: &FuzzData,
    banks_client: &mut BanksClient,
    payer: &Keypair,
    counter: &Keypair,
    blockhash: Hash,
) -> core::result::Result<(), BanksClientError> {
    let update_ix = update_ix(
        fuzz_data.input1,
        fuzz_data.input2,
        counter.pubkey(),
        payer.pubkey(),
    );

    let mut transaction = Transaction::new_with_payer(&[update_ix], Some(&payer.pubkey()));
    transaction.sign(&[payer], blockhash);

    banks_client.process_transaction(transaction).await
}
