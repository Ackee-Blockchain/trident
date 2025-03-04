# Instruction Data

Apart from accounts, additional instruction parameters can be specified on the Instruction's inputs.

Most of the time, these parameters are primitive data types, such as `u8`, `u16`, `u32`, `u64`, `bool`, etc. In this case, guidance is probably not required, and we can leave the fuzzer to generate random values. On the other hand, if the instruction expects, for example, a `Pubkey` as input, the `Pubkey` needs to be resolved similarly to how accounts are resolved (as you probably don't want the pubkey to be completely random but rather derived from the fuzzer's account storage).

## Manual Data Setup

The `set_data` method lets you manually set the instruction data (if required). Here's an example:

```rust
impl InstructionSetters for DepositObligationCollateralV2Instruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(
        &mut self,
        client: &mut impl FuzzClient,
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

In the example above:

- We retrieved the `owner` account address from the `FuzzAccounts::owner` storage using the generated `self.data.owner.account_id` index.
- We then set the `owner` account's public key in the instruction data.

This approach ensures that the instruction data contains valid and meaningful values while still maintaining the benefits of fuzz testing.
