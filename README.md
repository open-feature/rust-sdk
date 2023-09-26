<!-- markdownlint-disable MD033 -->
<!-- x-hide-in-docs-start -->
<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/open-feature/community/0e23508c163a6a1ac8c0ced3e4bd78faafe627c7/assets/logo/horizontal/white/openfeature-horizontal-white.svg" />
    <img align="center" alt="OpenFeature Logo" src="https://raw.githubusercontent.com/open-feature/community/0e23508c163a6a1ac8c0ced3e4bd78faafe627c7/assets/logo/horizontal/black/openfeature-horizontal-black.svg" />
  </picture>
</p>

<h2 align="center">OpenFeature Rust SDK</h2>

<!-- x-hide-in-docs-end -->
<!-- The 'github-badges' class is used in the docs -->
<p align="center" class="github-badges">
  <a href="https://github.com/open-feature/spec/tree/v0.7.0">
    <img alt="Specification" src="https://img.shields.io/static/v1?label=specification&message=v0.7.0&color=yellow&style=for-the-badge" />
  </a>
  <!-- TODO: update the Release Please config to include the readme -->
  <!-- x-release-please-start-version -->

<!-- TODO: update with your SDK repo and the latest release version
  <a href="https://github.com/open-feature/my-sdk/releases/tag/v0.0.1">
    <img alt="Release" src="https://img.shields.io/static/v1?label=release&message=v0.0.1&color=blue&style=for-the-badge" />
  </a>  
-->

  <!-- x-release-please-end -->
  <!-- TODO: update this when we have it set up
  <br/>
  <a href="https://bestpractices.coreinfrastructure.org/projects/6601">
    <img alt="CII Best Practices" src="https://bestpractices.coreinfrastructure.org/projects/6601/badge" />
  </a>
</p>
  -->
<!-- x-hide-in-docs-start -->

