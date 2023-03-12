use anyhow::{Error, Ok};
use std::collections::HashMap;

use crate::{
    evaluation::EvaluationContext, providers::types::ProviderMetadata, ClientMetadata,
};

use self::types::{HookContext, HookHints};

pub mod types;

pub trait Hooks {
    fn before(&self, hook_hints: HookHints) -> anyhow::Result<EvaluationContext>;
    // fn after<T>(
    //     &self,
    //     flag_evaluation_details: EvaluationDetails<T>,
    //     hook_hints: HookHints,
    // ) -> anyhow::Result<()>;
    fn error(&self, err: Error, hook_hints: HookHints);
    fn finally(&self, hook_hints: HookHints);
}
// NoopHooks is a default implementation of the Hooks trait
pub struct NoopHooks {}

impl Hooks for NoopHooks {
    fn before(&self, hook_hints: HookHints) -> anyhow::Result<EvaluationContext> {
        let mut targeting_key = "targeting_key".to_string();
        // If hook_hints contain targeting key use that
        // else use the default targeting key
        if hook_hints.map_of_hooks.contains_key(&targeting_key) {
            targeting_key = hook_hints
                .map_of_hooks
                .get(&targeting_key)
                .unwrap()
                .to_string();
        }
        let evaluation_context = EvaluationContext::new(targeting_key, HashMap::new());
        return Ok(evaluation_context);
    }
    // fn after<T>(
    //     &self,
    //     _flag_evaluation_details: EvaluationDetails<T>,
    //     _hook_hints: HookHints,
    // ) -> anyhow::Result<()> {
    //     return Ok(());
    // }
    fn error(&self, _err: Error, _hook_hints: HookHints) {
        // do nothing
    }
    fn finally(&self, _hook_hints: HookHints) {
        // do nothing
    }
}

impl HookHints {
    pub fn new(hooks_map: HashMap<String, String>) -> HookHints {
        HookHints {
            map_of_hooks: hooks_map,
        }
    }
    pub fn value(&self, key: String) -> anyhow::Result<String> {
        // if key is null or empty, return empty string
        if key.is_empty() {
            //not found error
            return Err(Error::msg("key is empty".to_string()));
        }
        // check if the key exists in the map
        match self.map_of_hooks.contains_key(&key) {
            true => {
                // if it does, return the value
                return Ok(self.map_of_hooks.get(&key).unwrap().to_string());
            }
            false => {
                // if it doesn't, return empty string
                return Err(Error::msg("key not found".to_string()));
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
        client_meta_data: ClientMetadata,
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
    pub fn client_meta_data(&self) -> ClientMetadata {
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
        hooks::types::{HookContext, HookHints},
        providers::{types::ProviderMetadata, DEFAULT_REASON},
        ClientMetadata,
    };

    #[test]
    fn hook_hints_get_value() {
        let mut hooks_map = HashMap::new();
        hooks_map.insert("key".to_string(), "value".to_string());
        let hooks_hints = HookHints::new(hooks_map);
        let result = hooks_hints.value("key".to_string());
        assert_eq!(result.unwrap(), "value".to_string());
    }

    #[test]
    fn test_hook_hint_value() {
        let hooks_hints = HookHints::new(HashMap::new());
        let result = hooks_hints.value("key".to_string());
        assert_eq!(result.is_err(), true);
    }
    #[test]
    fn test_hook_hint_value_1() {
        let mut hooks_map = HashMap::new();
        hooks_map.insert("key".to_string(), "value".to_string());
        let hooks_hints = HookHints::new(hooks_map);
        let result = hooks_hints.value("key".to_string());
        assert_eq!(result.unwrap(), "value".to_string());
    }
    #[test]
    fn test_hookcontext_getters() {
        let provider_meta_data = ProviderMetadata {
            name: "test".to_string(),
        };

        let hook_context = HookContext::new(
            "test".to_string(),
            true,
            ClientMetadata::new("test".to_string()),
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
