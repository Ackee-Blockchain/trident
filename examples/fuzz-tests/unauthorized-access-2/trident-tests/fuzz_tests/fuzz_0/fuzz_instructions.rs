pub mod unauthorized_access_2_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use solana_sdk::native_token::LAMPORTS_PER_SOL;
    use solana_sdk::system_program::ID as SYSTEM_PROGRAM_ID;
    use trident_client::fuzzing::*;
    use unauthorized_access_2::ESCROW_SEED;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
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
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeData {
        pub receiver: AccountId,
        pub amount: u64,
    }
    #[derive(Arbitrary, Debug)]
    pub struct Withdraw {
        pub accounts: WithdrawAccounts,
        pub data: WithdrawData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawAccounts {
        pub receiver: AccountId,
        pub escrow: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct WithdrawData {}
    impl<'info> IxOps<'info> for Initialize {
        type IxData = unauthorized_access_2::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
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
                amount: 100,
            };
            Ok(data)
        }
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

            let escrow = fuzz_accounts
                .escrow
                .get_or_create_account(
                    self.accounts.escrow,
                    &[
                        author.pubkey().as_ref(),
                        receiver.pubkey().as_ref(),
                        ESCROW_SEED.as_ref(),
                    ],
                    &unauthorized_access_2::ID,
                )
                .unwrap();
            let acc_meta = unauthorized_access_2::accounts::Initialize {
                author: author.pubkey(),
                escrow: escrow.pubkey(),
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);
            Ok((vec![author], acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Withdraw {
        type IxData = unauthorized_access_2::instruction::Withdraw;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WithdrawSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = unauthorized_access_2::instruction::Withdraw {};
            Ok(data)
        }
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

            let escrow = fuzz_accounts
                .escrow
                .get_or_create_account(
                    self.accounts.escrow,
                    &[
                        receiver.pubkey().as_ref(),
                        receiver.pubkey().as_ref(),
                        ESCROW_SEED.as_ref(),
                    ],
                    &unauthorized_access_2::ID,
                )
                .unwrap();

            let acc_meta = unauthorized_access_2::accounts::Withdraw {
                receiver: receiver.pubkey(),
                escrow: escrow.pubkey(),
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);
            Ok((vec![receiver], acc_meta))
        }
        fn check(
            &self,
            pre_ix: Self::IxSnapshot,
            post_ix: Self::IxSnapshot,
            _ix_data: Self::IxData,
        ) -> Result<(), FuzzingError> {
            if let Some(escrow_pre) = pre_ix.escrow {
                let receiver = pre_ix.receiver;
                let receiver_lamports_before = receiver.lamports();
                let receiver_lamports_after = post_ix.receiver.lamports();

                // If the Receiver (i.e. Signer in the Context) and stored Receiver inside Escrow Account,
                // do not match, however the receiver`s balance increased, we found an Error
                if receiver.key() != escrow_pre.receiver
                    && receiver_lamports_before < receiver_lamports_after
                {
                    return Err(FuzzingError::BalanceMismatch);
                }
            }

            Ok(())
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        receiver: AccountsStorage<Keypair>,
        // No need to fuzz system_program
        // system_program: AccountsStorage<ProgramStore>,
        author: AccountsStorage<Keypair>,
        escrow: AccountsStorage<PdaStore>,
    }
}
