pub mod fuzz_example2_fuzz_instructions {
    use crate::accounts_snapshots::*;
    use fuzz_example2::state::ESCROW_SEED;
    use trdelnik_client::{
        anchor_lang::Key, fuzzing::*, solana_sdk::native_token::LAMPORTS_PER_SOL,
    };
    #[derive(Arbitrary, Clone, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        Initialize(Initialize),
        Withdraw(Withdraw),
    }
    #[derive(Arbitrary, Clone)]
    pub struct Initialize {
        pub accounts: InitializeAccounts,
        pub data: InitializeData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InitializeAccounts {
        pub author: AccountId,
        pub escrow: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct InitializeData {
        pub receiver: [u8; 32],
        pub amount: u64,
    }
    #[derive(Arbitrary, Clone)]
    pub struct Withdraw {
        pub accounts: WithdrawAccounts,
        pub data: WithdrawData,
    }
    #[derive(Arbitrary, Clone)]
    pub struct WithdrawAccounts {
        pub receiver: AccountId,
        pub escrow: AccountId,
        pub system_program: AccountId,
    }
    #[derive(Arbitrary, Clone)]
    pub struct WithdrawData {}
    impl<'info> IxOps<'info> for Initialize {
        type IxData = fuzz_example2::instruction::Initialize;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example2::instruction::Initialize {
                receiver: Pubkey::new_from_array(self.data.receiver),
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

            let escrow = fuzz_accounts
                .escrow
                .get_or_create_account(
                    self.accounts.escrow,
                    &[
                        author.pubkey().as_ref(),
                        self.data.receiver.as_ref(),
                        ESCROW_SEED.as_ref(),
                    ],
                    &fuzz_example2::ID,
                )
                .unwrap();
            let acc_meta = fuzz_example2::accounts::Initialize {
                author: author.pubkey(),
                escrow: escrow.pubkey(),
                system_program: SYSTEM_PROGRAM_ID,
            }
            .to_account_metas(None);
            Ok((vec![author], acc_meta))
        }
    }
    impl<'info> IxOps<'info> for Withdraw {
        type IxData = fuzz_example2::instruction::Withdraw;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = WithdrawSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = fuzz_example2::instruction::Withdraw {};
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
                    &fuzz_example2::ID,
                )
                .unwrap();

            let acc_meta = fuzz_example2::accounts::Withdraw {
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
        ) -> Result<(), &'static str> {
            if let Some(escrow_pre) = pre_ix.escrow {
                // we can unwrap the receiver account because it has to be initialized before the instruction
                // execution and it is not supposed to be closed after the instruction execution either
                let receiver = pre_ix.receiver.unwrap();
                let receiver_lamports_before = receiver.lamports();
                let receiver_lamports_after = post_ix.receiver.unwrap().lamports();

                // If the Receiver (i.e. Signer in the Context) and stored Receiver inside Escrow Account,
                // do not match, however the receiver`s balance increased, we found an Error
                if receiver.key() != escrow_pre.receiver.key()
                    && receiver_lamports_before < receiver_lamports_after
                {
                    return Err("Un-authorized withdrawal");
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
        // system_program: AccountsStorage<ProgramStore>,
        author: AccountsStorage<Keypair>,
        escrow: AccountsStorage<PdaStore>,
    }
    impl FuzzAccounts {
        pub fn new() -> Self {
            Default::default()
        }
    }
}
