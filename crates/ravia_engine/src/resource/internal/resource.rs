use crate::ecs;

/// An external resource dynamically loaded from the filesystem.
#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    pub path: String,
}

impl Resource {
    /// Creates a new [`Resource`] from a path.
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

assert_impl_all!(Resource: ecs::storage::Component);

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path)
    }
}
