
# All-Suite

To initialize {{ config.site_name }} and generate all-suite test templates, navigate to your project's root directory and run

```bash
trident init
```

The command will generate the following folder structure:
```bash
project-root
├── .program_client
├── trident-tests
│   ├── fuzz_tests # fuzz tests folder
│   ├── poc_tests # integration tests folder
│   │   ├── tests
│   │   │   └── test.rs # Integration Tests implementation
│   │   └── Cargo.toml
├── Trident.toml
└── ...
```
