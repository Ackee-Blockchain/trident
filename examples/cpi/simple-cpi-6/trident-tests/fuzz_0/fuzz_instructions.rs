use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    InitializeCallee(InitializeCallee),
    InitializeCaller(InitializeCaller),
}
#[derive(Arbitrary, Debug)]
pub struct InitializeCallee {
    pub accounts: InitializeCalleeAccounts,
    pub data: InitializeCalleeData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeCalleeAccounts {
    pub signer: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct InitializeCalleeData {
    pub input: u16,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeCaller {
    pub accounts: InitializeCallerAccounts,
    pub data: InitializeCallerData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeCallerAccounts {
    pub signer: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct InitializeCallerData {
    pub input: u16,
}
///IxOps implementation for `InitializeCallee` with all required functions.
impl IxOps for InitializeCallee {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![164u8, 75u8, 79u8, 32u8, 57u8, 23u8, 116u8, 175u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("HJR1TK8bgrUWzysdpS1pBGBYKF7zi1tU9cS4qj8BW8ZL")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            args.extend(borsh::to_vec(&self.data.input).unwrap());
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
            let signer = fuzz_accounts.signer_callee.get_or_create_account(
                self.accounts.signer,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new_readonly(signer.pubkey(), true));
            signers.push(signer.insecure_clone());
        }
        Ok((signers, account_metas))
    }
}
///IxOps implementation for `InitializeCaller` with all required functions.
impl IxOps for InitializeCaller {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![204u8, 76u8, 200u8, 172u8, 185u8, 14u8, 99u8, 166u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("FWtSodrkUnovFPnNRCxneP6VWh6JH6jtQZ4PHoP8Ejuz")
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Vec<u8>, FuzzingError> {
        let mut args: Vec<u8> = self.get_discriminator();
        {
            args.extend(borsh::to_vec(&self.data.input).unwrap());
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
            let signer = fuzz_accounts.signer_caller.get_or_create_account(
                self.accounts.signer,
                client,
                500 * LAMPORTS_PER_SOL,
            );
            account_metas.push(AccountMeta::new_readonly(signer.pubkey(), true));
            signers.push(signer.insecure_clone());
        }
        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("HJR1TK8bgrUWzysdpS1pBGBYKF7zi1tU9cS4qj8BW8ZL"),
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
    signer_callee: AccountsStorage<KeypairStore>,
    signer_caller: AccountsStorage<KeypairStore>,
}
