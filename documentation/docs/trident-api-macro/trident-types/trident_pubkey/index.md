# Trident Pubkey

`TridentPubkey` is a wrapper around `Pubkey` and `AccountId`.

`Pubkey` is a type from `solana-sdk` crate, corresponds to Solana account address.

`AccountId` is randomly generated number which is used to identify account within its corresponding `AccountStorage`.

```rust
#[derive(Debug, Clone)]
pub struct TridentPubkey {
    pub account_id: AccountId,
    pub pubkey: Pubkey,
}
```
