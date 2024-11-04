

# How To start fuzzing.

## To start fuzzing, follow these steps:

- Install ***Honggfuzz***

```bash
cargo install honggfuzz
```

For supported versions check https://ackee.xyz/trident/docs/latest/getting-started/getting-started/#supported-versions

For more info about Honggfuzz installation check https://github.com/rust-fuzz/honggfuzz-rs?tab=readme-ov-file#dependencies

- Initialize ***Trident*** using

```bash
trident init
```

## Write Fuzz Test

- Implement the ***todo!*** placeholders in ***fuzz_instructions.rs*** based on the provided descriptions.

- Run fuzzing with ***Honggfuzz***

```bash
trident fuzz run-hfuzz <FUZZ_TARGET>
```

### For more details, refer to the Trident documentation: https://ackee.xyz/trident/docs/latest/
