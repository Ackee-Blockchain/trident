# Trident API & Macro Reference

This section contains the API and macro reference for Trident.

## Trident 

The Trident struct is the main entry point for fuzzing. It provides methods to interact with the fuzzing environment.

- [Trident](trident/trident.md)


## Trident's Fuzz Client

FuzzClient represents the API to interact with the client.

- [FuzzClient](trident-fuzz-client/fuzz-client-trait.md)


## Trident Accounts attributes

Trident Accounts attributes help to define accounts used in the fuzzing environment.

- [TridentAccounts](attributes-for-accounts/account-attributes.md)

## Hooks

Transaction Hooks are used to customize what should be done before the transaction is executed, after it is executed and to check if the changes are valid with invariant checks.

- [InstructionHooks](hooks/instruction-hooks.md)


Instruction Hooks are used to customize instruction data and accounts if the Trident Accounts attributes are not enough.

- [TransactionHooks](hooks/transaction-hooks.md)


## Trident Macros

Trident macros are used to define required methods for fuzzing.

- [TridentTransaction](trident-macros/trident-transaction.md)
- [TridentInstruction](trident-macros/trident-instruction.md)
- [TridentAccounts](trident-macros/trident-accounts.md)
- [TridentRemainingAccounts](trident-macros/trident-remaining-accounts.md)
- [TridentFlowExecutor](trident-macros/trident-flow-executor.md)

## Trident Types

Trident types represent the types utilized during fuzzing.

- [FuzzAccounts](trident-types/fuzz-accounts.md)
- [TridentAccount](trident-types/trident-account.md)
- [TridentPubkey](trident-types/trident-pubkey.md)
- [CustomTypes](trident-types/custom-types.md)
