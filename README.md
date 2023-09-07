# OpenFeature SDK for Rust

[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)

This is the rust implementation of [OpenFeature](https://openfeature.dev), a vendor-agnostic abstraction library for evaluating feature flags.

We support multiple data types for flags (numbers, strings, booleans, objects) as well as hooks, which can alter the lifecycle of a flag evaluation.

## Installation

```toml
open-feature = { git = "https://github.com/open-feature/rust-sdk", branch = "main" }
```

## Usage

### Initialization

TBD

## Roadmap

## Pending Feature List

- Some requirements of Flag Evaluation API.
- Provider hooks (2.3)
- Evaluation context levels and merging (3.2)
- Hooks (4)
- Events (5)
