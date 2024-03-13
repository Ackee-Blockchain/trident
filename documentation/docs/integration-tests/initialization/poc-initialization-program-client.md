# Program Client

By default, Integration Tests initialization generates also a `.program_client` crate with all necessary implementation for Instructions.

If you are interested in updating the `.program_client` implementation due to an update inside your program, run
```bash
trident build
```

This command will also initialize `.program_client` if the crate does not exist yet.
