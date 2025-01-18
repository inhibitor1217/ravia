use super::{error::Result, resource::Resource};

/// Resource manager handles loading external resources from filesystem or the web
/// and caching them for reuse.
#[derive(Debug, Default)]
pub struct ResourceManager {}

impl ResourceManager {
    /// Creates a new [`ResourceManager`].
    pub fn new() -> Self {
        Self {}
    }

    /// Loads resource and provide it as an [`std::io::Read`] stream.
    pub async fn load(&self, res: &Resource) -> Result<Box<dyn std::io::Read>> {
        todo!()
    }
}
