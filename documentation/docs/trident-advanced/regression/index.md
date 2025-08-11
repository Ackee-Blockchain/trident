# Regression testing

Regression testing is the process of re-running tests to ensure that previously working functionality still works after code changes.


Trident allows for regression testing by specifying which accounts should be included in the regression test, generating a JSON file format with all monitored state and providing a subcommand to compare two regression test results.

## Regression test setup

1. First, decide which account(s) should be included in the regression test.

2. Add the accounts for tracking, this can be done as shown in the following code snippet. Check the `add_to_regression` method.

    ```rust
    #[flow_executor]
    impl FuzzTest {
        fn new() -> Self {
            Self {
                trident: Trident::default(),
                fuzz_accounts: FuzzAccounts::default(),
            }
        }

        #[init]
        fn start(&mut self) {
            let mut ix = InitializeFnTransaction::build(&mut self.trident, &mut self.fuzz_accounts);

            self.trident.execute_transaction(&mut ix, Some("Init"));

            // Add the account to the regression test
            self.trident.add_to_regression(
                &ix.instruction.accounts.hello_world_account.pubkey(),
                "hello_world_account",
            );
        }

        #[flow(weight = 5)]
        fn flow1(&mut self) {
            // This flow will be executed 60% of the time
            let mut ix = InitializeFnTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
            self.trident.execute_transaction(&mut ix, Some("Flow1"));
        }

        #[flow(weight = 5)]
        fn flow2(&mut self) {
            // This flow will be executed 40% of the time
            let mut ix = InitializeFnTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
            self.trident.execute_transaction(&mut ix, Some("Flow2"));
        }
        #[flow(weight = 90)]
        fn flow3(&mut self) {
            // This flow will be executed 40% of the time
            let mut ix = InitializeFnTransaction::build(&mut self.trident, &mut self.fuzz_accounts);
            self.trident.execute_transaction(&mut ix, Some("Flow3"));
        }

        #[end]
        fn cleanup(&mut self) -> Result<(), FuzzingError> {
            // This method will be called after all flows have been executed
            Ok(())
        }
    }

    fn main() {
        FuzzTest::fuzz(1000, 100);
    }
    ```

3. Enable the regression test in the [Trident manifest](../../trident-manifest/index.md#fuzzing-metrics) by setting:

    ```toml
    [fuzz.regression]
    enabled = true
    ```

4. Run the fuzz test to generate the regression test data.

5. Run the same fuzz test with the same MASTER SEED on a different version of the program.

6. Run the following command to compare the generated JSON files.

```bash
trident compare <path_to_json_file_1> <path_to_json_file_2>
```


