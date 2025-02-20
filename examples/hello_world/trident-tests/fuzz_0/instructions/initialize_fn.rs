use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[program_id("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit")]
# [discriminator ([18u8 , 187u8 , 169u8 , 213u8 , 94u8 , 180u8 , 86u8 , 152u8 ,])]
pub struct InitializeFnInstruction {
    pub accounts: InitializeFnInstructionAccounts,
    pub data: InitializeFnInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct InitializeFnInstructionAccounts {
    pub author: TridentAccount,
    pub hello_world_account: TridentAccount,
    #[account(address = "11111111111111111111111111111111", skip_snapshot)]
    pub system_program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct InitializeFnInstructionData {
    pub input: u8,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionSetters for InitializeFnInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let author = fuzz_accounts.author.get_or_create_account(
            self.accounts.author.account_id,
            client,
            50 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .author
            .set_account_meta(author.pubkey(), true, true);

        let hello_world_account = fuzz_accounts.hello_world_account.get_or_create_account(
            self.accounts.hello_world_account.account_id,
            client,
            &[b"hello_world_seed"],
            &self.get_program_id(),
        );
        self.accounts
            .hello_world_account
            .set_account_meta(hello_world_account, false, true);
    }
}
