# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased Changes

### Features

- Support Anchor v0.24 [#42](https://github.com/saber-hq/vipers/pull/42)

## [2.0.2] - 2022-03-21

### Features

- Support Anchor v0.23 [#41](https://github.com/saber-hq/vipers/pull/41)

## [2.0.1] - 2022-02-27

### Features

- Adds the `unwrap_bump!` macro, which extracts the bump from the instruction `Context` ([#36](https://github.com/saber-hq/vipers/pull/36)).

## [2.0.0] - 2022-02-21

### Features

- Adds `CmpError`, which allows comparing errors ([#33](https://github.com/saber-hq/vipers/pull/33)).

### Breaking

- Make `spl-associated-token-account` an optional dependency ([#32](https://github.com/saber-hq/vipers/pull/32)).
- Remove support for Anchor versions older than 0.22 ([#33](https://github.com/saber-hq/vipers/pull/33)).

## [1.6.0] - 2022-02-14

### Fixes

- Support Anchor 0.21.x by adding `AsKeyRef` ([#31](https://github.com/saber-hq/vipers/pull/31)).

### Features

- Adds the `AsKeyRef` trait, which allows reading the `&Pubkey` of any account or `Pubkey`. ([#31](https://github.com/saber-hq/vipers/pull/31))
