# Run

To execute the desired fuzz test, run:

```bash
# Replace <TARGET_NAME> with the name of particular
# fuzz test (for example: "fuzz_0")
trident fuzz run <TARGET_NAME>
```

## Trident output

!!! important

    The output provided by Honggfuzz is as follows

    1. Number of Fuzzing Iterations.
    2. Feedback Driven Mode = Honggfuzz generates data based on the feedback (i.e. feedback based on Coverage progress).
    3. Average Iterations per second
    4. Number of crashes it found (**panics** or failed **invariant checks**)


```bash
------------------------[  0 days 00 hrs 00 mins 01 secs ]----------------------
  Iterations : 688 (out of: 1000 [68%]) # -- 1. --
  Mode [3/3] : Feedback Driven Mode # -- 2. --
      Target : trident-tests/fuzz_tests/fuzzing.....wn-linux-gnu/release/fuzz_0
     Threads : 16, CPUs: 32, CPU%: 1262% [39%/CPU]
       Speed : 680/sec [avg: 688] # -- 3. --
     Crashes : 1 [unique: 1, blocklist: 0, verified: 0] # -- 4. --
    Timeouts : 0 [10 sec]
 Corpus Size : 98, max: 1048576 bytes, init: 0 files
  Cov Update : 0 days 00 hrs 00 mins 00 secs ago
    Coverage : edge: 10345/882951 [1%] pc: 163 cmp: 622547
---------------------------------- [ LOGS ] ------------------/ honggfuzz 2.6 /-
```


## Customize Fuzzing
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

??? note

    Or you can pass any parameter via [environment variables](https://github.com/rust-fuzz/honggfuzz-rs#environment-variables).

    A list of hongfuzz parameters can be found in honggfuzz [usage documentation](https://github.com/google/honggfuzz/blob/master/docs/USAGE.md#cmdline---help). The parameters passed via **environment variables** have **higher priority**. For example:

    ```bash
    # Time-out: 10 secs
    # Number of concurrent fuzzing threads: 1
    # Number of fuzzing iterations: 10000
    # Display Solana logs in the terminal
    HFUZZ_RUN_ARGS="-t 10 -n 1 -N 10000 -Q" trident fuzz run <TARGET_NAME>
    ```
