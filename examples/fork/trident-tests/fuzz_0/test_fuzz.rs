use fuzz_accounts::*;
use trident_fuzz::{fuzzing::*, invariant_eq};
mod fuzz_accounts;
mod types;
use types::*;

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

        let sender_wallet = self.trident.random_keypair();

        self.trident
            .airdrop(&sender_wallet.pubkey(), 100 * LAMPORTS_PER_SOL);

        let receiver_wallet = self.trident.random_keypair();

        let wrapped_sol = self
            .trident
            .get_mint(pubkey!("So11111111111111111111111111111111111111112"))
            .expect("Wrapped SOL not found");

        invariant_eq!(
            wrapped_sol.mint.decimals,
            9,
            "Wrapped SOL decimals should be 9"
        );

        let init_ix = fork::InitializeInstruction::data(fork::InitializeInstructionData::new())
            .accounts(fork::InitializeInstructionAccounts::new(
                sender_wallet.pubkey(),
                receiver_wallet.pubkey(),
            ))
            .instruction();

        let res = self
            .trident
            .process_transaction(&[init_ix], Some("Initialize"));

        invariant!(res.is_success());
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
    FuzzTest::fuzz(10, 0);
}
