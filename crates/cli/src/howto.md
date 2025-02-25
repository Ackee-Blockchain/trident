

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

Make sure to check out the table with supported versions: https://ackee.xyz/trident/docs/latest/basics/installation/#supported-versions

## 2. Initialize **Trident**

Navigate to the project directory and run:

```bash
trident init
```

## 3. Write Fuzz Test

In order to start fuzzing, you need to guide the fuzzer to use correct and meaningful instruction inputs. Follow this short guide to learn how to write a fuzz test: https://ackee.xyz/trident/docs/latest/start-fuzzing/writting-fuzz-test/

## 4. Customize Fuzz Test (Optional)

Trident provides various features to tailor your fuzz tests to your specific needs, behavior, and use cases. To learn more, visit: https://ackee.xyz/trident/docs/latest/trident-advanced/

## 5. Run Fuzz Test

You can run the fuzz test using either AFL or Honggfuzz:

```bash
trident fuzz run-hfuzz <FUZZ_TARGET>
```

```bash
trident fuzz run-afl <FUZZ_TARGET>
```

## 6. Debugging

To debug your program, run:

```bash
trident fuzz debug-hfuzz <FUZZ_TARGET> <CRASH_FILE>
```

```bash
trident fuzz debug-afl <FUZZ_TARGET> <CRASH_FILE>
```

## Resources

- Use the `--help` flag for more information
- For detailed explanation refer to the documentation: https://ackee.xyz/trident/docs/latest/
