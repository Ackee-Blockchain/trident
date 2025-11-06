# Writing Fuzz Tests

Trident is a manually guided fuzzing framework for Solana programs.

To start fuzzing effectively, you need to guide the fuzzer by specifying the expected inputs to instructions and the expected sequences of instructions to execute.

## Why Manual Guidance is Important

Allowing the fuzzer to generate completely random instruction inputs and sequences would lead to:

- **Transaction failures** - The fuzzer wouldn't properly test the logic within your program
- **Irrelevant sequences** - Random instruction sequences would cause transaction failures instead of meaningful testing

## Guide the Fuzzer

Learn how to properly configure your fuzz tests:

- [Constructing Instructions](./construction-instructions.md) - How to build and execute instructions in your tests
- [Fuzzing Flows](./fuzzing-flows.md) - How to define instruction sequences and execution patterns

!!! tip "Start Simple"

    Begin with basic instruction construction and gradually add more complex flows as you become familiar with the framework.
