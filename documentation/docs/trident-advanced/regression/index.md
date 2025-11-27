# Regression Testing

Regression testing is the process of re-running tests to ensure that previously working functionality still works after code changes.


Trident enables regression testing by allowing you to specify which accounts should be included in the regression test, generating a JSON file with all monitored state, and providing a subcommand to compare two regression test results.

## Regression Test Setup

1. First, decide which account(s) should be included in the regression test.

2. Add accounts for tracking using the `add_to_regression` method:

    ```rust
    #[flow_executor]
    impl FuzzTest {
        #[init]
        fn start(&mut self) {
            let account = self.fuzz_accounts.target_account.insert(&mut self.trident, None);
            
            let instruction = create_initialize_instruction(account);
            let result = self.trident.process_transaction(&[instruction], Some("initialize"));
            
            if result.is_success() {
                // Add the account to regression tracking
                self.trident.add_to_regression(&account, "target_account");
            }
        }
    }
    ```

3. Enable the regression test in the [Trident manifest](../../trident-manifest/index.md#fuzzing-metrics) by setting:

    ```toml
    [fuzz.regression]
    enabled = true
    ```

4. Run the fuzz test to generate the regression test data.

5. Run the same fuzz test with the same MASTER SEED on a different version of the program.

6. Run the following command to compare the generated JSON files:

```bash
trident compare <path_to_json_file_1> <path_to_json_file_2>
```


