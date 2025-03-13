# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) (SemVer).

**Note**: Version 0 of Semantic Versioning is handled differently from version 1 and above. The minor version will be
incremented upon a breaking change and the patch version will be incremented for features.

## [dev] - Unreleased

**Added**

**Removed**

**Changed**

## [0.10.0] - 2025-03-13

**Added**

- TransactionSelector allows to select and execute random transaction with hooks and without hooks ([297](https://github.com/Ackee-Blockchain/trident/pull/297))
- Added "random_tail" attribute to TridentFlowExecutor macro to allow random tail generation ([296](https://github.com/Ackee-Blockchain/trident/pull/296))
- Added support for seeds constraint for accounts structs ([284](https://github.com/Ackee-Blockchain/trident/pull/284))
- Additional methods accessing the AccountsStorage and creating corresponding accounts are now optional ("token","vote", "stake") ([279](https://github.com/Ackee-Blockchain/trident/pull/279))
- Derive macros now use own syn parser for better extensibility and UX ([275](https://github.com/Ackee-Blockchain/trident/pull/275))
- Program ID inside test_fuzz.rs file is now automatically filled in is present in the program IDL ([272](https://github.com/Ackee-Blockchain/trident/pull/272))
- Added additional attributes to TridentAccounts, mut and signer ([268](https://github.com/Ackee-Blockchain/trident/pull/268))
- Users can now specify a program for which they want to add or initialize a fuzz test using `--program-name` flag ([273](https://github.com/Ackee-Blockchain/trident/pull/273))
- Allow custom test name specification in fuzz test creation with `--test-name` flag ([274](https://github.com/Ackee-Blockchain/trident/pull/274))

**Removed**

- The DisplayIx macro is now removed, Debug trait was implemented for instruction inputs ([281](https://github.com/Ackee-Blockchain/trident/pull/281))

**Changed**

- FuzzTestExecutor and FlowExecutor are grouped together and FuzzTest has client as generic instead of TridentSVM ([294](https://github.com/Ackee-Blockchain/trident/pull/294))
- Changed Transaction sequence logic ([289](https://github.com/Ackee-Blockchain/trident/pull/289))
- AccountsStorages are simplified, no types are required to define ([276](https://github.com/Ackee-Blockchain/trident/pull/276))
- Trident Architecture changes containing multiple features and improvements (details in the PR)([267](https://github.com/Ackee-Blockchain/trident/pull/267))
- get or create token account methods fixed for native token accounts ([262](https://github.com/Ackee-Blockchain/trident/pull/262))


## [0.9.1] - 2025-02-03

**Added**

- added warp to time manipulation ([259](https://github.com/Ackee-Blockchain/trident/pull/259))
- added methods to check if account storage is empty ([258](https://github.com/Ackee-Blockchain/trident/pull/258))
- added support for post-instruction behavior ([257](https://github.com/Ackee-Blockchain/trident/pull/257))

**Removed**

**Changed**

## [0.9.0] - 2025-01-15

**Added**

- added support for composite accounts ([245](https://github.com/Ackee-Blockchain/trident/pull/245))
- Trident SVM + AFL (see the PR for more details) ([234](https://github.com/Ackee-Blockchain/trident/pull/234))

**Removed**

- removed fuzz_iteration from test_fuzz.rs ([243](https://github.com/Ackee-Blockchain/trident/pull/243))

**Changed**

- renamed Config to TridentConfig ([246](https://github.com/Ackee-Blockchain/trident/pull/246))
- errors are simplified and transaction error contains only transaction error ([244](https://github.com/Ackee-Blockchain/trident/pull/244))

## [0.8.1] - 2024-11-14

**Removed**

- removed unnecesarry deserialization with AccountsSnapshots, to deserialize data implementation AccountDeserialize can be used ([221](https://github.com/Ackee-Blockchain/trident/pull/221))

**Changed**

- improve AccountsStorage module structure and remove unnecessary methods in FuzzClient ([223](https://github.com/Ackee-Blockchain/trident/pull/223))
- improve manipulations with AccountsStorages in get_accounts() function ([219](https://github.com/Ackee-Blockchain/trident/pull/219))

**Added**

- add pre_sequence!, middle_sequence! and post_sequence! for easier sequence definition ([220](https://github.com/Ackee-Blockchain/trident/pull/220))
- add/ add support for Clock sysvar manipulations with the client(i.e. warp to slot/epoch and forward in time) ([217](https://github.com/Ackee-Blockchain/trident/pull/217))

## [0.8.0] - 2024-10-21

**Added**

- impr/ allow to specify programs and accounts in the Trident Manifest ([207](https://github.com/Ackee-Blockchain/trident/pull/207))
- impr/ added get_program_id function to the IxOps and FuzzTestExecutor ([199](https://github.com/Ackee-Blockchain/trident/pull/199))

**Changed**

- impr/ improve init command, modify program manifest automatically, add init force, add howto subcommand ([208](https://github.com/Ackee-Blockchain/trident/pull/208))
- impr/ allow to derive AccountsSnapshots for empty Account Context ([209](https://github.com/Ackee-Blockchain/trident/pull/209))
- impr/ fuzz flags are read at start of fuzzing session from Config instead of env variable and transaction dispatch was added to increase FuzzTestExecutor readability ([204](https://github.com/Ackee-Blockchain/trident/pull/204))
- impr/ allow various instructions to be generated in case of multiple programs in the Anchor workspace ([200](https://github.com/Ackee-Blockchain/trident/pull/200))
- feat/ option to add account into Fuzz Test environment with base64 data ([197](https://github.com/Ackee-Blockchain/trident/pull/197))
- impr/ instead of parsing source code and creating our IDL, read anchor IDL ([198](https://github.com/Ackee-Blockchain/trident/pull/196))

**Removed**

- del/remove integration tests supported by Trident, this feature adds more unnecessary overhead compared to its value ([196](https://github.com/Ackee-Blockchain/trident/pull/198))

## [0.7.0] - 2024-08-14

**Added**

- impr/ add feature flag to the AccountsSnapshots macro ([183](https://github.com/Ackee-Blockchain/trident/pull/183))
- feat/ add Support for CPI ([182](https://github.com/Ackee-Blockchain/trident/pull/182))
- feat/ add option to initialize Trident with Macro/File (for Snapshots) option based on preference ([179](https://github.com/Ackee-Blockchain/trident/pull/179))
- feat/create AccountsSnapshots derive macro for Snapshots creation ([#177](https://github.com/Ackee-Blockchain/trident/pull/177))
- feat/fuzzing moved to separate crate trident-fuzz ([#175](https://github.com/Ackee-Blockchain/trident/pull/175))
- feat/unify dependencies provided by the Trident ([#172](https://github.com/Ackee-Blockchain/trident/pull/172))
- feat/fuzzer-stats-logging, an optional statistics output for fuzzing session ([#144](https://github.com/Ackee-Blockchain/trident/pull/144))

**Fixed**

- fix/in case of fuzzing failure throw error instead of only printing message ([#167](https://github.com/Ackee-Blockchain/trident/pull/167))
- fix/snapshot's zeroed account as optional ([#170](https://github.com/Ackee-Blockchain/trident/pull/170))

**Removed**

- del/remove localnet subcommand ([178](https://github.com/Ackee-Blockchain/trident/pull/178))
- del/remove unnecessary fuzzing feature as trident is mainly fuzzer ([#176](https://github.com/Ackee-Blockchain/trident/pull/176))
- del/remove Trident explorer ([#171](https://github.com/Ackee-Blockchain/trident/pull/171))

## [0.6.0] - 2024-05-20

**Added**

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

**Fixed**

- fix/refactored fuzz test executor error handling ([#127](https://github.com/Ackee-Blockchain/trident/pull/127))
- fix/warn user on composite accounts and continue fuzz test generation ([#133](https://github.com/Ackee-Blockchain/trident/pull/133))
- fix/progress bar loop lock release ([#132](https://github.com/Ackee-Blockchain/trident/pull/132))
- fix/fixed fuzz test generation with init-if-needed Anchor feature ([#131](https://github.com/Ackee-Blockchain/trident/pull/131))
- fix/program client custom types ([#117](https://github.com/Ackee-Blockchain/trident/pull/117))
- fix/check fuzz test name collision by checking the name against HashSet ([#114](https://github.com/Ackee-Blockchain/trident/pull/114))


## [0.5.0] - 2023-08-28

**Added**

- cli: Added trident subcommand `fuzz` to run and debug fuzz tests using honggfuzz-rs.
- cli: Added trident `--skip-fuzzer` option for `init` subcommand to skip generation of fuzz test templates.
- client: Added new Cargo feature `fuzzing` that enables optional dependencies related to fuzz testing.

## [0.4.1] - 2023-08-21

**Changed**

- Upgrade Solana (`=1.16.6`) and Anchor framework (`=0.28.0`) versions.

**Fixed**

- Implemented Anchor Client logic was not able to work with newer version of Anchor. Fixed with `async_rpc` and `async` feature.
- Trident init IDL Parse Error on newer version of Rust, fixed with updated `accounts` token.


## [0.3.0] - 2022-09-23

**Changed**

- Upgrade Solana (`~1.10`) and Anchor framework (`~0.25`) versions

**Added**

- Custom Solana RPC error reporter. If the Solana RPC error is thrown, the error code, message and data (logs) are reported to the output.
- Custom imports in the `.program_client`. User is able to import custom types and structures into program client. The import part of the code would not be re-generated.

## [0.2.0] - 2022-07-27

**Added**

- Trident is now configurable. This requires `Trident.toml` file to exist in the project's root directory - without this file the execution will fail. To solve this re-run `trident init` or just create an empty `Trident.toml` file in the project's root directory.
