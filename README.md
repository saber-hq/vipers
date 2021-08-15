# `vipers` 😎

Various checks and validations for writing safer Solana programs.

## Motivation

Solana's fee mechanism is unlike Ethereum's, in that the number of bytecode instructions execued _does not_ add to the cost of a transaction. Due to this, it is wise to design Solana code with plenty of safety checks in order to minimize the chance of hacks.

This library provides several utilities for Anchor programs to validate account structs and check for invariants.

## License

Apache 2.0
