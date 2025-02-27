# Honggfuzz Configuration

## `run_time`

- Number of seconds this fuzzing session will last.

`(default: 0 [no limit])`

```bash
[honggfuzz]
run_time = 0
```

---

## `iterations`

- Number of fuzzing iterations.

`(default: 0 [no limit])`

```bash
[honggfuzz]
iterations = 0
```

---

## `threads`

- Number of concurrent fuzzing threads.

`(default: 0 [number of CPUs / 2])`

```bash

```bash
[honggfuzz]
threads = 0
```

---

## `keep_output`

- Don't close children's stdin, stdout, stderr; can be noisy.

`(default: false)`

```bash
[honggfuzz]
keep_output = false
```

---

## `verbose`

- Disable ANSI console; use simple log output.

`(default: false)`

```bash
[honggfuzz]
verbose = false
```

---

## `exit_upon_crash`

- Exit upon seeing the first crash.

`(default: false)`

```bash
[honggfuzz]
exit_upon_crash = false
```

---

## `mutations_per_run`

- Maximal number of mutations per one run.

`(default: 6)`

```bash
[honggfuzz]
mutations_per_run = 6
```

---

## `cargo_target_dir`

- Target compilation directory, to not clash with cargo build's default target directory.

`(default: "" ["trident-tests/fuzzing/honggfuzz/hfuzz_target"])`

```bash
[honggfuzz]
cargo_target_dir = ""
```

---

## `hfuzz_workspace`

- Honggfuzz working directory.

`(default: "" ["trident-tests/fuzzing/honggfuzz/hfuzz_workspace"])`

```bash
[honggfuzz]
hfuzz_workspace = ""
```

---

## `crashdir`

- Directory where crashes are saved to.

`(default: "" [workspace directory])`

```bash
[honggfuzz]
crashdir = ""
```

---

## `extension`

- Input file extension.

`(default: "" ['fuzz'])`

```bash
[honggfuzz]
extension = "fuzz"
```

---

## `timeout`

- Timeout in seconds. This will terminate the fuzzing thread if it's running for more than this time.

`(default: 10)`

```bash
[honggfuzz]
timeout = 10
```

---

## `max_file_size`

- Maximal size of files processed by the fuzzer in bytes.

`(default: 1048576 = 1MB)`

```bash
[honggfuzz]
max_file_size = 1048576
```

---

## `save_all`

- Save all test-cases (not only the unique ones) by appending the current time-stamp to the filenames.

`(default: false)`

```bash
[honggfuzz]
save_all = false
```
