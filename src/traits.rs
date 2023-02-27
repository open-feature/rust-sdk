use std::fmt::Error;
use anyhow::Result;
use crate::{ClientMetaData, evaluation, EvaluationDetails};

pub trait ClientTraits {
    fn new(name: String) -> Self;
    fn meta_data(&self) -> ClientMetaData;
    fn set_evaluation_context(&mut self,eval_ctx: evaluation::EvaluationContext);
    fn evaluation_context(&self) -> evaluation::EvaluationContext;
    fn evaluate<T>(&self,flag: String, default_value: T,
        eval_ctx: evaluation::EvaluationContext) -> (EvaluationDetails<T>, Error) where T: Copy;
    fn value<T>(&self,flag: String, default_value: T, eval_ctx: evaluation::EvaluationContext) -> (EvaluationDetails<T>, Error) where T: Copy;
    fn value_details<T>(&self,flag: String, default_value: T, eval_ctx: evaluation::EvaluationContext) -> (EvaluationDetails<T>,Result<bool>);
}