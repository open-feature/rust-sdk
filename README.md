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
use rust_sdk::providers::NoopProvider;
use rust_sdk::providers::traits::FeatureProvider;
use rust_sdk::traits::ClientTraits;

fn main() {
    
    let client = Client::<NoopProvider>::new(
        "client-name".to_string(),
        NoopProvider::new(),
    );
    let result = client.value::<i64>("flag-key-here".to_string(),
        0, client.evaluation_context() );
    println!("result: {}", result.unwrap());
}

```