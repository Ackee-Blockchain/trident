use assert_matches::*;
use trdelnik_client::fuzzing::*;

#[derive(Arbitrary)]
pub struct FuzzData {
    param1: u8,
    param2: u8,
}

fn main() {
    let rt = Runtime::new().unwrap();
    let program_test = ProgramTest::new(PROGRAM_NAME, PROGRAM_ID, processor!(entry));

    let mut ctx = rt.block_on(program_test.start_with_context());

    // TODO: replace this instruction with one of your generated instructions from trdelnik_client
    let init_ix = init_dummy_ix();
    let mut transaction =
        Transaction::new_with_payer(&[init_ix], Some(&ctx.payer.pubkey().clone()));

    transaction.sign(&[&ctx.payer], ctx.last_blockhash);
    let res = rt.block_on(ctx.banks_client.process_transaction(transaction));
    assert_matches!(res, Ok(()));

    loop {
        fuzz!(|fuzz_data: FuzzData| {
            rt.block_on(async {
                let res = fuzz_ix(
                    &fuzz_data,
                    &mut ctx.banks_client,
                    &ctx.payer,
                    ctx.last_blockhash,
                )
                .await;
                assert_matches!(res, Ok(()));
            });
        });
    }
}

async fn fuzz_ix(
    fuzz_data: &FuzzData,
    banks_client: &mut BanksClient,
    payer: &Keypair,
    blockhash: Hash,
) -> core::result::Result<(), BanksClientError> {
    // TODO: replace this instruction with one of your generated instructions from trdelnik_client
    let update_ix = update_dummy_ix(fuzz_data.param1, fuzz_data.param2);

    let mut transaction = Transaction::new_with_payer(&[update_ix], Some(&payer.pubkey()));
    transaction.sign(&[payer], blockhash);

    banks_client.process_transaction(transaction).await
}

fn init_dummy_ix() -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        data: vec![],
        accounts: vec![],
    }
}

fn update_dummy_ix(param1: u8, param2: u8) -> Instruction {
    Instruction {
        program_id: PROGRAM_ID,
        data: vec![param1, param2],
        accounts: vec![],
    }
}
