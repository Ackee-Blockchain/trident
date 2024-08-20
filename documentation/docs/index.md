---
hide:
  - navigation
  - toc
---

# {{ config.site_name }}

![Trident](./images/trident-logo-smaller.png){ align=right }

Rust-based Fuzzing framework for Solana programs to help you ship secure code.

<div class="grid cards" markdown>

-   :material-download:{ .lg .middle } __Getting Started__

    ---

    Install the Trident Fuzz Testing Framework

    [Getting started](./getting-started/getting-started.md)

-   :material-test-tube:{ .lg .middle } __Start Fuzzing__

    ---

    Focus on security and start fuzzing immediatelly

    [Start Fuzzing](./fuzzing/first-steps/fuzz-test-initialization.md)

-   :octicons-mark-github-24:{ .lg .middle } __Check the GitHub for unreleased features__

    ---

    Check our GitHub repository to see the unreleased features

    [Trident Repository](https://github.com/Ackee-Blockchain/trident/tree/develop)

-   :material-run-fast:{ .lg .middle } __Trident by Examples__

    ---

    Try the Fuzzing Examples

    [Trident Examples](./fuzzing/extra/examples.md)

</div>


## What is Fuzzing ?

*"Fuzz testing is an automated technique that provides generated random, invalid, or unexpected input data to your program. This helps discover unknown bugs and vulnerabilities, potentially preventing zero-day exploits."*


{{ config.site_name }} equips developers with tools to efficiently develop fuzz tests for Anchor-based programs. It streamlines the fuzz testing process through automation and comprehensive support

<div class="grid cards" markdown>

- __Trident Workflow__

    ---

    - **Automatically parses Anchor-based programs** to generate necessary implementations for deserializing instruction accounts.
    - **Generates templates** for developers to customize according to the specific needs of their fuzz test scenarios.
    - **Offers derive macros** to effortlessly implement required traits, reducing manual coding efforts.
    - **Includes a bank client** and helper functions for simplified account management during testing.
    - **Provides a Command-Line Interface** (CLI) for executing and debugging fuzz tests with ease.

- __Trident Capabilities__

    ---

    - **Execution Order of Instructions**: Test different sequences and their effects on the program to uncover sequence-related vulnerabilities.
    - **Instruction Parameters**: Identify how variations in inputs influence program behavior, testing for robustness against a wide range of data.
    - **Instruction Accounts**: Explore the impact of different account states on the software's functionality, ensuring comprehensive account testing.
    - **Comprehensive Testing**: Conduct thorough and effective fuzz testing by combining any of the above aspects.

</div>
