use crate::Node;
use std::sync::Arc;

#[salsa::query_group(InputsStorage)]
pub trait Inputs {
    #[salsa::input]
    fn nodes(&self) -> Arc<[Node]>;
}
