use async_trait::async_trait;

use crate::{evaluation, providers::traits::FeatureProvider, ClientMetadata, EvaluationDetails};

#[path = "hooks/hooks.rs"]
pub mod hooks;

#[async_trait(?Send)]
pub trait Client<C>
where
    C: FeatureProvider<C>,
{
    fn add_hooks<T>(&self, hooks: T)
    where
        T: hooks::Hooks;
    async fn connect(&self);
    fn evaluate<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: evaluation::EvaluationContext,
    ) -> anyhow::Result<EvaluationDetails<T>>
    where
        T: Clone;
    fn evaluation_context(&self) -> evaluation::EvaluationContext;
    fn meta_data(&self) -> ClientMetadata;
    fn new(name: String, provider: C) -> Self;
    fn set_evaluation_context(&mut self, eval_ctx: evaluation::EvaluationContext);
    fn value<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: evaluation::EvaluationContext,
    ) -> anyhow::Result<T>
    where
        T: Clone;
    fn value_details<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: evaluation::EvaluationContext,
    ) -> anyhow::Result<EvaluationDetails<T>>
    where
        T: Clone;
}
