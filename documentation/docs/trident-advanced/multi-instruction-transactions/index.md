# Multi-Instruction Transactions

Trident supports executing multiple instructions within a single transaction using the `process_transaction` method with an array of instructions.

## Basic Usage

To execute multiple instructions in one transaction, pass an array of instructions to the `process_transaction` method:

```rust
#[flow_executor]
impl FuzzTest {
    #[flow]
    fn multi_instruction_flow(&mut self) {
        let instruction1 = create_first_instruction();
        let instruction2 = create_second_instruction();
        
        // Execute multiple instructions in a single transaction
        let result = self.trident.process_transaction(
            &[instruction1, instruction2], 
            "multi_instruction_transaction"
        );
        
        // Handle the result and perform assertions
        if result.is_success() {
            // Verify the combined effect of both instructions
            self.verify_multi_instruction_invariants();
        } else {
            // Handle transaction failure
            panic!("Multi-instruction transaction failed: {:?}", result.logs());
        }
    }
}
```

## Why Use Multi-Instruction Transactions

- **Atomic Operations**: All instructions succeed or fail together
- **Complex Workflows**: Test instruction sequences that depend on each other
- **State Consistency**: Ensure related operations maintain program invariants

For more complex examples and patterns, see the [Trident Examples](../../trident-examples/index.md) page.
