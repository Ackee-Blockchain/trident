---
hide:
  - toc
---

# Trident Features

Trident contains multiple features to enhance the fuzzing experience and increase ability to discover bugs.

<div class="grid cards" markdown>

-   :material-store:{ .lg .middle } __AccountStorages__

    ---

    Initialize, store and re-use random Accounts.


    [__AccountStorages__](./account-storages.md)

-   :material-instrument-triangle:{ .lg .middle } __FuzzInstructions__

    ---

    All available Program Instructions as FuzzInstructions Variants.


    [__FuzzInstructions__](./fuzz-instructions.md)

-   :material-frequently-asked-questions:{ .lg .middle } __Custom Instruction Sequences__

    ---

    Specify Instruction Sequqnces you would like to executed instead of completely random execution order.


    [__Custom Instruction Sequences__](./instructions-sequences.md)

-   :material-vector-difference:{ .lg .middle } __Invariant Checks__

    ---

    Allows to compare Account contents before and after the Instruction was successfully executed.


    [__Invariant Checks__](./invariant-checks.md)

-   :octicons-cross-reference-16:{ .lg .middle } __Cross Program Invocations__

    ---

    Use Cross Program Invocation of Native and SBF programs dumped for example from Mainnet.


    [__Cross Program Invocations__](./cross-program-invocation.md)

-   :simple-statista:{ .lg .middle } __Fuzzing Statistics__

    ---

    Show Fuzzing Statistics after the Fuzzing Session ended.


    [__Fuzzing Statistics__](./fuzzing-statistics.md)

-   :simple-instructure:{ .lg .middle } __Arbitrary Data__

    ---

    Customize structure of Instruction Input arguments.

    [__Arbitrary Data__](./arbitrary-data.md)

-   :material-weight-lifter:{ .lg .middle } __Trident Manifest__

    ---

    Customize Fuzzing experience with the Trident Manifest.


    [__Trident Manifest__](./trident-manifest.md)

-   :octicons-copilot-error-16:{ .lg .middle } __Custom Error Handlers__

    ---

    Continue Instruction Sequence in case of Instruction Error was returned or specify custom behavior based on the returned error.


    [__Custom Error Handlers__](./error-handlers.md)


</div>

<!-- - AccountStorages
- FuzzInstructions
- Invariant Checks
- Custom Instruction Sequences
- Custom Error Handlers
- Arbitrary Data
- Cross Program Invocations
- Trident Manifest
- Fuzzing Statistics






!!! important

    At the current development stage, there are some manual steps required to start the Fuzzing Session. In principle:

    **Prerequisites:**

     - Add dependencies specific to your program to `trident-tests/fuzz_tests/Cargo.toml` (such as anchor-spl etc.).

    **Writing Fuzz Tests**

     1. Include desired Programs [Include Programs](../writing-fuzz-test/programs.md).
     2. Specify pseudo-random accounts to re-use [Accounts to re-use](../writing-fuzz-test/accounts.md).
     3. Specify instruction data [Instruction Data](../writing-fuzz-test/instruction-data.md).
     4. Specify instruction accounts [Instruction Accounts](../writing-fuzz-test/instruction-accounts.md).

    **Run and Debug**

     1. Execute desired fuzz test [Run](../execute/run.md)
     2. See the found crash with more details [Debug](../execute/debug.md)

!!! note

    For better fuzzing results and experience you can also manually adjust the following:

     1. Define Invariants checks [Invariants Checks](../writing-fuzz-test-extra/invariants-checks.md).
     2. Specify instruction sequences [Instruction sequences](../writing-fuzz-test-extra/instruction-sequences.md).
     3. Specify custom data types [Custom Data types](../writing-fuzz-test-extra/custom-data-types.md).
     4. Well structured data [Arbitrary](../writing-fuzz-test-extra/arbitrary.md).
     4. AccountsSnapshots macro [AccountsSnapshots](../writing-fuzz-test-extra/accounts-snapshots.md). -->
