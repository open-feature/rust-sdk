use crate::providers::traits::FeatureProvider;
use std::{fmt::Error, collections::HashMap};

use anyhow::Result;

use traits::ClientTraits;

mod evaluation;
mod providers;
mod traits;

pub struct Client<C> where C: FeatureProvider {  
    meta_data: ClientMetaData,
    evaluation_context: evaluation::EvaluationContext,
    provider: C,
}
#[derive(Clone)]
pub struct ClientMetaData {
    pub name: String
}
pub struct EvaluationDetails<T> {
    value:  T,
    flag_key: String,
    variant: String,
    reason: String,
    error_code: String,
    error_message: String,
}

impl<C> ClientTraits<C> for Client<C> where C: FeatureProvider {
    
    fn new(name: String, provider: C ) -> Self {
        Self {
            meta_data: ClientMetaData{ name: name.clone()},
            evaluation_context: evaluation::EvaluationContext::new(name.clone(),
                 HashMap::new()),
            provider: provider,
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

    fn value<T>(&self,flag: String, default_value: T,
         eval_ctx: evaluation::EvaluationContext) ->  (EvaluationDetails<T>, Error) where T: Copy, {

        self.evaluate::<T>(flag, default_value, eval_ctx)
       
    }
    fn evaluate<T>(&self,flag: String, default_value: T,
        eval_ctx: evaluation::EvaluationContext) -> (EvaluationDetails<T>, Error)
        where T: Copy, {

        let eval_default_value: T = default_value;
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

        let result = self.provider.evaluation::<T>(flag.clone(), result_default_value, flatten_ctx);
        
        eval_details.variant = result.varient;
        eval_details.reason = result.reason;
        eval_details.error_code = result.resolution_error.code;
        eval_details.error_message = result.resolution_error.message;

        (eval_details, Error)

    }
    fn value_details<T>(&self,flag: String, default_value: T,
         eval_ctx: evaluation::EvaluationContext) -> (EvaluationDetails<T>,Result<bool>) {
        todo!()
    }
}


// ClientMetaData impl
impl ClientMetaData {
    pub fn new(name: String) -> Self {
        Self {
            name: name
        }
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
 
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{ClientMetaData, providers::{self, traits::FeatureProvider}, Client, traits::ClientTraits, evaluation::{self, EvaluationContext}};

    #[test]
    fn test_set_name_client_meta_data() {
        let client_meta_data = ClientMetaData::new("test".to_string());
        assert_eq!(client_meta_data.get_name(), "test");
    }
    #[test]

    #[test]
    fn test_evaluate() {
        let mut client = Client::<providers::NoOProvider>::new("test".
         to_string(), providers::NoOProvider::new());
            assert_eq!(client.meta_data().get_name(), "test");

        let mut attributes = HashMap::new();
        attributes.insert("test".to_string(), "test".to_string());
        
            client.set_evaluation_context(evaluation::EvaluationContext::new("test".to_string(),
            attributes));

            let (eval_details, error) = 
            client.evaluate::<bool>("test".to_string(),
             true, client.evaluation_context());
            assert_eq!(eval_details.value, true);

    }

}