type Reason = String;
// DefaultReason - the resolved value was configured statically, or otherwise fell back to a pre-configured value.
const DefaultReason: Reason = "DEFAULT";
const TargetingMatchReason: Reason = "TARGETING_MATCH";
// SplitReason - the resolved value was the result of pseudorandom assignment.
const SplitReason: Reason = "SPLIT";
// DisabledReason - the resolved value was the result of the flag being disabled in the management system.
const DisabledReason: Reason = "DISABLED";
// StaticReason - the resolved value is static (no dynamic evaluation)
const StaticReason: Reason = "STATIC";
// CachedReason - the resolved value was retrieved from cache
const CachedReason: Reason = "CACHED";
// UnknownReason - the reason for the resolved value could not be determined.	
const UnknownReason: Reason = "UNKNOWN";
// ErrorReason - the resolved value was the result of an error.
const ErrorReason: Reason = "ERROR";

const TargetingKey: String = "targetingKey"; // eva

trait FeatureProvider {
    
}