use std::io::Read;

use super::{
    error::{Error, Result},
    resource::Resource,
};

/// Resource manager handles loading external resources from filesystem or the web
/// and caching them for reuse.
#[derive(Debug)]
pub struct ResourceManager {
    #[cfg(not(target_arch = "wasm32"))]
    resource_root: std::path::PathBuf,
}

impl ResourceManager {
    /// Creates a new [`ResourceManager`].
    pub fn new() -> Self {
        #[cfg(target_arch = "wasm32")]
        {
            Self {}
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let root = std::env::var("RAVIA_RES").expect("RAVIA_RES is not set");
            let root = std::path::PathBuf::from(&root);

            Self {
                resource_root: root,
            }
        }
    }

    /// Loads resource and provide it as an [`std::io::Read`] stream.
    pub async fn load(&self, res: &Resource) -> Result<Box<dyn Read>> {
        #[cfg(target_arch = "wasm32")]
        {
            todo!()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.load_from_filesystem(res).await
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn load_from_filesystem(&self, res: &Resource) -> Result<Box<dyn Read>> {
        log::info!("loading resource from filesystem: {}", res);

        let path = self.resource_root.join(&res.path);
        match std::fs::File::open(path) {
            Ok(file) => Ok(Box::new(file)),
            Err(_) => Err(Error::ResourceNotFound(res.clone())),
        }
    }
}
