# Trident Manifest

You can pass supported parameters via the **{{ config.site_name }}.toml** configuration file:

### [honggfuzz]

#### timeout
- Timeout in seconds (default: 10)

```toml
[honggfuzz]
# Timeout in seconds (default: 10)
timeout = 10
```

#### iterations
- Number of fuzzing iterations (default: 0 [no limit])

```toml
[honggfuzz]
# Number of fuzzing iterations (default: 0 [no limit])
iterations = 0
```

#### threads
- Number of concurrent fuzzing threads (default: 0 [number of CPUs / 2])

```toml
[honggfuzz]
# Number of concurrent fuzzing threads (default: 0 [number of CPUs / 2])
threads = 0
```

#### keep_output
- Don't close children's stdin, stdout, stderr; can be noisy (default: false)

```toml
[honggfuzz]
# Don't close children's stdin, stdout, stderr; can be noisy (default: false)
keep_output = false
```

#### verbose
- Disable ANSI console; use simple log output (default: false)

```toml
[honggfuzz]
# Disable ANSI console; use simple log output (default: false)
verbose = false
```

#### exit_upon_crash
- Exit upon seeing the first crash (default: false)

```toml
[honggfuzz]
# Exit upon seeing the first crash (default: false)
exit_upon_crash = false
```

#### mutations_per_run
- Maximal number of mutations per one run (default: 6)

```toml
[honggfuzz]
# Maximal number of mutations per one run (default: 6)
mutations_per_run = 6
```

#### cargo_target_dir
- Target compilation directory, (default: "" ["trident-tests/fuzz_tests/fuzzing/honggfuzz/hfuzz_target"]).
- To not clash with cargo build's default target directory.

```toml
[honggfuzz]
# Target compilation directory,
# (default: "" ["trident-tests/fuzz_tests/fuzzing/honggfuzz/hfuzz_target"]).
# To not clash with cargo build's default target directory.
cargo_target_dir = ""
```

#### hfuzz_workspace
- Honggfuzz working directory, (default: "" ["trident-tests/fuzz_tests/fuzzing/honggfuzz/hfuzz_workspace"]).

```toml
[honggfuzz]
# Honggfuzz working directory,
# (default: "" ["trident-tests/fuzz_tests/fuzzing/honggfuzz/hfuzz_workspace"]).
hfuzz_workspace = ""
```

#### crashdir
- Directory where crashes are saved to (default: "" [workspace directory])

```toml
[honggfuzz]
# Directory where crashes are saved to (default: "" [workspace directory])
crashdir = ""
```

#### extension
- Input file extension (e.g. 'swf'), (default: "" ['fuzz'])

```toml
[honggfuzz]
# Input file extension (e.g. 'swf'), (default: "" ['fuzz'])
extension = ""
```

#### run_time
- Number of seconds this fuzzing session will last (default: 0 [no limit])

```toml
[honggfuzz]
# Number of seconds this fuzzing session will last (default: 0 [no limit])
run_time = 0
```

#### max_file_size
- Maximal size of files processed by the fuzzer in bytes (default: 1048576 = 1MB)

```toml
[honggfuzz]
# Maximal size of files processed by the fuzzer in bytes
# (default: 1048576 = 1MB)
max_file_size = 1048576
```

#### save_all
- Save all test-cases (not only the unique ones) by appending the current time-stamp to the filenames (default: false)

```toml
[honggfuzz]
# Save all test-cases
# (not only the unique ones) by appending the current
# time-stamp to the filename (default: false)
save_all = false
```

### [afl]

#### cargo_target_dir
- Target compilation directory, (default: "" ["trident-tests/fuzz_tests/fuzzing/afl/afl_target"]).
- To not clash with cargo build's default target directory.

```toml
[afl]
# Target compilation directory,
# (default: "" ["trident-tests/fuzz_tests/fuzzing/afl/afl_target"]).
# To not clash with cargo build's default target directory.
cargo_target_dir = ""
```

#### afl_workspace_in
- AFL working input directory,
- (default: "" ["trident-tests/fuzz_tests/fuzzing/afl/afl_workspace/in"]).

```toml
[afl]
# AFL working input directory,
# (default: "" ["trident-tests/fuzz_tests/fuzzing/afl/afl_workspace/in"]).
afl_workspace_in = ""
```

#### afl_workspace_out
- AFL working output directory,
- (default: "" ["trident-tests/fuzz_tests/fuzzing/afl/afl_workspace/out"]).

```toml
[afl]
# AFL working output directory,
# (default: "" ["trident-tests/fuzz_tests/fuzzing/afl/afl_workspace/out"]).
afl_workspace_out = ""
```

#### seeds
- Predefined inputs to the AFL
- You can specify multiple input seeds.

!!! importnat

    `bytes_count` has precedence before `seed`, in that case if both are specified. Seed is generated as random array of `bytes_count` bytes.

```toml
[[afl.seeds]]
# Filename under which the test input is generated.
# The location of file is afl_workspace_in directory.
# (default: "" ["trident-seed"]).
file_name = ""
# String used as seed.
# (default: "" ["0"]).
seed = ""
# If the file already exists at specific location,
# select if override.
# (default: false).
override_file = true
# Number of randomly generated bytes.
# (default: 0).
bytes_count = 20
```

### [fuzz]

#### allow_duplicate_txs
- Allow processing of duplicate transactions. Setting to true might speed up fuzzing but can cause false positive crashes (default: false)

```toml
[fuzz]
# Allow processing of duplicate transactions.
# Setting to true might speed up fuzzing but can cause
# false positive crashes (default: false)
allow_duplicate_txs = false
```

#### fuzzing_with_stats
- Trident will show statistics after the fuzzing session. This option forces use of honggfuzz parameter `keep_output` as true in order to be able to catch fuzzer stdout. (default: false)

```toml
[fuzz]
# Trident will show statistics after the fuzzing session.
# This option forces use of honggfuzz parameter
# `keep_output` as true in order to be able to catch fuzzer stdout.
# (default: false)
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

!!! tip

    Consider checking the [Examples](../examples/examples.md) section for more tips.