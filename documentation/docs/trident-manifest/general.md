# General Configuration

## `programs`

- Use dumped program from desired cluster, during fuzzing.

```bash
[[fuzz.programs]]
address = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
program = "metaplex-program/metaplex-token-metadata.so"
```

---

## `accounts`

- Use dumped accounts from desired cluster, during fuzzing.

```bash
[[fuzz.accounts]]
address = "6YG3J7PaxyMnnbU67ifyrgF3BzNzc7cD8hPkqK6ATweE"
filename = "tests/accounts/core_bridge_mainnet/guardian_set_5_mock.json"
```

---

## `fuzzing_with_stats`

- Trident will show statistics after the fuzzing session. This option forces use of honggfuzz parameter `keep_output` as true in order to be able to catch fuzzer stdout.

`(default: false)`

!!! warning "Statistics Support Limitation"
    Fuzzing with stats is currently not supported with `AFL`.

```bash
[fuzz]
fuzzing_with_stats = true
```

---

## `allow_duplicate_txs`

- Allow processing of duplicate transactions. Setting to true might speed up fuzzing but can cause false positive crashes.

`(default: false)`

!!! warning "Transaction Validity Check"
    With swtich from `ProgramTest` to `TridentSVM`, there is no check for the transactions validity, so this check currently does nothing.

```bash
[fuzz]
allow_duplicate_txs = false
```
