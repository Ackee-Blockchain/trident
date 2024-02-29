In the sequence diagram below you can see a simplified fuzz test lifecycle.

Some diagram states are labeled with emojis:

- 🟥 Mandatory methods that must be implemented by the user.
- 👩‍💻 Optional methods that can be implemented by the user.

## Lifecycle

1.  The fuzzer is running until:
    1. The maximal number of iterations is reached (if specified).
    2. A crash was detected and the `exit_upon_crash` parameter was set.
    3. User interrupted the test manually (for example by hitting `CTRL+C`).
2. In each iteration, the fuzzer generates a sequence of random instructions to execute.
    1. User can optionally customize how the instructions are generated and can specify the instructions that should be executed at the beginning (`pre_ixs`), in the middle (`ixs`) and at the end (`post_ixs`) of each iteration. This can be useful for example if your program needs an initialization or you want to fuzz some specific program state.
3. For each instruction:
    1. User defined mandatory method `get_accounts()` is called to collect necessary instruction accounts.
    2. User defined mandatory method `get_data()` is called to collect instruction data.
    3. A snapshot of all instruction accounts before the instruction execution is saved.
    4. The instruction is executed.
    5. A snapshot of all instruction accounts after the instruction execution is saved.
    6. User defined optional method `check()` is called to check accounts data and evaluate invariants.

<!-- https://mermaid.js.org/intro/ -->
```mermaid
graph TB
    start(("1. Fuzzer Iterations = 0"))
    check{2. Fuzzer Iterations < Max Iterations}
    done(("Done"))
    gen[/"3. Generate instructions"/]
    ins("`
    👩‍💻
    3.1 Pre-Instructions
    3.2 Instructions
    3.3 Post-Instructions
    `")
    for{4. for Ix in Instructions}
    get_("`
    🟥
    4.2 Get Ix Accounts
    4.3 Get Ix Data
    `")
    create_pre_snap("4.4 Create Pre-Ix Accounts Snapshots")
    execute("4.5 Execute Ix")
    create_post_snap("4.6 Create Post-Ix Accounts Snapshots")
    check_inv("`
    👩‍💻
    4.7 Check Invariants
    `")
    done_loop("Done")
    inc(6. Fuzzer Iterations ++)

    start --> check
    check -- YES --> gen
    gen --> ins
    ins --> for
    inc --> check
    check -- NO --> done
    for --> get_ --> create_pre_snap --> execute --> create_post_snap --> check_inv --> for
    for --> done_loop  --> inc
```
