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

### Basic Usage

```rust
#[derive(Clone, Default, Debug)]
struct MyStruct {}

#[tokio::test]
async fn example() {
    // Acquire an OpenFeature API instance.
    // Note the `await` call here because asynchronous lock is used to guarantee thread safety.
    let mut api = OpenFeature::singleton_mut().await;

    // Createa a global evaluation context and set it into the API.
    // Note that this is optional. By default it uses an empty one.
    let global_evaluation_context = EvaluationContext::default();
    api.set_evaluation_context(global_evaluation_context).await;

    // Set the default feature provider.
    // If you do not do that, [`NoOpProvider`] will be used by default.
    //
    // By default, [`NoOpProvider`] will simply return the default value of each type.
    // You can inject value you want via its builder or evaluation context. See its document
    // for more details.
    //
    // If you set a new provider after creating some clients, the existing clients will pick up
    // the new provider you just set.
    api.set_provider(NoOpProvider::default()).await;

    // Create an unnamed client.
    let client = api.create_client();

    // Createa an evaluation context.
    // It supports types mentioned in the specification.
    //
    // You have multiple ways to add a custom field.
    let evaluation_context = EvaluationContext::builder()
        .targeting_key("Targeting")
        .build()
        .with_custom_field("bool_key", true)
        .with_custom_field("int_key", 100)
        .with_custom_field("float_key", 3.14)
        .with_custom_field("string_key", "Hello".to_string())
        .with_custom_field("datetime_key", time::OffsetDateTime::now_utc())
        .with_custom_field(
            "struct_key",
            EvaluationContextFieldValue::Struct(Arc::new(MyStruct::default())),
        )
        .with_custom_field("another_struct_key", Arc::new(MyStruct::default()))
        .with_custom_field(
            "yet_another_struct_key",
            EvaluationContextFieldValue::new_struct(MyStruct::default()),
        );

    assert_eq!(
        client
            .get_bool_value("key", Some(&evaluation_context), None)
            .await
            .unwrap(),
        bool::default()
    );

    // Create a named provider and bind it.
    api.set_named_provider("named", NoOpProvider::builder().int_value(42).build())
        .await;

    // This named client will use the feature provider bound to this name.
    let client = api.create_named_client("named");

    assert_eq!(42, client.get_int_value("key", None, None).await.unwrap());

    // Let's get evaluation details.
    // Note that we will inject `300` as the int value via evaluation context.
    // It is not a feature mentioned in the standard but rather implemented for the
    // convenience.
    let result = client
        .get_int_details(
            "key",
            Some(&EvaluationContext::default().with_custom_field("Value", 300)),
            None,
        )
        .await;

    match result {
        Ok(details) => {
            assert_eq!(details.value, 300);
            assert_eq!(details.reason, Some(EvaluationReason::Static));
            assert_eq!(details.variant, Some("Static".to_string()));
            assert_eq!(details.flag_metadata.values.iter().count(), 1);
        }
        Err(error) => {
            println!(
                "Error: {}\nMessage: {:?}\n",
                error.code.to_string(),
                error.message
            );
        }
    }
}
```

### Retrieve a struct

It is possible to extract a struct from the provider. Internally, this SDK defines a type `StructValue` to store any structure value. The `client.get_struct_value()` functions takes a type parameter `T`. It will try to parse `StructValue` resolved by the provider to `T`, as long as `T` implements trait `FromStructValue`.

You can pass in a type that satisfies this trait bound. When the conversion fails, it returns an `Err` with `EvaluationReason::TypeMismatch`.

## Roadmap

* Implement hooks and events.
* Implement derive macro for `FromStructValue`.
