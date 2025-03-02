# Flow Executor

The Flow Executor macros are a set of attribute macros used to define and control execution flows. These macros help organize and manage the execution of complex test scenarios.

## Available Macros

The Flow Executor functionality consists of four main attribute macros:

1. `#[flow_executor]` - Implements the flow executor for a struct
2. `#[flow]` - Marks a method as part of the execution flow
3. `#[init]` - Marks a method as the initialization method for the flow
4. `#[flow_ignore]` - Marks a flow method to be skipped during execution

## Usage

!!! warning "Default callback"

    If not flows are defined, or all of the flows are ignored, the default callback will execute a `random selection sequence` of transactions from the `FuzzTransactions` enum.

```rust
use trident_fuzz::prelude::*;


#[derive(Default)]
struct FuzzTest<C> {
    client: C,
}

#[flow_executor]
impl<C: FuzzClient + std::panic::RefUnwindSafe> FuzzTest<C> {
    fn new(client: C) -> Self {
        Self { client }
    }
    #[init]
    fn initialize(&mut self, client: &mut C) -> Result<(), FlowError> {
        // Initialization logic
        Ok(())
    }

    #[flow]
    fn flow1(&mut self, client: &mut C) -> Result<(), FlowError> {
        // First step in the flow
        Ok(())
    }

    #[flow]
    fn flow2(&mut self, client: &mut C) -> Result<(), FlowError> {
        // Second step in the flow
        Ok(())
    }

    #[flow_ignore]
    #[flow]
    fn skipped_flow(&mut self, client: &mut C) -> Result<(), FlowError> {
        // This step will be skipped during execution
        Ok(())
    }
}
```

## Macro Details

### `#[flow_executor]`

The `flow_executor` attribute macro is applied to an `impl` block and implements the flow executor functionality for the struct.

```rust
#[flow_executor]
impl<C: FuzzClient + std::panic::RefUnwindSafe> FuzzTest<C> {
    // Flow methods...
}
```

This macro:

- Analyzes all methods in the impl block
- Identifies methods marked with `#[flow]` and `#[init]`
- Generates code to execute these methods in the defined order

### `#[flow]`

The `flow` attribute macro marks a method as part of the execution flow.

!!! warning "Flow Methods"
    It is possible to define multiple flow methods.

    Multiple flows are executed sequentially.

```rust
#[flow]
fn step_one(
    &mut self,
    fuzzer_data: &mut FuzzerData,
    accounts: &mut FuzzAccounts
) -> Result<(), FuzzingError> {
    // Flow step implementation
    Ok(())
}
```

### `#[init]`

The `init` attribute macro marks a method as the initialization method for the flow.

!!! warning "Initialization Method"
    It is possible to define only one initialization method.

```rust
#[init]
fn initialize(&mut self) {
    // Initialization logic
    // For example, deploy program here
}
```

The init method:
- Is executed before any flow methods
- Should set up any necessary state for the flow
- Must have the same signature as flow methods
- Is optional - if not provided, the flow will start with the first flow method

### `#[flow_ignore]`

The `flow_ignore` attribute macro marks a flow method to be skipped during execution.

```rust
#[flow_ignore]
#[flow]
fn step_one(
    &mut self,
    fuzzer_data: &mut FuzzerData,
    accounts: &mut FuzzAccounts
) -> Result<(), FuzzingError> {
    // Flow step implementation
    Ok(())
}
```
