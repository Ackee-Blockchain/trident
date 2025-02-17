use crate::fuzz_transactions::FuzzAccounts;
use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
#[derive(Arbitrary, Debug, TridentInstruction)]
#[accounts("accounts")]
#[program_id("3XtULmXDGS867VbBXiPkjYr4EMjytGW8X12F6BS23Zcw")]
# [discriminator ([175u8 , 175u8 , 109u8 , 31u8 , 13u8 , 152u8 , 155u8 , 237u8 ,])]
pub struct InitializeInstruction {
    pub accounts: InitializeInstructionAccounts,
    pub data: InitializeInstructionData,
}
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct InitializeInstructionAccounts {
    pub signer: TridentAccount,
    pub mint: TridentAccount,
    pub metadata_account: TridentAccount,
    #[skip_snapshot]
    #[address("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s")]
    pub mpl_token_metadata: TridentAccount,
    #[skip_snapshot]
    #[address("11111111111111111111111111111111")]
    pub system_program: TridentAccount,
    #[skip_snapshot]
    #[address("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")]
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
        let signer = fuzz_accounts.signer.get_or_create_account(
            self.accounts.signer.account_id,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .signer
            .set_account_meta(signer.pubkey(), true, true);

        let mint = fuzz_accounts.mint.get_or_create_account(
            self.accounts.mint.account_id,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        self.accounts
            .mint
            .set_account_meta(mint.pubkey(), true, true);

        let metadata_account = fuzz_accounts.metadata_account.get_or_create_account(
            self.accounts.metadata_account.account_id,
            client,
            &[
                b"metadata",
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").as_ref(),
                mint.pubkey().as_ref(),
            ],
            &pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
        );
        self.accounts
            .metadata_account
            .set_account_meta(metadata_account, false, true);
    }
}
