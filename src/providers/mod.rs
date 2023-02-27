use crate::evaluation::FlattenedContext;

use self::{types::{Metadata, ResolutionDetails}, traits::FeatureProvider};

pub mod traits;
pub mod types;
// DefaultReason - the resolved value was configured statically, or otherwise fell back to a pre-configured value.
const DefaultReason: &str = "DEFAULT";
const TargetingMatchReason: &str = "TARGETING_MATCH";
// SplitReason - the resolved value was the result of pseudorandom assignment.
const SplitReason: &str = "SPLIT";
// DisabledReason - the resolved value was the result of the flag being disabled in the management system.
const DisabledReason: &str = "DISABLED";
// StaticReason - the resolved value is static (no dynamic evaluation)
const StaticReason: &str = "STATIC";
// CachedReason - the resolved value was retrieved from cache
const CachedReason: &str = "CACHED";
// UnknownReason - the reason for the resolved value could not be determined.	
const UnknownReason: &str = "UNKNOWN";
// ErrorReason - the resolved value was the result of an error.
const ErrorReason: &str = "ERROR";

const TargetingKey: &str = "targetingKey"; // eva


pub struct Provider {}

impl FeatureProvider for Provider {
      fn new() -> Self {
        // Rust has a real lack of OO programming which means we have a single provider type that has
        // to infer the type of the value it is returning. This is a bit of a hack to get around that.
        // E.g....
        // match "file" {
        //     "file" => {
        //         // Create a new instance of the file provider
        //         let provider = FileProvider::new();
        //         // Return the provider
        //         provider
        //     },
        //     _ => {
        //         // Create a new instance of the file provider
        //         let provider = FileProvider::new();
        //         // Return the provider
        //         provider
        //     }
        // }

        Self {}
    }

    fn meta_data(&self) -> Metadata {
        todo!()
    }

     fn evaluation<T>(&self,flag: String, default_value: T,
         eval_ctx: FlattenedContext) -> ResolutionDetails<T> where T: Copy, {
        todo!()
    }
}

// tests
