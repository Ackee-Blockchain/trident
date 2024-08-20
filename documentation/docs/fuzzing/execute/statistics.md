# Fuzzing statistics

!!! important

    In order to show statistics of fuzzing session (statistics are displayed after the fuzzing session ended), set `fuzzing_with_stats` within the Trident.toml to `true`.

    ```toml
    [fuzz]
    # ...
    fuzzing_with_stats = true
    # ...
    ```


Currently exported statistics from the Fuzzing Session

#### Simple

- Number of invocations of each instruction during the fuzzing session.
- Number of successful invocations of each instruction during the fuzzing session.
- Number of failed invariants checks for each instruction during the fuzzing session.

??? note

    Keep in mind that the number of fuzz iterations does not directly correspond to the total number of invocations. In one fuzz iteration, the fuzzer might be unable to deserialize fuzz data into instructions, causing the entire iteration to be skipped.

    On the other hand this is expected behavior as the underlying data are randomly (with coverage feedback) generated, so the Honggfuzz will not necessarily find appropriate data each iteration.
