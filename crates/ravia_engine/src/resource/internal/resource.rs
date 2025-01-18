use crate::ecs;

use super::resource_manager::ResourceKey;

/// An external resource dynamically loaded from the filesystem.
///
/// If attached to an entity, the resource will be loaded by the [`super::resource_manager::ResourceManager`].
#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    pub path: String,

    pub(crate) key: Option<ResourceKey>,
}

impl Resource {
    /// Creates a new [`Resource`] from a path.
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
            key: None,
        }
    }

    /// Returns true if the resource should be requested.
    pub(crate) fn should_request(&self) -> bool {
        self.key.is_none()
    }
}

assert_impl_all!(Resource: ecs::storage::Component);
