# Flow Executor

The Flow Executor macros are a set of attribute macros used to define and control execution flows. These macros help organize and manage the execution of complex test scenarios.

## Available Macros

The Flow Executor functionality consists of four main attribute macros:

1. `#[flow_executor]` - Implements the flow executor for a struct
2. `#[flow]` - Marks a method as part of the execution flow
3. `#[init]` - Marks a method to run once at the beginning of each iteration before all flow methods
4. `#[end]` - Marks a method to run once at the end of each iteration after all flow methods

## Usage

```rust

#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// for fuzzing
    trident: Trident,
    /// for storing fuzzing accounts
    fuzz_accounts: FuzzAccounts,
}

#[flow_executor]
impl FuzzTest {
    fn new() -> Self {
        Self {
            trident: Trident::default(),
            fuzz_accounts: FuzzAccounts::default(),
        }
    }

    #[init]
    fn start(&mut self) {
        // perform any initialization here, this method will be executed
        // at start of each iteration
    }

    #[flow]
    fn flow1(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows
    }

    #[flow]
    fn flow2(&mut self) {
        // perform logic which is meant to be fuzzed
        // this flow is selected randomly from other flows
    }

    #[end]
    fn end(&mut self) {
        // perform any cleanup here, this method will be executed
        // at the end of each iteration
    }
}
```

## Implementation-Level attributes

### `#[flow_executor]`

The `flow_executor` attribute macro is applied to an `impl` block and implements the flow executor functionality for the struct.

```rust
#[flow_executor]
impl FuzzTest {
    // Flow methods...
}
```

---


## Method-Level attributes

### `#[init]`

The `init` attribute macro marks a method as the initialization method for the flows. This method is executed at the beginning of each iteration before all flow methods.

!!! warning "Initialization Method"
    It is possible to define only one initialization method.

```rust
#[init]
fn initialize(&mut self) {
    // Initialization logic
    // Perform initialization Transaction here
}
```

---


### `#[flow]`

The `flow` attribute macro marks a method as part of the execution flow.

!!! warning "Flow Methods"
    It is possible to define multiple flow methods.

    If multiple flows are defined, the fuzzer will pick randomly and generate a sequence of random flows to execute.

```rust
#[flow]
fn step_one(&mut self) {
    // Flow step implementation
}
```

---




### `#[end]`

The `end` attribute macro marks a method to run once at the end of each iteration after all flow methods.

!!! warning "Cleanup Method"
    It is possible to define only one end method.

```rust
#[end]
fn cleanup(&mut self) {
    // Cleanup logic
}
```

## Generated Methods


### `fuzz`

Runs the fuzzing process with multiple concurrent fuzzing threads.

```rust
fn fuzz(iterations: u64, flow_calls_per_iteration: u64)
```

- `iterations` - Number of iterations to run
- `flow_calls_per_iteration` - Number of flow methods called in each iteration (e.g. if the `flow_calls_per_iteration` is 100, the fuzzer will pick a random sequence of 100 flow methods to execute in each iteration)
