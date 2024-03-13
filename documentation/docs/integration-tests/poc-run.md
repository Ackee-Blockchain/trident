# Run
Once you have finished the implementation of the Integration Test, you can run the Test as follows:

```bash
trident test
```

## Skipping tests

- You can add the `#[ignore]` macro to skip the test.

```rust
#[trident_test]
#[ignore]
async fn test() {}
```
