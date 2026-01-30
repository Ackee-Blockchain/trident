Command line used to find this crash:

/Users/andrej/.local/share/afl.rs/rustc-1.91.0-f8297e3/afl.rs-0.17.0/afl/bin/afl-fuzz -c0 -i seeds -o out -E 250000 target/debug/fuzz_0

If you can't reproduce a bug outside of afl-fuzz, be sure to set the same
memory limit. The limit used for this fuzzing session was 0 B.

Need a tool to minimize test cases before investigating the crashes or sending
them to a vendor? Check out the afl-tmin that comes with the fuzzer!

Found any cool bugs in open-source tools using afl-fuzz? If yes, please post
to https://github.com/AFLplusplus/AFLplusplus/issues/286 once the issues
 are fixed :)

