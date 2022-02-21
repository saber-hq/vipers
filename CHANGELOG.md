# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Features

- Adds `ComparableError`, which allows comparing errors.

### Breaking

- Make `spl-associated-token-account` an optional dependency ([#32](https://github.com/saber-hq/vipers/pull/32)).
- Remove support for Anchor versions older than 0.22.

## [1.6.0] - 2022-02-14

### Fixes

- Support Anchor 0.21.x by adding `AsKeyRef` ([#31](https://github.com/saber-hq/vipers/pull/31)).

### Features

- Adds the `AsKeyRef` trait, which allows reading the `&Pubkey` of any account or `Pubkey`. ([#31](https://github.com/saber-hq/vipers/pull/31))
