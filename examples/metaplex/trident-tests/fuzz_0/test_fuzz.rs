use fuzz_accounts::*;
use trident_fuzz::fuzzing::*;
mod fuzz_accounts;
mod types;
use types::*;

use crate::types::metaplex::{
    InitializeInstruction, InitializeInstructionAccounts, InitializeInstructionData,
};

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// Trident client for interacting with the Solana program
    trident: Trident,
    /// Storage for all account addresses used in fuzz testing
    fuzz_accounts: AccountAddresses,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: AccountAddresses::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        // Perform any initialization here, this method will be executed
        // at the start of each iteration

        let signer = self.fuzz_accounts.signer.insert(&mut self.trident, None);

        let mint = self.fuzz_accounts.mint.insert(&mut self.trident, None);

        let metadata_account = self.fuzz_accounts.metadata_account.insert(
            &mut self.trident,
            Some(PdaSeeds::new(
                &[
                    b"metadata",
                    pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").as_ref(),
                    mint.as_ref(),
                ],
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
            )),
        );

        self.trident.airdrop(&signer, 10 * LAMPORTS_PER_SOL);

        let ix = InitializeInstruction::data(InitializeInstructionData::new(
            self.trident.random_from_range(0..=u8::MAX),
            self.trident.random_string(10),
            self.trident.random_string(5),
            self.trident.random_string(25),
        ))
        .accounts(InitializeInstructionAccounts::new(
            signer,
            mint,
            metadata_account,
            pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
        ))
        .instruction();

        let _ = self.trident.process_transaction(&[ix], Some("initialize"));
    }

    #[flow]
    fn flow1(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows
    }

    #[flow]
    fn flow2(&mut self) {
        // Perform logic which is meant to be fuzzed
        // This flow is selected randomly from other flows
    }

    #[end]
    fn end(&mut self) {
        // Perform any cleanup here, this method will be executed
        // at the end of each iteration
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
