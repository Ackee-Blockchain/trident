# Instruction Accounts

Each program instruction has a corresponding file in the instructions directory. Instructions consist of two main components:

- Instruction Accounts
- Instruction Data

## Using the TridentAccounts Macro

The `TridentAccounts` macro is used to derive account handling functionality for instruction accounts (similar to the Context structure in Anchor-based programs).

### Account Attributes

The following example shows the following attributes:

- `mut` - Marks the account as mutable
- `signer` - Marks the account as a transaction signer
- `address = "..."` - Sets a constant address for the account
- `storage = <target_account_storage>` - Links the account to a storage location for address retrieval
- `skip_snapshot` - Excludes the account from snapshot creation

```rust
/// Example of Instruction Accounts
#[derive(Arbitrary, Debug, Clone, TridentAccounts)]
pub struct ExampleInstructionAccounts {
    #[account(mut, signer, storage = author)]
    pub author: TridentAccount,
    #[account(mut)]
    pub hello_world_account: TridentAccount,
    #[account(address = "11111111111111111111111111111111", skip_snapshot)]
    pub system_program: TridentAccount,
}
```

!!! warning "Storage Prerequisite for PDAs"
    Program Derived Addresses (PDAs) must be inserted into storage before they can be referenced using the `storage` attribute. More about the `TridentAccounts` macro can be found in the [TridentAccounts](../../trident-api-macro/trident-macros/trident-accounts.md).

## Manual Account Setup

If you need more control than the `TridentAccounts` macro provides, you can manually set up accounts using the `set_accounts` function:

!!! warning "Precedence of `set_accounts`"
    `set_accounts` method has precedence over the `TridentAccounts` macro, attributes. So behavior defined in the `set_accounts` method will override the behavior defined in the `TridentAccounts` macro.

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

In this example:

1. We create and store a PDA
2. We set the account address
3. We use the `set_address` method to set the account address
