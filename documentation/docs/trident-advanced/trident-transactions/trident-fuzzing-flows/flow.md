# Flow Definition

Flows allow you to define custom sequences of behavior during each fuzzing iteration.

For more reference see [TridentFlowExecutor](../../../trident-api-macro/trident-macros/trident-flow-executor.md)

## The `#[flow]` Attribute

Flow is marked with the `#[flow]` attribute.

!!! warning "Flow methods definition"
    - It is allowed to specify multiple flows (i.e., marking multiple methods with `#[flow]`)
    - The flow methods have to adhere to the strict method interface as shown in the example below
    - If no flows are specified, or all flows are marked with `#[flow_ignore]`, the default callback will be called, executing random transactions from the `FuzzTransactions` enum
    - If multiple flows are defined, they are executed sequentially


```rust
#[derive(Default, FuzzTestExecutor)]
struct FuzzTest {
    client: TridentSVM,
}
#[flow_executor]
impl FuzzTest {
    #[init]
    fn start(&mut self) {
        self.client.deploy_native_program(ProgramEntrypoint::new(
            pubkey!("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit"),
            None,
            processor!(entry_hello_world),
        ));
    }
    #[flow]
    fn flow1(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {
        // Execute transactions in a specific sequence
        SomeTransaction::build(fuzzer_data, &mut self.client, accounts)?
            .execute(&mut self.client)?;

        AnotherTransaction::build(fuzzer_data, &mut self.client, accounts)?
            .execute(&mut self.client)?;

        Ok(())
    }

    #[flow]
    fn flow2(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {

        AnotherTransaction::build(fuzzer_data, &mut self.client, accounts)?
            .execute(&mut self.client)?;

        Ok(())
    }
}
```
