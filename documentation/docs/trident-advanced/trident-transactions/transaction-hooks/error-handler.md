# Transaction Error Hook

Trident lets you handle errors that occur during transaction execution.

This feature is helpful when you want to investigate errors and verify if the behavior is as expected.

!!! warning "Error Hook Execution Order"
    The `transaction_error_handler` method is called only when the transaction has `FAILED`.

```rust
impl TransactionHooks for ExampleTransaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_error_handler(&self, e: TransactionError) -> Result<(), TransactionError> {
        // Your custom error handling logic goes here
    }
}
```


## Example

The following example demonstrates:

- Logging the error
- Returning Ok to suppress the error


```rust
impl TransactionHooks for ExampleTransaction {
    type IxAccounts = FuzzAccounts;

    fn transaction_error_handler(&self, e: TransactionError) -> Result<(), TransactionError> {
        // Example: Log the error and decide whether to propagate it
        eprintln!("Transaction failed with error: {:?}", e);

        // Return Ok to suppress the error, or Err to propagate it
        Ok(())
    }
}
```
