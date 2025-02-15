# Pre and Post Transaction Hooks

Trident lets you perform specific logic before and after transaction execution, with `pre_transaction` and `post_transaction` methods.

These methods expect a `FuzzClient` as an argument, which provides a set of methods to interact with the fuzzing environment, check the [FuzzClient](../../../trident-api-macro/trident-fuzz-client/index.md) for more details.

!!! warning "Post-Transaction Execution order"
    The `post_transaction` method is called only after the transaction is `SUCCESSFULLY` executed.

## Pre-Transaction Execution

Example: always forward in time before the transaction.

```rust
impl TransactionCustomMethods for ExampleTransaction {
    fn pre_transaction(&self, client: &mut impl FuzzClient) {
        // before the transaction, always forward in time
        client.forward_in_time(500);
    }
}
```

## Post-Transaction Execution

Example: always forward in time after the transaction.

```rust
impl TransactionCustomMethods for ExampleTransaction {
    fn post_transaction(&self, client: &mut impl FuzzClient) {
        // after the transaction, always forward in time
        client.forward_in_time(500);
    }
}
```
