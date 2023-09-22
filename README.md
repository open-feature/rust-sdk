# OpenFeature SDK for Rust

[![Project Status: WIP – Initial development is in progress, but there has not yet been a stable, usable release suitable for the public.](https://www.repostatus.org/badges/latest/wip.svg)](https://www.repostatus.org/#wip)

This is the rust implementation of [OpenFeature](https://openfeature.dev), a vendor-agnostic abstraction library for evaluating feature flags.

We support multiple data types for flags (numbers, strings, booleans, objects) as well as hooks, which can alter the lifecycle of a flag evaluation.

## Installation

```toml
open-feature = { git = "https://github.com/open-feature/rust-sdk", branch = "main" }
```

## Usage

First please check the [document](https://openfeature.dev/docs/reference/intro) to have a better understanding of the OpenFeature architecture. We will assume that you have read it. :-)

This SDK is compliant with the [OpenFeature specification](https://openfeature.dev/specification/) most of the time. Sometimes, it takes a more Rusty way to model a requirement, as long as the basic functionality is covered. These parts will be explicitly noted though.

Let's roll.

```rust
let api = OpenFeature::singleton_mut();

```


## Roadmap

## Pending Feature List

- Provider hooks (2.3)
- Hooks (4)
- Events (5)
