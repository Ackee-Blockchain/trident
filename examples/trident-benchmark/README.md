<p align="center">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://abchprod.wpengine.com/wp-content/uploads/2024/05/Trident-Github.png?raw=true">
      <img alt="Trident Github" src="https://abchprod.wpengine.com/wp-content/uploads/2024/05/Trident-Github.png?raw=true" width="auto">
    </picture>
  </a>
</p>

# Trident Benchmark
[Mazes](https://github.com/Consensys/daedaluzz/blob/master/generated-mazes/) from solidity fuzz test benchmark [daedaluzz](https://github.com/Consensys/daedaluzz) rewritten into rust.

## Fuzz Test
Each maze includes written fuzz test.


### Run/Debug Fuzz Test
To run the fuzz test use the following command in the maze directory:
```bash
trident fuzz run-hfuzz fuzz_0
```

To run debug use:
```bash
trident fuzz debug-hfuzz fuzz_0 <CRASH_FILE_PATH>
```
