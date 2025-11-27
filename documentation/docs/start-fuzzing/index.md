# Start Fuzzing

Trident helps you create comprehensive fuzz tests for your Solana programs. This section guides you through the complete process from initialization to execution.

## Quick Start

### 1. Initialize Trident

To start writing fuzz tests, initialize Trident in your Anchor-based workspace:

```bash
trident init
```

### 2. Add a Fuzz Test

If you have already initialized Trident, add a new fuzz test:

```bash
trident fuzz add
```

### 3. Project Structure

Trident creates a new fuzz test template in the `trident-tests` directory:

```bash
project-root
├── trident-tests
│   ├── .fuzz-artifacts         # Fuzzing artifacts (dashboard, metrics, etc.)
│   ├── fuzz_0                  # Your first fuzz test
│   │   ├── test_fuzz.rs        # Main fuzz test logic
│   │   ├── fuzz_accounts.rs    # Account addresses storage
│   │   └── types.rs            # IDL-like generated types
│   ├── fuzz_1                  # Additional fuzz tests
│   ├── fuzz_X                  # Multiple fuzz tests supported
│   ├── fuzzing                 # Compilation and crash artifacts
│   ├── Cargo.toml              # Rust dependencies
│   └── Trident.toml            # Trident configuration
└── ...
```

## Next Steps

Now that you have the basic structure set up, learn how to:

1. **[Write Fuzz Tests](./writting-fuzz-test/index.md)** - Learn how to construct instructions and define fuzzing flows
2. **[Execute Fuzz Tests](./executing-fuzz-test/index.md)** - Run your fuzz tests and analyze results

!!! tip "Start Simple"

    Begin with a single fuzz test to understand the workflow, then expand to multiple test scenarios as needed.
