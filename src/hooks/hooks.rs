use anyhow::Error;
use std::collections::HashMap;

use crate::{
    evaluation::EvaluationContext, providers::types::ProviderMetadata, ClientMetaData,
    EvaluationDetails,
};

use self::types::{HookContext, HookHints};

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

impl<T> HookContext<T>
where
    T: Clone,
{
    pub fn new(
        flag_key: String,
        default_value: T,
        client_meta_data: ClientMetaData,
        provider_meta_data: ProviderMetadata,
        evaluation_context: EvaluationContext,
    ) -> HookContext<T> {
        HookContext {
            flag_key,
            default_value,
            client_meta_data,
            provider_meta_data,
            evaluation_context,
        }
    }
    pub fn flag_key(&self) -> String {
        self.flag_key.clone()
    }
    pub fn flag_type(&self) -> String {
        std::any::type_name::<T>().to_owned()
    }
    pub fn default_value(&self) -> T {
        self.default_value.clone()
    }
    pub fn client_meta_data(&self) -> ClientMetaData {
        self.client_meta_data.clone()
    }
    pub fn provider_meta_data(&self) -> ProviderMetadata {
        self.provider_meta_data.clone()
    }
    pub fn evaluation_context(&self) -> EvaluationContext {
        self.evaluation_context.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        evaluation::EvaluationContext,
        hooks::{types::HookContext, HookHints},
        providers::{types::ProviderMetadata, DEFAULT_REASON},
        ClientMetaData,
    };

    #[test]
    fn test_hook_hint_value() {
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
    #[test]
    fn test_hookcontext_getters() {
        let provider_meta_data = ProviderMetadata {
            name: "test".to_string(),
        };

        let hook_context = HookContext::new(
            "test".to_string(),
            true,
            ClientMetaData::new("test".to_string()),
            provider_meta_data,
            EvaluationContext::new(DEFAULT_REASON.to_string(), HashMap::new()),
        );
        assert_eq!(hook_context.flag_key(), "test".to_string());
        assert_eq!(hook_context.flag_type(), "bool".to_string());
        assert_eq!(hook_context.default_value(), true);
        assert_eq!(hook_context.client_meta_data().name(), "test".to_string());
        assert_eq!(hook_context.provider_meta_data().name, "test".to_string());
    }
}
