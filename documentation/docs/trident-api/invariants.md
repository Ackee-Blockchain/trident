# Invariant Macros

Invariant macros are used to define intentional checks in your fuzz tests. When an invariant fails, it is counted and collected separately from unexpected panics, and fuzzing continues to find more issues.

For conceptual guidance on how and when to use invariants, see [Invariants and Assertions](../trident-advanced/invariants-assertions/index.md).


!!! note "Invariants vs Regular Panics"
    Only `invariant!` failures are collected and allow fuzzing to continue. Regular panics (like `unwrap()` on `None`) are treated as bugs in your fuzz test and will crash the worker immediately.

    
---

## `invariant!`

Checks a boolean condition. The most general-purpose invariant macro.

```rust
invariant!(condition);
invariant!(condition, "message with {} formatting", value);
```

**Failure message (default):**

```
invariant violation: balance > 0
```

**Example:**

```rust
invariant!(account.is_initialized);
invariant!(balance > 0, "Balance must be positive, got {}", balance);
```

---

## `invariant_eq!`

Checks that two expressions are equal. Displays actual values on failure.

```rust
invariant_eq!(left, right);
invariant_eq!(left, right, "custom message");
```

**Failure message (default):**

```
invariant violation: `balance` == `100`, got: 42 vs 100
```

**Example:**

```rust
invariant_eq!(balance_after, balance_before - amount);
invariant_eq!(account.owner, program_id, "wrong owner");
```

---

## `invariant_ne!`

Checks that two expressions are not equal. Displays the shared value on failure.

```rust
invariant_ne!(left, right);
invariant_ne!(left, right, "custom message");
```

**Failure message (default):**

```
invariant violation: `owner` != `Pubkey::default()`, but both are: 11111111111111111111111111111111
```

**Example:**

```rust
invariant_ne!(owner, Pubkey::default());
invariant_ne!(balance_after, balance_before, "balance should have changed");
```

---

## `invariant_gt!`

Checks that the first expression is strictly greater than the second.

```rust
invariant_gt!(left, right);
invariant_gt!(left, right, "custom message");
```

**Failure message (default):**

```
invariant violation: `balance` > `0`, got: 0 vs 0
```

**Example:**

```rust
invariant_gt!(balance, 0);
invariant_gt!(supply_after, supply_before, "supply should increase after mint");
```

---

## `invariant_gte!`

Checks that the first expression is greater than or equal to the second.

```rust
invariant_gte!(left, right);
invariant_gte!(left, right, "custom message");
```

**Failure message (default):**

```
invariant violation: `lamports` >= `rent_exempt`, got: 500 vs 1000
```

**Example:**

```rust
invariant_gte!(balance, minimum_balance);
invariant_gte!(lamports, rent_exempt, "account not rent-exempt");
```

---

## `invariant_lt!`

Checks that the first expression is strictly less than the second.

```rust
invariant_lt!(left, right);
invariant_lt!(left, right, "custom message");
```

**Failure message (default):**

```
invariant violation: `fee` < `max_fee`, got: 500 vs 100
```

**Example:**

```rust
invariant_lt!(balance_after, balance_before);
invariant_lt!(fee, max_fee, "fee exceeds maximum");
```

---

## `invariant_lte!`

Checks that the first expression is less than or equal to the second.

```rust
invariant_lte!(left, right);
invariant_lte!(left, right, "custom message");
```

**Failure message (default):**

```
invariant violation: `total_supply` <= `max_supply`, got: 1000001 vs 1000000
```

**Example:**

```rust
invariant_lte!(withdrawal, balance);
invariant_lte!(total_supply, max_supply, "supply overflow");
```

---



