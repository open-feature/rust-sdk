use crate::evaluation::FlattenedContext;

mod noop;

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

trait FeatureProvider {
    fn new() -> Self;
    fn meta_data(&self) -> Metadata;
    fn evaluation<T>(&self,flag: String, default_value: T,
         eval_ctx: FlattenedContext) -> ResolutionDetails<T>;
}

pub struct Provider {}

impl FeatureProvider for Provider {
    fn new() -> Self {
        // Rust has a real lack of OO programming which means we have a single provider type that has
        // to infer the type of the value it is returning. This is a bit of a hack to get around that.
        Self {}
    }

    fn meta_data(&self) -> Metadata {
        todo!()
    }

    fn evaluation<T>(&self,flag: String, default_value: T,
         eval_ctx: FlattenedContext) -> ResolutionDetails<T> {
        todo!()
    }
}
struct Metadata {
    name: String,
}

struct ResolutionError {
    code:    String,
    message: String
}
struct ResolutionDetails<T> {
    value: T,
    resolution_error: ResolutionError,
	reason:          String,
	varient:         String
}

// tests
#[cfg(test)]
mod tests {
    use crate::providers::noop::NoOPProvider;
    use crate::providers::FeatureProvider;

    #[test]
    fn test_noop_provider() {
        let noop_provider = NoOPProvider {};
        assert_eq!(noop_provider.meta_data().name, "NoOPProvider");
    }
}