# Fuzzing Flows

Flows are a way to define `custom sequences of behavior` that should happen during each fuzzing iteration. By default, Trident generates random sequences of Transactions which are part of the `FuzzTransactions` enum. On the other hand, with flows it is possible to specify custom sequences of Transactions, and additionally, custom logic can be performed between transactions.

The fuzz test iteration is composed of the following steps:

- Initializations, check the [Initialization](./initialization.md)
- Flows definition, check the [Flows](./flow.md)
