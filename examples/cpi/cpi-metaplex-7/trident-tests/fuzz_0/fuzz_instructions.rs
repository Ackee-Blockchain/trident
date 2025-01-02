use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    Initialize(Initialize),
}
#[derive(Arbitrary, Debug)]
pub struct Initialize {
    pub accounts: InitializeAccounts,
    pub data: InitializeData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeAccounts {
    pub signer: AccountId,
    pub mint: AccountId,
    pub metadata_account: AccountId,
    pub _token_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct InitializeData {
    pub input: u8,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
///IxOps implementation for `Initialize` with all required functions.
impl IxOps for Initialize {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("3XtULmXDGS867VbBXiPkjYr4EMjytGW8X12F6BS23Zcw")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            args.extend(borsh::to_vec(&self.data.input).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.name).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.symbol).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.uri).unwrap());
        }
        Ok(args)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let mut account_metas = vec![];
        let mut signers = vec![];
        {
            let signer = fuzz_accounts.signer_cpi_metaplex_7.get_or_create_account(
                self.accounts.signer,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new(signer.pubkey(), true));
            signers.push(signer.insecure_clone());
        }
        let mint = {
            let mint = fuzz_accounts.mint_cpi_metaplex_7.get_or_create_account(
                self.accounts.mint,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new(mint.pubkey(), true));
            signers.push(mint.insecure_clone());
            mint.pubkey()
        };
        {
            let metadata_account = fuzz_accounts
                .metadata_account_cpi_metaplex_7
                .get_or_create_account(
                    self.accounts.metadata_account,
                    client,
                    &[
                        b"metadata",
                        pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").as_ref(),
                        mint.as_ref(),
                    ],
                    &pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                );
            account_metas.push(AccountMeta::new(metadata_account, false));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                false,
            ));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ));
        }
        Ok((signers, account_metas))
    }
}
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    metadata_account_cpi_metaplex_7: AccountsStorage<PdaStore>,
    mint_cpi_metaplex_7: AccountsStorage<KeypairStore>,
    signer_cpi_metaplex_7: AccountsStorage<KeypairStore>,
    _token_program_cpi_metaplex_7: AccountsStorage<KeypairStore>,
}
