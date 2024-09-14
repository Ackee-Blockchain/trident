# Trident Manifest

You can pass supported parameters via the **{{ config.site_name }}.toml** configuration file:

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

### [honggfuzz]

#### timeout
- Timeout in seconds (default: 10)

#### iterations
- Number of fuzzing iterations (default: 0 [no limit])

#### threads
- Number of concurrent fuzzing threads (default: 0 [number of CPUs / 2])

#### keep_output
- Don't close children's stdin, stdout, stderr; can be noisy (default: false)

#### verbose
- Disable ANSI console; use simple log output (default: false)

#### exit_upon_crash
- Exit upon seeing the first crash (default: false)

#### mutations_per_run
- Maximal number of mutations per one run (default: 6)

#### cargo_target_dir
- Target compilation directory, (default: "" ["trident-tests/fuzz_tests/fuzzing/hfuzz_target"]).
- To not clash with cargo build's default target directory.

#### hfuzz_workspace
- Honggfuzz working directory, (default: "" ["trident-tests/fuzz_tests/fuzzing/hfuzz_workspace"]).

#### crashdir
- Directory where crashes are saved to (default: "" [workspace directory])

#### extension
- Input file extension (e.g. 'swf'), (default: "" ['fuzz'])

#### run_time
- Number of seconds this fuzzing session will last (default: 0 [no limit])

#### max_file_size
- Maximal size of files processed by the fuzzer in bytes (default: 1048576 = 1MB)

#### save_all
- Save all test-cases (not only the unique ones) by appending the current time-stamp to the filenames (default: false)


### [fuzz]

#### allow_duplicate_txs
- Allow processing of duplicate transactions. Setting to true might speed up fuzzing but can cause false positive crashes (default: false)


#### fuzzing_with_stats
- Trident will show statistics after the fuzzing session. This option forces use of honggfuzz parameter `keep_output` as true in order to be able to catch fuzzer stdout. (default: false)


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

!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.
