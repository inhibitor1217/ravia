mod internal;

pub use anyhow::{Error, Result};
pub use internal::obj::load_mesh_from_obj;
pub use internal::resource::{read_resource, Resource};
