# Printers

Trident provides built-in printer methods for debugging during fuzzing. All output is routed through a safe channel that prevents mixing with the progress bar in parallel mode.

## `tlog!` Macro

Use `tlog!` for ad-hoc debug printing. It works like `println!` but is safe to use during fuzzing — output won't collide with the progress bar.

```rust
tlog!("balance: {}", balance);
tlog!("account: {:#?}", account_data);
```

!!! note
    Prefer `tlog!` over `println!` or `eprintln!` in your fuzz tests. Direct prints will mix with the progress bar in parallel mode.

In addition to `tlog!`, Trident provides several print methods on the `Trident` struct for inspecting transaction results, data accounts, token accounts, mint accounts, and programs. See the [Printers API Reference](../../trident-api/printers.md) for the full list with signatures and output examples.

## Example

```rust
#[flow]
fn debug_flow(&mut self) {
    let ix = create_my_instruction(&self.trident);
    let result = self.trident.process_transaction(&[ix], Some("my_ix"));

    // Print the transaction result
    self.trident.print_transaction_result(&result);

    // Print a deserialized data account
    self.trident.print_account::<MyAccountType>(&account_key, None);

    // Print raw account metadata (no deserialization needed)
    self.trident.print_raw_account(&wallet_key);

    // Print program info (auto-detects loader v2/v3/v4)
    self.trident.print_program(&program_id);

    // Print token accounts (requires `token` feature)
    self.trident.print_token_account(token_account_key);
    self.trident.print_mint_account(mint_key);

    // Ad-hoc printing
    tlog!("custom value: {}", my_value);
}
```

For detailed method signatures and output examples, see the [Printers API Reference](../../trident-api/printers.md).
