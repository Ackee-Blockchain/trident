use borsh::{BorshDeserialize, BorshSerialize};
use trident_fuzz::fuzzing::*;
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
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
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
}
/// Custom data types must derive `Debug` and `Arbitrary`.
/// To do this, redefine the type in the fuzz test and implement the `From`
/// trait
/// to convert it into the type defined in the program.
/// For more details, see: https://ackee.xyz/trident/docs/latest/features/fuzz-instructions/#custom-data-types
#[derive(Arbitrary, Debug, BorshDeserialize, BorshSerialize)]
pub struct WithdrawData {}
///IxOps implementation for `Initialize` with all required functions.
impl IxOps for Initialize {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![175u8, 175u8, 109u8, 31u8, 13u8, 152u8, 155u8, 237u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5XvBmfPNcHLCgbRK4nRYvfodAnhjArHSed2B3rhkF1Ug")
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
            let receiver: Pubkey = fuzz_accounts
                .receiver
                .get_or_create_account(self.data.receiver, client, 10 * LAMPORTS_PER_SOL)
                .pubkey();
            args.extend(borsh::to_vec(&receiver).unwrap());
        }
        {
            args.extend(borsh::to_vec(&self.data.amount).unwrap());
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

        let author = fuzz_accounts.author.get_or_create_account(
            self.accounts.author,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        account_metas.push(AccountMeta::new(author.pubkey(), true));
        signers.push(author.insecure_clone());

        let receiver = fuzz_accounts.receiver.get_or_create_account(
            self.data.receiver,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        let escrow = fuzz_accounts.escrow.get_or_create_account(
            self.accounts.escrow,
            client,
            &[
                &author.pubkey().to_bytes(),
                &receiver.pubkey().to_bytes(),
                b"escrow_seed",
            ],
            &self.get_program_id(),
        );
        account_metas.push(AccountMeta::new(escrow, false));

        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        Ok((signers, account_metas))
    }
}
///IxOps implementation for `Withdraw` with all required functions.
impl IxOps for Withdraw {
    type IxAccounts = FuzzAccounts;
    /// Definition of the instruction DISCRIMINATOR.
    fn get_discriminator(&self) -> Vec<u8> {
        vec![183u8, 18u8, 70u8, 156u8, 148u8, 109u8, 161u8, 34u8]
    }
    /// Definition of the program ID that the Instruction is associated with.
    fn get_program_id(&self) -> solana_sdk::pubkey::Pubkey {
        pubkey!("5XvBmfPNcHLCgbRK4nRYvfodAnhjArHSed2B3rhkF1Ug")
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
        let args: Vec<u8> = self.get_discriminator();
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

        let receiver = fuzz_accounts.receiver.get_or_create_account(
            self.accounts.receiver,
            client,
            500 * LAMPORTS_PER_SOL,
        );
        account_metas.push(AccountMeta::new(receiver.pubkey(), true));
        signers.push(receiver.insecure_clone());

        let escrow = fuzz_accounts.escrow.get(self.accounts.escrow);
        account_metas.push(AccountMeta::new(escrow, false));

        {
            account_metas.push(AccountMeta::new_readonly(
                pubkey!("11111111111111111111111111111111"),
                false,
            ));
        }
        Ok((signers, account_metas))
    }

    fn check(
        &self,
        pre_ix: &[SnapshotAccount],
        post_ix: &[SnapshotAccount],
        _ix_data: Vec<u8>,
    ) -> Result<(), FuzzingError> {
        if let Ok(escrow_pre) = Escrow::deserialize(&mut pre_ix[1].data()) {
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
/// Check supported AccountsStorages at
/// https://ackee.xyz/trident/docs/latest/features/account-storages/
#[derive(Default)]
pub struct FuzzAccounts {
    author: AccountsStorage<KeypairStore>,
    escrow: AccountsStorage<PdaStore>,
    receiver: AccountsStorage<KeypairStore>,
}
#[derive(Debug, BorshDeserialize, BorshSerialize, Clone)]
pub struct Escrow {
    author: Pubkey,
    receiver: Pubkey,
    amount: u64,
    bump: u8,
}
