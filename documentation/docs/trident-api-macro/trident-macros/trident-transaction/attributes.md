# Attributes


This macro accepts the following attributes:

### `name`

The custom name of the transaction.

`This attribute is optional`

```rust
#[derive(Arbitrary, Debug, TridentTransaction)]
#[name("Custom Transaction Name")]
pub struct ExampleTransaction {
    pub instruction1: ExampleInstruction,
}
```
