# Run and Debug

## Run
Once you have finished the implementation of the Fuzz Test, you can run the Test as follows:

```bash
# Replace <TARGET_NAME> with the name of particular
# fuzz test (for example: "fuzz_0")
trident fuzz run <TARGET_NAME>
```

Under the hood {{ config.site_name }} uses [honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs).

You can pass [supported parameters](https://github.com/Ackee-Blockchain/trident/blob/develop/examples/fuzz-tests/hello_world/Trident.toml) via the **{{ config.site_name }}.toml** configuration file:

```toml
# Content of {{ config.site_name }}.toml
[honggfuzz]
# Timeout in seconds (default: 10)
timeout = 10
# Number of fuzzing iterations (default: 0 [no limit])
iterations = 0
# Number of concurrent fuzzing threads (default: 0 [number of CPUs / 2])
threads = 0
# Don't close children's stdin, stdout, stderr; can be noisy (default: false)
keep_output = false
# Disable ANSI console; use simple log output (default: false)
verbose = false
# Exit upon seeing the first crash (default: false)
exit_upon_crash = false
# Maximal number of mutations per one run (default: 6)
mutations_per_run = 6
# Target compilation directory, (default: "" ["trident-tests/fuzz_tests/fuzzing/hfuzz_target"]).
# To not clash with cargo build's default target directory.
cargo_target_dir = ""
# Honggfuzz working directory, (default: "" ["trident-tests/fuzz_tests/fuzzing/hfuzz_workspace"]).
hfuzz_workspace = ""
# Directory where crashes are saved to (default: "" [workspace directory])
crashdir = ""
# Input file extension (e.g. 'swf'), (default: "" ['fuzz'])
extension = ""
# Number of seconds this fuzzing session will last (default: 0 [no limit])
run_time = 0
# Maximal size of files processed by the fuzzer in bytes (default: 1048576 = 1MB)
max_file_size = 1048576
# Save all test-cases (not only the unique ones) by appending the current time-stamp to the filenames (default: false)
save_all = false

[fuzz]
# Allow processing of duplicate transactions. Setting to true might speed up fuzzing but can cause false positive crashes (default: false)
allow_duplicate_txs = false
# Trident will show statistics after the fuzzing session. This option forces use of honggfuzz parameter
# `keep_output` as true in order to be able to catch fuzzer stdout. (default: false)
fuzzing_with_stats = true
```

Or you can pass any parameter via [environment variables](https://github.com/rust-fuzz/honggfuzz-rs#environment-variables).

A list of hongfuzz parameters can be found in honggfuzz [usage documentation](https://github.com/google/honggfuzz/blob/master/docs/USAGE.md#cmdline---help). The parameters passed via **environment variables** have **higher priority**. For example:

```bash
# Time-out: 10 secs
# Number of concurrent fuzzing threads: 1
# Number of fuzzing iterations: 10000
# Display Solana logs in the terminal
HFUZZ_RUN_ARGS="-t 10 -n 1 -N 10000 -Q" trident fuzz run <TARGET_NAME>
```

### Fuzzing statistics
Sometimes, it's useful to know how often a particular instruction has been invoked and how many times it has succeeded or failed. To display these statistics when fuzzing is finished or interrupted, set the `fuzzing_with_stats` option to `true` in the `[fuzz]` section of the Trident.toml configuration file. Please note that this option is disabled by default because it impacts performance.

The statistics show the total number of invocations for each instruction, which is the sum of successful and failed invocations. Successful invocations are those that return an `Ok()` result. Failed invocations are those that return an `Err()` result. Additionally, the statistics also show as `Check Failed` the number of successful invocations that did not pass the user-defined invariants check. Note that unhandled panics are currently logged only as crashes and are not displayed in the fuzzing statistics table.

Keep in mind that the number of fuzz iterations does not directly correspond to the total number of invocations. In one fuzz iteration, the fuzzer might be unable to deserialize fuzz data into instructions, causing the entire iteration to be skipped.

## Debug
To debug your program with values from a crash file:

```bash
# fuzzer will run the <TARGET_NAME> with the specified <CRASH_FILE_PATH>
trident fuzz run-debug <TARGET_NAME> <CRASH_FILE_PATH>
# for example:
trident fuzz run-debug fuzz_0 trident-tests/fuzz_tests/fuzzing/fuzz_0/cr1.fuzz
```
