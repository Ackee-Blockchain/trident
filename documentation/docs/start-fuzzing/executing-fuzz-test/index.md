# Executing the Fuzz Test

To execute the fuzz test, navigate to the `trident-tests` directory and run the following command to execute the fuzz test:

```bash
trident fuzz run <FUZZ_TARGET> <SEED>
# for example:
# trident fuzz run fuzz_0
# seed is optional, if not provided, a random seed will be used
```

!!! warning "Directory Note"
    Execute fuzz tests from the `trident-tests` directory.

!!! warning "Solana Logs"
    In case you want to see the logs of the fuzzed transactions, prefix the command with `TRIDENT_LOG=1`.
    ```bash
    TRIDENT_LOG=1 trident fuzz run <fuzz_target> <seed>
    ```

For the output reference and additional options such as debugging the found issues, check out [Commands](../../basics/commands.md).

For the additional customization through the `trident.toml` file, check out the [Trident Manifest](../../trident-manifest/index.md) page.
