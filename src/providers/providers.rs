use crate::evaluation::FlattenedContext;

use self::{
    traits::FeatureProvider,
    types::{ProviderMetadata, ResolutionDetails, ResolutionError},
};

pub mod traits;
pub mod types;
// DefaultReason - the resolved value was configured statically, or otherwise fell back to a pre-configured value.
pub const DEFAULT_REASON: &str = "DEFAULT";

pub const TARGETING_MATCH_REASON: &str = "TARGETING_MATCH";
// SplitReason - the resolved value was the result of pseudorandom assignment.
pub const SPLIT_REASON: &str = "SPLIT";
// DisabledReason - the resolved value was the result of the flag being disabled in the management system.
pub const DISABLED_REASON: &str = "DISABLED";
// StaticReason - the resolved value is static (no dynamic evaluation)
pub const STATIC_REASON: &str = "STATIC";
// CachedReason - the resolved value was retrieved from cache
pub const CACHED_REASON: &str = "CACHED";
// UnknownReason - the reason for the resolved value could not be determined.
pub const UNKNOWN_REASON: &str = "UNKNOWN";
// ErrorReason - the resolved value was the result of an error.
pub const ERROR_REASON: &str = "ERROR";

pub const TARGETING_KEY: &str = "targetingKey";

// NoopProvider - a provider that does nothing
pub struct NoopProvider {}

impl FeatureProvider for NoopProvider {
    fn new() -> Self {
        return NoopProvider {};
    }

    fn meta_data(&self) -> ProviderMetadata {
        return ProviderMetadata {
            name: "NoopProvider".to_string(),
        };
    }

    fn evaluation<T>(
        &self,
        _flag: String,
        default_value: T,
        _eval_ctx: FlattenedContext,
    ) -> Result<ResolutionDetails<T>, anyhow::Error>
    where
        T: Clone,
    {
        let resolution_error = ResolutionError {
            code: 0.to_string(),
            message: "".to_string(),
        };
        let reason = DEFAULT_REASON.to_string();
        let variant = "".to_string();
        let value = default_value;
        let resolution_details = ResolutionDetails {
            value,
            variant,
            reason,
            resolution_error,
        };
        return Ok(resolution_details);
    }
}
