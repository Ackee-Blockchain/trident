use solana_sdk::native_token::LAMPORTS_PER_SOL;
use trident_client::fuzzing::*;
use unauthorized_access_2::ESCROW_SEED;
/// FuzzInstruction contains all available Instructions.
/// Below, the instruction arguments (accounts and data) are defined.
#[derive(Arbitrary, DisplayIx, FuzzTestExecutor)]
pub enum FuzzInstruction {
    Initialize(Initialize),
    Withdraw(Withdraw),
}
#[derive(Arbitrary, Debug)]
pub struct Initialize {
    pub accounts: InitializeAccounts,
    pub data: InitializeData,
}
#[derive(Arbitrary, Debug)]
pub struct InitializeAccounts {
    pub author: AccountId,
    pub escrow: AccountId,
    pub _system_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct InitializeData {
    pub receiver: AccountId,
    pub amount: u64,
}
#[derive(Arbitrary, Debug)]
pub struct Withdraw {
    pub accounts: WithdrawAccounts,
    pub _data: WithdrawData,
}
#[derive(Arbitrary, Debug)]
pub struct WithdrawAccounts {
    pub receiver: AccountId,
    pub escrow: AccountId,
    pub _system_program: AccountId,
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/dev/features/arbitrary-data/#custom-data-types
#[derive(Arbitrary, Debug)]
pub struct WithdrawData {}
///IxOps implementation for `Initialize` with all required functions.
impl IxOps for Initialize {
    type IxData = unauthorized_access_2::instruction::Initialize;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        unauthorized_access_2::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let receiver = fuzz_accounts.receiver.get_or_create_account(
            self.data.receiver,
            client,
            10 * LAMPORTS_PER_SOL,
        );
        let data = unauthorized_access_2::instruction::Initialize {
            receiver: receiver.pubkey(),
            amount: self.data.amount,
        };
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let author = fuzz_accounts.author.get_or_create_account(
            self.accounts.author,
            client,
            10 * LAMPORTS_PER_SOL,
        );

        let receiver = fuzz_accounts.receiver.get_or_create_account(
            self.data.receiver,
            client,
            10 * LAMPORTS_PER_SOL,
        );

        let escrow = fuzz_accounts.escrow.get_or_create_account(
            self.accounts.escrow,
            client,
            &[
                author.pubkey().as_ref(),
                receiver.pubkey().as_ref(),
                ESCROW_SEED.as_ref(),
            ],
            &unauthorized_access_2::ID,
        );
        let acc_meta = unauthorized_access_2::accounts::Initialize {
            author: author.pubkey(),
            escrow,
            system_program: solana_sdk::system_program::ID,
        }
        .to_account_metas(None);
        Ok((vec![author], acc_meta))
    }
}
///IxOps implementation for `Withdraw` with all required functions.
impl IxOps for Withdraw {
    type IxData = unauthorized_access_2::instruction::Withdraw;
    type IxAccounts = FuzzAccounts;
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        unauthorized_access_2::ID
    }
    /// Definition of the Instruction data.
    /// Use randomly generated data from the fuzzer using `self.data.arg_name`
    /// or customize the data as needed.
    /// For more details, visit: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-data
    fn get_data(
        &self,
        _client: &mut impl FuzzClient,
        _fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<Self::IxData, FuzzingError> {
        let data = unauthorized_access_2::instruction::Withdraw {};
        Ok(data)
    }
    /// Definition of of the accounts required by the Instruction.
    /// To utilize accounts stored in `FuzzAccounts`, use
    /// `fuzz_accounts.account_name.get_or_create_account()`.
    /// If no signers are required, leave the vector empty.
    /// For AccountMetas use <program>::accounts::<corresponding_metas>
    /// For more details, see: https://ackee.xyz/trident/docs/dev/features/fuzz-instructions/#get-accounts
    fn get_accounts(
        &self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut FuzzAccounts,
    ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
        let receiver = fuzz_accounts.receiver.get_or_create_account(
            self.accounts.receiver,
            client,
            10 * LAMPORTS_PER_SOL,
        );

        let escrow = fuzz_accounts.escrow.get_or_create_account(
            self.accounts.escrow,
            client,
            &[
                receiver.pubkey().as_ref(),
                receiver.pubkey().as_ref(),
                ESCROW_SEED.as_ref(),
            ],
            &unauthorized_access_2::ID,
        );

        let acc_meta = unauthorized_access_2::accounts::Withdraw {
            receiver: receiver.pubkey(),
            escrow,
            system_program: solana_sdk::system_program::ID,
        }
        .to_account_metas(None);
        Ok((vec![receiver], acc_meta))
    }
    fn check(
        &self,
        pre_ix: &[SnapshotAccount],
        post_ix: &[SnapshotAccount],
        _ix_data: Self::IxData,
    ) -> Result<(), FuzzingError> {
        if let Ok(escrow_pre) =
            unauthorized_access_2::Escrow::try_deserialize(&mut pre_ix[1].data())
        {
            let receiver_key = pre_ix[0].pubkey();
            let receiver_lamports_before = pre_ix[0].lamports();
            let receiver_lamports_after = post_ix[0].lamports();

            // If the Receiver (i.e. Signer in the Context) and stored Receiver inside Escrow Account,
            // do not match, however the receiver`s balance increased, we found an Error
            if receiver_key != escrow_pre.receiver
                && receiver_lamports_before < receiver_lamports_after
            {
                return Err(FuzzingError::BalanceMismatch);
            }
        }

        Ok(())
    }
}
/// Use AccountsStorage<T> where T can be one of:
/// Keypair, PdaStore, TokenStore, MintStore, ProgramStore
#[derive(Default)]
pub struct FuzzAccounts {
    receiver: AccountsStorage<KeypairStore>,
    // No need to fuzz system_program
    // system_program: AccountsStorage<ProgramStore>,
    author: AccountsStorage<KeypairStore>,
    escrow: AccountsStorage<PdaStore>,
}
