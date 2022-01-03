use crate::{Node, Sequence};

#[salsa::query_group(InputsStorage)]
pub trait Inputs {
    #[salsa::input]
    fn nodes(&self) -> Sequence<Node>;
}
