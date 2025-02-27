use fuzz_transactions::{FuzzAccounts, FuzzTransactions};
use trident_fuzz::fuzzing::*;
mod fuzz_transactions;
mod instructions;
mod transactions;
mod types;
use hello_world::entry as entry_hello_world;
pub use transactions::*;

#[derive(FuzzTestExecutor)]
struct FuzzTest;

#[flow_executor]
impl FuzzTest {
    #[init]
    fn start(&mut self, client: &mut impl FuzzClient) {
        let program_hello_world = ProgramEntrypoint::new(
            pubkey!("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit"),
            None,
            processor!(entry_hello_world),
        );

        client.deploy(&program_hello_world);
    }

    #[flow]
    fn flow1(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        client: &mut impl FuzzClient,
        accounts: &mut FuzzAccounts,
    ) -> std::result::Result<(), arbitrary::Error> {
        let mut tx1 = InitializeFnTransaction::build(fuzzer_data, client, accounts)?;


        tx1.execute_with_hooks(client, accounts).unwrap();

        let mut tx2 = InitializeFnTransaction2::build(fuzzer_data, client, accounts)?;

        tx2.instruction1.accounts.author.set_account_meta(address, is_signer, is_writable);

        let mut tx3 = InitializeFnTransaction2::build(fuzzer_data, client, accounts)?;

        tx1.execute_with_hooks(client, accounts).unwrap();

        tx2.execute_with_hooks(client, accounts).unwrap();

        tx3.execute(client, accounts).unwrap();

        tx3.post_transaction(client);

        tx3.transaction_invariant_check().unwrap();

        Ok(())
    }

    #[flow]
    fn flow2(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        client: &mut impl FuzzClient,
        accounts: &mut FuzzAccounts,
    ) -> std::result::Result<(), arbitrary::Error> {
        let mut tx1 = InitializeFnTransaction::build(fuzzer_data, client, accounts)?;

        tx1.execute_with_hooks(client, accounts).unwrap();

        Ok(())
    }
}

fn main() {
    FuzzTest::fuzz();
}
