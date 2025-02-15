# Transaction Error Hook

Trident lets you handle errors that occur during the transaction execution.

This might be helpful if you want to investigate the error and check if the behavior is as expected.

!!! warning "Error Handler Execution order"
    The `transaction_error_handler` method is called only after the transaction has `FAILED` to complete.

```rust
impl TransactionCustomMethods for DepositObligationCollateralV2Transaction {
    fn transaction_error_handler(&self, e: TransactionError) -> Result<(), TransactionError> {
        // investigae the error and check if the behavior is valid
    }
}
```
