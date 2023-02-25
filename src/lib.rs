use std::{fmt::Error, collections::HashMap};

use anyhow::Result;
use evaluation::EvaluationOptions;
use providers::Provider;

mod evaluation;
mod providers;

enum Type {
    Bool,
    String,
    Float,
    Int,
}
trait ClientTraits {
    fn new(name: String) -> Self;
    fn meta_data(&self) -> ClientMetaData;
    fn set_evaluation_context(&mut self,eval_ctx: evaluation::EvaluationContext);
    fn evaluation_context(&self) -> evaluation::EvaluationContext;
    fn evaluate<T>(&self,flag: String, default_value: T,
        eval_ctx: evaluation::EvaluationContext) -> (EvaluationDetails<T>, Error);
    fn value<T>(&self,flag: String, default_value: T, eval_ctx: evaluation::EvaluationContext) -> (EvaluationDetails<T>, Error);
    fn value_details<T>(&self,flag: String, default_value: T, eval_ctx: evaluation::EvaluationContext) -> (EvaluationDetails<T>,Result<bool>);
}
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
    value: T,
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
         eval_ctx: evaluation::EvaluationContext) ->  (EvaluationDetails<T>, Error) {

        self.evaluate::<T>(flag, default_value, eval_ctx)
       
    }
    fn evaluate<T>(&self,flag: String, default_value: T,
        eval_ctx: evaluation::EvaluationContext) -> (EvaluationDetails<T>, Error){

        let eval_details = EvaluationDetails::<T> {
            value: default_value,
            flag_key: flag,
            variant: "".to_string(),
            reason: "".to_string(),
            error_code: "".to_string(),
            error_message: "".to_string(),
        };
           
        let flatten_ctx = evaluation::flatten_context(eval_ctx);

        let result = self.provider.evaluation::<T>(flag, default_value, flatten_ctx);
        


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