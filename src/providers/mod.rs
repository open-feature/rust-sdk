use crate::evaluation::FlattenedContext;

use self::{
    traits::FeatureProvider,
    types::{Metadata, ResolutionDetails, ResolutionError},
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

pub const TARGETING_KEY: &str = "targetingKey"; // eva

pub struct NoOProvider {}

impl FeatureProvider for NoOProvider {
    fn new() -> Self {
        return NoOProvider {};
    }

    fn meta_data(&self) -> Metadata {
        return Metadata {
            name: "NoOProvider".to_string(),
        };
    }

    fn evaluation<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: FlattenedContext,
    ) -> ResolutionDetails<T>
    where
        T: Clone,
    {
        return ResolutionDetails::<T> {
            value: default_value,
            varient: "".to_string(),
            reason: "".to_string(),
            resolution_error: ResolutionError {
                code: "".to_string(),
                message: "".to_string(),
            },
        };
    }
}
