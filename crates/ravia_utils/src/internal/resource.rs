use std::io::Read;

/// Returns a reader for a resource.
pub fn read_resource<S: AsRef<str>>(path: S) -> crate::Result<Box<dyn Read>> {
    #[cfg(target_arch = "wasm32")]
    {
        todo!()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let res_dir =
            std::env::var("RAVIA_RES").expect("unsupported platform: RAVIA_RES is not set");
        let path = std::path::Path::new(&res_dir).join(path.as_ref());
        let res = std::fs::File::open(path)?;
        Ok(Box::new(res))
    }
}
