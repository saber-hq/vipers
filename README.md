# `vipers` 😎

Assorted checks and validations for writing safer Solana programs.

## Motivation

Solana's fee mechanism is unlike Ethereum's, in that the number of bytecode instructions execued _does not_ add to the cost of a transaction. Due to this, it is wise to design Solana code with excessive safety checks in order to minimize the chance of exploits.

This library provides several utilities for Anchor programs to validate account structs and check for invariants.

## Support

For support, join the Saber Discord at [chat.saber.so](https://chat.saber.so)!

## License

Apache 2.0
