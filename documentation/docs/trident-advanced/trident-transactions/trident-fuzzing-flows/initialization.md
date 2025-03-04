# Initialization

Initialization configures the test environment before flow execution. It sets up program deployment and establishes necessary preconditions, for example program deployments.

## The `#[init]` Attribute

The `#[init]` attribute marks a method within a `#[flow_executor]` implementation block as the initialization method.

!!! warning "Init Method definition"
    - There can be only one method marked as `#[init]`.
    - The method interface is strict, meaning the method has to have the same interface as shown in the example below.

```rust
#[derive(FuzzTestExecutor)]
struct FuzzTest {
    client: TridentSVM,
}

#[flow_executor]
impl FuzzTest {
    #[init]
    fn example_init_method(&mut self) {
        // ...
    }
    // ...
}
```


## Example

The following example demonstrates:

- Deploying a native program in the method marked with `#[init]`

```rust
#[derive(FuzzTestExecutor)]
struct FuzzTest {
    client: TridentSVM,
}

#[flow_executor]
impl FuzzTest {
    #[init]
    fn example_init_method(&mut self) {
        // Example deploy hello-world program

        self.client.deploy_native_program(ProgramEntrypoint::new(
            pubkey!("FtevoQoDMv6ZB3N9Lix5Tbjs8EVuNL8vDSqG9kzaZPit"),
            None,
            processor!(entry_hello_world),
        ));

        // ...
    }
    // ...
}
```
