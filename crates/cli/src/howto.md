

# HOW TO START FUZZING

## 1. Initialize **Trident**

Navigate to the project directory and run:

```bash
trident init
```

## 2. Write Fuzz Test

In order to start fuzzing, you need to guide the fuzzer to use correct and meaningful instruction inputs. Trident also provides various features to tailor your fuzz tests to your specific needs, behavior, and use cases. To learn more, check out the documentation.

## 3. Run Fuzz Test

You can run the fuzz test using Trident's built-in fuzzing engine:

```bash
trident fuzz run <FUZZ_TARGET>
```

## 4. Debugging

To debug your program using a specific seed, run:

```bash
trident fuzz debug <FUZZ_TARGET> <SEED>
```

## Resources

- Use the `--help` flag for more information
- Documentation: https://ackee.xyz/trident/docs/latest/
