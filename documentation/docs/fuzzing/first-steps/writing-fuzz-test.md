# Writing Fuzz Test

!!! important

    At the current development stage, there are some manual steps required to start the Fuzzing Session. In principle:

    **Prerequisites:**

     - Add dependencies specific to your program to `trident-tests/fuzz_tests/Cargo.toml` (such as anchor-spl etc.).
     - Add necessary `use` statements into `trident-tests/fuzz_tests/<FUZZ_TEST_NAME>/accounts_snapshots.rs` to import missing types.

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
     2. Specify instruction sequences[Instruction sequences](../writing-fuzz-test-extra/instruction-sequences.md).
     3. Specify custom data types[Custom Data types](../writing-fuzz-test-extra/custom-data-types.md).
     4. Well structured data[Arbitrary](../writing-fuzz-test-extra/arbitrary.md).
     4. AccountsSnapshots macro[AccountsSnapshots](../writing-fuzz-test-extra/accounts-snapshots.md).
