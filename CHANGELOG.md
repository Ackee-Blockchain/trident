# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) (SemVer).

**Note**: Version 0 of Semantic Versioning is handled differently from version 1 and above. The minor version will be
incremented upon a breaking change and the patch version will be incremented for features.

## [Unreleased]

## [0.5.0] - 2023-08-28
### Added
- cli: Added trdelnik subcommand `fuzz` to run and debug fuzz tests using honggfuzz-rs.
- cli: Added trdelnik `--skip-fuzzer` option for `init` subcommand to skip generation of fuzz test templates.
- client: Added new Cargo feature `fuzzing` that enables optional dependencies related to fuzz testing.

## [0.4.1] - 2023-08-21
### Changed
- Upgrade Solana (`=1.16.6`) and Anchor framework (`=0.28.0`) versions.
### Fixed
- Implemented Anchor Client logic was not able to work with newer version of Anchor. Fixed with `async_rpc` and `async` feature.
- Trdelnik init IDL Parse Error on newer version of Rust, fixed with updated `accounts` token.


## [0.3.0] - 2022-09-23
### Changed
- Upgrade Solana (`~1.10`) and Anchor framework (`~0.25`) versions

### Added
- Custom Solana RPC error reporter. If the Solana RPC error is thrown, the error code, message and data (logs) are reported to the output.
- Custom imports in the `.program_client`. User is able to import custom types and structures into program client. The import part of the code would not be re-generated.

## [0.2.0] - 2022-07-27
### Added
- Trdelnik is now configurable. This requires `Trdelnik.toml` file to exist in the project's root directory - without this file the execution will fail. To solve this re-run `trdelnik init` or just create an empty `Trdelnik.toml` file in the project's root directory.
