# Fuzzing Flows

Before you start fuzzing, you need to guide the fuzzer on which transactions it should execute. This is important because if your program contains many instructions and we let the fuzzer randomly pick from them, most of the time the fuzzer would pick incorrect sequences. This is because Solana programs expect a specific order of operations.

For example, DeFi protocols typically expect something like:

1. Initialize Config
2. Initialize User
3. User deposit
4. User swap (there might be multiple swaps, with random order, this is achieved with flows)
5. User withdraw
6. User account remove
7. Collect trading fees
8. ...

For this reason, you need to guide the fuzzer. The following code shows the default structure of the `test_fuzz.rs` file:



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
        // perform any cleaning here, this method will be executed
        // at the end of each iteration
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}
```

Let's examine the components of the `test_fuzz.rs` file:

## The Main Function

```rust
fn main() {
    FuzzTest::fuzz(1000, 100);
}
```

This is the entry point for fuzzing. Here you specify:

- **Iterations**: How many complete test cycles to run (1000 in this example)
- **Flows per iteration**: How many flow methods to execute in each iteration (100 in this example)

## The FuzzTest Struct

This struct contains:

- **Trident**: Provides methods for fuzzing, accounts management, and transaction execution
- **FuzzAccounts**: Stores addresses used in fuzzing

```rust
#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// for fuzzing
    trident: Trident,
    /// for storing fuzzing accounts
    fuzz_accounts: FuzzAccounts,
}
```

## The FuzzTest Implementation

This implementation block contains the methods that guide the fuzzer's behavior:

### The `new` Method

The `new` method is called once at the start of fuzzing to initialize the fuzzer and fuzzing accounts to their default (empty) state.

### The `#[init]` Method

The `#[init]` method executes at the start of each iteration. Use this method to perform setup transactions that initialize your program state, such as:

- Global configuration
- User accounts  
- Token accounts
- Other prerequisite state

### The `#[flow]` Methods

Methods marked with `#[flow]` are where the main fuzzing occurs. These methods:

- Are selected randomly from all available flow methods
- Execute in random order during each iteration
- Contain the core logic you want to fuzz

For example, with `flow1` and `flow2` methods marked with `#[flow]`, in each iteration 100 flow methods will be selected randomly. The execution order might be: `flow1`, `flow1`, `flow2`, `flow2`, `flow1`, `flow1`, etc., until 100 flows complete.

!!! tip "Best Practices"

    - Start with simple flows and gradually add complexity
    - Use meaningful names for your flow methods
    - Keep initialization logic separate from fuzzing logic
    - Test edge cases by creating specific flow combinations

## Complete Example

For detailed, working examples of fuzzing flows in action, check out the [Trident Examples](../../trident-examples/trident-examples.md).

