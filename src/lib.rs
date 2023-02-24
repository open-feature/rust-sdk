use std::{fmt::Error, collections::HashMap};

use anyhow::Result;

pub mod components;

enum Type {
    Bool,
    String,
    Float,
    Int,
}
trait ClientTraits {

    fn meta_data(&self) -> ClientMetaData;
    fn set_evaluation_context(&mut self,eval_ctx: components::EvaluationContext);
    fn evaluation_context(&self) -> components::EvaluationContext;
    fn value<T>(&self,flag: String, default_value: T, eval_ctx: components::EvaluationContext) -> Result<T>;
    fn value_details<T>(&self,flag: String, default_value: T, eval_ctx: components::EvaluationContext) -> (EvaluationDetails<T>,Result<bool>);
}
struct Client {  
    meta_data: ClientMetaData,
    evaluation_context: components::EvaluationContext,
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
        evaluation_context: components::EvaluationContext) -> Self {
        Self {
            meta_data: meta_data,
            evaluation_context: evaluation_context
        }
    }
    pub fn evaluate<T>(flag: String, flagType: Type, defaultValue: T,
         eval_ctx: components::EvaluationContext) -> Result<String,Error> {
             
            let eval_details = EvaluationDetails {
                value: defaultValue,
                flag_key: flag,
                flag_type: flagType,
                variant: todo!(),
                reason: todo!(),
                error_code: todo!(),
                error_message: todo!(),
            };

            let flat_ctx = components::flatten_context(eval_ctx);

        
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

    fn set_evaluation_context(&mut self, eval_ctx: components::EvaluationContext) {
        self.evaluation_context = eval_ctx;
    }

    fn evaluation_context(&self) -> components::EvaluationContext {
        return self.evaluation_context.clone();
    }

    fn value<T>(&self,flag: String, default_value: T, eval_ctx: components::EvaluationContext) -> Result<T> {
        todo!()
    }

    fn value_details<T>(&self,flag: String, default_value: T, eval_ctx: components::EvaluationContext) -> (EvaluationDetails<T>,Result<bool>) {
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
    use crate::{Client, ClientMetaData, components};

    #[test]
    fn test_set_name() {
        let client_meta_data = ClientMetaData::new("test".to_string());
        assert_eq!(client_meta_data.get_name(), "test");
    }
    #[test]
    fn test_client_impl() {
        // assert client impl
        
        let client = Client::new(ClientMetaData { name: ("test").to_string() }, 
            components::EvaluationContext{ targetting_key: todo!(), attributes: todo!() });
        assert_eq!(client.meta_data.get_name(), "test");
    }
 

}