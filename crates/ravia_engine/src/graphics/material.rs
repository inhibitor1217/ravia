use crate::ecs;

#[derive(Debug, Clone, Copy)]
pub struct Material {}

assert_impl_all!(Material: ecs::storage::Component);
