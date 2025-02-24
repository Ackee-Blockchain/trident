use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, TridentInstruction)]
#[program_id("3XtULmXDGS867VbBXiPkjYr4EMjytGW8X12F6BS23Zcw")]
# [discriminator ([175u8 , 175u8 , 109u8 , 31u8 , 13u8 , 152u8 , 155u8 , 237u8 ,])]
pub struct InitializeInstruction {
    pub accounts: InitializeInstructionAccounts,
    pub data: InitializeInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct InitializeInstructionAccounts {
    #[account(signer,mut,storage = signer)]
    pub signer: TridentAccount,
    #[account(signer,mut,storage = mint)]
    pub mint: TridentAccount,
    #[account(mut)]
    pub metadata_account: TridentAccount,
    #[account(address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s", skip_snapshot)]
    pub mpl_token_metadata: TridentAccount,
    #[account(address = "11111111111111111111111111111111", skip_snapshot)]
    pub system_program: TridentAccount,
    #[account(address = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", skip_snapshot)]
    pub token_program: TridentAccount,
}
/// Instruction Data
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct InitializeInstructionData {
    pub input: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
/// Implementation of instruction setters for fuzzing
///
/// Provides methods to:
/// - Set instruction data during fuzzing
/// - Configure instruction accounts during fuzzing
/// - (Optional) Set remaining accounts during fuzzing
impl InstructionSetters for InitializeInstruction {
    type IxAccounts = FuzzAccounts;
    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        let mint = self.accounts.mint.pubkey();

        let metadata_account = fuzz_accounts.metadata_account.get_or_create(
            self.accounts.metadata_account.account_id,
            client,
            Some(PdaSeeds::new(
                &[
                    b"metadata",
                    pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").as_ref(),
                    mint.as_ref(),
                ],
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
            )),
            None,
        );
        self.accounts.metadata_account.set_address(metadata_account);
    }
}
