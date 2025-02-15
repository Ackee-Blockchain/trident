# Writing fuzz test

In order to start fuzzing, you need to guide the fuzzer to use correct and meaningful instruction inputs.

Trident generates random data for instruction executions. However, allowing the fuzzer to generate completely random account addresses would lead to numerous failed transactions, making the fuzzing process ineffective.

Each program instruction has a corresponding file in the instructions directory. Instructions consist of two main components:

- Instruction Accounts
- Instruction Data
- (Optional) Remaining Accounts

## Guide the Instruction Inputs

Start with properly configuring different types of instruction inputs:

- [Instruction Accounts](./instruction-accounts.md)
- [Instruction Data](./instruction-data.md)
- (Optional) [Remaining Accounts](./remaining-accounts.md)
