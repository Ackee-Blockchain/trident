

# HOW TO START FUZZING

## 1. Install Honggfuzz and AFL:

To install **Honggfuzz**, run:

```bash
cargo install honggfuzz
```

To install **AFL**, run:

```bash
cargo install cargo-afl
```

## 2. Initialize **Trident**

Navigate to the project directory and run:

```bash
trident init
```

## 3. Write Fuzz Test

In order to start fuzzing, you need to guide the fuzzer to use correct and meaningful instruction inputs. Trident also provides various features to tailor your fuzz tests to your specific needs, behavior, and use cases. To learn more, check out the documentation.

## 4. Run Fuzz Test

You can run the fuzz test using either AFL or Honggfuzz:

```bash
trident fuzz run-hfuzz <FUZZ_TARGET>
```

```bash
trident fuzz run-afl <FUZZ_TARGET>
```

## 5. Debugging

To debug your program, run:

```bash
trident fuzz debug-hfuzz <FUZZ_TARGET> <CRASH_FILE>
```

```bash
trident fuzz debug-afl <FUZZ_TARGET> <CRASH_FILE>
```

## Resources

- Use the `--help` flag for more information
- Documentation: https://ackee.xyz/trident/docs/latest/
