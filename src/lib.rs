//! This is the official Rust package for OpenFeature.
//! Check OpenFeature website for the background.
//! Check its README for examples.

#![deny(clippy::pedantic)]
#![warn(missing_docs)]
#![deny(warnings)]
#![allow(clippy::doc_markdown)]
#![allow(clippy::implicit_hasher)]
#![allow(clippy::manual_let_else)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::new_without_default)]
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::uninlined_format_args)]

/// The OpenFeature API and client.
mod api;
pub use api::*;

/// Evaluation related.
mod evaluation;
pub use evaluation::*;

/// Feature provider related.
pub mod provider;
