# TransactionSelector

The `TransactionSelector` macro is used to derive required methods for random transaction selection during fuzzing operations. This macro automatically implements functionality for selecting and executing transactions based on enum variants.

!!! note "Selection Behavior"
    The macro will automatically implement random selection between the enum variants during fuzzing operations.

## Derived Traits

The macro implements the following trait:

- `TransactionSelector<T>` - Methods to select and execute transactions

!!! warning "Manual Implementation Note"
    There is no need to manually implement any methods of this trait. The macro handles all implementations automatically based on the structure of your transaction selector.

## Transaction Selector Methods

### `transaction_selector`

Selects and executes a transaction based on the enum variant.

```rust
fn transaction_selector(
    &mut self,
    client: &mut impl FuzzClient,
    fuzz_accounts: &mut T,
) -> Result<(), FuzzingError>
```
