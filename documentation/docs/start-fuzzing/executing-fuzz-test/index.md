# Executing the Fuzz Test

To execute the fuzz test, navigate to the `trident-tests` directory and run the following command to execute the fuzz test using Honggfuzz:

```bash
trident fuzz run-hfuzz <FUZZ_TARGET>
# for example:
# trident fuzz run-hfuzz fuzz_0
```



To execute the fuzz test using AFL, run the following command:

```bash
trident fuzz run-afl <FUZZ_TARGET>
# for example:
# trident fuzz run-afl fuzz_0
```

!!! warning "Directory Note"
    Execute fuzz tests from the `trident-tests` directory.

!!! warning "Solana Logs"
    In case you want to see the logs of the fuzzed transactions, prefix the command with `TRIDENT_LOG=1`.
    ```bash
    TRIDENT_LOG=1 trident fuzz run-afl <fuzz_target>
    TRIDENT_LOG=1 trident fuzz run-hfuzz <fuzz_target>
    ```

For the output reference and additional options such as debugging the found issues, check out [Commands](../../basics/commands.md).
