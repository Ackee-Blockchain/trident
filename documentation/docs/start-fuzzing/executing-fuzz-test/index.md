# Executing Fuzz Tests

Once you've written your fuzz tests, it's time to run them and analyze the results. This guide covers the execution process and available options.

## Basic Execution

Navigate to the `trident-tests` directory and run your fuzz test:

```bash
cd trident-tests
trident fuzz run <FUZZ_TARGET> [SEED]
```

### Examples

```bash
# Run fuzz_0 with a random seed
trident fuzz run fuzz_0

# Run fuzz_0 with a specific seed for reproducibility
trident fuzz run fuzz_0 12345
```

!!! warning "Directory Requirement"
    
    Always execute fuzz tests from the `trident-tests` directory.

## Execution Options

### Enable Detailed Logging

To see detailed logs of the fuzzed transactions:

```bash
TRIDENT_LOG=1 trident fuzz run <fuzz_target> [seed]
```

### Seed Management

- **Random seed**: Omit the seed parameter for random testing
- **Specific seed**: Provide a seed number to reproduce specific test runs
- **Debugging**: Use the same seed to reproduce and debug issues

## Understanding Results

When running fuzz tests, Trident will:

1. **Execute iterations**: Run the specified number of test iterations
2. **Identify issues**: Report any undesired reverts or assertion failures
3. **Generate reports**: (optionaly) Create detailed reports in `.fuzz-artifacts`
4. **Collect code coverage**: (optionaly) Gather coverage data

## Additional Resources

- **[Commands Reference](../../basics/commands.md)** - Complete command options and debugging features
- **[Trident Manifest](../../trident-manifest/index.md)** - Configuration options through `Trident.toml`
- **[Advanced Customization](../../trident-advanced/)** - Advanced features and customization options

!!! tip "Debugging Tips"

    - Use specific seeds to reproduce issues consistently
    - Enable logging to understand transaction flow
    - Check the `.fuzz-artifacts` directory for detailed reports
    - Start with fewer iterations during development
