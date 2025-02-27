# Remaining Accounts

Some Solana instructions require additional accounts beyond their primary accounts. These are called "remaining accounts" and can be configured using the `set_remaining_accounts` function.

## Manual Remaining Accounts Setup

Remaining accounts are defined as a struct with the `TridentRemainingAccounts` derive macro. The macro expects remaining accounts as an array of `TridentAccount`, allowing you to specify any number of additional accounts.

!!! warning "Remaining Accounts Configuration"
    Do not forget to add remaining accounts to the `Instuction` struct. Additionally the field has to be named `remaining_accounts`.

Here's an example of how to set up remaining accounts:

```rust
pub struct SomeInstruction {
    pub accounts: SomeInstructionAccounts,
    pub data: SomeInstructionData,
    pub remaining_accounts: SomeInstructionRemainingAccounts,
}

#[derive(Arbitrary, Debug, Clone, TridentRemainingAccounts)]
pub struct SomeInstructionRemainingAccounts {
    pub remaining_accounts: [TridentAccount; 5],
}

impl InstructionSetters for SomeInstruction {
    type IxAccounts = FuzzAccounts;

    fn set_remaining_accounts(
        &mut self,
        client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts,
    ) {
        // Get the account from storage using the generated index
        let owner = fuzz_accounts
            .owner
            .get_or_create(self.remaining_accounts.remaining_accounts[0].account_id, client, None, None);

        // Configure the account meta with:
        // - Account public key
        // - is_signer flag (false in this example)
        // - is_writable flag (true in this example)
        self.remaining_accounts.remaining_accounts[0].set_account_meta(
            owner,
            false,
            true
        );
    }
}
```


- The `set_remaining_accounts` function works similarly to `set_accounts`
- Each remaining account can be configured with its own signer and writable permissions
- You can access remaining accounts using array indexing (e.g., `remaining_accounts[0]`)
- The number of remaining accounts is fixed at compile time by the array size
