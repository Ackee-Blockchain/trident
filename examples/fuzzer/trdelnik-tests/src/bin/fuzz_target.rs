use fuzzer::entry;
use program_client::fuzzer_instruction::*;
const PROGRAM_NAME: &str = "fuzzer";
use assert_matches::*;
use trdelnik_client::fuzzing::*;

#[derive(Arbitrary)]
pub struct FuzzData {
    param1: u8,
    param2: u8,
}

fn main() {
    loop {
        fuzz!(|fuzz_data: FuzzData| {
            Runtime::new().unwrap().block_on(async {
                let program_test = ProgramTest::new(PROGRAM_NAME, PROGRAM_ID, processor!(entry));

                let mut ctx = program_test.start_with_context().await;

                let counter = Keypair::new();

                let init_ix =
                    initialize_ix(counter.pubkey(), ctx.payer.pubkey(), SYSTEM_PROGRAM_ID);
                let mut transaction =
                    Transaction::new_with_payer(&[init_ix], Some(&ctx.payer.pubkey().clone()));

                transaction.sign(&[&ctx.payer, &counter], ctx.last_blockhash);
                let res = ctx.banks_client.process_transaction(transaction).await;
                assert_matches!(res, Ok(()));

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
        fuzz_data.param1,
        fuzz_data.param2,
        counter.pubkey(),
        payer.pubkey(),
    );

    let mut transaction = Transaction::new_with_payer(&[update_ix], Some(&payer.pubkey()));
    transaction.sign(&[payer], blockhash);

    banks_client.process_transaction(transaction).await
}
