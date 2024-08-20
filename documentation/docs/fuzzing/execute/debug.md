# Debug

To debug your program with values from a crash file:

```bash
# fuzzer will run the <TARGET_NAME> with the specified <CRASH_FILE_PATH>
trident fuzz run-debug <TARGET_NAME> <CRASH_FILE_PATH>
# for example:
trident fuzz run-debug fuzz_0 trident-tests/fuzz_tests/fuzzing/fuzz_0/cr1.fuzz
```


## Debug Output

!!! important

    The debug output is at current development stage really verbose and contains lldb parts. We are working on improving this experience. In the picture below you can see an example of provided debug output.

    1. Series of Transaction Logs
    2. Structures of data send within the Instructions
    3. **Panic** or **Crash**, based on if the Fuzzing paniced within the Solana Program or Invariant Check failed.



![alt text](../../images/run-debug.png)
