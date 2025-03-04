# TransactionSelector

The `TransactionSelector` macro is used to derive required methods for random transaction selection during fuzzing operations. This macro automatically implements functionality for selecting and executing transactions based on enum variants.

## Derived Traits

The macro implements the following trait:

- `TransactionSelector<T>` - Methods to select and execute transactions

!!! warning "Manual Implementation Note"
    There is no need to manually implement any methods of this trait. The macro handles all implementations automatically based on the structure of your transaction selector.

## Transaction Selector Methods

### `transaction_selector`

Internal method used by the fuzzer to execute selected transactions. This method should not be called directly - use `select_n_execute` instead.

```rust
fn transaction_selector(
    &mut self,
    client: &mut impl FuzzClient,
    fuzz_accounts: &mut T,
) -> Result<(), FuzzingError>
```

---

### `select_n_execute`

Selects a random transaction variant and executes it.

!!! warning "Transaction Hooks"
    The `select_n_execute` method executes the transaction `with` all transaction hooks enabled.

```rust
fn select_n_execute(
    fuzzer_data: &mut FuzzerData,
    client: &mut impl FuzzClient,
    accounts: &mut T,
) -> Result<(), FuzzingError>
```

---

### `select_n_execute_no_hooks`

Selects a random transaction variant and executes it.

!!! warning "Transaction Hooks"
    The `select_n_execute_no_hooks` method executes the transaction `without` any transaction hooks.

```rust
fn select_n_execute_no_hooks(
    fuzzer_data: &mut FuzzerData,
    client: &mut impl FuzzClient,
    accounts: &mut T,
) -> Result<(), FuzzingError>
```
