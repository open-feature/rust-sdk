use crate::evaluation::FlattenedContext;

use self::{
    traits::FeatureProvider,
    types::{Metadata, ResolutionDetails, ResolutionError},
};

pub mod traits;
pub mod types;
// DefaultReason - the resolved value was configured statically, or otherwise fell back to a pre-configured value.
pub const DefaultReason: &str = "DEFAULT";
pub const TargetingMatchReason: &str = "TARGETING_MATCH";
// SplitReason - the resolved value was the result of pseudorandom assignment.
pub const SplitReason: &str = "SPLIT";
// DisabledReason - the resolved value was the result of the flag being disabled in the management system.
pub const DisabledReason: &str = "DISABLED";
// StaticReason - the resolved value is static (no dynamic evaluation)
pub const StaticReason: &str = "STATIC";
// CachedReason - the resolved value was retrieved from cache
pub const CachedReason: &str = "CACHED";
// UnknownReason - the reason for the resolved value could not be determined.
pub const UnknownReason: &str = "UNKNOWN";
// ErrorReason - the resolved value was the result of an error.
pub const ErrorReason: &str = "ERROR";

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
