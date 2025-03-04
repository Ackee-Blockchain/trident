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
        // Initialize the fuzz test
    }
    #[flow]
    fn flow1(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {
        // Some logic here

        Ok(())
    }
}
```


## Example

The following example demonstrates:

- Deploying a native program in the method marked with `#[init]`
- Executing a sequence of transactions (`SomeTransaction` and `AnotherTransaction`) in the method (`flow1`) marked with `#[flow]`
- Executing another sequence of transactions (`AnotherTransaction`) in the method (`flow2`) marked with `#[flow]`
- Executing random transaction from the `FuzzTransactions` enum in the method (`flow3`) marked with `#[flow]`
- The execution is sequential, i.e., the flow methods are executed one after another
- Using `#[flow_executor(random_tail = true)]` to execute random Transactions at the end (after all of the flow methods are executed)

```rust
#[derive(Default, FuzzTestExecutor)]
struct FuzzTest {
    client: TridentSVM,
}
#[flow_executor(random_tail = true)]
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
    #[flow]
    fn flow3(
        &mut self,
        fuzzer_data: &mut FuzzerData,
        accounts: &mut FuzzAccounts,
    ) -> Result<(), FuzzingError> {

        FuzzTransactions::select_n_execute(fuzzer_data, &mut self.client, accounts)?;

        Ok(())
    }
}
```
