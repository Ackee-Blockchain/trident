# Instruction Data

While Trident automatically generates random instruction data in most cases, there are situations where manual configuration is necessary. For example, when an instruction expects a specific `Pubkey` as input, using completely random data would be ineffective.

## Manual Data Setup

The `FuzzAccounts` struct provides methods to obtain specific accounts and their public keys for instruction data. Here's an example:

```rust
impl InstructionSetters for DepositObligationCollateralV2Instruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(
        &mut self,
        _client: &mut impl FuzzClient,
        fuzz_accounts: &mut Self::IxAccounts
    ) {
        // Retrieve account from storage using the generated index
        let owner = fuzz_accounts.owner.get_or_create(
            self.data.owner.account_id,
            client,
            None,
            None,
        );

        // Set the instruction data pubkey
        self.data.owner.pubkey = owner;
    }
}
```

In this example:

1. We use the `get_or_create` method to retrieve an account using its index
2. We then set the account's public key in the instruction data

This approach ensures that the instruction data contains valid and meaningful values while still maintaining the benefits of fuzz testing.
