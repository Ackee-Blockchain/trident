# First Steps

At the current development stage, there are some manual steps required to make your fuzz test compile:

1. Add dependencies specific to your program to `trident-tests/fuzz_tests/Cargo.toml` (such as anchor-spl etc.).
2. Add necessary `use` statements into `trident-tests/fuzz_tests/<FUZZ_TEST_NAME>/accounts_snapshots.rs` to import missing types.
