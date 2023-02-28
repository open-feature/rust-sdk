use crate::providers::traits::FeatureProvider;
use std::{collections::HashMap, fmt::Error};

use anyhow::Result;

use traits::ClientTraits;

pub mod evaluation;
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
            evaluation_context: evaluation::EvaluationContext::new(name,
                 HashMap::new()),
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
    ) -> (EvaluationDetails<T>, Error)
    where
        T: Copy,
    {
        self.evaluate::<T>(flag, default_value, eval_ctx)
    }
    fn evaluate<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: evaluation::EvaluationContext,
    ) -> (EvaluationDetails<T>, Error)
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

        eval_details.variant = result.varient;
        eval_details.reason = result.reason;
        eval_details.error_code = result.resolution_error.code;
        eval_details.error_message = result.resolution_error.message;

        (eval_details, Error)
    }
    fn value_details<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: evaluation::EvaluationContext,
    ) -> (EvaluationDetails<T>, Result<bool>) {
        todo!()
    }
}

// ClientMetaData impl
impl ClientMetaData {
    pub fn new(name: String) -> Self {
        Self { name }
    }
    fn get_name(&self) -> String {
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
        assert_eq!(client_meta_data.get_name(), "test");
    }

    #[test]
    fn test_evaluate_bool() {
        let  client = Client::<providers::NoOProvider>::new(
            "test".to_string(),
            providers::NoOProvider::new(),
        );
        assert_eq!(client.meta_data().get_name(), "test");

        let mut attributes = HashMap::new();
        attributes.insert("test".to_string(), "test".to_string());

        let (eval_details, error) =
            client.evaluate::<bool>("test".to_string(), true, client.evaluation_context());
        assert_eq!(eval_details.value, true);
    }
    #[test]
    fn test_evaluate_string() {
        let client = Client::<providers::NoOProvider>::new(
            "test".to_string(),
            providers::NoOProvider::new(),
        );
        assert_eq!(client.meta_data().get_name(), "test");

        client.evaluate::<String>(
            "test".to_string(),
            "test".to_string(),
            client.evaluation_context(),
        );
    }
    #[test]
    fn test_evaluate_i64() {
        let client = Client::<providers::NoOProvider>::new(
            "test".to_string(),
            providers::NoOProvider::new(),
        );
        assert_eq!(client.meta_data().get_name(), "test");

        client.evaluate::<i64>("test".to_string(), 1, client.evaluation_context());
    }
    #[test]
    fn test_evaluate_f64() {
        let client = Client::<providers::NoOProvider>::new(
            "test".to_string(),
            providers::NoOProvider::new(),
        );
        assert_eq!(client.meta_data().get_name(), "test");

        client.evaluate::<f64>("test".to_string(), 1.0, client.evaluation_context());
    }
}
