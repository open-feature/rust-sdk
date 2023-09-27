/// Evaluation details.
mod details;
pub use details::ResolutionDetails;

/// Feature provider trait.
mod feature_provider;
pub use feature_provider::{FeatureProvider, ProviderMetadata, ProviderStatus};

/// The default no-op provider.
mod no_op_provider;
pub use no_op_provider::NoOpProvider;
