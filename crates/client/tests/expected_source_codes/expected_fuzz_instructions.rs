pub mod dummy_example_fuzz_instructions {
    use trident_client::fuzzing::*;
    type InitializeIxSnapshot<'info> = InitializeIxAlias<'info>;
    #[derive(Arbitrary, DisplayIx, FuzzTestExecutor, FuzzDeserialize)]
    pub enum FuzzInstruction {
        InitializeIx(InitializeIx),
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeIx {
        pub accounts: InitializeIxAccounts,
        pub data: InitializeIxData,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeIxAccounts {
        pub account: AccountId,
        pub account_info: AccountId,
        pub account_loader: AccountId,
        pub boxed: AccountId,
        pub interace: AccountId,
        pub interface_account: AccountId,
        pub option: AccountId,
        pub program: AccountId,
        pub signer: AccountId,
        pub system_account: AccountId,
        pub sysvar: AccountId,
        pub unchecked_account: AccountId,
    }
    #[derive(Arbitrary, Debug)]
    pub struct InitializeIxData {
        pub _var1: bool,
        pub _var2: u8,
        pub _var3: i8,
        pub _var4: u16,
        pub _var5: i16,
        pub _var6: u32,
        pub _var7: i32,
        pub _var8: u64,
        pub _var9: i32,
        pub _var10: f64,
        pub _var11: u128,
        pub _var12: i128,
        pub _ver13: Vec<u8>,
        pub _var14: String,
        pub _var15: AccountId,
        pub _var16: Option<i16>,
        pub _var17: Vec<u32>,
        pub _var18: [i128; 5usize],
        pub _var19: InputParameter,
        pub _var20: Vec<Vec<Vec<Vec<Vec<u8>>>>>,
        pub _var21: Vec<Vec<Vec<Vec<Option<u8>>>>>,
        pub _var22: Vec<Vec<Vec<Vec<Option<Vec<u8>>>>>>,
    }
    impl<'info> IxOps<'info> for InitializeIx {
        type IxData = dummy_example::instruction::InitializeIx;
        type IxAccounts = FuzzAccounts;
        type IxSnapshot = InitializeIxSnapshot<'info>;
        fn get_data(
            &self,
            _client: &mut impl FuzzClient,
            _fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<Self::IxData, FuzzingError> {
            let data = dummy_example::instruction::InitializeIx {
                _var1: self.data._var1,
                _var2: self.data._var2,
                _var3: self.data._var3,
                _var4: self.data._var4,
                _var5: self.data._var5,
                _var6: self.data._var6,
                _var7: self.data._var7,
                _var8: self.data._var8,
                _var9: self.data._var9,
                _var10: self.data._var10,
                _var11: self.data._var11,
                _var12: self.data._var12,
                _ver13: self.data._ver13.clone(),
                _var14: self.data._var14.clone(),
                _var15: todo!(),
                _var16: self.data._var16,
                _var17: self.data._var17.clone(),
                _var18: self.data._var18,
                _var19: todo!(),
                _var20: self.data._var20.clone(),
                _var21: self.data._var21.clone(),
                _var22: self.data._var22.clone(),
            };
            Ok(data)
        }
        fn get_accounts(
            &self,
            client: &mut impl FuzzClient,
            fuzz_accounts: &mut FuzzAccounts,
        ) -> Result<(Vec<Keypair>, Vec<AccountMeta>), FuzzingError> {
            let signers = vec![todo!()];
            let acc_meta = todo!();
            Ok((signers, acc_meta))
        }
    }
    #[doc = r" Use AccountsStorage<T> where T can be one of:"]
    #[doc = r" Keypair, PdaStore, TokenStore, MintStore, ProgramStore"]
    #[derive(Default)]
    pub struct FuzzAccounts {
        account: AccountsStorage<PdaStore>,
        account_info: AccountsStorage<todo!()>,
        account_loader: AccountsStorage<todo!()>,
        boxed: AccountsStorage<todo!()>,
        interace: AccountsStorage<todo!()>,
        interface_account: AccountsStorage<todo!()>,
        option: AccountsStorage<todo!()>,
        program: AccountsStorage<todo!()>,
        signer: AccountsStorage<todo!()>,
        system_account: AccountsStorage<todo!()>,
        sysvar: AccountsStorage<todo!()>,
        unchecked_account: AccountsStorage<todo!()>,
    }
}