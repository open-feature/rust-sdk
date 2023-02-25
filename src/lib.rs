use std::{fmt::Error, collections::HashMap};

use anyhow::Result;
use evaluation::EvaluationOptions;

mod evaluation;
mod providers;

enum Type {
    Bool,
    String,
    Float,
    Int,
}
trait ClientTraits {

    fn meta_data(&self) -> ClientMetaData;
    fn set_evaluation_context(&mut self,eval_ctx: evaluation::EvaluationContext);
    fn evaluation_context(&self) -> evaluation::EvaluationContext;
    fn value<T>(&self,flag: String, default_value: T, eval_ctx: evaluation::EvaluationContext) -> (Result<T>, Error);
    fn value_details<T>(&self,flag: String, default_value: T, eval_ctx: evaluation::EvaluationContext) -> (EvaluationDetails<T>,Result<bool>);
}
struct Client {  
    meta_data: ClientMetaData,
    evaluation_context: evaluation::EvaluationContext,
}
#[derive(Clone)]
struct ClientMetaData {
    name: String
}
struct EvaluationDetails<T> {
    value: T,
    flag_key: String,
    flag_type: Type,
    variant: String,
    reason: String,
    error_code: String,
    error_message: String,
}
// Client impl
impl Client {
    pub fn new(meta_data: ClientMetaData, 
        evaluation_context: evaluation::EvaluationContext) -> Self {
        Self {
            meta_data: meta_data,
            evaluation_context: evaluation_context
        }
    }
    pub fn evaluate<T>(flag: String, flagType: Type, defaultValue: T,
         eval_ctx: evaluation::EvaluationContext) -> Result<String,Error> {
             
            // let eval_details = EvaluationDetails {
            //     value: defaultValue,
            //     flag_key: flag,
            //     flag_type: flagType,
            //     variant: EvaluationDetails<String>{},
            //     reason: "".to_owned(),
            //     error_code: "".to_owned(),
            //     error_message: "".to_owned(),
            // };

            let flat_ctx = evaluation::flatten_context(eval_ctx);

        
            // match flag type
            match flagType {
                Type::String => {
    
                },            
                Type::Bool => {
    
                }, 
                Type::Float => {
    
                },
                Type::Int => {
    
                },               
            }


            // Rust has some really sucky parts to it, 
            // I am not adding a trait to do this, here be dragons


       
            return Ok("".to_string());
    }
}
impl ClientTraits for Client {
    // Return metadata
    fn meta_data(&self) -> ClientMetaData {
        return self.meta_data.clone();
    }

    fn set_evaluation_context(&mut self, eval_ctx: evaluation::EvaluationContext) {
        self.evaluation_context = eval_ctx;
    }

    fn evaluation_context(&self) -> evaluation::EvaluationContext {
        return self.evaluation_context.clone();
    }

    fn value<T>(&self,flag: String, default_value: T, eval_ctx: evaluation::EvaluationContext) ->  (Result<T>, Error) {
        
        //let eval_options = EvaluationOptions{};
        // evaluate
        todo!()

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