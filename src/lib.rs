use crate::providers::traits::FeatureProvider;
use anyhow::Error;
use std::collections::HashMap;
use traits::ClientTraits;

#[path = "evaluation/evaluation.rs"]
pub mod evaluation;
#[path = "hooks/hooks.rs"]
pub mod hooks;
#[path = "providers/providers.rs"]
pub mod providers;
pub mod traits;

pub struct Client<C>
where
    C: FeatureProvider,
{
    meta_data: ClientMetaData,
    evaluation_context: evaluation::EvaluationContext,
    provider: C,
}
#[derive(Clone)]
pub struct ClientMetaData {
    pub name: String,
}
#[derive(Debug)]
pub struct EvaluationDetails<T> {
    value: T,
    flag_key: String,
    variant: String,
    reason: String,
    error_code: String,
    error_message: String,
}

impl<C> ClientTraits<C> for Client<C>
where
    C: FeatureProvider,
{
    fn new(name: String, provider: C) -> Self {
        Self {
            meta_data: ClientMetaData { name: name.clone() },
            evaluation_context: evaluation::EvaluationContext::new(name, HashMap::new()),
            provider,
        }
    }
    fn meta_data(&self) -> ClientMetaData {
        return self.meta_data.clone();
    }

    fn set_evaluation_context(&mut self, eval_ctx: evaluation::EvaluationContext) {
        self.evaluation_context = eval_ctx;
    }

    fn evaluation_context(&self) -> evaluation::EvaluationContext {
        return self.evaluation_context.clone();
    }

    fn value<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: evaluation::EvaluationContext,
    ) -> anyhow::Result<T>
    where
        T: Clone,
    {
        let result = self.evaluate(flag, default_value, eval_ctx);
        if result.is_err() {
            return Err(Error::msg("something went wrong evaluating".to_string()));
        }
        return Ok(result.unwrap().value);
    }
    fn evaluate<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: evaluation::EvaluationContext,
    ) -> anyhow::Result<EvaluationDetails<T>>
    where
        T: Clone,
    {
        let eval_default_value: T = default_value.clone();
        let mut eval_details = EvaluationDetails::<T> {
            value: eval_default_value,
            flag_key: flag.clone(),
            variant: "".to_string(),
            reason: "".to_string(),
            error_code: "".to_string(),
            error_message: "".to_string(),
        };

        let flatten_ctx = evaluation::flatten_context(eval_ctx);

        let result_default_value: T = default_value;

        let result = self
            .provider
            .evaluation::<T>(flag.clone(), result_default_value, flatten_ctx);

        let response_resolution_details = result.unwrap();

        eval_details.variant = response_resolution_details.variant;
        eval_details.reason = response_resolution_details.reason;
        eval_details.error_code = response_resolution_details.resolution_error.code;
        eval_details.error_message = response_resolution_details.resolution_error.message;
        eval_details.flag_key = flag.clone();
        if eval_details.error_code != "0" {
            return Err(Error::msg(eval_details.error_message));
        }
        return Ok(eval_details);
    }
    fn value_details<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: evaluation::EvaluationContext,
    ) -> anyhow::Result<EvaluationDetails<T>>
    where
        T: Clone,
    {
        let result = self.evaluate(flag, default_value, eval_ctx);
        if result.is_err() {
            return Err(Error::msg("something went wrong evaluating".to_string()));
        }
        return Ok(result.unwrap());
    }
}

// ClientMetaData impl
impl ClientMetaData {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    fn name(&self) -> String {
        self.name.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        providers::{self, traits::FeatureProvider},
        traits::ClientTraits,
        Client, ClientMetaData,
    };

    #[test]
    fn test_set_name_client_meta_data() {
        let client_meta_data = ClientMetaData::new("test".to_string());
        assert_eq!(client_meta_data.name(), "test");
    }

    #[test]
    fn test_evaluate_bool() {
        let client = Client::<providers::NoopProvider>::new(
            "test".to_string(),
            providers::NoopProvider::new(),
        );
        assert_eq!(client.meta_data().name(), "test");

        let mut attributes = HashMap::new();
        attributes.insert("test".to_string(), "test".to_string());

        let result = client.evaluate::<bool>("test".to_string(), true, client.evaluation_context());
        assert_eq!(result.unwrap().value, true);
    }
    #[test]
    fn test_evaluate_string() {
        let client = Client::<providers::NoopProvider>::new(
            "test".to_string(),
            providers::NoopProvider::new(),
        );
        assert_eq!(client.meta_data().name(), "test");

        client.evaluate::<String>(
            "test".to_string(),
            "test".to_string(),
            client.evaluation_context(),
        );
    }
    #[test]
    fn test_evaluate_i64() {
        let client = Client::<providers::NoopProvider>::new(
            "test".to_string(),
            providers::NoopProvider::new(),
        );
        assert_eq!(client.meta_data().name(), "test");

        let result = client.evaluate::<i64>("test".to_string(), 1, client.evaluation_context());

        assert!(result.is_ok());
    }
    #[test]
    fn test_evaluate_f64() {
        let client = Client::<providers::NoopProvider>::new(
            "test".to_string(),
            providers::NoopProvider::new(),
        );
        assert_eq!(client.meta_data().name(), "test");

        let result = client.evaluate::<f64>("test".to_string(), 1.0, client.evaluation_context());
        assert!(result.is_ok());
    }
    #[test]
    fn test_evaluate_detail() {
        let client = Client::<providers::NoopProvider>::new(
            "test".to_string(),
            providers::NoopProvider::new(),
        );
        assert_eq!(client.meta_data().name(), "test");
        let result = client.value_details::<String>(
            "test".to_string(),
            "test".to_string(),
            client.evaluation_context(),
        );
        let eval_details = result.unwrap();
        assert_eq!(eval_details.flag_key, "test");
        assert_eq!(eval_details.variant, "");
    }

    #[test]
    fn test_client_value_i64() {
        let client = Client::<providers::NoopProvider>::new(
            "test".to_string(),
            providers::NoopProvider::new(),
        );
        assert_eq!(client.meta_data().name(), "test");

        let result = client.value::<i64>("test".to_string(), 1, client.evaluation_context());
        assert_eq!(result.unwrap(), 1);
    }
    #[test]
    fn test_client_value_string() {
        let client = Client::<providers::NoopProvider>::new(
            "test".to_string(),
            providers::NoopProvider::new(),
        );
        assert_eq!(client.meta_data().name(), "test");

        let result = client.value::<String>(
            "test".to_string(),
            "test".to_string(),
            client.evaluation_context(),
        );
        assert_eq!(result.unwrap(), "test");
    }
    #[test]
    fn test_client_value_f64() {
        let client = Client::<providers::NoopProvider>::new(
            "test".to_string(),
            providers::NoopProvider::new(),
        );
        assert_eq!(client.meta_data().name(), "test");

        let result = client.value::<f64>("test".to_string(), 1.0, client.evaluation_context());
        assert_eq!(result.unwrap(), 1.0);
    }
}
