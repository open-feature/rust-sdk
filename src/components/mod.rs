use std::collections::HashMap;

pub struct EvaluationContext {
    pub targetting_key: String,
    pub attributes: HashMap<String,String>,
}
// impl
impl EvaluationContext {
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

fn NewEvaluationContext(targetting_key: String, 
    attributes: HashMap<String,String>) -> EvaluationContext {
    EvaluationContext {
        targetting_key: targetting_key,
        attributes: attributes
    }
}