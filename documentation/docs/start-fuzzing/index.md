# Start Fuzzing

To start writting fuzz tests, you need to initialize Trident in your Anchor-based workspace.

```bash
trident init
```

If you have already initialized Trident, you can add a new fuzz test using:

```bash
trident fuzz add
```

Trident creates a new fuzz test template in the `trident-tests` directory, with the following structure:

```bash
project-root
├── trident-tests
│   ├── fuzz_0 # particular fuzz test
│   │   ├── instructions # instructions folder
│   │   ├── transactions # transactions folder
│   │   ├── test_fuzz.rs # the binary target of your fuzz test
│   │   ├── types.rs # the types of your fuzz test
│   │   └── fuzz_instructions.rs # the definition of your fuzz test
│   ├── fuzz_1
│   ├── fuzz_X # possible multiple fuzz tests
│   ├── fuzzing # compilations and crashes folder
│   └── Cargo.toml
├── Trident.toml
└── ...
```


`Instructions` folder contains the `instructions` that can be used in the fuzz test.

`Transactions` folder contains the `transactions` that will be used during the fuzzing, each transaction can contain `one` or `multiple instructions`, based on the developers choice. By default there is only one instruction in each transaction.