[OpenFeature](https://openfeature.dev) is an open standard that provides a vendor-agnostic, community-driven API for feature flagging that works with your favorite feature flag management tool.

<!-- x-hide-in-docs-end -->
## 🚀 Quick start

### Requirements

This package was built with Rust version `1.70.0`. Earlier versions might work, but is not guaranteed.

### Install

Add the following content to the `Cargo.toml` file:

```toml
open-feature = { git = "https://github.com/open-feature/rust-sdk", branch = "main" }
```

### Usage

#### Basic Usage

```rust
#[derive(Clone, Default, Debug)]
struct MyStruct {}

#[tokio::test]
async fn example() {
    // Acquire an OpenFeature API instance.
    // Note the `await` call here because asynchronous lock is used to guarantee thread safety.
    let mut api = OpenFeature::singleton_mut().await;

    // Create an unnamed client.
    let client = api.create_client();

    // Create an evaluation context.
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

    // This function returns a `Result`. You can process it with functions provided by std.
    let is_feature_enabled = client
        .get_bool_value("SomeFlagEnabled", Some(&evaluation_context), None)
        .await
        .unwrap_or(false);

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

#### Getting a Struct from a Provider

It is possible to extract a struct from the provider. Internally, this SDK defines a type `StructValue` to store any structure value. The `client.get_struct_value()` functions takes a type parameter `T`. It will try to parse `StructValue` resolved by the provider to `T`, as long as `T` implements trait `FromStructValue`.

You can pass in a type that satisfies this trait bound. When the conversion fails, it returns an `Err` with `EvaluationReason::TypeMismatch`.

### API Reference

<!-- TODO: link to formal API docs (ie: Javadoc) if available -->

## 🌟 Features

| Status | Features                        | Description                                                                                                                        |
| ------ | ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| ✅      | [Providers](#providers)         | Integrate with a commercial, open source, or in-house feature management tool.                                                     |
| ✅      | [Targeting](#targeting)         | Contextually-aware flag evaluation using [evaluation context](https://openfeature.dev/docs/reference/concepts/evaluation-context). |
| ❌      | [Hooks](#hooks)                 | Add functionality to various stages of the flag evaluation life-cycle.                                                             |
| ❌      | [Logging](#logging)             | Integrate with popular logging packages.                                                                                           |
| ✅      | [Named clients](#named-clients) | Utilize multiple providers in a single application.                                                                                |
| ❌      | [Eventing](#eventing)           | React to state changes in the provider or flag management system.                                                                  |
| ✅      | [Shutdown](#shutdown)           | Gracefully clean up a provider during application shutdown.                                                                        |
| ❌      | [Extending](#extending)         | Extend OpenFeature with custom providers and hooks.                                                                                |

<sub>Implemented: ✅ | In-progress: ⚠️ | Not implemented yet: ❌</sub>

### Providers

[Providers](https://openfeature.dev/docs/reference/concepts/provider) are an abstraction between a flag management system and the OpenFeature SDK.
Look [here](https://openfeature.dev/ecosystem?instant_search%5BrefinementList%5D%5Btype%5D%5B0%5D=Provider&instant_search%5BrefinementList%5D%5Btechnology%5D%5B0%5D=Rust) for a complete list of available providers.
If the provider you're looking for hasn't been created yet, see the [develop a provider](#develop-a-provider) section to learn how to build it yourself.

Once you've added a provider as a dependency, it can be registered with OpenFeature like this:

```rust
// Set the default feature provider. Please replace the `NoOpProvider` with the one you want.
// If you do not do that, [`NoOpProvider`] will be used by default.
//
// By default, [`NoOpProvider`] will simply return the default value of each type.
// You can inject value you want via its builder or evaluation context. See other sections
// for more details.
//
// If you set a new provider after creating some clients, the existing clients will pick up
// the new provider you just set.
//
// You must `await` it to let the provider's initialization to finish.
let mut api = OpenFeature::singleton_mut().await;
api.set_provider(NoOpProvider::default()).await;
```

In some situations, it may be beneficial to register multiple providers in the same application.
This is possible using [named clients](#named-clients), which is covered in more details below.

### Targeting

Sometimes, the value of a flag must consider some dynamic criteria about the application or user, such as the user's location, IP, email address, or the server's location.
In OpenFeature, we refer to this as [targeting](https://openfeature.dev/specification/glossary#targeting).
If the flag management system you're using supports targeting, you can provide the input data using the [evaluation context](https://openfeature.dev/docs/reference/concepts/evaluation-context).

```rust
// Create a global evaluation context and set it into the API.
// Note that this is optional. By default it uses an empty one.
let mut api = OpenFeature::singleton_mut().await;
api.set_evaluation_context(global_evaluation_context).await;

// Set client level evaluation context.
// It will overwrite the global one for the existing keys.
let mut client = api.create_client();
client.set_evaluation_context(client_evaluation_context);

// Pass evaluation context in evaluation functions.
// This one will overwrite the globla evaluation context and 
// the client level one.
client.get_int_value("flag", &evaluation_context, None);
```

### Hooks

[Hooks](https://openfeature.dev/docs/reference/concepts/hooks) allow for custom logic to be added at well-defined points of the flag evaluation life-cycle.
Look [here](https://openfeature.dev/ecosystem/?instant_search%5BrefinementList%5D%5Btype%5D%5B0%5D=Hook&instant_search%5BrefinementList%5D%5Btechnology%5D%5B0%5D=Rust) for a complete list of available hooks.
If the hook you're looking for hasn't been created yet, see the [develop a hook](#develop-a-hook) section to learn how to build it yourself.

Once you've added a hook as a dependency, it can be registered at the global, client, or flag invocation level.

<!-- TODO: code example of setting hooks at all levels -->

### Logging

<!-- TODO: talk about logging config, and code example -->

### Named clients

Clients can be given a name.
A name is a logical identifier which can be used to associate clients with a particular provider.
If a name has no associated provider, the global provider is used.

```rust
// Create a named provider and bind it.
api.set_named_provider(
    "named",
    NoOpProvider::builder().int_value(42).build())
.await;

// This named client will use the feature provider bound to this name.
let client = api.create_named_client("named");

assert_eq!(client.get_int_value("key", None, None).await.unwrap(), 42);
```

### Eventing

<!-- TOOD: Uncomment it when we support events
Events allow you to react to state changes in the provider or underlying flag management system, such as flag definition changes, provider readiness, or error conditions.
Initialization events (`PROVIDER_READY` on success, `PROVIDER_ERROR` on failure) are dispatched for every provider.
Some providers support additional events, such as `PROVIDER_CONFIGURATION_CHANGED`.

Please refer to the documentation of the provider you're using to see what events are supported.
-->

<!-- TODO: code example of a PROVIDER_CONFIGURATION_CHANGED event for the client and a PROVIDER_STALE event for the API -->

### Shutdown

The OpenFeature API provides a close function to perform a cleanup of all registered providers.
This should only be called when your application is in the process of shutting down.

```rust
// This will clean all the registered providers and invokes their `shutdown()` function.
let api = OpenFeature::singleton_mut().await;
api.shutdown();
```

## Extending

### Develop a provider

To develop a provider, you need to create a new project and include the OpenFeature SDK as a dependency.
This can be a new repository or included in [the existing contrib repository](https://github.com/open-feature/rust-sdk-contrib) available under the OpenFeature organization.
You’ll then need to write the provider by implementing the `FeatureProvider` interface exported by the OpenFeature SDK.

Check the source of [`NoOpProvider`](https://github.com/open-feature/rust-sdk/blob/main/src/provider/no_op_provider.rs) for an example.

> Built a new provider? [Let us know](https://github.com/open-feature/openfeature.dev/issues/new?assignees=&labels=provider&projects=&template=document-provider.yaml&title=%5BProvider%5D%3A+) so we can add it to the docs!

### Develop a hook

To develop a hook, you need to create a new project and include the OpenFeature SDK as a dependency.
This can be a new repository or included in [the existing contrib repository](https://github.com/open-feature/rust-sdk-contrib) available under the OpenFeature organization.
Implement your own hook by conforming to the `Hook interface`.
To satisfy the interface, all methods (`Before`/`After`/`Finally`/`Error`) need to be defined.
To avoid defining empty functions make use of the `UnimplementedHook` struct (which already implements all the empty functions).

<!-- TODO: code example of hook implementation -->

> Built a new hook? [Let us know](https://github.com/open-feature/openfeature.dev/issues/new?assignees=&labels=hook&projects=&template=document-hook.yaml&title=%5BHook%5D%3A+) so we can add it to the docs!

<!-- x-hide-in-docs-start -->
## ⭐️ Support the project

- Give this repo a ⭐️!
- Follow us on social media:
  - Twitter: [@openfeature](https://twitter.com/openfeature)
  - LinkedIn: [OpenFeature](https://www.linkedin.com/company/openfeature/)
- Join us on [Slack](https://cloud-native.slack.com/archives/C0344AANLA1)
- For more, check out our [community page](https://openfeature.dev/community/)

## 🤝 Contributing

Interested in contributing? Great, we'd love your help! To get started, take a look at the [CONTRIBUTING](CONTRIBUTING.md) guide.

### Thanks to everyone that has already contributed

<a href="https://github.com/open-feature/rust-sdk/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=open-feature/rust-sdk" alt="Pictures of the folks who have contributed to the project" />
</a>


Made with [contrib.rocks](https://contrib.rocks).
<!-- x-hide-in-docs-end -->
