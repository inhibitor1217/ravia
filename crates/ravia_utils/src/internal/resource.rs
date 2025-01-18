use std::io::Read;

/// An external resource dynamically loaded from the filesystem.
#[derive(Debug, Clone)]
pub struct Resource {
    pub path: String,
}

impl Resource {
    /// Creates a new [`Resource`] from a path.
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

/// Returns a reader for a resource.
pub async fn read_resource(res: &Resource) -> crate::Result<Box<dyn Read>> {
    #[cfg(target_arch = "wasm32")]
    {
        todo!()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let res_dir =
            std::env::var("RAVIA_RES").expect("unsupported platform: RAVIA_RES is not set");
        let path = std::path::Path::new(&res_dir).join(res.path.clone());
        let res = std::fs::File::open(path)?;
        Ok(Box::new(res))
    }
}
