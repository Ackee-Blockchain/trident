# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) (SemVer).

**Note**: Version 0 of Semantic Versioning is handled differently from version 1 and above. The minor version will be
incremented upon a breaking change and the patch version will be incremented for features.

## [dev] - Unreleased


## [0.7.0] - 2024-08-14
### Added
- impr/ add feature flag to the AccountsSnapshots macro ([183](https://github.com/Ackee-Blockchain/trident/pull/183))
- feat/ add Support for CPI ([182](https://github.com/Ackee-Blockchain/trident/pull/182))
- feat/ add option to initialize Trident with Macro/File (for Snapshots) option based on preference ([179](https://github.com/Ackee-Blockchain/trident/pull/179))
- feat/create AccountsSnapshots derive macro for Snapshots creation ([#177](https://github.com/Ackee-Blockchain/trident/pull/177))
- feat/fuzzing moved to separate crate trident-fuzz ([#175](https://github.com/Ackee-Blockchain/trident/pull/175))
- feat/unify dependencies provided by the Trident ([#172](https://github.com/Ackee-Blockchain/trident/pull/172))
- feat/fuzzer-stats-logging, an optional statistics output for fuzzing session ([#144](https://github.com/Ackee-Blockchain/trident/pull/144))

### Fixed
- fix/in case of fuzzing failure throw error instead of only printing message ([#167](https://github.com/Ackee-Blockchain/trident/pull/167))
- fix/snapshot's zeroed account as optional ([#170](https://github.com/Ackee-Blockchain/trident/pull/170))

### Removed
- del/remove localnet subcommand ([178](https://github.com/Ackee-Blockchain/trident/pull/178))
- del/remove unnecessary fuzzing feature as trident is mainly fuzzer ([#176](https://github.com/Ackee-Blockchain/trident/pull/176))
- del/remove Trident explorer ([#171](https://github.com/Ackee-Blockchain/trident/pull/171))

## [0.6.0] - 2024-05-20
### Added
- feat/anchor 0.30.0 support ([#148](https://github.com/Ackee-Blockchain/trident/pull/148))
- fix/allow to process duplicate transactions ([#147](https://github.com/Ackee-Blockchain/trident/pull/147))
- feat/possibility to implement custom transaction error handling ([#145](https://github.com/Ackee-Blockchain/trident/pull/145))
- feat/support of automatically obtaining fully qualified paths of Data Accounts Custom types for `accounts_snapshots.rs` ([#141](https://github.com/Ackee-Blockchain/trident/pull/141))
- feat/allow direct accounts manipulation and storage ([#142](https://github.com/Ackee-Blockchain/trident/pull/142))
- feat/support of non-corresponding instruction and context names ([#130](https://github.com/Ackee-Blockchain/trident/pull/130))
- feat/refactored and improved program flow during init and build, added activity indicator ([#129](https://github.com/Ackee-Blockchain/trident/pull/129))
- feat/allow solana versions up to v1.17.* and pin Rust 1.77 nightly compiler ([#128](https://github.com/Ackee-Blockchain/trident/pull/128))
- feat/new init command option to initialize fuzz or poc tests only ([#124](https://github.com/Ackee-Blockchain/trident/pull/124))
- feat/debug-mode detailed output ([#125](https://github.com/Ackee-Blockchain/trident/pull/125))
- feat/anchor 0.29.0 support ([#121](https://github.com/Ackee-Blockchain/trident/pull/121))
- doc/add help comment + update documentation ([#120](https://github.com/Ackee-Blockchain/trident/pull/120))
- feat/fuzzer error handling ([#118](https://github.com/Ackee-Blockchain/trident/pull/118))
- feat/convert fuzz Pubkey to AccountId ([#116](https://github.com/Ackee-Blockchain/trident/pull/116))
- feat/additional anchor types ([#115](https://github.com/Ackee-Blockchain/trident/pull/115))
- feat/import ToAccountInfo trait in fuzzing prelude ([#113](https://github.com/Ackee-Blockchain/trident/pull/113))
- test/added code generation and macros tests ([#112](https://github.com/Ackee-Blockchain/trident/pull/112))
- feat/fuzzer framework core, macros, helpers, templates, and examples. ([#111](https://github.com/Ackee-Blockchain/trident/pull/111))
- feat/improved trident-tests folder structure for PoC and Fuzz Tests ([#109](https://github.com/Ackee-Blockchain/trident/pull/109))
- feat/support for additional fuzzer parameters in Trident.toml config file ([#107](https://github.com/Ackee-Blockchain/trident/pull/107))
- feat/posibility to pass params to the fuzzer via Trident.toml config file ([#106](https://github.com/Ackee-Blockchain/trident/pull/106))
- feat/client now reads by default keypair from default location ([#105](https://github.com/Ackee-Blockchain/trident/pull/105))
- feat/added new --exit-code option to return corresponding exit code based on fuzz test result ([#104](https://github.com/Ackee-Blockchain/trident/pull/104))
- feat/removed/updated deprecated functions, removed allow deprecated macros ([#103](https://github.com/Ackee-Blockchain/trident/pull/103))
- feat/added new function to read keypair file generated by Anchor ([#102](https://github.com/Ackee-Blockchain/trident/pull/102))
- feat/clean command ([#101](https://github.com/Ackee-Blockchain/trident/pull/101))
- feat/improved program_client generated code ([#100](https://github.com/Ackee-Blockchain/trident/pull/100))
- feat/automatically add hfuzz_target to .gitignore file ([#99](https://github.com/Ackee-Blockchain/trident/pull/99))
- feat/support for dynamic templates. ([#98](https://github.com/Ackee-Blockchain/trident/pull/98))

### Fixed
- fix/refactored fuzz test executor error handling ([#127](https://github.com/Ackee-Blockchain/trident/pull/127))
- fix/warn user on composite accounts and continue fuzz test generation ([#133](https://github.com/Ackee-Blockchain/trident/pull/133))
- fix/progress bar loop lock release ([#132](https://github.com/Ackee-Blockchain/trident/pull/132))
- fix/fixed fuzz test generation with init-if-needed Anchor feature ([#131](https://github.com/Ackee-Blockchain/trident/pull/131))
- fix/program client custom types ([#117](https://github.com/Ackee-Blockchain/trident/pull/117))
- fix/check fuzz test name collision by checking the name against HashSet ([#114](https://github.com/Ackee-Blockchain/trident/pull/114))


## [0.5.0] - 2023-08-28
### Added
- cli: Added trident subcommand `fuzz` to run and debug fuzz tests using honggfuzz-rs.
- cli: Added trident `--skip-fuzzer` option for `init` subcommand to skip generation of fuzz test templates.
- client: Added new Cargo feature `fuzzing` that enables optional dependencies related to fuzz testing.

## [0.4.1] - 2023-08-21
### Changed
- Upgrade Solana (`=1.16.6`) and Anchor framework (`=0.28.0`) versions.
### Fixed
- Implemented Anchor Client logic was not able to work with newer version of Anchor. Fixed with `async_rpc` and `async` feature.
- Trident init IDL Parse Error on newer version of Rust, fixed with updated `accounts` token.


## [0.3.0] - 2022-09-23
### Changed
- Upgrade Solana (`~1.10`) and Anchor framework (`~0.25`) versions

### Added
- Custom Solana RPC error reporter. If the Solana RPC error is thrown, the error code, message and data (logs) are reported to the output.
- Custom imports in the `.program_client`. User is able to import custom types and structures into program client. The import part of the code would not be re-generated.

## [0.2.0] - 2022-07-27
### Added
- Trident is now configurable. This requires `Trident.toml` file to exist in the project's root directory - without this file the execution will fail. To solve this re-run `trident init` or just create an empty `Trident.toml` file in the project's root directory.
