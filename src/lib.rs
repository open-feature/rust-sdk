use crate::providers::traits::FeatureProvider;
use std::{fmt::Error};

use anyhow::Result;

use providers::Provider;
use traits::ClientTraits;

mod evaluation;
mod providers;
mod traits;

struct Client {  
    meta_data: ClientMetaData,
    evaluation_context: evaluation::EvaluationContext,
    provider: Provider,
}
#[derive(Clone)]
struct ClientMetaData {
    name: String
}
struct EvaluationDetails<T> {
    value:  T,
    flag_key: String,
    variant: String,
    reason: String,
    error_code: String,
    error_message: String,
}
 
impl ClientTraits for Client {
    
    fn new(name: String ) -> Self {
        Self {
            meta_data: ClientMetaData{ name: name},
            evaluation_context: evaluation::EvaluationContext::new(),
            provider: Provider::new(),
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
    fn value_details<T>(&self,flag: String, default_value: T, eval_ctx: evaluation::EvaluationContext) -> (EvaluationDetails<T>,Result<bool>) {
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
    use crate::{Client, ClientMetaData, evaluation};

    #[test]
    fn test_set_name() {
        let client_meta_data = ClientMetaData::new("test".to_string());
        assert_eq!(client_meta_data.get_name(), "test");
    }
    #[test]
    fn test_client_impl() {
        // test Client
        
    }
 

}