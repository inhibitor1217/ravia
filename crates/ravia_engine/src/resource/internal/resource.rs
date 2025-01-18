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
