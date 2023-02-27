use crate::evaluation::FlattenedContext;

use super::types::{Metadata, ResolutionDetails};



pub trait FeatureProvider {
    fn new() -> Self;
    fn meta_data(&self) -> Metadata;
    fn evaluation<T>(&self,flag: String, default_value: T,
         eval_ctx: FlattenedContext) -> ResolutionDetails<T> where T: Copy;
}