# Writing fuzz test

Trident is Manually Guided fuzzing framework for Solana programs.

In order to start fuzzing, you need to guide the fuzzer by specifying what are the expected inputs to instruction or the expected sequences of instructions to execute.

**Why is this important?**

Letting the fuzzer to generate completely random instruction inputs and completely random sequqnces of instruction would lead in most case 

- to transaction failures, which would mean the fuzzer is not properly fuzzing the logic within the program; and
- executing irrelevant random instruction sequences, which would again lead to transaction failures.


## Guide the fuzzer

Start with properly configuring different types of instruction inputs:

- [Instruction Data](./instruction-data.md)
- [Instruction Accounts](./instruction-accounts.md)
- [Fuzzing Flows](./fuzzing-flows.md)
- (Optional) [Remaining Accounts](./remaining-accounts.md)
