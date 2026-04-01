# Printers

Methods for printing formatted account data, transaction results, and program details during fuzzing. All output is routed through a safe channel to avoid mixing with the progress bar.

## `tlog!`

Ad-hoc debug printing macro. Works like `println!` but safe to use during parallel fuzzing.

```rust
tlog!("balance: {}", balance);
tlog!("account: {:#?}", account_data);
tlog!("before: {}, after: {}", before.amount, after.amount);
```

!!! note
    Prefer `tlog!` over `println!` or `eprintln!` in your fuzz tests. Direct prints will mix with the progress bar in parallel mode.

---

## `print_transaction_result`

Prints a formatted transaction result including status, compute units, timestamp, and logs.

```rust
pub fn print_transaction_result(&self, result: &TridentTransactionResult)
```

**Parameters:**

- `result` - The transaction result returned by `process_transaction`

**Example:**

```rust
let result = self.trident.process_transaction(&[ix], Some("transfer"));
self.trident.print_transaction_result(&result);
```

**Output:**

```
── Transaction Result ─────────────────────────────
  Status:    OK
  CU used:   12345
  Timestamp: 1234567890
  Logs:
    Program 11111... invoke [1]
    Program 11111... success
```

---

## `print_account`

Fetches, deserializes, and pretty-prints a data account. The type must implement `BorshDeserialize`, `AccountDiscriminator`, and `Debug`.

```rust
pub fn print_account<T: BorshDeserialize + AccountDiscriminator + fmt::Debug>(
    &mut self,
    key: &Pubkey,
    discriminator_size_override: Option<usize>,
)
```

**Parameters:**

- `key` - The public key of the account
- `discriminator_size_override` - Optional override for discriminator size to skip. If `None`, uses the length from the type's `AccountDiscriminator` implementation.

**Example:**

```rust
// Use discriminator from AccountDiscriminator trait
self.trident.print_account::<MyAccountType>(&account_key, None);

// Override discriminator size (e.g. for non-Anchor accounts)
self.trident.print_account::<MyAccountType>(&account_key, Some(0));
```

**Output:**

```
── Account (7xKX..m3Qp) ──────────────────────────
  MyAccountType {
      balance: 1000000,
      owner: 7xKX...m3Qp,
      is_initialized: true,
  }
```

---

## `print_raw_account`

Prints raw account metadata without deserialization. Works for any account — wallets, PDAs, unknown layouts, or accounts with no data.

```rust
pub fn print_raw_account(&self, key: &Pubkey)
```

**Parameters:**

- `key` - The public key of the account

**Example:**

```rust
self.trident.print_raw_account(&some_pubkey);
```

**Output:**

```
── Account (7xKX..m3Qp) ──────────────────────────
  Owner:      11111111111111111111111111111111
  Lamports:   1000000000 (1.0000 SOL)
  Data:       0 bytes
  Executable: false
```

---

## `print_program`

Prints program account details. Automatically detects the loader type and prints relevant metadata. For upgradeable programs (v3), both the program account and its ProgramData account are printed. The ELF binary is never shown.

```rust
pub fn print_program(&mut self, key: &Pubkey)
```

**Parameters:**

- `key` - The public key of the program

**Supported loaders:**

| Loader | Details shown |
|---|---|
| BPF Loader v3 (upgradeable) | Program + ProgramData (authority, slot, ELF size) |
| Loader v4 | Status (Deployed/Retracted/Finalized), authority, slot, ELF size |
| BPF Loader v2 (non-upgradeable) | Basic metadata |
| Native Loader | Identifies built-in programs |

**Example:**

```rust
self.trident.print_program(&program_id);
```

**Output (v3 upgradeable):**

```
── Program (6Kd1..Aq3V) ──────────────────────────
  Owner:      BPFLoaderUpgradeab1e111111111111111111111111
  Executable: true
  Lamports:   1141440
  Data:       36 bytes
  Loader:     BPF Loader v3 (upgradeable)
  ProgramData: 3Hq8kR...Xm2P

  ── ProgramData (3Hq8..Xm2P) ──
  Slot:       285102847
  Authority:  jvzRJsqnPtEfiSkotmGsoFt7kKzVqtyJaqYiWduZoht
  ELF:        245760 bytes (not shown)
  Lamports:   1830480
```

**Output (v4):**

```
── Program (9xQe..Bm7P) ──────────────────────────
  Owner:      LoaderV411111111111111111111111111111111111
  Executable: true
  Lamports:   2500000
  Data:       131120 bytes
  Loader:     Loader v4
  Slot:       300000000
  Status:     Deployed
  Authority:  jvzRJsqnPtEfiSkotmGsoFt7kKzVqtyJaqYiWduZoht
  ELF:        131072 bytes (not shown)
```

---

## `print_token_account`

!!! note "Requires `token` feature"

Prints a token account showing mint, owner, amount, delegate, state, and any Token-2022 extensions.

```rust
pub fn print_token_account(&mut self, account: Pubkey)
```

**Parameters:**

- `account` - The public key of the token account

**Example:**

```rust
self.trident.print_token_account(token_account_pubkey);
```

**Output:**

```
── Token Account (9xQe..Bm7P) ────────────────────
  Mint:      EPjFW...4Cph
  Owner:     7xKX...m3Qp
  Amount:    1000000
  Delegate:  None
  State:     Initialized
  Extensions (2)
    ImmutableOwner
    TransferFeeAmount: withheld = 0
```

---

## `print_mint_account`

!!! note "Requires `token` feature"

Prints a mint account showing authority, supply, decimals, freeze authority, and any Token-2022 extensions with human-readable values.

```rust
pub fn print_mint_account(&mut self, account: Pubkey)
```

**Parameters:**

- `account` - The public key of the mint account

**Example:**

```rust
self.trident.print_mint_account(mint_pubkey);
```

**Output:**

```
── Mint (EPjF..4Cph) ─────────────────────────────
  Authority: jvzRJsqnPtEfiSkotmGsoFt7kKzVqtyJaqYiWduZoht
  Supply:    1000000000
  Decimals:  6
  Freeze:    None
  Extensions (3)
    TransferFeeConfig: 1% (100 bps), max_fee = 1000000
    MetadataPointer: E9pCkonLma6xWWwXLvNUnS48W6ncFBX8VEnj4Eru3rjC
    TokenMetadata: name = "My Token", symbol = "MTK", uri = "https://..."
```