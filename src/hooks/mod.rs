use std::collections::HashMap;

use anyhow::Error;

use crate::{evaluation::EvaluationContext, EvaluationDetails};

trait Hooks {
    fn Before(hook_hints: HookHints) -> (EvaluationContext, Error);
    fn After<T>(flag_evaluation_details: EvaluationDetails<T>, hook_hints: HookHints) -> Error;
    fn Error(err: Error, hook_hints: HookHints);
    fn Finally(hook_hints: HookHints);
}

struct HookHints {
    map_of_hooks: HashMap<String, String>,
}

impl HookHints {
    fn new(hooks_map: HashMap<String, String>) -> HookHints {
        HookHints {
            map_of_hooks: hooks_map,
        }
    }
    fn Value<T>(&self, key: String) -> (T, Error) {
        todo!()
    }
}