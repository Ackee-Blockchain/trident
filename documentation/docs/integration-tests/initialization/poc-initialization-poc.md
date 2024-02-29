# Integration test-only

If you are interested only in generating templates for Integration Tests run
```bash
trdelnik init poc
```

The command will generate the following folder structure:
```bash
project-root
├── .program_client
├── trdelnik-tests
│   ├── poc_tests # integration tests folder
│   │   ├── tests
│   │   │   └── test.rs
│   │   └── Cargo.toml
├── Trdelnik.toml
└── ...
```
