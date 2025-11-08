# Invariants and Assertions

Invariants are conditions that must always hold true for your program to be considered correct. In Trident, you validate program behavior by capturing account states before and after transactions, then checking the expected changes with custom invariant methods.

## How Invariants Work

The validation pattern in Trident follows these steps:

1. **Capture state before transaction**
2. **Execute the transaction**  
3. **Capture state after transaction**
4. **Validate changes with invariant methods**

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
        let result = self.trident.process_transaction(&[instruction], "transfer");
        
        // Validate the result
        if result.is_success() {
            let balance_after = self.trident
                .get_account_with_type::<UserAccount>(&account, 8)
                .expect("Account not found");
            
            self.transfer_invariant(balance_before, balance_after, 100);
        } else {
            // Handle expected failures
            assert!(
                result.is_custom_error_with_code(6001_u32),
                "Expected insufficient funds error"
            );
        }
    }
    
    fn transfer_invariant(
        &mut self,
        before: UserAccount,
        after: UserAccount,
        amount: u64,
    ) {
        assert_eq!(
            after.balance, 
            before.balance - amount,
            "Balance should decrease by transfer amount"
        );
    }
}
```

## Why Invariants Are Useful

- **Catch Logic Errors**: Detect when your program doesn't behave as expected
- **Validate State Changes**: Ensure account modifications are correct
- **Test Edge Cases**: Verify behavior under various conditions
- **Prevent Regressions**: Catch bugs introduced by code changes

## Writing Invariant Methods

Invariant methods should:

- Take before/after states as parameters
- Use descriptive assertion messages
- Focus on one specific behavior
- Handle both success and failure cases

```rust
fn token_mint_invariant(
    &mut self,
    mint_before: MintAccount,
    mint_after: MintAccount,
    minted_amount: u64,
) {
    assert_eq!(
        mint_after.supply,
        mint_before.supply + minted_amount,
        "Token supply should increase by minted amount"
    );
}
```

For more complex examples and patterns, see the [Trident Examples](../../trident-examples/trident-examples.md) page.
