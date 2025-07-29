# Fuzzing Flows

Before you start fuzzing, you need to guide the fuzzer on which transactions it should execute. This is important because if your program contains a lot of instructions and we let the fuzzer randomly pick from them, most of the time the fuzzer would pick incorrect sequences. This is due to the fact that Solana programs expect some kind of order. For example, DeFi protocols most likely expect something like:

1. Initialize Config
2. Initialize User
3. User deposit
4. User swap (there might be multiple swaps, with random order, this is achieved with flows)
5. User withdraw
6. User account remove
7. Collect trading fees
8. ...

For that reason, it is required to slightly guide the fuzzer. The following code shows the default state of the `test_fuzz.rs` file.



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

Let's describe the `test_fuzz.rs` file.

## The main

```rust
fn main() {
    FuzzTest::fuzz(1000, 100);
}
```

This is the starting point of the fuzzing. Here you can specify how many iterations you want to run and how many flows you want to execute in each iteration. Flows are described below.

## The `FuzzTest` struct

This is a struct containing Trident, which contains multiple methods for fuzzing, collecting fuzzing metrics, etc., and FuzzAccounts, which is used to store account IDs and their corresponding Pubkeys (addresses).

```rust
#[derive(FuzzTestMethods)]
struct FuzzTest {
    /// for fuzzing
    trident: Trident,
    /// for storing fuzzing accounts
    fuzz_accounts: FuzzAccounts,
}
```

## The `impl FuzzTest` block

This is the main block where you guide the fuzzer to perform the fuzzing.

### The `new` method

The `new` method is called once at the start of the fuzzing. It is used to initialize the fuzzer and the fuzzing accounts to their default state (so empty).

### The `#[init]` method

The `#[init]` method is executed at the start of each iteration. As described above, it is possible to specify the number of iterations and flows, meaning that this `#[init]` method will be executed at the start of each iteration. Within it, you can perform execution of transactions which should initialize something in your program, for example Global Config, User account, Token Accounts, etc.

### The `#[flow]` method

Methods marked with `#[flow]` are where the fuzzing happens (apart from the fact that the instructions contain random data on their inputs). Methods marked with `#[flow]` are selected randomly from other flows and executed in random order.

As shown in the source code below, the `flow1` and `flow2` methods are marked with `#[flow]`. This means, in each iteration, 100 flow methods will be selected in random order and the logic specified in the methods will be executed. So if we have two flow methods, the random order can be `flow1`, `flow1`, `flow2`, `flow2`, `flow1`, `flow1`, etc., until 100 flows are executed.

```rust
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
```


## An example

As an example:

- Within the block below, we have 4 flows. The `start` method is executed at the start of each iteration. The `flow1`, `flow2`, `flow3`, and `flow4` methods are executed in random order in each iteration.
- There will be 1000 iterations and 100 flows executed in each iteration.
- A random order of `MoveEastTransaction`, `MoveSouthTransaction`, `MoveNorthTransaction`, and `MoveWestTransaction` will be executed in each iteration, due to the fact that they are marked with `#[flow]`.



```rust
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
        let mut tx = InitializeTransaction::build(&mut self.trident, &mut self.fuzz_accounts);

        self.trident
            .execute_transaction(&mut tx, Some("Initialize"));
    }

    #[flow]
    fn flow1(&mut self) {
        let mut tx = MoveEastTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut tx, Some("MoveEast"));
    }
    #[flow]
    fn flow2(&mut self) {
        let mut tx = MoveSouthTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut tx, Some("MoveSouth"));
    }
    #[flow]
    fn flow3(&mut self) {
        let mut tx = MoveNorthTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut tx, Some("MoveNorth"));
    }
    #[flow]
    fn flow4(&mut self) {
        let mut tx = MoveWestTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
        self.trident.execute_transaction(&mut tx, Some("MoveWest"));
    }
}

fn main() {
    FuzzTest::fuzz(1000, 100);
}

```








