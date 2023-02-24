use anyhow::Result;

pub mod components;
trait ClientTraits {
    fn meta_data() -> ClientMetaData;
    fn set_evaluation_context(eval_ctx: components::EvaluationContext);
    fn evaluation_context() -> components::EvaluationContext;
    fn boolean_value(flag: String, default_value: bool, eval_ctx: components::EvaluationContext) -> Result<bool>;
    fn string_value(flag: String, default_value: String, eval_ctx: components::EvaluationContext) -> Result<String>;
    fn float_value(flag: String, default_value: f64, eval_ctx: components::EvaluationContext) -> Result<f64>;
    fn int_value(flag: String, default_value: i64, eval_ctx: components::EvaluationContext) -> Result<i64>;
    fn boolean_value_details(flag: String, default_value: String, eval_ctx: components::EvaluationContext) -> (bool_evaluation_details,Result<bool>);
    fn string_value_details(flag: String, default_value: String, eval_ctx: components::EvaluationContext) -> (string_evaluation_details,Result<String>);
    fn float_value_details(flag: String, default_value: f64, eval_ctx: components::EvaluationContext) -> (float_evaluation_details,Result<f64>);
    fn int_value_details(flag: String, default_value: i64, eval_ctx: components::EvaluationContext) -> (int_evaluation_details,Result<i64>);
}
struct Client {  
    meta_data: ClientMetaData,
    evaluation_context: components::EvaluationContext
}
struct ClientMetaData {
    name: String
}
struct bool_evaluation_details {
    value: bool,
    flag_key: String,
    flag_type: String,
    variant: String,
    reason: String,
    error_code: String,
    error_message: String,
}
struct string_evaluation_details {
    value: String,
    flag_key: String,
    flag_type: String,
    variant: String,
    reason: String,
    error_code: String,
    error_message: String,
}
struct float_evaluation_details {
    value: f64,
    flag_key: String,
    flag_type: String,
    variant: String,
    reason: String,
    error_code: String,
    error_message: String,
}
struct int_evaluation_details {
    value: i64,
    flag_key: String,
    flag_type: String,
    variant: String,
    reason: String,
    error_code: String,
    error_message: String,
}
// Client impl
impl ClientTraits for Client {
    fn meta_data() -> ClientMetaData {
        todo!()
    }

    fn set_evaluation_context(eval_ctx: components::EvaluationContext) {
        todo!()
    }

    fn evaluation_context() -> components::EvaluationContext {
        todo!()
    }

    fn boolean_value(flag: String, default_value: bool, eval_ctx: components::EvaluationContext) -> Result<bool> {
        todo!()
    }

    fn string_value(flag: String, default_value: String, eval_ctx: components::EvaluationContext) -> Result<String> {
        todo!()
    }

    fn float_value(flag: String, default_value: f64, eval_ctx: components::EvaluationContext) -> Result<f64> {
        todo!()
    }

    fn int_value(flag: String, default_value: i64, eval_ctx: components::EvaluationContext) -> Result<i64> {
        todo!()
    }

    fn boolean_value_details(flag: String, default_value: String, eval_ctx: components::EvaluationContext) -> (bool_evaluation_details,Result<bool>) {
        todo!()
    }

    fn string_value_details(flag: String, default_value: String, eval_ctx: components::EvaluationContext) -> (string_evaluation_details,Result<String>) {
        todo!()
    }

    fn float_value_details(flag: String, default_value: f64, eval_ctx: components::EvaluationContext) -> (float_evaluation_details,Result<f64>) {
        todo!()
    }

    fn int_value_details(flag: String, default_value: i64, eval_ctx: components::EvaluationContext) -> (int_evaluation_details,Result<i64>) {
        todo!()
    }
}

fn new_client(meta_data: ClientMetaData, evaluation_context: components::EvaluationContext) -> Client {
    Client {
        meta_data: meta_data,
        evaluation_context: evaluation_context
    }
}

// ClientMetaData impl
impl ClientMetaData {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}
fn new_client_meta_data(name: String) -> ClientMetaData {
    ClientMetaData {
        name: name
    }
}

#[cfg(test)]
mod tests {
    use crate::{new_client_meta_data, Client, components, new_client};

    #[test]
    fn test_set_name() {
        let client_meta_data = new_client_meta_data("test".to_string());
        // assert set
        assert_eq!(client_meta_data.get_name(), "test");
    }
    #[test]
    fn test_client_impl() {
        // assert client impl
        let client = Client{
            meta_data: new_client_meta_data("test".to_string()),
            evaluation_context: components::EvaluationContext{
                targetting_key: "test".to_string(),
                attributes: std::collections::HashMap::new()
            }
        };
        assert_eq!(client.meta_data.get_name(), "test");
    }
    #[test]
    fn test_client() {
        // assert client
        let client = new_client(new_client_meta_data("test".to_string()), components::EvaluationContext{
            targetting_key: "test".to_string(),
            attributes: std::collections::HashMap::new()
        });
        assert_eq!(client.meta_data.get_name(), "test");
    }

}