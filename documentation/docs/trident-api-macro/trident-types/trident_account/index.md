# Trident Account

Trident Account is a wrapper around `AccountMeta`, `SnapshotAccount` and `AccountId`.

`AccountMeta` is type which is used within `Transaction`, it specifies account address, `isSigner` and `isWritable` flags.

`SnapshotAccount` is Trident's custom type which is used to capture account state before and after the transaction.

`AccountId` is randomly generated number which is used to identify account within its corresponding `AccountStorage`.


```rust
#[derive(Debug, Clone)]
pub struct TridentAccount {
    pub account_id: AccountId,
    account_meta: Option<AccountMeta>,
    snapshot_before: Option<SnapshotAccount>,
    snapshot_after: Option<SnapshotAccount>,
}
```
