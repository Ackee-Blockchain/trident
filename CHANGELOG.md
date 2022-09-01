# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html) (SemVer).

**Note**: Version 0 of Semantic Versioning is handled differently from version 1 and above. The minor version will be
incremented upon a breaking change and the patch version will be incremented for features.

## [Unreleased]

### Added
- Custom Solana RPC error reporter. If the Solana RPC error is thrown, the error code, message and data (logs) are reported to the output.
- Custom imports in the `.program_client`. User is able to import custom types and structures into program client. The import part of the code would not be re-generated.

## [0.2.0] - 2022-07-27
### Added
- Trdelnik is now configurable. This requires `Trdelnik.toml` file to exist in the project's root directory - without this file the execution will fail. To solve this re-run `trdelnik init` or just create an empty `Trdelnik.toml` file in the project's root directory.
