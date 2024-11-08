//! This is the official Rust package for OpenFeature.
//! Check OpenFeature website for the background.
//! Check its README for examples.

#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_inception)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::too_many_lines)]

/// The OpenFeature API and client.
mod api;
pub use api::*;

/// Evaluation related.
mod evaluation;
pub use evaluation::*;

/// Hooks related.
mod hooks;
pub use hooks::*;

/// Feature provider related.
pub mod provider;
pub use async_trait::async_trait;

/// Optional support for [`serde_json::Value`].
#[cfg(feature = "serde_json")]
pub mod serde_json;
