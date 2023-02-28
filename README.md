# OpenFeature SDK for Rust

This is the rust implementation of [OpenFeature](https://openfeature.dev), a vendor-agnostic abstraction library for evaluating feature flags.

We support multiple data types for flags (numbers, strings, booleans, objects) as well as hooks, which can alter the lifecycle of a flag evaluation.


## Installation

```
rust-sdk = { git = "https://github.com/open-feature/rust-sdk", branch = "main" }
```

## Usage

### Initialization

```rust
use rust_sdk::Client;
use rust_sdk::providers;
use rust_sdk::providers::traits::FeatureProvider;
use rust_sdk::traits::ClientTraits;

fn main() {
    
    let client = Client::<providers::NoOProvider>::new(
        "test".to_string(),
        providers::NoOProvider::new(),
    );
    let (result, err) = client.value::<i64>("test".to_string(),
        0, client.evaluation_context() );
}

```