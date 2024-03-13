# Integration test-only

If you are interested only in generating templates for Integration Tests run
```bash
trident init poc
```

The command will generate the following folder structure:
```bash
project-root
├── .program_client
├── trident-tests
│   ├── poc_tests # integration tests folder
│   │   ├── tests
│   │   │   └── test.rs
│   │   └── Cargo.toml
├── Trident.toml
└── ...
```
