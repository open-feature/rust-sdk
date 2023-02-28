use crate::{evaluation, providers::traits::FeatureProvider, ClientMetaData, EvaluationDetails};

pub trait ClientTraits<C>
where
    C: FeatureProvider,
{
    fn new(name: String, provider: C) -> Self;
    fn meta_data(&self) -> ClientMetaData;
    fn set_evaluation_context(&mut self, eval_ctx: evaluation::EvaluationContext);
    fn evaluation_context(&self) -> evaluation::EvaluationContext;
    fn evaluate<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: evaluation::EvaluationContext,
    ) -> (EvaluationDetails<T>, anyhow::Error)
    where
        T: Clone;
    fn value<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: evaluation::EvaluationContext,
    ) -> (T, anyhow::Error)
    where
        T: Clone;
    fn value_details<T>(
        &self,
        flag: String,
        default_value: T,
        eval_ctx: evaluation::EvaluationContext,
    ) -> (EvaluationDetails<T>, anyhow::Error) where T: Clone;
}
