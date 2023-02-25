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
    pub fn new(targetting_key: String, 
        attributes: HashMap<String,String>) -> Self {
        Self {
            targetting_key: targetting_key,
            attributes: attributes
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
        let mut attributes = HashMap::new();
        attributes.insert("key1".to_string(), "value1".to_string());
        attributes.insert("key2".to_string(), "value2".to_string());
        let evaluation_context = EvaluationContext::new("targetting_key".to_string(), attributes);
        assert_eq!(evaluation_context.targetting_key(), "targetting_key");
        assert_eq!(evaluation_context.attribute("key1".to_string()), "value1");
        assert_eq!(evaluation_context.attribute("key2".to_string()), "value2");
        assert_eq!(evaluation_context.attributes().get("key1").unwrap(), "value1");
        assert_eq!(evaluation_context.attributes().get("key2").unwrap(), "value2");
    }
}
