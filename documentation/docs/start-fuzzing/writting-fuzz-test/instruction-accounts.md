# Instruction Accounts


Accounts are storage entities on the Solana Blockchain. They may be simple wallets (consisting of regular keypairs) or accounts with more complex address structures, such as Program Derived Addresses (PDAs).

During fuzzing, Trident sends random Accounts to the program's instructions. However, the account addresses are not completely random but are taken from the Fuzz Test's account storage. Trident generates random indexes to the account storage. This approach is crucial, as allowing the fuzzer to generate completely random account addresses would lead to many failed transactions (i.e., nothing would be fuzzed).

Your responsibility is to guide Trident on which storage locations it should retrieve accounts from. Additionally, if particular account types are expected, such as Token Account, Mint Account, etc., your responsibility is to initialize them within the `set_accounts` method.

## Setup Instruction Accounts

There are two ways to guide the fuzzer about which storage locations to retrieve accounts from:

1. Using the `TridentAccounts` macro
2. Manually setting the accounts using the `set_accounts` method

### Using the TridentAccounts Macro

The `TridentAccounts` macro is used to derive account handling functionality for instruction accounts (similar to the Context structure in Anchor-based programs).

You can check the [TridentAccounts](../../trident-api-macro/trident-macros/trident-accounts.md/#field-level-attributes) page to see the attributes that can be used.

Consider the following example:

```rust
/// Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
#[instruction_data(InitializeFnInstructionData)]
#[storage(FuzzAccounts)]
pub struct InitializeFnInstructionAccounts {
    #[account(mut,signer,storage = author)]
    pub author: TridentAccount,
    #[account(
        mut,
        storage = hello_world_account,
        seeds = [b"hello_world_seed"],
    )]
    pub hello_world_account: TridentAccount,
    #[account(address = "11111111111111111111111111111111", skip_snapshot)]
    pub system_program: TridentAccount,
}
```

The example specifies these attributes:

- `#[instruction_data(InitializeFnInstructionData)]` - Specifies the instruction data type (helpful if instruction data input is part of PDA seeds)
- `#[storage(FuzzAccounts)]` - Specifies the account storage type
- `mut` - Marks the account as mutable
- `signer` - Marks the account as a transaction signer
- `address = "..."` - Sets a constant address for the account
- `storage = <target_account_storage>` - Links the account to a storage location for address retrieval
- `skip_snapshot` - Excludes the account from snapshot creation
- `seeds = [b"hello_world_seed"]` - Specifies the seeds for Program Derived Addresses (PDAs)


### Manual Accounts Setup

If you need more control than the `TridentAccounts` macro provides, you can manually set up accounts using the `set_accounts` function:

!!! warning "Precedence of `set_accounts`"
    The `set_accounts` method takes precedence over the `TridentAccounts` macro attributes. Any behavior defined in the `set_accounts` method will override the behavior defined in the `TridentAccounts` macro.

```rust
impl InstructionSetters for ExampleInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_accounts(&mut self, client: &mut impl FuzzClient, fuzz_accounts: &mut Self::IxAccounts) {
        // Create and store a PDA
        let hello_world_account = fuzz_accounts.hello_world_account.get_or_create(
            self.accounts.hello_world_account.account_id,
            client,
            Some(PdaSeeds::new(&[b"hello_world_seed"], self.get_program_id())),
            None,
        );

        // Set the account address
        self.accounts
            .hello_world_account
            .set_address(hello_world_account);
    }
}
```

In the previous examples:

- We configured the fuzzer to always keep the `author` account as a `signer` and as `mutable`. It will be stored in the `FuzzAccounts::author` storage.
- We configured the fuzzer to always keep the `hello_world_account` as a `mutable` account, which will be stored in the `FuzzAccounts::hello_world_account` storage. Additionally, the `hello_world_account` is a PDA with an address derived from the `seeds` attribute.
- We configured the fuzzer to always keep the `system_program` account as a `constant`. Snapshots of this account before and after the instruction execution will not be taken.
