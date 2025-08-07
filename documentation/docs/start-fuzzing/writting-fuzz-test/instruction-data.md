# Instruction Data

Instructions on Solana can have parameters. Trident lets you to send random data as the instruction parameters.

## Manual Data Setup

The following example shows how to specify what data should be sent to the instruction. In this case we are guiding the Trident to always generate random number from 0 to `u8::MAX` and use it as the `input` parameter of the instruction.

```rust
impl InstructionSetters for DepositObligationCollateralV2Instruction {
    type IxAccounts = FuzzAccounts;

    fn set_data(
        &mut self, trident: &mut Trident, _fuzz_accounts: &mut Self::IxAccounts
    ) {
        self.data.input = trident.gen_range(0..u8::MAX);
    }
}
```