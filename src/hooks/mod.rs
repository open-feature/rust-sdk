use std::collections::HashMap;
use anyhow::Error;

use crate::{evaluation::EvaluationContext, EvaluationDetails};

use self::types::HookHints;

pub mod types;

trait Hooks {
    fn before(hook_hints: HookHints) -> (EvaluationContext, Error);
    fn after<T>(flag_evaluation_details: EvaluationDetails<T>, hook_hints: HookHints) -> Error;
    fn error(err: Error, hook_hints: HookHints);
    fn finally(hook_hints: HookHints);
}

impl HookHints {
    fn new(hooks_map: HashMap<String, String>) -> HookHints {
        HookHints {
            map_of_hooks: hooks_map,
        }
    }   
    fn value(&self, key: String) -> String {
        // if key is null or empty, return empty string
        if key.is_empty() {
            return "".to_string();
        }
        // check if the key exists in the map
        match self.map_of_hooks.contains_key(&key) {
            true => {
                // if it does, return the value
                return self.map_of_hooks.get(&key).unwrap().to_string();
            }
            false => {
                // if it doesn't, return empty string
                return "".to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::hooks::HookHints;

    #[test]
    fn test_hook_hint_value(){

       let hooks_hints = HookHints::new(HashMap::new());
        let value = hooks_hints.value("key".to_string());
            assert_eq!(value, "".to_string());
    }
    #[test]
    fn test_hook_hint_value_1() {

        let mut hooks_map = HashMap::new();
        hooks_map.insert("key".to_string(), "value".to_string());
        let hooks_hints = HookHints::new(hooks_map);
        let value = hooks_hints.value("key".to_string());
        assert_eq!(value, "value".to_string());
    }
}