mod api;
pub use api::OpenFeature;

mod client;
pub use client::{Client, ClientMetadata};

mod provider_registry;

mod global_evaluation_context;
mod global_hooks;
