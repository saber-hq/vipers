# vipers 😎

[![Crates.io](https://img.shields.io/crates/v/vipers?style=flat-square)](https://crates.io/crates/vipers)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/saber-hq/vipers/blob/master/LICENSE-APACHE)
[![Build Status](https://img.shields.io/github/workflow/status/saber-hq/vipers/CI/master?style=flat-square)](https://github.com/saber-hq/vipers/actions/workflows/ci.yml?query=branch%3Amaster)
[![Contributors](https://img.shields.io/github/contributors/saber-hq/vipers?style=flat-square)](https://github.com/saber-hq/vipers/graphs/contributors)

<p align="center">
    <img src="/images/banner.png" />
</p>

<p align="center">
    Assorted checks and validations for writing safer Solana programs.
</p>

## Motivation

Solana's fee mechanism is unlike Ethereum's, in that the number of bytecode instructions execued _does not_ add to the cost of a transaction. Due to this, it is wise to design Solana code with excessive safety checks in order to minimize the chance of exploits.

This library provides several utilities for Anchor programs to validate account structs and check for invariants.

## Support

For support, join the Saber Discord at [chat.saber.so](https://chat.saber.so)!

## License

Apache 2.0
