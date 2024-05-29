# {{ config.site_name }}

{{ config.site_name }} is a Rust-based framework to fuzz and integration test Solana programs to help you ship secure code.

# Features

- **Automated Test Generation**: Simplifies the testing process by automatically creating templates for fuzz and integration tests for programs written using the Anchor Framework.

- **Dynamic Data Generation**: Increases test coverage with random instruction data and pseudo-random accounts for unpredictable fuzz test scenarios.

- **Custom Instruction Sequences**: Provides the flexibility to design specific sequences of instructions to meet particular testing needs or to focus on particular aspects of program behavior during fuzz testing.
Invariant Checks: Allows for custom invariants checks to spot vulnerabilities and unwanted behaviors.
