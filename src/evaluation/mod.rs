use std::collections::HashMap;

#[derive(Clone)]
pub struct EvaluationContext {
    pub targetting_key: String,
    pub attributes: HashMap<String,String>,
}

pub struct EvaluationOptions {

}

pub type FlattenedContext = HashMap<String,String>;

pub fn flatten_context(context: EvaluationContext) -> FlattenedContext {
    let mut flattened_context = HashMap::new();
    flattened_context.insert(context.targetting_key(), context.attribute(context.targetting_key()));
    for (key, value) in context.attributes() {
        flattened_context.insert(key, value);
    }
    flattened_context
}
// impl
impl EvaluationContext {
    pub fn new() -> Self {
        Self {
            targetting_key: "".to_string(),
            attributes: HashMap::new()
        }
    }
    fn attribute(&self, key: String) -> String {
        self.attributes.get(&key).unwrap().clone()
    }
    fn targetting_key(&self) -> String {
        self.targetting_key.clone()
    }
    fn attributes(&self) -> HashMap<String,String> {
        self.attributes.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::evaluation::EvaluationContext;

    #[test]
    fn test_evaluation_context_1() {
        
    }
}
