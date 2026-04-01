# Invariants and Assertions

Invariants are conditions that must always hold true for your program to be considered correct. In Trident, you validate program behavior by capturing account states before and after transactions, then checking the expected changes using the `invariant!` macro.

## The `invariant!` Macro

Use the `invariant!` macro to define intentional invariant checks. When an invariant fails:

- It is counted and collected separately from unexpected panics
- Fuzzing continues to find more issues
- All failures are reported at the end with their seeds for reproduction

```rust
// Simple condition
invariant!(balance_after == balance_before - amount);

// With custom message
invariant!(balance > 0, "Balance must be positive");

// Comparison macros (auto-display values on failure)
invariant_eq!(balance_after, balance_before - amount);
invariant_gt!(balance, 0, "Balance must be positive");
```

!!! note "Invariants vs Regular Panics"
    Regular panics (like `unwrap()` on `None` or index out of bounds) are treated as bugs in your fuzz test and will crash immediately. Only `invariant!` failures are collected and allow fuzzing to continue.

## How Invariants Work

The validation pattern in Trident follows these steps:

1. **Capture state before transaction**
2. **Execute the transaction**  
3. **Capture state after transaction**
4. **Validate changes with `invariant!`**

## Basic Example

```rust
#[flow_executor]
impl FuzzTest {
    #[flow]
    fn transfer_flow(&mut self) {
        let account = self.fuzz_accounts.user_account.get(&mut self.trident);
        
        // Capture state before transaction
        let balance_before = self.trident
            .get_account_with_type::<UserAccount>(&account, 8)
            .expect("Account not found");
        
        // Execute the transaction
        let instruction = create_transfer_instruction(account, 100);
        let result = self.trident.process_transaction(&[instruction], Some("transfer"));
        
        // Validate the result
        if result.is_success() {
            let balance_after = self.trident
                .get_account_with_type::<UserAccount>(&account, 8)
                .expect("Account not found");
            
            self.transfer_invariant(balance_before, balance_after, 100);
        }
    }
    
    fn transfer_invariant(
        &mut self,
        before: UserAccount,
        after: UserAccount,
        amount: u64,
    ) {
        invariant!(
            after.balance == before.balance - amount,
            "Balance should decrease by transfer amount: expected {}, got {}",
            before.balance - amount,
            after.balance
        );
    }
}
```

## Why Invariants Are Useful

- **Catch Logic Errors**: Detect when your program doesn't behave as expected
- **Validate State Changes**: Ensure account modifications are correct
- **Test Edge Cases**: Verify behavior under various conditions
- **Prevent Regressions**: Catch bugs introduced by code changes
- **Continue Fuzzing**: Find multiple issues in a single run

## Writing Invariant Methods

Invariant methods should:

- Take before/after states as parameters
- Use descriptive messages with `invariant!`
- Focus on one specific behavior
- Include relevant values in error messages

```rust
fn token_mint_invariant(
    &mut self,
    mint_before: MintAccount,
    mint_after: MintAccount,
    minted_amount: u64,
) {
    invariant!(
        mint_after.supply == mint_before.supply + minted_amount,
        "Token supply should increase by minted amount: {} + {} != {}",
        mint_before.supply,
        minted_amount,
        mint_after.supply
    );
}
```

## Exit Code Modes

When running fuzz tests in CI/CD, use `--exit-code` to control which failures cause non-zero exit:

```bash
# Exit non-zero on any failure
trident fuzz run fuzz_0 --exit-code all

# Exit non-zero only on invariant failures
trident fuzz run fuzz_0 --exit-code invariants
```

Notes:

- Without `--exit-code`, invariant failures and program panics are reported but do not force a non-zero process exit.
- Unexpected fuzz-test panics (for example `unwrap()` on `None`) are always treated as runtime errors and fail the run.

## Available Macros

Beyond the basic `invariant!` macro shown above, Trident provides a full family of comparison macros that automatically display values on failure.

See the [Invariant Macros API Reference](../../trident-api/invariants.md) for signatures, failure messages, and examples.

---

For more complex examples and patterns, see the [Trident Examples](../../trident-examples/trident-examples.md) page.
