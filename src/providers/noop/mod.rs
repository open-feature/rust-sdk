use std::fmt::Error;

use crate::evaluation::FlattenedContext;

use super::{FeatureProvider, Metadata, DefaultReason};


pub struct NoOPProvider {

}

impl FeatureProvider for NoOPProvider {
    fn meta_data(&self) -> Metadata {
        Metadata {
            name: "NoOPProvider".to_string(),
        }
    }

    fn evaluation<T>(&self, flag: String, default_value: T, 
        val_ctx: FlattenedContext) -> super::ResolutionDetails<T> {
        return super::ResolutionDetails {
            value: default_value,
            resolution_error: super::ResolutionError {
                code: "0".to_string(),
                message: "NoOPProvider".to_string(),
            },
            reason: "DEFAULT".to_string(),
            varient: DefaultReason.to_owned(),
        };
    }
}


#[cfg(test)]
mod tests {
    use crate::providers::noop::NoOPProvider;
    use crate::providers::FeatureProvider;


    #[test]
    fn test_noop_provider() {
        let noop_provider = NoOPProvider {};
        assert_eq!(noop_provider.meta_data().name, "NoOPProvider");
    }
    #[test]
    fn test_evaluation_bool() {
        let noop_provider = NoOPProvider {};

        let result = noop_provider.evaluation("test".to_string(), true, Default::default());

        assert_eq!(result.value, true);
    }
    #[test]
    fn test_evaluation_i64() {
        let noop_provider = NoOPProvider {};

        let result = noop_provider.evaluation("test".to_string(), 1, Default::default());

        assert_eq!(result.value, 1);
    }
    #[test]
    fn test_evaluation_string() {
        let noop_provider = NoOPProvider {};

        let result = noop_provider.evaluation("test".to_string(), "test".to_string(), Default::default());

        assert_eq!(result.value, "test");
    }
 }