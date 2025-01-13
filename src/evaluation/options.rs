use crate::Hook;

/// Contain hooks.
#[derive(Default, Clone)]
pub struct EvaluationOptions {
    /// The hooks to be used during evaluation.
    pub hooks: Vec<crate::hooks::HookWrapper>,

    /// Hints to be passed to the hooks.
    pub hints: crate::hooks::HookHints,
}

impl EvaluationOptions {
    /// Create a new instance of `EvaluationOptions`.
    pub fn new(hooks: Vec<crate::hooks::HookWrapper>, hints: crate::hooks::HookHints) -> Self {
        Self { hooks, hints }
    }

    /// Add a hook to the evaluation options.
    #[must_use]
    pub fn with_hook<T: Hook + 'static>(mut self, hook: T) -> Self {
        self.hooks.push(crate::hooks::HookWrapper::new(hook));
        self
    }
}
