# TridentPubkey

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

## Implemented Methods

The following section contains the methods that are available for the `TridentPubkey` struct.

### `set_pubkey`

Sets the public key for this instance.
```rust
fn set_pubkey(&mut self, pubkey: Pubkey)
```

---

### `get_pubkey`

Returns the stored public key.
```rust
fn get_pubkey(&self) -> Pubkey
```

---

## Implemented Traits

- `From<AccountId>` - Creates a new instance with the given account_id and default pubkey
- `BorshSerialize` - Serializes only the pubkey field
- `BorshDeserialize` - Deserializes only the pubkey field, sets default account_id
- `Serialize` (serde) - Serializes only the pubkey field
- `Deserialize` (serde) - Deserializes only the pubkey field, sets default account_id
- `Arbitrary` - Generates random account_id and default pubkey
