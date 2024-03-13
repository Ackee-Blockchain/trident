## Run
Once you have finished the implementation of the Fuzz Test, you can run the Test as follows:

```bash
# Replace <TARGET_NAME> with the name of particular
# fuzz test (for example: "fuzz_0")
trident fuzz run <TARGET_NAME>
```

Under the hood {{ config.site_name }} uses [honggfuzz-rs](https://github.com/rust-fuzz/honggfuzz-rs).

You can pass [supported parameters](https://github.com/Ackee-Blockchain/trident/blob/develop/examples/fuzz_example0/Trident.toml) via the **{{ config.site_name }}.toml** configuration file. For example:

```toml
# Content of {{ config.site_name }}.toml
[fuzz]
# Timeout in seconds (default: 10)
timeout = 10
# Number of fuzzing iterations (default: 0 [no limit])
iterations = 10000
# Number of concurrent fuzzing threads (default: 0 [number of CPUs / 2])
threads = 0
# Don't close children's stdin, stdout,
# stderr; can be noisy (default: false)
keep_output = false
# Disable ANSI console; use simple log output (default: false)
verbose = false
# Exit upon seeing the first crash (default: false)
exit_upon_crash = true
# Maximal number of mutations per one run (default: 6)
mutations_per_run = 6
# Target compilation directory,
# (default: "" ["trident-tests/fuzz_tests/fuzzing/hfuzz_target"]).
# To not clash with cargo build's default target directory.
cargo_target_dir = ""
# Honggfuzz working directory,
# (default: "" ["trident-tests/fuzz_tests/fuzzing/hfuzz_workspace"]).
hfuzz_workspace = ""
# Directory where crashes are saved to (default: "" [workspace directory])
crashdir = ""
# Input file extension (e.g. 'swf'), (default: "" ['fuzz'])
extension = ""
# Number of seconds this fuzzing session will last (default: 0 [no limit])
run_time = 0
# Maximal size of files processed by the fuzzer
# in bytes (default: 1048576 = 1MB)
max_file_size = 1048576
# Save all test-cases (not only the unique ones) by
# appending the current time-stamp to the filenames (default: false)
save_all = false
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

## Debug
To debug your program with values from a crash file:

```bash
# fuzzer will run the <TARGET_NAME> with the specified <CRASH_FILE_PATH>
trident fuzz run-debug <TARGET_NAME> <CRASH_FILE_PATH>
# for example:
trident fuzz run-debug fuzz_0 trident-tests/fuzz_tests/fuzzing/fuzz_0/cr1.fuzz
```
